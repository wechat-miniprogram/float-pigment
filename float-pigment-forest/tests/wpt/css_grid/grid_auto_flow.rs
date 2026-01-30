// WPT-style tests for `grid-auto-flow`
// Reference: CSS Grid Layout Module Level 1
// https://www.w3.org/TR/css-grid-1/#auto-placement-algo
//
// The `grid-auto-flow` property controls how auto-placed items are inserted
// into the grid:
// - `row` (default): Fill rows first, adding new rows as needed
// - `column`: Fill columns first, adding new columns as needed
// - `dense`: Attempt to fill holes earlier in the grid (can be combined with row/column)

use crate::*;

// Case: Default auto-flow (row)
// Spec points:
//   - Items are placed in row-major order by default
//   - New rows are added as items overflow
// In this test:
//   - Container: 2 columns, auto rows
//   - 4 items placed: (0,0), (1,0), (0,1), (1,1)
#[test]
fn grid_auto_flow_row_default() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 50px;" expect_left="0" expect_top="0"></div>
          <div style="height: 50px;" expect_left="100" expect_top="0"></div>
          <div style="height: 50px;" expect_left="0" expect_top="50"></div>
          <div style="height: 50px;" expect_left="100" expect_top="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Explicit grid-auto-flow: row
// Spec points:
//   - `grid-auto-flow: row` is the default behavior
//   - Items fill across columns, then wrap to new rows
// In this test:
//   - Container: 3 columns, auto-flow: row
//   - 5 items: first row (3 items), second row (2 items)
#[test]
fn grid_auto_flow_row_explicit() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 80px 80px 80px; grid-auto-flow: row;">
          <div style="height: 40px;" expect_left="0" expect_top="0"></div>
          <div style="height: 40px;" expect_left="80" expect_top="0"></div>
          <div style="height: 40px;" expect_left="160" expect_top="0"></div>
          <div style="height: 40px;" expect_left="0" expect_top="40"></div>
          <div style="height: 40px;" expect_left="80" expect_top="40"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-flow: column
// Spec points:
//   - Items fill down columns first
//   - New columns are added as items overflow
// In this test:
//   - Container: 2 explicit rows, auto-flow: column
//   - 4 items: first column (2 items), second column (2 items)
#[test]
fn grid_auto_flow_column() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-rows: 50px 50px; grid-auto-flow: column;">
          <div style="width: 80px;" expect_left="0" expect_top="0"></div>
          <div style="width: 80px;" expect_left="0" expect_top="50"></div>
          <div style="width: 80px;" expect_left="80" expect_top="0"></div>
          <div style="width: 80px;" expect_left="80" expect_top="50"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-flow: column with 3 rows
// Spec points:
//   - Column flow fills vertically through explicit rows
//   - After explicit rows, moves to next column
// In this test:
//   - Container: 3 explicit rows, auto-flow: column
//   - 5 items: first column (3 items), second column (2 items)
#[test]
fn grid_auto_flow_column_3_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-rows: 40px 40px 40px; grid-auto-flow: column;">
          <div style="width: 60px;" expect_left="0" expect_top="0"></div>
          <div style="width: 60px;" expect_left="0" expect_top="40"></div>
          <div style="width: 60px;" expect_left="0" expect_top="80"></div>
          <div style="width: 60px;" expect_left="60" expect_top="0"></div>
          <div style="width: 60px;" expect_left="60" expect_top="40"></div>
        </div>
    "#,
        true
    )
}

// Case: Row flow with single column
// Spec points:
//   - With single column, row flow stacks items vertically
//   - Each item occupies its own row
// In this test:
//   - Container: 1 column, row flow
//   - 3 items stacked vertically
#[test]
fn grid_auto_flow_row_single_column() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 150px; grid-auto-flow: row;">
          <div style="height: 30px;" expect_left="0" expect_top="0" expect_width="150"></div>
          <div style="height: 40px;" expect_left="0" expect_top="30" expect_width="150"></div>
          <div style="height: 50px;" expect_left="0" expect_top="70" expect_width="150"></div>
        </div>
    "#,
        true
    )
}

// Case: Column flow with single row
// Spec points:
//   - With single row, column flow places items horizontally
//   - Each item occupies its own column
// In this test:
//   - Container: 1 row, column flow
//   - 3 items placed horizontally
#[test]
fn grid_auto_flow_column_single_row() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-rows: 60px; grid-auto-flow: column;">
          <div style="width: 50px;" expect_left="0" expect_top="0" expect_height="60"></div>
          <div style="width: 70px;" expect_left="50" expect_top="0" expect_height="60"></div>
          <div style="width: 60px;" expect_left="120" expect_top="0" expect_height="60"></div>
        </div>
    "#,
        true
    )
}

// Case: Row flow with explicit row template
// Spec points:
//   - Explicit rows define the grid structure
//   - Items exceeding explicit rows create implicit rows
// In this test:
//   - Container: 2 columns, 2 explicit rows (40px, 60px)
//   - 6 items: 4 in explicit rows, 2 in implicit row
#[test]
fn grid_auto_flow_row_with_explicit_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; grid-template-rows: 40px 60px; grid-auto-flow: row;">
          <div expect_left="0" expect_top="0" expect_height="40"></div>
          <div expect_left="100" expect_top="0" expect_height="40"></div>
          <div expect_left="0" expect_top="40" expect_height="60"></div>
          <div expect_left="100" expect_top="40" expect_height="60"></div>
          <div style="height: 30px;" expect_left="0" expect_top="100" expect_height="30"></div>
          <div style="height: 30px;" expect_left="100" expect_top="100" expect_height="30"></div>
        </div>
    "#,
        true
    )
}

// Case: Column flow with explicit column template
// Spec points:
//   - Explicit columns define the grid structure
//   - Items exceeding explicit columns create implicit columns
// In this test:
//   - Container: 2 rows, 2 explicit columns (80px, 120px)
//   - 6 items: 4 in explicit columns, 2 in implicit column
#[test]
fn grid_auto_flow_column_with_explicit_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-rows: 50px 50px; grid-template-columns: 80px 120px; grid-auto-flow: column;">
          <div expect_left="0" expect_top="0" expect_width="80"></div>
          <div expect_left="0" expect_top="50" expect_width="80"></div>
          <div expect_left="80" expect_top="0" expect_width="120"></div>
          <div expect_left="80" expect_top="50" expect_width="120"></div>
          <div style="width: 60px;" expect_left="200" expect_top="0" expect_width="60"></div>
          <div style="width: 60px;" expect_left="200" expect_top="50" expect_width="60"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto-flow row with varying item heights
// Spec points:
//   - Row height is determined by the tallest item in the row
//   - All items in the row share the same row height
// In this test:
//   - Container: 2 columns, row flow
//   - Row 1: heights 30px and 60px, row height = 60px
//   - Row 2: heights 40px and 20px, row height = 40px
#[test]
fn grid_auto_flow_row_varying_heights() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; grid-auto-flow: row;">
          <div style="height: 30px;" expect_top="0" expect_height="30"></div>
          <div style="height: 60px;" expect_top="0" expect_height="60"></div>
          <div style="height: 40px;" expect_top="60" expect_height="40"></div>
          <div style="height: 20px;" expect_top="60" expect_height="20"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto-flow column with varying item widths
// Spec points:
//   - Column width is determined by the widest item in the column
//   - All items in the column share the same column width
// In this test:
//   - Container: 2 rows, column flow
//   - Column 1: widths 50px and 80px, column width = 80px
//   - Column 2: widths 60px and 40px, column width = 60px
#[test]
fn grid_auto_flow_column_varying_widths() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-rows: 50px 50px; grid-auto-flow: column;">
          <div style="width: 50px;" expect_left="0" expect_width="50"></div>
          <div style="width: 80px;" expect_left="0" expect_width="80"></div>
          <div style="width: 60px;" expect_left="80" expect_width="60"></div>
          <div style="width: 40px;" expect_left="80" expect_width="40"></div>
        </div>
    "#,
        true
    )
}

// Case: Large grid with row flow
// Spec points:
//   - Row flow scales to large grids
//   - Items continue to wrap correctly
// In this test:
//   - Container: 4 columns, row flow
//   - 8 items fill 2 complete rows
#[test]
fn grid_auto_flow_row_large_grid() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 50px 50px 50px 50px; grid-auto-flow: row;">
          <div style="height: 30px;" expect_left="0" expect_top="0"></div>
          <div style="height: 30px;" expect_left="50" expect_top="0"></div>
          <div style="height: 30px;" expect_left="100" expect_top="0"></div>
          <div style="height: 30px;" expect_left="150" expect_top="0"></div>
          <div style="height: 30px;" expect_left="0" expect_top="30"></div>
          <div style="height: 30px;" expect_left="50" expect_top="30"></div>
          <div style="height: 30px;" expect_left="100" expect_top="30"></div>
          <div style="height: 30px;" expect_left="150" expect_top="30"></div>
        </div>
    "#,
        true
    )
}

// Case: Large grid with column flow
// Spec points:
//   - Column flow scales to large grids
//   - Items fill down columns before moving to next column
// In this test:
//   - Container: 4 rows, column flow
//   - 8 items fill 2 complete columns
#[test]
fn grid_auto_flow_column_large_grid() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-rows: 30px 30px 30px 30px; grid-auto-flow: column;">
          <div style="width: 50px;" expect_left="0" expect_top="0"></div>
          <div style="width: 50px;" expect_left="0" expect_top="30"></div>
          <div style="width: 50px;" expect_left="0" expect_top="60"></div>
          <div style="width: 50px;" expect_left="0" expect_top="90"></div>
          <div style="width: 50px;" expect_left="50" expect_top="0"></div>
          <div style="width: 50px;" expect_left="50" expect_top="30"></div>
          <div style="width: 50px;" expect_left="50" expect_top="60"></div>
          <div style="width: 50px;" expect_left="50" expect_top="90"></div>
        </div>
    "#,
        true
    )
}
