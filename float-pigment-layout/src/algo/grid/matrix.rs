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
//! Separates occupancy tracking from item storage for better performance:
//! - Occupancy: 1-byte enum per cell
//! - Items: stored in Vec for efficient iteration

use std::fmt::Debug;

use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::{
        dynamic_grid::DynamicGrid,
        grid_item::{GridItem, GridLayoutItem},
    },
    LayoutTreeNode,
};

/// The occupancy state of a single grid cell.
///
/// CSS Grid §8.5: During auto-placement, cells can be occupied or empty.
/// This is a compact 1-byte representation for efficient memory usage.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) enum CellOccupancyState {
    /// Cell is empty (available for auto-placement)
    #[default]
    Unoccupied,
    /// Cell is occupied by an auto-placed item
    Occupied,
}

/// The grid matrix stores grid items during the placement phase.
///
/// CSS Grid §7.1: The grid is a two-dimensional structure with:
/// - Explicit grid: Defined by `grid-template-rows` and `grid-template-columns`
/// - Implicit grid: Automatically created when items overflow the explicit grid
///
/// This structure uses `DynamicGrid` internally to support dynamic expansion
/// during item placement, eliminating the need for a separate estimation pass.
///
/// Separates occupancy tracking (1 byte per cell) from item storage (Vec)
/// for efficient item iteration.
#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    /// The 2D grid storing only occupancy state (1 byte per cell)
    occupancy: DynamicGrid<CellOccupancyState>,
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
            // Start with empty grid - cells created on-demand during placement
            occupancy: DynamicGrid::new(),
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
    /// 1. Marks the cell as occupied in the occupancy grid
    /// 2. Adds the item to the items Vec
    pub(crate) fn place_item(&mut self, row: usize, column: usize, item: GridItem<'a, T>) {
        // Mark cell as occupied (this triggers dynamic expansion if needed)
        self.occupancy
            .set(row, column, CellOccupancyState::Occupied);
        // Store item in the Vec
        self.items.push(item);
    }

    /// Check if a cell is occupied.
    #[allow(dead_code)]
    pub(crate) fn is_occupied(&self, row: usize, column: usize) -> bool {
        self.occupancy
            .get(row, column)
            .map(|state| *state == CellOccupancyState::Occupied)
            .unwrap_or(false)
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
        self.occupancy.rows()
    }

    /// Returns the current number of columns (may exceed explicit grid).
    #[inline(always)]
    pub(crate) fn column_count(&self) -> usize {
        self.occupancy.cols()
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            offset = offset + size;
            if i < sizes.len() - 1 {
                offset = offset + gap;
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
            offset = offset + size;
            if i < sizes.len() - 1 {
                offset = offset + gap;
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
