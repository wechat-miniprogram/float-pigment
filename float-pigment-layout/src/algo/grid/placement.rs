//! Grid Item Placement Algorithm
//!
//! CSS Grid Layout Module Level 1 - §8 Placing Grid Items
//! <https://www.w3.org/TR/css-grid-1/#placement>

use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::{grid_item::GridItem, GridMatrix},
    LayoutTreeNode,
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
/// - `row` (default): Fill each row before moving to the next (sparse)
/// - `column`: Fill each column before moving to the next (sparse)
/// - `row dense`: Fill holes left by larger items (dense packing)
/// - `column dense`: Fill holes left by larger items (dense packing)
///
/// ### Sparse vs Dense Packing
///
/// - **Sparse** (default): Cursor only moves forward, may leave holes
/// - **Dense**: For each item, search from beginning for first available cell
///
/// ## Implicit Grid (§7.5)
/// <https://www.w3.org/TR/css-grid-1/#implicit-grids>
///
/// When items don't fit in the explicit grid, new rows or columns are
/// created automatically (implicit grid tracks).
///
/// ## Performance Optimization
///
/// Dense mode uses a search hint to avoid re-scanning filled rows/columns:
/// - Track the first row/column that may have empty cells
/// - Skip fully occupied rows/columns in subsequent searches
/// - Reduces O(N × R × C) to approximately O(N + R × C) in typical cases
pub(crate) fn place_grid_items<'a, T: LayoutTreeNode>(
    grid_matrix: &mut GridMatrix<'a, T>,
    children_iter: impl Iterator<Item = (usize, &'a T)>,
) {
    // Get dimensions from explicit grid template
    let explicit_row_count = grid_matrix.explicit_row_count();
    let explicit_column_count = grid_matrix.explicit_column_count();
    let flow = grid_matrix.flow();

    // Current auto-placement cursor position (for sparse mode)
    let mut cur_row = 0;
    let mut cur_column = 0;

    // Dense mode optimization: track the first row/column that may have space
    // This avoids re-scanning rows/columns that are known to be full
    let mut dense_hint = 0;

    // Process each grid item according to grid-auto-flow
    // CSS Grid §8.5: Auto-placement algorithm
    // https://www.w3.org/TR/css-grid-1/#auto-placement-algo
    children_iter.for_each(|(origin_idx, child)| match flow {
        // Items are placed row by row, cursor only moves forward.
        GridAutoFlow::Row => {
            // Wrap to next row if we've filled the current row
            if cur_column >= explicit_column_count.max(1) {
                cur_column = 0;
                cur_row += 1;
            }

            // Create and place the grid item
            let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
            grid_matrix.place_item(cur_row, cur_column, grid_item);
            cur_column += 1;
        }
        // For each item, search from hint for first unoccupied cell.
        GridAutoFlow::RowDense => {
            let (row, col) = grid_matrix.find_first_unoccupied(&mut dense_hint);

            let grid_item = GridItem::new(child, origin_idx, row, col);
            grid_matrix.place_item(row, col, grid_item);
        }
        // Items are placed column by column, cursor only moves forward.
        GridAutoFlow::Column => {
            // Wrap to next column if we've filled the current column
            if cur_row >= explicit_row_count.max(1) {
                cur_row = 0;
                cur_column += 1;
            }

            // Create and place the grid item
            let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
            grid_matrix.place_item(cur_row, cur_column, grid_item);
            cur_row += 1;
        }
        // For each item, search from hint for first unoccupied cell.
        GridAutoFlow::ColumnDense => {
            let (row, col) = grid_matrix.find_first_unoccupied(&mut dense_hint);

            let grid_item = GridItem::new(child, origin_idx, row, col);
            grid_matrix.place_item(row, col, grid_item);
        }
    });
}
