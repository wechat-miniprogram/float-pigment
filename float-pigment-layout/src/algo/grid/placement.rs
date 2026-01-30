//! Grid Item Placement Algorithm
//!
//! CSS Grid Layout Module Level 1 - §8 Placing Grid Items
//! <https://www.w3.org/TR/css-grid-1/#placement>

use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::{grid_item::GridItem, matrix::MatrixCell, GridMatrix},
    is_display_none, is_independent_positioning, LayoutStyle, LayoutTrackListItem, LayoutTreeNode,
    LayoutTreeVisitor,
};

/// Place grid items into the grid using the auto-placement algorithm.
///
/// CSS Grid §8: Placing Grid Items
/// <https://www.w3.org/TR/css-grid-1/#placement>
///
/// ## Auto-Placement Algorithm (§8.5)
/// <https://www.w3.org/TR/css-grid-1/#auto-placement-algo>
///
/// The algorithm processes items in order and places them into the first
/// available grid cell, controlled by `grid-auto-flow`:
///
/// - `row` (default): Fill each row before moving to the next
/// - `column`: Fill each column before moving to the next
/// - `dense`: Fill holes left by larger items (not yet implemented)
///
/// ## Implicit Grid (§7.5)
/// <https://www.w3.org/TR/css-grid-1/#implicit-grids>
///
/// When items don't fit in the explicit grid, new rows or columns are
/// created automatically (implicit grid tracks).
pub(crate) fn place_grid_items<'a, T: LayoutTreeNode>(
    grid_matrix: &mut GridMatrix<'a, T>,
    node: &'a T,
    style: &'a T::Style,
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
) {
    let mut row_num = row_track_list.len().max(1);
    let mut column_num = column_track_list.len().max(1);

    // Current auto-placement cursor position
    let mut cur_row = 0;
    let mut cur_column = 0;

    // Filter out absolutely positioned and display:none items
    // CSS Grid §9: Absolutely positioned items are not grid items for placement
    // https://www.w3.org/TR/css-grid-1/#abspos
    let children_iter = node
        .tree_visitor()
        .children_iter()
        .enumerate()
        .filter(|(_, node)| {
            !is_independent_positioning(*node) && !is_display_none::<T>(node.style())
        });

    // Process each grid item according to grid-auto-flow
    // CSS Grid §8.5: Auto-placement algorithm
    // https://www.w3.org/TR/css-grid-1/#auto-placement-algo
    // TODO: Implement grid-auto-flow: dense (sparse vs dense packing)
    children_iter.for_each(|(origin_idx, child)| match style.grid_auto_flow() {
        // ═══════════════════════════════════════════════════════════════════
        // Row-major auto-placement (grid-auto-flow: row)
        // CSS Grid §8.5: https://www.w3.org/TR/css-grid-1/#auto-placement-algo
        // Items are placed row by row, moving to the next row when needed.
        // ═══════════════════════════════════════════════════════════════════
        GridAutoFlow::Row | GridAutoFlow::RowDense => {
            // Wrap to next row if we've filled the current row
            if cur_column >= column_num {
                cur_column = 0;
                cur_row += 1;
                // Create implicit row if needed
                if cur_row >= row_num {
                    row_num = cur_row + 1;
                }
            }
            let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
            grid_matrix.update_item(cur_row, cur_column, MatrixCell::AutoPlaced(grid_item));
            cur_column += 1;
        }
        // ═══════════════════════════════════════════════════════════════════
        // Column-major auto-placement (grid-auto-flow: column)
        // Items are placed column by column, moving to the next column when needed.
        // ═══════════════════════════════════════════════════════════════════
        GridAutoFlow::Column | GridAutoFlow::ColumnDense => {
            // Wrap to next column if we've filled the current column
            if cur_row >= row_num {
                cur_row = 0;
                cur_column += 1;
                // Create implicit column if needed
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
