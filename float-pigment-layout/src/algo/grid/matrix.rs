//! Grid Matrix Data Structure
//!
//! CSS Grid Layout Module Level 1 - §7.1 The Explicit Grid
//! <https://www.w3.org/TR/css-grid-1/#explicit-grids>
//!
//! This module provides the matrix data structure for storing grid items
//! and managing the grid's row/column structure during layout computation.
//!
//! ## Optimization
//!
//! Uses a bitmap for occupancy tracking:
//! - O(1) bit-level lookup for occupied cells
//! - Cache-friendly: sequential bytes for dense scanning along auto-flow direction
//! - Items stored in Vec for efficient iteration

use core::fmt::Debug;
use float_pigment_css::num_traits::Zero;

use alloc::vec::Vec;
use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::grid_item::{GridItem, GridLayoutItem},
    LayoutTreeNode,
};

/// Bitmap-based occupancy tracker for the grid.
///
/// Stores one bit per cell, laid out so that the auto-flow scanning
/// direction corresponds to sequential bits (cache-friendly).
///
/// - `row_order = true` (row flow): `bit_index = row * stride + col`, stride = column count
/// - `row_order = false` (column flow): `bit_index = col * stride + row`, stride = row count
///
/// The stride can grow dynamically via `grow_stride()` when items are placed
/// beyond the current fixed dimension.
#[derive(Clone, PartialEq)]
pub(crate) struct OccupiedBitmap {
    /// Bit storage, each u8 holds 8 cells.
    bits: Vec<u8>,
    /// The fixed-dimension length.
    /// For row flow: stride = column count; for column flow: stride = row count.
    /// Stride always >= 1
    stride: usize,
    /// Bytes per line, always `(stride + 7) / 8`.
    bytes_per_line: usize,
    /// Whether the bitmap is in row order
    row_order: bool,
}

impl OccupiedBitmap {
    /// Create a new bitmap.
    ///
    /// `stride` is the fixed dimension length and `row_order` indicates
    /// whether bits are laid out row-first (row flow) or column-first (column flow).
    fn new(stride: usize, row_order: bool, capacity: usize) -> Self {
        debug_assert!(stride >= 1);
        let stride: usize = stride.max(1);
        let bytes_per_line = (stride + 7) / 8;
        let estimated_lines = if stride > 0 {
            (capacity + stride - 1) / stride
        } else {
            0
        };
        let total_bytes = estimated_lines * bytes_per_line;
        Self {
            bits: alloc::vec![0u8; total_bytes],
            stride,
            bytes_per_line,
            row_order,
        }
    }

    /// Compute the byte index and bit position from (row, col).
    #[inline(always)]
    fn byte_and_bit(&self, row: usize, col: usize) -> (usize, usize) {
        let (line, offset) = if self.row_order {
            (row, col)
        } else {
            (col, row)
        };
        let byte_idx = line * self.bytes_per_line + offset / 8;
        let bit_idx = offset % 8;
        (byte_idx, bit_idx)
    }

    /// Ensure the bitmap has enough capacity for the given byte index.
    #[inline]
    fn ensure_capacity(&mut self, byte_idx: usize) {
        let required_bytes = byte_idx + 1;
        if required_bytes > self.bits.len() {
            self.bits.resize(required_bytes, 0u8);
        }
    }

    /// Mark the cell at (row, col) as occupied.
    #[inline]
    fn set(&mut self, row: usize, col: usize) {
        let (byte, bit) = self.byte_and_bit(row, col);
        self.ensure_capacity(byte);
        self.bits[byte] |= 1u8 << bit;
    }

    /// Check if the cell at (row, col) is occupied.
    #[cfg(test)]
    #[inline]
    fn get(&self, row: usize, col: usize) -> bool {
        let (byte, bit) = self.byte_and_bit(row, col);
        if byte >= self.bits.len() {
            return false;
        }
        (self.bits[byte] & (1u8 << bit)) != 0
    }

    /// Grow the stride to `new_stride` in-place.
    ///
    /// When an item is placed beyond the current fixed dimension,
    /// the stride must grow to accommodate it. Since each line is
    /// byte-aligned, the bit layout within each byte is unchanged —
    /// only the line start offsets shift. We resize the Vec and then
    /// move line data from back to front so that no unread data is
    /// overwritten (new_start >= old_start for every line).
    ///
    /// `max_lines` is the number of lines already occupied (tracked by
    /// GridMatrix). The bitmap is resized to `max_lines * new_bytes_per_line`;
    /// any further growth is handled by `ensure_capacity()` in `set()`.
    fn grow_stride(&mut self, new_stride: usize, max_lines: usize) {
        debug_assert!(new_stride >= 1);
        debug_assert!(new_stride > self.stride, "stride can only grow");
        let new_stride: usize = new_stride.max(1);
        let old_bytes_per_line = self.bytes_per_line;
        let new_bytes_per_line = (new_stride + 7) / 8;
        let new_total_bytes = max_lines * new_bytes_per_line;

        self.bits.resize(new_total_bytes, 0u8);

        // Move line data from back to front so we never overwrite unread data.
        // For every line: new_start = line * new_bytes_per_line >= line * old_bytes_per_line = old_start,
        // so reverse iteration is safe.
        for line_idx in (0..max_lines).rev() {
            let old_start = line_idx * old_bytes_per_line;
            let new_start = line_idx * new_bytes_per_line;
            // Copy preserved bytes in reverse order within the line.
            for byte_idx in (0..old_bytes_per_line).rev() {
                let src = old_start + byte_idx;
                let dst = new_start + byte_idx;
                debug_assert!(src < self.bits.len(), "src out of bounds");
                debug_assert!(dst < self.bits.len(), "dst out of bounds");
                self.bits[dst] = self.bits[src];
            }
            // Zero the new trailing bytes introduced by the wider stride.
            for byte_idx in old_bytes_per_line..new_bytes_per_line {
                self.bits[new_start + byte_idx] = 0;
            }
        }

        self.stride = new_stride;
        self.bytes_per_line = new_bytes_per_line;
    }

    /// Find the first unoccupied cell starting from `(start_line, start_offset)`.
    /// Returns `(line, offset)` of the first zero bit.
    fn find_first_zero(&self, start_line_idx: usize) -> (usize, usize) {
        let mut line_idx = start_line_idx;

        loop {
            let total_bytes = self.bits.len();
            let line_start_byte_idx = line_idx * self.bytes_per_line;
            let line_end_byte_idx = line_start_byte_idx + self.bytes_per_line;

            let mut byte_idx = line_start_byte_idx;
            let mut offset = 0usize;

            while byte_idx < line_end_byte_idx {
                if byte_idx >= total_bytes {
                    return (line_idx, offset);
                }
                let b = self.bits[byte_idx];
                if b != 0xFF {
                    let bit_pos = b.trailing_ones() as usize;
                    let candidate = offset + bit_pos;
                    if candidate < self.stride {
                        return (line_idx, candidate);
                    }
                    break;
                }
                byte_idx += 1;
                offset += 8;
            }
            line_idx += 1;
        }
    }

    /// Return the current stride value.
    #[inline(always)]
    fn stride(&self) -> usize {
        self.stride
    }
}

/// The grid matrix stores grid items during the placement phase.
///
/// CSS Grid §7.1: The grid is a two-dimensional structure with:
/// - Explicit grid: Defined by `grid-template-rows` and `grid-template-columns`
/// - Implicit grid: Automatically created when items overflow the explicit grid
///
/// Uses a bitmap for occupancy tracking - O(1) bit-level lookup and
/// cache-friendly sequential access along the auto-flow direction.
#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    /// Bitmap tracking occupied cell positions
    occupied: OccupiedBitmap,
    /// Maximum row index + 1 (tracks grid boundary)
    max_row: usize,
    /// Maximum column index + 1 (tracks grid boundary)
    max_col: usize,
    /// The grid items stored separately for efficient iteration
    items: Vec<GridItem<'a, T>>,
    /// Minimum row count from explicit grid template
    explicit_row_count: usize,
    /// Minimum column count from explicit grid template
    explicit_column_count: usize,
    /// The auto-placement flow direction
    flow: GridAutoFlow,
}

impl<'a, 'b: 'a, T: LayoutTreeNode> GridMatrix<'a, T> {
    /// Create a new empty grid matrix.
    ///
    /// The grid starts empty and expands dynamically when items are placed.
    /// The bitmap layout is chosen based on `flow`:
    /// - Row/RowDense: row order, stride = column count
    /// - Column/ColumnDense: column order, stride = row count
    pub(crate) fn new(
        explicit_row_count: usize,
        explicit_column_count: usize,
        flow: GridAutoFlow,
        capacity: usize,
    ) -> Self {
        let (row_order, stride) = match flow {
            GridAutoFlow::Row | GridAutoFlow::RowDense => (true, explicit_column_count.max(1)),
            GridAutoFlow::Column | GridAutoFlow::ColumnDense => (false, explicit_row_count.max(1)),
        };
        Self {
            occupied: OccupiedBitmap::new(stride, row_order, capacity),
            max_row: 0,
            max_col: 0,
            items: Vec::with_capacity(capacity),
            explicit_row_count,
            explicit_column_count,
            flow,
        }
    }

    /// Place an item at the specified position, expanding the grid if needed.
    ///
    /// This method:
    /// 1. Grows the bitmap stride if the item exceeds the current fixed dimension
    /// 2. Marks the cell as occupied in the bitmap
    /// 3. Updates grid boundaries
    /// 4. Adds the item to the items Vec
    pub(crate) fn place_item(&mut self, row: usize, column: usize, item: GridItem<'a, T>) {
        // Check if stride needs to grow for implicit grid tracks.
        // Row flow: stride = column count, so check if col exceeds stride.
        // Column flow: stride = row count, so check if row exceeds stride.
        let stride = if self.occupied.row_order {
            column + 1
        } else {
            row + 1
        };
        if stride > self.occupied.stride() {
            let max_lines = if self.occupied.row_order {
                self.max_row
            } else {
                self.max_col
            };
            self.occupied.grow_stride(stride, max_lines);
        }

        // Mark cell as occupied
        self.occupied.set(row, column);
        // Update boundaries
        self.max_row = self.max_row.max(row + 1);
        self.max_col = self.max_col.max(column + 1);
        // Store item in the Vec
        self.items.push(item);
    }

    /// Find the first unoccupied cell starting from the given hint line.
    pub(crate) fn find_first_unoccupied(&self, hint_line: &mut usize) -> (usize, usize) {
        let (line, offset) = self.occupied.find_first_zero(*hint_line);
        if line > *hint_line {
            *hint_line = line;
        }
        if self.occupied.row_order {
            (line, offset)
        } else {
            (offset, line)
        }
    }

    /// Get an iterator over all placed items.
    pub(crate) fn items(&self) -> impl Iterator<Item = &GridItem<'a, T>> {
        self.items.iter()
    }

    /// Get a mutable iterator over all placed items.
    pub(crate) fn items_mut(&mut self) -> impl Iterator<Item = &mut GridItem<'a, T>> {
        self.items.iter_mut()
    }

    /// Returns the current number of rows (may exceed explicit grid).
    #[inline(always)]
    pub(crate) fn row_count(&self) -> usize {
        self.max_row
    }

    /// Returns the current number of columns (may exceed explicit grid).
    #[inline(always)]
    pub(crate) fn column_count(&self) -> usize {
        self.max_col
    }

    /// Returns the explicit row count defined by grid-template-rows.
    #[inline(always)]
    pub(crate) fn explicit_row_count(&self) -> usize {
        self.explicit_row_count
    }

    /// Returns the explicit column count defined by grid-template-columns.
    #[inline(always)]
    pub(crate) fn explicit_column_count(&self) -> usize {
        self.explicit_column_count
    }

    /// Returns the auto-placement flow direction for this grid.
    #[inline(always)]
    pub(crate) fn flow(&self) -> GridAutoFlow {
        self.flow.clone()
    }
}

impl<'a, T: LayoutTreeNode> Debug for GridMatrix<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GridMatrix {{ row_count: {}, column_count: {}, explicit: {}x{}, items: {} }}",
            self.row_count(),
            self.column_count(),
            self.explicit_row_count,
            self.explicit_column_count,
            self.items.len()
        )
    }
}

/// The layout matrix stores computed layout information for each grid cell.
///
/// This is used in the final positioning phase after track sizes have been
/// determined. It stores the actual computed sizes and positions of items.
///
/// Uses precomputed cumulative offsets for efficient position lookup during
/// item positioning.
pub(crate) struct GridLayoutMatrix<'a, T: LayoutTreeNode> {
    /// Precomputed cumulative row offsets: row_offsets[i] = sum of row heights [0..i) + gaps
    row_offsets: Vec<T::Length>,
    /// Precomputed cumulative column offsets: column_offsets[j] = sum of column widths [0..j) + gaps
    column_offsets: Vec<T::Length>,
    /// The layout items with computed sizes
    items: Vec<GridLayoutItem<'a, T>>,
    /// Number of rows
    row_count: usize,
    /// Number of columns
    column_count: usize,
}

impl<'a, T: LayoutTreeNode> GridLayoutMatrix<'a, T> {
    /// Create a new layout matrix with the given dimensions.
    pub(crate) fn new(row_count: usize, column_count: usize, capacity: usize) -> Self {
        Self {
            row_offsets: vec![T::Length::zero(); row_count + 1],
            column_offsets: vec![T::Length::zero(); column_count + 1],
            items: Vec::with_capacity(capacity),
            row_count,
            column_count,
        }
    }

    /// Returns the number of rows in the layout matrix.
    #[inline(always)]
    pub(crate) fn row_count(&self) -> usize {
        self.row_count
    }

    /// Returns the number of columns in the layout matrix.
    #[inline(always)]
    pub(crate) fn column_count(&self) -> usize {
        self.column_count
    }

    /// Add a layout item.
    pub(crate) fn add_item(&mut self, item: GridLayoutItem<'a, T>) {
        self.items.push(item);
    }

    /// Get an iterator over all layout items.
    pub(crate) fn items(&self) -> impl Iterator<Item = &GridLayoutItem<'a, T>> {
        self.items.iter()
    }

    /// Get a mutable iterator over all layout items.
    pub(crate) fn items_mut(&mut self) -> impl Iterator<Item = &mut GridLayoutItem<'a, T>> {
        self.items.iter_mut()
    }

    /// Set the row sizes and compute cumulative offsets.
    ///
    /// After calling this, `get_row_offset(i)` returns the y position of row i.
    pub(crate) fn set_row_sizes(&mut self, sizes: &[T::Length], gap: T::Length) {
        use float_pigment_css::num_traits::Zero;
        let mut offset = T::Length::zero();
        self.row_offsets[0] = offset;
        for (i, &size) in sizes.iter().enumerate() {
            offset += size;
            if i < sizes.len() - 1 {
                offset += gap;
            }
            self.row_offsets[i + 1] = offset;
        }
    }

    /// Set the column sizes and compute cumulative offsets.
    ///
    /// After calling this, `get_column_offset(j)` returns the x position of column j.
    pub(crate) fn set_column_sizes(&mut self, sizes: &[T::Length], gap: T::Length) {
        use float_pigment_css::num_traits::Zero;
        let mut offset = T::Length::zero();
        self.column_offsets[0] = offset;
        for (i, &size) in sizes.iter().enumerate() {
            offset += size;
            if i < sizes.len() - 1 {
                offset += gap;
            }
            self.column_offsets[i + 1] = offset;
        }
    }

    /// Get the y offset for a row (O(1) lookup).
    #[inline(always)]
    pub(crate) fn get_row_offset(&self, row: usize) -> T::Length {
        self.row_offsets[row]
    }

    /// Get the x offset for a column (O(1) lookup).
    #[inline(always)]
    pub(crate) fn get_column_offset(&self, column: usize) -> T::Length {
        self.column_offsets[column]
    }
}

#[cfg(test)]
mod tests {
    use super::OccupiedBitmap;

    #[test]
    fn set_get_basic_row_order() {
        // 3-column grid, row order (row flow)
        let mut bm = OccupiedBitmap::new(3, true, 9);
        bm.set(0, 0);
        bm.set(1, 2);
        bm.set(2, 1);
        assert!(bm.get(0, 0));
        assert!(bm.get(1, 2));
        assert!(bm.get(2, 1));
    }

    #[test]
    fn get_unset_returns_false() {
        // Unoccupied cells must return false so auto-placement can use them.
        let bm = OccupiedBitmap::new(4, true, 16);
        for r in 0..4 {
            for c in 0..4 {
                assert!(!bm.get(r, c), "cell ({r},{c}) should be empty");
            }
        }
    }

    #[test]
    fn set_get_column_order() {
        // Column flow — bitmap is in column order.
        // bit_index = col * stride + row, stride = row count
        let mut bm = OccupiedBitmap::new(3, false, 9);
        bm.set(2, 0); // row=2, col=0 → line=0(col=0), offset=2(row=2)
        bm.set(0, 1); // row=0, col=1 → line=1(col=1), offset=0(row=0)
        assert!(bm.get(2, 0));
        assert!(bm.get(0, 1));
        assert!(!bm.get(0, 0));
        assert!(!bm.get(2, 1));
    }

    // --- Auto-expand (ensure_capacity) ---
    // Implicit grid tracks are created on-the-fly.

    #[test]
    fn auto_expand_beyond_initial_capacity() {
        // Start with capacity for 4 cells but write far beyond.
        let mut bm = OccupiedBitmap::new(2, true, 4);
        bm.set(10, 0);
        bm.set(10, 1);
        assert!(bm.get(10, 0));
        assert!(bm.get(10, 1));
        // Earlier rows still empty.
        assert!(!bm.get(0, 0));
    }

    // --- grow_stride ---
    // When items exceed the fixed dimension,
    // the grid expands and existing data must be preserved.

    #[test]
    fn grow_stride_preserves_existing_data() {
        // 2-column row-order grid
        let mut bm = OccupiedBitmap::new(2, true, 6);
        bm.set(0, 0);
        bm.set(0, 1);
        bm.set(1, 0);
        bm.set(2, 1);
        // Grow to 5 columns (max_lines = 3)
        bm.grow_stride(5, 3);
        assert_eq!(bm.stride(), 5);
        // All previous data intact.
        assert!(bm.get(0, 0));
        assert!(bm.get(0, 1));
        assert!(bm.get(1, 0));
        assert!(bm.get(2, 1));
        // New columns are empty.
        assert!(!bm.get(0, 2));
        assert!(!bm.get(0, 3));
        assert!(!bm.get(0, 4));
    }

    #[test]
    fn grow_stride_allows_writing_new_columns() {
        let mut bm = OccupiedBitmap::new(2, true, 4);
        bm.set(0, 0);
        bm.set(1, 1);
        bm.grow_stride(4, 2);
        bm.set(0, 3);
        bm.set(1, 2);
        assert!(bm.get(0, 0));
        assert!(bm.get(0, 3));
        assert!(bm.get(1, 1));
        assert!(bm.get(1, 2));
        assert!(!bm.get(0, 2));
    }

    #[test]
    fn grow_stride_column_order() {
        // column order: stride = row count
        let mut bm = OccupiedBitmap::new(2, false, 6);
        bm.set(0, 0); // line=0(col=0), offset=0(row=0)
        bm.set(1, 1); // line=1(col=1), offset=1(row=1)
                      // Grow stride (more rows)
        bm.grow_stride(4, 2);
        assert!(bm.get(0, 0));
        assert!(bm.get(1, 1));
        assert!(!bm.get(2, 0));
        // Write in new row slots.
        bm.set(3, 0);
        assert!(bm.get(3, 0));
    }

    // --- find_first_zero ---
    // Auto-placement scans left-to-right, top-to-bottom
    // (row flow) or top-to-bottom, left-to-right (column flow).

    #[test]
    fn find_first_zero_empty_bitmap() {
        let bm = OccupiedBitmap::new(3, true, 9);
        // Completely empty — first zero is (0, 0).
        assert_eq!(bm.find_first_zero(0), (0, 0));
    }

    #[test]
    fn find_first_zero_partially_filled() {
        let mut bm = OccupiedBitmap::new(4, true, 16);
        bm.set(0, 0);
        bm.set(0, 1);
        // First zero at offset 2 in line 0.
        assert_eq!(bm.find_first_zero(0), (0, 2));
    }

    #[test]
    fn find_first_zero_skips_full_line() {
        let mut bm = OccupiedBitmap::new(3, true, 9);
        bm.set(0, 0);
        bm.set(0, 1);
        bm.set(0, 2);
        // Line 0 fully occupied → should return (1, 0).
        assert_eq!(bm.find_first_zero(0), (1, 0));
    }

    #[test]
    fn find_first_zero_with_start_line() {
        let mut bm = OccupiedBitmap::new(3, true, 9);
        bm.set(0, 0);
        bm.set(1, 0);
        bm.set(1, 1);
        // Start scanning from line 1 → first zero at (1, 2).
        assert_eq!(bm.find_first_zero(1), (1, 2));
    }

    #[test]
    fn find_first_zero_beyond_allocated() {
        // Bitmap only has 1 line allocated, search from line 5.
        let bm = OccupiedBitmap::new(4, true, 4);
        // Line 5 has no allocated bytes → first zero at (5, 0).
        assert_eq!(bm.find_first_zero(5), (5, 0));
    }

    #[test]
    fn find_first_zero_stride_gt_8() {
        // Stride > 8 to exercise multi-byte scanning.
        let mut bm = OccupiedBitmap::new(10, true, 20);
        // Fill first 9 columns of row 0.
        for c in 0..9 {
            bm.set(0, c);
        }
        // First zero in row 0 is at column 9.
        assert_eq!(bm.find_first_zero(0), (0, 9));
    }

    #[test]
    fn find_first_zero_full_byte_boundary() {
        // Stride = 8, fill entire first row (exactly 1 byte = 0xFF).
        let mut bm = OccupiedBitmap::new(8, true, 16);
        for c in 0..8 {
            bm.set(0, c);
        }
        // Row 0 full → (1, 0).
        assert_eq!(bm.find_first_zero(0), (1, 0));
    }

    // --- Edge case: stride = 1 ---

    #[test]
    fn single_column_grid() {
        // stride = 1: each line is 1 bit, 1 byte per line.
        let mut bm = OccupiedBitmap::new(1, true, 4);
        bm.set(0, 0);
        bm.set(1, 0);
        assert_eq!(bm.find_first_zero(0), (2, 0));
    }

    // --- grow_stride across byte boundary ---

    #[test]
    fn grow_stride_across_byte_boundary() {
        // Grow from stride=7 (1 byte/line) to stride=10 (2 bytes/line).
        let mut bm = OccupiedBitmap::new(7, true, 14);
        bm.set(0, 6);
        bm.set(1, 3);
        bm.grow_stride(10, 2);
        assert!(bm.get(0, 6));
        assert!(bm.get(1, 3));
        assert!(!bm.get(0, 7));
        assert!(!bm.get(1, 9));
        // Write new columns beyond old stride.
        bm.set(0, 9);
        assert!(bm.get(0, 9));
    }
}
