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
//! Uses HashSet for occupancy tracking instead of a 2D array:
//! - O(1) lookup for occupied cells
//! - O(N) space where N = number of items (not R × C)
//! - Items stored in Vec for efficient iteration

use hashbrown::HashSet;
use core::fmt::Debug;

use alloc::vec::Vec;
use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::grid_item::{GridItem, GridLayoutItem},
    LayoutTreeNode,
};

/// The grid matrix stores grid items during the placement phase.
///
/// CSS Grid §7.1: The grid is a two-dimensional structure with:
/// - Explicit grid: Defined by `grid-template-rows` and `grid-template-columns`
/// - Implicit grid: Automatically created when items overflow the explicit grid
///
/// Uses HashSet for occupancy tracking - more efficient for sparse grids
/// where most cells are empty.
#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    /// Set of occupied cell positions (row, column)
    occupied: HashSet<(usize, usize)>,
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
    /// This avoids pre-allocating cells that may not be needed (e.g., when
    /// all children are absolutely positioned or display:none).
    pub(crate) fn new(
        explicit_row_count: usize,
        explicit_column_count: usize,
        row_auto_count: usize,
        column_auto_count: usize,
        flow: GridAutoFlow,
    ) -> Self {
        Self {
            occupied: HashSet::new(),
            max_row: 0,
            max_col: 0,
            items: Vec::new(),
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
    /// 1. Marks the cell as occupied in the HashSet
    /// 2. Updates grid boundaries
    /// 3. Adds the item to the items Vec
    pub(crate) fn place_item(&mut self, row: usize, column: usize, item: GridItem<'a, T>) {
        // Mark cell as occupied
        self.occupied.insert((row, column));
        // Update boundaries
        self.max_row = self.max_row.max(row + 1);
        self.max_col = self.max_col.max(column + 1);
        // Store item in the Vec
        self.items.push(item);
    }

    /// Check if a cell is occupied.
    pub(crate) fn is_occupied(&self, row: usize, column: usize) -> bool {
        self.occupied.contains(&(row, column))
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
    /// Precomputed cumulative row offsets: row_offsets[i] = sum of row heights [0..i)
    row_offsets: Vec<T::Length>,
    /// Precomputed cumulative column offsets: column_offsets[j] = sum of column widths [0..j)
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
    pub(crate) fn new(row_count: usize, column_count: usize) -> Self {
        Self {
            row_offsets: vec![T::Length::zero(); row_count + 1],
            column_offsets: vec![T::Length::zero(); column_count + 1],
            items: Vec::new(),
            row_count,
            column_count,
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

    /// Get the height of a row.
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn get_row_size(&self, row: usize) -> T::Length {
        if row + 1 < self.row_offsets.len() {
            self.row_offsets[row + 1] - self.row_offsets[row]
        } else {
            T::Length::zero()
        }
    }

    /// Get the width of a column.
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn get_column_size(&self, column: usize) -> T::Length {
        if column + 1 < self.column_offsets.len() {
            self.column_offsets[column + 1] - self.column_offsets[column]
        } else {
            T::Length::zero()
        }
    }
}

use float_pigment_css::num_traits::Zero;
