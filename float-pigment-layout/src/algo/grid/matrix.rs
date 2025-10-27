use std::fmt::Debug;

use float_pigment_css::typing::GridAutoFlow;
use grid::Grid;

use crate::{
    algo::grid::grid_item::{GridItem, GridLayoutItem},
    is_display_none, is_independent_positioning, LayoutStyle, LayoutTrackListItem, LayoutTreeNode,
    LayoutTreeVisitor,
};

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum MatrixCell<T> {
    Unoccupied,
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

#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    inner: Grid<MatrixCell<GridItem<'a, T>>>,
    row_count: usize,
    row_auto_count: usize,
    column_count: usize,
    column_auto_count: usize,
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

pub(crate) fn estimate_track_count<'a, T: LayoutTreeNode>(
    node: &'a T,
    style: &'a T::Style,
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
) -> (usize, usize) {
    let mut row_num = row_track_list.len().max(1);
    let mut column_num = column_track_list.len().max(1);

    let mut cur_row = 0;
    let mut cur_column = 0;

    let children_iter = node
        .tree_visitor()
        .children_iter()
        .enumerate()
        .filter(|(_, node)| {
            !is_independent_positioning(*node) && !is_display_none::<T>(node.style())
        });

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
