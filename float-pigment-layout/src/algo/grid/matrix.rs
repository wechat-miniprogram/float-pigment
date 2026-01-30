//! Grid Matrix Data Structure
//!
//! CSS Grid Layout Module Level 1 - §7.1 The Explicit Grid
//! <https://www.w3.org/TR/css-grid-1/#explicit-grids>
//!
//! This module provides the matrix data structure for storing grid items
//! and managing the grid's row/column structure during layout computation.

use std::fmt::Debug;

use float_pigment_css::typing::GridAutoFlow;
use grid::Grid;

use crate::{
    algo::grid::grid_item::{GridItem, GridLayoutItem},
    is_display_none, is_independent_positioning, LayoutStyle, LayoutTrackListItem, LayoutTreeNode,
    LayoutTreeVisitor,
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
/// This structure tracks both the explicit track counts and the auto track counts
/// to properly implement the track sizing algorithm.
#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    /// The 2D grid storing cell contents
    inner: Grid<MatrixCell<GridItem<'a, T>>>,
    /// Total number of rows (explicit + implicit)
    row_count: usize,
    /// Number of rows with `auto` sizing function
    row_auto_count: usize,
    /// Total number of columns (explicit + implicit)
    column_count: usize,
    /// Number of columns with `auto` sizing function
    column_auto_count: usize,
    /// The auto-placement flow direction
    flow: GridAutoFlow,
}

impl<'a, 'b: 'a, T: LayoutTreeNode> GridMatrix<'a, T> {
    pub(crate) fn new(
        row_count: usize,
        column_count: usize,
        row_auto_count: usize,
        column_auto_count: usize,
        flow: GridAutoFlow,
    ) -> Self {
        Self {
            inner: Grid::new(row_count, column_count),
            row_count,
            row_auto_count,
            column_count,
            column_auto_count,
            flow,
        }
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

    #[inline(always)]
    pub(crate) fn row_count(&self) -> usize {
        self.row_count
    }

    #[inline(always)]
    pub(crate) fn column_count(&self) -> usize {
        self.column_count
    }

    #[inline(always)]
    pub(crate) fn row_auto_count(&self) -> usize {
        self.row_auto_count
    }

    #[inline(always)]
    pub(crate) fn column_auto_count(&self) -> usize {
        self.column_auto_count
    }

    pub(crate) fn update_item(
        &mut self,
        row: usize,
        column: usize,
        cell: MatrixCell<GridItem<'a, T>>,
    ) {
        self.inner[(row, column)] = cell;
    }
}

impl<'a, T: LayoutTreeNode> Debug for GridMatrix<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = write!(
            f,
            "GridMatrix {{ grid_items: {:?} row_count: {}, column_count: {} }}",
            self.inner, self.row_count, self.column_count
        );
        r
    }
}

/// Estimate the grid dimensions before placement.
///
/// CSS Grid §7.1: Resolving the Grid
/// <https://www.w3.org/TR/css-grid-1/#explicit-grids>
///
/// This function calculates the total number of rows and columns needed
/// for the grid, considering both:
/// - Explicit tracks from `grid-template-rows/columns`
/// - Implicit tracks needed for auto-placed items
///
/// The estimation follows the same algorithm as `place_grid_items` but
/// only counts positions without creating actual grid items.
pub(crate) fn estimate_track_count<'a, T: LayoutTreeNode>(
    node: &'a T,
    style: &'a T::Style,
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
) -> (usize, usize) {
    // Start with explicit track counts (minimum 1)
    let mut row_num = row_track_list.len().max(1);
    let mut column_num = column_track_list.len().max(1);

    let mut cur_row = 0;
    let mut cur_column = 0;

    // Count only grid-participating children
    let children_iter = node
        .tree_visitor()
        .children_iter()
        .enumerate()
        .filter(|(_, node)| {
            !is_independent_positioning(*node) && !is_display_none::<T>(node.style())
        });

    // Simulate auto-placement to count needed rows/columns
    children_iter.for_each(|_| match style.grid_auto_flow() {
        GridAutoFlow::Row | GridAutoFlow::RowDense => {
            if cur_column >= column_num {
                cur_column = 0;
                cur_row += 1;
                if cur_row >= row_num {
                    row_num = cur_row + 1;
                }
            }

            cur_column += 1;
        }
        GridAutoFlow::Column | GridAutoFlow::ColumnDense => {
            if cur_row >= row_num {
                cur_row = 0;
                cur_column += 1;
                if cur_column >= column_num {
                    column_num = cur_column + 1;
                }
            }
            cur_row += 1;
        }
    });
    (
        row_num.max(row_track_list.len().max(1)),
        column_num.max(column_track_list.len().max(1)),
    )
}

/// The layout matrix stores computed layout information for each grid cell.
///
/// This is used in the final positioning phase after track sizes have been
/// determined. It stores the actual computed sizes and positions of items.
pub(crate) struct GridLayoutMatrix<'a, T: LayoutTreeNode> {
    pub(crate) inner: Grid<MatrixCell<GridLayoutItem<'a, T>>>,
    row_count: usize,
    column_count: usize,
}

impl<'a, T: LayoutTreeNode> GridLayoutMatrix<'a, T> {
    pub(crate) fn new(row_count: usize, column_count: usize) -> Self {
        Self {
            inner: Grid::new(row_count, column_count),
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

    pub(crate) fn update_item(
        &mut self,
        row: usize,
        column: usize,
        cell: MatrixCell<GridLayoutItem<'a, T>>,
    ) {
        self.inner[(row, column)] = cell;
    }
}
