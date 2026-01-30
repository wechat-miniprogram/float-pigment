//! Grid Matrix Data Structure
//!
//! CSS Grid Layout Module Level 1 - §7.1 The Explicit Grid
//! <https://www.w3.org/TR/css-grid-1/#explicit-grids>
//!
//! This module provides the matrix data structure for storing grid items
//! and managing the grid's row/column structure during layout computation.

use std::fmt::Debug;

use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::{
        dynamic_grid::DynamicGrid,
        grid_item::{GridItem, GridLayoutItem},
    },
    LayoutTreeNode,
};

/// Represents the state of a cell in the grid matrix.
///
/// CSS Grid §8.5: During auto-placement, cells can be occupied or empty.
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum MatrixCell<T> {
    /// Cell is empty (available for auto-placement)
    Unoccupied,
    /// Cell contains an auto-placed grid item
    AutoPlaced(T),
}

impl<T> Default for MatrixCell<T> {
    fn default() -> Self {
        Self::Unoccupied
    }
}

impl<T> MatrixCell<T> {
    pub(crate) fn is_unoccupied(&self) -> bool {
        matches!(self, Self::Unoccupied)
    }

    pub(crate) fn get_auto_placed_unchecked(&self) -> &T {
        match self {
            Self::AutoPlaced(item) => item,
            _ => unreachable!(),
        }
    }
    pub(crate) fn get_auto_placed_mut_unchecked(&mut self) -> &mut T {
        match self {
            Self::AutoPlaced(item) => item,
            _ => unreachable!(),
        }
    }
}

/// The grid matrix stores grid items during the placement phase.
///
/// CSS Grid §7.1: The grid is a two-dimensional structure with:
/// - Explicit grid: Defined by `grid-template-rows` and `grid-template-columns`
/// - Implicit grid: Automatically created when items overflow the explicit grid
///
/// This structure uses `DynamicGrid` internally to support dynamic expansion
/// during item placement, eliminating the need for a separate estimation pass.
#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    /// The 2D grid storing cell contents (dynamically expandable)
    inner: DynamicGrid<MatrixCell<GridItem<'a, T>>>,
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
    ///
    /// The explicit_row_count and explicit_column_count are used to control
    /// the auto-placement algorithm's wrapping behavior, but don't cause
    /// pre-allocation.
    pub(crate) fn new(
        explicit_row_count: usize,
        explicit_column_count: usize,
        row_auto_count: usize,
        column_auto_count: usize,
        flow: GridAutoFlow,
    ) -> Self {
        Self {
            // Start with empty grid - cells created on-demand during placement
            inner: DynamicGrid::new(),
            row_auto_count,
            column_auto_count,
            explicit_row_count,
            explicit_column_count,
            flow,
        }
    }

    /// Place an item at the specified position, expanding the grid if needed.
    ///
    /// This is the key method for dynamic grid expansion - it ensures the
    /// grid is large enough before placing the item.
    pub(crate) fn place_item(&mut self, row: usize, column: usize, item: GridItem<'a, T>) {
        self.inner.set(row, column, MatrixCell::AutoPlaced(item));
    }

    pub(crate) fn get_item_mut(
        &mut self,
        row: usize,
        column: usize,
    ) -> Option<&mut MatrixCell<GridItem<'a, T>>> {
        self.inner.get_mut(row, column)
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut MatrixCell<GridItem<'a, T>>> {
        self.inner.iter_mut()
    }

    /// Returns the current number of rows (may exceed explicit grid).
    #[inline(always)]
    pub(crate) fn row_count(&self) -> usize {
        self.inner.rows()
    }

    /// Returns the current number of columns (may exceed explicit grid).
    #[inline(always)]
    pub(crate) fn column_count(&self) -> usize {
        self.inner.cols()
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

    #[allow(dead_code)]
    pub(crate) fn update_item(
        &mut self,
        row: usize,
        column: usize,
        cell: MatrixCell<GridItem<'a, T>>,
    ) {
        self.inner.set(row, column, cell);
    }
}

impl<'a, T: LayoutTreeNode> Debug for GridMatrix<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridMatrix {{ row_count: {}, column_count: {}, explicit: {}x{} }}",
            self.row_count(),
            self.column_count(),
            self.explicit_row_count,
            self.explicit_column_count
        )
    }
}

/// The layout matrix stores computed layout information for each grid cell.
///
/// This is used in the final positioning phase after track sizes have been
/// determined. It stores the actual computed sizes and positions of items.
pub(crate) struct GridLayoutMatrix<'a, T: LayoutTreeNode> {
    pub(crate) inner: DynamicGrid<MatrixCell<GridLayoutItem<'a, T>>>,
}

impl<'a, T: LayoutTreeNode> GridLayoutMatrix<'a, T> {
    pub(crate) fn new(row_count: usize, column_count: usize) -> Self {
        Self {
            inner: DynamicGrid::with_size(row_count, column_count),
        }
    }

    #[inline(always)]
    pub(crate) fn row_count(&self) -> usize {
        self.inner.rows()
    }

    #[inline(always)]
    pub(crate) fn column_count(&self) -> usize {
        self.inner.cols()
    }

    pub(crate) fn update_item(
        &mut self,
        row: usize,
        column: usize,
        cell: MatrixCell<GridLayoutItem<'a, T>>,
    ) {
        self.inner.set(row, column, cell);
    }
}
