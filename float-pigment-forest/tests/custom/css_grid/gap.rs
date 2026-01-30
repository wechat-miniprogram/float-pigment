// Tests for the `gap` property in CSS Grid Layout
// Based on CSS Grid Layout Module Level 1 specification:
// - `gap` creates spacing between grid tracks (rows and columns)
// - Can be specified as a single value (applies to both axes) or as `row-gap` and `column-gap` separately
// - Gap is applied between tracks, not at the edges of the container
// - Gap does not affect the size of grid items, only their positioning

use crate::*;

// Case: `column-gap: 10px` with fixed column sizes
// Spec points:
// - Column gap creates spacing between columns
// - Gap is added between consecutive columns, not at the edges
// In this test:
// - Container: width=320px, 3 columns of 100px each, column-gap=10px
// - First item: expect_left=0
// - Second item: expect_left=110 (100 + 10 gap)
// - Third item: expect_left=220 (100 + 10 + 100 + 10)
#[test]
fn grid_with_column_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 320px; grid-template-columns: 100px 100px 100px; column-gap: 10px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="110" expect_width="100"></div>
          <div style="height: 50px;" expect_left="220" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: `row-gap: 20px` with fixed row sizes
// Spec points:
// - Row gap creates spacing between rows
// - Gap is added between consecutive rows, not at the edges
// In this test:
// - Container: 2 rows of 50px each, row-gap=20px
// - Total height: 50 + 20 + 50 = 120px
// - First item: expect_top=0
// - Second item: expect_top=70 (50 + 20 gap)
#[test]
fn grid_with_row_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: 50px 50px; row-gap: 20px;" expect_height="120">
          <div expect_top="0" expect_height="50"></div>
          <div expect_top="70" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: `gap: 10px 30px` (row-gap and column-gap separately)
// Spec points:
// - `gap` shorthand can specify row-gap and column-gap separately
// - First value is row-gap, second value is column-gap
// In this test:
// - Container: width=230px, 2x2 grid, row-gap=10px, column-gap=30px
// - Total height: 50 + 10 + 50 = 110px
// - Column positions: 0, 130 (100 + 30 gap)
// - Row positions: 0, 60 (50 + 10 gap)
#[test]
fn grid_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 230px; grid-template-columns: 100px 100px; grid-template-rows: 50px 50px; gap: 10px 30px;" expect_height="110">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="130" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="60" expect_width="100" expect_height="50"></div>
          <div expect_left="130" expect_top="60" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: `gap: 20px` (single value for both axes)
// Spec points:
// - Single `gap` value applies to both row-gap and column-gap
// In this test:
// - Container: width=220px, 2x2 grid, gap=20px (both row and column)
// - Total height: 50 + 20 + 50 = 120px
// - Column positions: 0, 120 (100 + 20 gap)
// - Row positions: 0, 70 (50 + 20 gap)
#[test]
fn grid_with_gap_single_value() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 220px; grid-template-columns: 100px 100px; grid-template-rows: 50px 50px; gap: 20px;" expect_height="120">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="120" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="70" expect_width="100" expect_height="50"></div>
          <div expect_left="120" expect_top="70" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Single column grid with column-gap
// Spec points:
// - Column gap has no effect when there is only one column
// - Row gap still applies between rows
// In this test:
// - Container: 1 column, 2 rows, column-gap=20px (should have no effect), row-gap=10px
// - Total height: 50 + 10 + 50 = 110px
// - Both items at expect_left=0 (no column gap effect)
#[test]
fn grid_single_column_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 100px; grid-template-columns: 100px; grid-template-rows: 50px 50px; column-gap: 20px; row-gap: 10px;" expect_height="110" expect_width="100">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="60" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Single row grid with row-gap
// Spec points:
// - Row gap has no effect when there is only one row
// - Column gap still applies between columns
// In this test:
// - Container: 2 columns, 1 row, row-gap=20px (should have no effect), column-gap=30px
// - Total height: 50px (no row gap effect)
// - Second item at expect_left=130 (100 + 30 gap)
#[test]
fn grid_single_row_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 230px; grid-template-columns: 100px 100px; grid-template-rows: 50px; row-gap: 20px; column-gap: 30px;" expect_height="50">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="130" expect_top="0" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: 3x3 grid with gap
// Spec points:
// - Gap applies consistently across all rows and columns
// - Total gap space: (n-1) * gap for n tracks
// In this test:
// - Container: 3x3 grid, row-gap=10px, column-gap=20px
// - Total width: 100*3 + 20*2 = 340px
// - Total height: 50*3 + 10*2 = 170px
// - Column positions: 0, 120 (100+20), 240 (100+20+100+20)
// - Row positions: 0, 60 (50+10), 120 (50+10+50+10)
#[test]
fn grid_3x3_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 340px; grid-template-columns: 100px 100px 100px; grid-template-rows: 50px 50px 50px; gap: 10px 20px;" expect_height="170">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="120" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="240" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="60" expect_width="100" expect_height="50"></div>
          <div expect_left="120" expect_top="60" expect_width="100" expect_height="50"></div>
          <div expect_left="240" expect_top="60" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="120" expect_width="100" expect_height="50"></div>
          <div expect_left="120" expect_top="120" expect_width="100" expect_height="50"></div>
          <div expect_left="240" expect_top="120" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap with auto-sized columns
// Spec points:
// - Gap is subtracted from available space before auto columns are sized
// - Remaining space is distributed equally among auto columns
// In this test:
// - Container: width=280px, 3 auto columns, column-gap=20px
// - Total gap space: 20 * 2 = 40px
// - Remaining space: 280 - 40 = 240px
// - Each auto column: 240 / 3 = 80px
// - Column positions: 0, 100 (80+20), 200 (80+20+80+20)
#[test]
fn grid_gap_with_auto_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 280px; grid-template-columns: auto auto auto; column-gap: 20px;" expect_height="50">
          <div style="height: 50px;" expect_left="0" expect_width="80" expect_height="50"></div>
          <div style="height: 50px;" expect_left="100" expect_width="80" expect_height="50"></div>
          <div style="height: 50px;" expect_left="200" expect_width="80" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap with item margins
// Spec points:
// - Gap creates space between grid cells, not between items
// - Item margins are applied within the grid cell
// - Gap and margin stack (gap between cells, margin within cell)
// In this test:
// - Container: 2x2 grid, row-gap=10px, column-gap=30px
// - Each item has margin=5px
// - First item: expect_left=5 (margin), expect_top=5 (margin)
// - Second item: expect_left=135 (100 cell + 30 gap + 5 margin)
#[test]
fn grid_gap_with_item_margin() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 230px; grid-template-columns: 100px 100px; grid-template-rows: 60px 60px; gap: 10px 30px;">
          <div style="margin: 5px; width: 50px; height: 50px;" expect_left="5" expect_top="5"></div>
          <div style="margin: 5px; width: 50px; height: 50px;" expect_left="135" expect_top="5"></div>
          <div style="margin: 5px; width: 50px; height: 50px;" expect_left="5" expect_top="75"></div>
          <div style="margin: 5px; width: 50px; height: 50px;" expect_left="135" expect_top="75"></div>
        </div>
    "#,
        true
    )
}

// Case: Large gap value
// Spec points:
// - Large gap values are valid and create proportionally large spacing
// In this test:
// - Container: 2x2 grid, gap=50px
// - Total height: 50 + 50 + 50 = 150px
// - Column positions: 0, 150 (100+50)
// - Row positions: 0, 100 (50+50)
#[test]
fn grid_large_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 350px; grid-template-columns: 100px 100px; grid-template-rows: 50px 50px; gap: 50px;" expect_height="150">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="150" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="50"></div>
          <div expect_left="150" expect_top="100" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Zero gap (explicit)
// Spec points:
// - `gap: 0px` is valid and results in no spacing between tracks
// - Items are placed directly adjacent to each other
// In this test:
// - Container: 2x2 grid, gap=0px
// - Total height: 50 + 50 = 100px (no gap)
// - Items placed at: (0,0), (100,0), (0,50), (100,50)
#[test]
fn grid_zero_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 100px 100px; grid-template-rows: 50px 50px; gap: 0px;" expect_height="100">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="50" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}
