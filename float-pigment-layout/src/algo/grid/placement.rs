//! Grid Item Placement Algorithm
//!
//! CSS Grid Layout Module Level 1 - §8 Placing Grid Items
//! <https://www.w3.org/TR/css-grid-1/#placement>

use float_pigment_css::typing::GridAutoFlow;

use crate::{
    algo::grid::{grid_item::GridItem, GridMatrix},
    is_display_none, is_independent_positioning, LayoutTreeNode, LayoutTreeVisitor,
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
    node: &'a T,
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
    let mut dense_hint_row = 0;
    let mut dense_hint_col = 0;

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
    children_iter.for_each(|(origin_idx, child)| match flow {
        // ═══════════════════════════════════════════════════════════════════
        // Row-major sparse auto-placement (grid-auto-flow: row)
        // CSS Grid §8.5: https://www.w3.org/TR/css-grid-1/#auto-placement-algo
        // Items are placed row by row, cursor only moves forward.
        // ═══════════════════════════════════════════════════════════════════
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
        // ═══════════════════════════════════════════════════════════════════
        // Row-major dense auto-placement (grid-auto-flow: row dense)
        // CSS Grid §8.5: https://www.w3.org/TR/css-grid-1/#auto-placement-algo
        // For each item, search from hint for first unoccupied cell.
        // ═══════════════════════════════════════════════════════════════════
        GridAutoFlow::RowDense => {
            let max_cols = explicit_column_count.max(1);
            let (row, col) =
                find_first_unoccupied_row_major(grid_matrix, max_cols, &mut dense_hint_row);

            let grid_item = GridItem::new(child, origin_idx, row, col);
            grid_matrix.place_item(row, col, grid_item);
        }
        // ═══════════════════════════════════════════════════════════════════
        // Column-major sparse auto-placement (grid-auto-flow: column)
        // Items are placed column by column, cursor only moves forward.
        // ═══════════════════════════════════════════════════════════════════
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
        // ═══════════════════════════════════════════════════════════════════
        // Column-major dense auto-placement (grid-auto-flow: column dense)
        // For each item, search from hint for first unoccupied cell.
        // ═══════════════════════════════════════════════════════════════════
        GridAutoFlow::ColumnDense => {
            let max_rows = explicit_row_count.max(1);
            let (row, col) =
                find_first_unoccupied_column_major(grid_matrix, max_rows, &mut dense_hint_col);

            let grid_item = GridItem::new(child, origin_idx, row, col);
            grid_matrix.place_item(row, col, grid_item);
        }
    });
}

/// Find the first unoccupied cell in row-major order.
///
/// CSS Grid §8.5: Dense packing - search from the start for holes.
/// Returns (row, column) of the first available cell.
///
/// ## Optimization
///
/// Uses `hint_row` to skip rows that are known to be full:
/// - Start searching from `hint_row` instead of row 0
/// - When a row is found to be completely full, advance `hint_row`
/// - This reduces repeated scanning of filled rows
fn find_first_unoccupied_row_major<'a, T: LayoutTreeNode>(
    grid_matrix: &GridMatrix<'a, T>,
    max_cols: usize,
    hint_row: &mut usize,
) -> (usize, usize) {
    let mut row = *hint_row;
    let mut col = 0;
    let mut row_start_col = 0; // Track where we started in this row

    loop {
        // Check if current cell is unoccupied
        if !grid_matrix.is_occupied(row, col) {
            return (row, col);
        }

        // Move to next cell in row-major order
        col += 1;
        if col >= max_cols {
            // Completed scanning a full row without finding space
            // If we started from col 0, this row is full - advance hint
            if row_start_col == 0 && row == *hint_row {
                *hint_row = row + 1;
            }
            col = 0;
            row += 1;
            row_start_col = 0;
        }
    }
}

/// Find the first unoccupied cell in column-major order.
///
/// CSS Grid §8.5: Dense packing - search from the start for holes.
/// Returns (row, column) of the first available cell.
///
/// ## Optimization
///
/// Uses `hint_col` to skip columns that are known to be full:
/// - Start searching from `hint_col` instead of column 0
/// - When a column is found to be completely full, advance `hint_col`
/// - This reduces repeated scanning of filled columns
fn find_first_unoccupied_column_major<'a, T: LayoutTreeNode>(
    grid_matrix: &GridMatrix<'a, T>,
    max_rows: usize,
    hint_col: &mut usize,
) -> (usize, usize) {
    let mut row = 0;
    let mut col = *hint_col;
    let mut col_start_row = 0; // Track where we started in this column

    loop {
        // Check if current cell is unoccupied
        if !grid_matrix.is_occupied(row, col) {
            return (row, col);
        }

        // Move to next cell in column-major order
        row += 1;
        if row >= max_rows {
            // Completed scanning a full column without finding space
            // If we started from row 0, this column is full - advance hint
            if col_start_row == 0 && col == *hint_col {
                *hint_col = col + 1;
            }
            row = 0;
            col += 1;
            col_start_row = 0;
        }
    }
}
