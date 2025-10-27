use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::{grid_item::GridItem, matrix::MatrixCell, GridMatrix},
    is_display_none, is_independent_positioning, LayoutStyle, LayoutTrackListItem, LayoutTreeNode,
    LayoutTreeVisitor,
};

pub(crate) fn place_grid_items<'a, T: LayoutTreeNode>(
    grid_matrix: &mut GridMatrix<'a, T>,
    node: &'a T,
    style: &'a T::Style,
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
) {
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

    // TODO: Implement grid auto flow dense
    children_iter.for_each(|(origin_idx, child)| match style.grid_auto_flow() {
        GridAutoFlow::Row | GridAutoFlow::RowDense => {
            if cur_column >= column_num {
                cur_column = 0;
                cur_row += 1;
                if cur_row >= row_num {
                    row_num = cur_row + 1;
                }
            }
            let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
            // grid_items.push(grid_item);
            grid_matrix.update_item(cur_row, cur_column, MatrixCell::AutoPlaced(grid_item));
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

            let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
            grid_matrix.update_item(cur_row, cur_column, MatrixCell::AutoPlaced(grid_item));
            cur_row += 1;
        }
    });
}
