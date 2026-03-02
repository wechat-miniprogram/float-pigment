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
        debug_assert!(new_stride > self.stride, "stride can only grow");
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
                if src != dst {
                    self.bits[src] = 0;
                }
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

            // Move to the next line.
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
    /// Number of rows with `auto` sizing function
    row_auto_count: usize,
    /// Number of columns with `auto` sizing function
    column_auto_count: usize,
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
        row_auto_count: usize,
        column_auto_count: usize,
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
            row_auto_count,
            column_auto_count,
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

    /// Check if a cell is occupied.
    #[allow(unused)]
    pub(crate) fn is_occupied(&self, row: usize, column: usize) -> bool {
        self.occupied.get(row, column)
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

    #[inline(always)]
    pub(crate) fn row_auto_count(&self) -> usize {
        self.row_auto_count
    }

    #[inline(always)]
    pub(crate) fn column_auto_count(&self) -> usize {
        self.column_auto_count
    }

    #[inline(always)]
    pub(crate) fn explicit_row_count(&self) -> usize {
        self.explicit_row_count
    }

    #[inline(always)]
    pub(crate) fn explicit_column_count(&self) -> usize {
        self.explicit_column_count
    }

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
    /// Row gap (CSS `row-gap`), stored for `get_row_size` to exclude gap from track size.
    row_gap: T::Length,
    /// Column gap (CSS `column-gap`), stored for `get_column_size` to exclude gap from track size.
    column_gap: T::Length,
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
            row_gap: T::Length::zero(),
            column_gap: T::Length::zero(),
        }
    }

    #[inline(always)]
    pub(crate) fn row_count(&self) -> usize {
        self.row_count
    }

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
        self.row_gap = gap;
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
        self.column_gap = gap;
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

    /// Get the height of a row.
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn get_row_size(&self, row: usize) -> T::Length {
        if row + 1 < self.row_offsets.len() {
            let diff = self.row_offsets[row + 1] - self.row_offsets[row];
            // Non-last rows have gap baked into the offset difference.
            if row + 1 < self.row_count {
                diff - self.row_gap
            } else {
                diff
            }
        } else {
            T::Length::zero()
        }
    }

    /// Get the width of a column.
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn get_column_size(&self, column: usize) -> T::Length {
        if column + 1 < self.column_offsets.len() {
            let diff = self.column_offsets[column + 1] - self.column_offsets[column];
            // Non-last columns have gap baked into the offset difference.
            if column + 1 < self.column_count {
                diff - self.column_gap
            } else {
                diff
            }
        } else {
            T::Length::zero()
        }
    }
}

use float_pigment_css::num_traits::Zero;
