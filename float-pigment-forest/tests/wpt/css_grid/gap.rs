// WPT-style tests for the `gap` property in CSS Grid Layout
// Reference: CSS Grid Layout Module Level 1
// https://www.w3.org/TR/css-grid-1/#gutters
//
// The `gap` property (and its longhands `row-gap` and `column-gap`) defines
// the size of the gutters between grid rows and columns.
//
// Key behaviors:
// - Gap creates spacing between tracks, not at the edges of the container
// - Gap is added to the grid layout after track sizing
// - Percentage gaps are relative to the grid container's size

use crate::*;

// Case: `column-gap` with fixed columns
// Spec points:
//   - `column-gap` creates spacing between columns
//   - Gap is added between consecutive columns, not at container edges
// In this test:
//   - Container: width=320px, 3 columns of 100px, column-gap=10px
//   - Column positions: 0, 110 (100+10), 220 (100+10+100+10)
#[test]
fn grid_gap_column_fixed() {
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

// Case: `row-gap` with fixed rows
// Spec points:
//   - `row-gap` creates spacing between rows
//   - Gap is added between consecutive rows, not at container edges
// In this test:
//   - Container: 1 column, 2 rows of 50px, row-gap=20px
//   - Total height: 50 + 20 + 50 = 120px
//   - Row positions: 0, 70 (50+20)
#[test]
fn grid_gap_row_fixed() {
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

// Case: `gap` shorthand with two values
// Spec points:
//   - `gap` shorthand: first value is row-gap, second is column-gap
//   - Both axes get their specified gap values
// In this test:
//   - Container: 2x2 grid, gap: 10px 30px (row-gap=10px, column-gap=30px)
//   - Total height: 50 + 10 + 50 = 110px
//   - Column positions: 0, 130 (100+30)
//   - Row positions: 0, 60 (50+10)
#[test]
fn grid_gap_shorthand_two_values() {
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

// Case: `gap` shorthand with single value
// Spec points:
//   - Single `gap` value applies to both row-gap and column-gap
// In this test:
//   - Container: 2x2 grid, gap: 20px
//   - Total height: 50 + 20 + 50 = 120px
//   - Column positions: 0, 120 (100+20)
//   - Row positions: 0, 70 (50+20)
#[test]
fn grid_gap_shorthand_single_value() {
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

// Case: Gap with single column (column-gap has no effect)
// Spec points:
//   - Column-gap only applies when there are multiple columns
//   - With single column, column-gap is effectively ignored
// In this test:
//   - Container: 1 column, 2 rows, column-gap=20px (no effect), row-gap=10px
//   - Total height: 50 + 10 + 50 = 110px
#[test]
fn grid_gap_single_column() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 100px; grid-template-columns: 100px; grid-template-rows: 50px 50px; column-gap: 20px; row-gap: 10px;" expect_height="110">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="60" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap with single row (row-gap has no effect)
// Spec points:
//   - Row-gap only applies when there are multiple rows
//   - With single row, row-gap is effectively ignored
// In this test:
//   - Container: 2 columns, 1 row, row-gap=20px (no effect), column-gap=30px
//   - Total height: 50px (no row gap)
#[test]
fn grid_gap_single_row() {
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
//   - Gap applies consistently across all rows and columns
//   - Total gap space: (n-1) * gap for n tracks
// In this test:
//   - Container: 3x3 grid, row-gap=10px, column-gap=20px
//   - Total width: 100*3 + 20*2 = 340px
//   - Total height: 50*3 + 10*2 = 170px
#[test]
fn grid_gap_3x3() {
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

// Case: Gap with auto columns
// Spec points:
//   - Gap is subtracted from available space before auto sizing
//   - Remaining space is distributed among auto tracks
// In this test:
//   - Container: width=280px, 3 auto columns, column-gap=20px
//   - Total gap: 20 * 2 = 40px
//   - Remaining: 280 - 40 = 240px, each auto = 80px
//   - Column positions: 0, 100 (80+20), 200 (80+20+80+20)
#[test]
fn grid_gap_with_auto_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 280px; grid-template-columns: auto auto auto; column-gap: 20px;" expect_height="50">
          <div style="height: 50px;" expect_left="0" expect_width="80"></div>
          <div style="height: 50px;" expect_left="100" expect_width="80"></div>
          <div style="height: 50px;" expect_left="200" expect_width="80"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap with item margins
// Spec points:
//   - Gap creates space between grid cells
//   - Item margins are applied within the grid cell
//   - Gap and margin stack independently
// In this test:
//   - Container: 2x2 grid, row-gap=10px, column-gap=30px
//   - Items have margin=5px
//   - First item: expect_left=5, expect_top=5
//   - Second item: expect_left=135 (100 cell + 30 gap + 5 margin)
#[test]
fn grid_gap_with_item_margins() {
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
//   - Large gap values are valid and create proportionally large spacing
// In this test:
//   - Container: 2x2 grid, gap=50px
//   - Total height: 50 + 50 + 50 = 150px
//   - Column positions: 0, 150 (100+50)
#[test]
fn grid_gap_large() {
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
//   - `gap: 0` or `gap: 0px` results in no spacing between tracks
//   - Items are placed directly adjacent to each other
// In this test:
//   - Container: 2x2 grid, gap=0px
//   - Total height: 50 + 50 = 100px (no gap)
//   - Items at: (0,0), (100,0), (0,50), (100,50)
#[test]
fn grid_gap_zero() {
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

// Case: Gap with auto rows
// Spec points:
//   - Gap works with auto-sized rows
//   - Row height determined by content, gap applied between rows
// In this test:
//   - Container: 2 columns, auto rows, row-gap=15px
//   - Row 1: tallest item 40px, row height = 40px
//   - Row 2: starts at 55px (40 + 15 gap)
#[test]
fn grid_gap_with_auto_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; row-gap: 15px;">
          <div style="height: 40px;" expect_top="0" expect_height="40"></div>
          <div style="height: 30px;" expect_top="0" expect_height="30"></div>
          <div style="height: 50px;" expect_top="55" expect_height="50"></div>
          <div style="height: 25px;" expect_top="55" expect_height="25"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap with percentage columns
// Spec points:
//   - Gap is applied after percentage column sizing
//   - Container width accommodates both column percentages and gap
// In this test:
//   - Container: width=230px, 2 columns at 100px each, column-gap=30px
//   - Column positions: 0, 130 (100+30)
#[test]
fn grid_gap_with_fixed_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 230px; grid-template-columns: 100px 100px; column-gap: 30px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="130" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap in 4x4 grid
// Spec points:
//   - Gap scales correctly with larger grids
//   - Total gap = (tracks - 1) * gap per axis
// In this test:
//   - Container: 4x4 grid, row-gap=5px, column-gap=10px
//   - Row positions: 0, 35, 70, 105 (30 + 5 gap each)
//   - Column positions: 0, 60, 120, 180 (50 + 10 gap each)
#[test]
fn grid_gap_4x4() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 50px 50px 50px 50px; grid-template-rows: 30px 30px 30px 30px; gap: 5px 10px;">
          <div expect_left="0" expect_top="0"></div>
          <div expect_left="60" expect_top="0"></div>
          <div expect_left="120" expect_top="0"></div>
          <div expect_left="180" expect_top="0"></div>
          <div expect_left="0" expect_top="35"></div>
          <div expect_left="60" expect_top="35"></div>
          <div expect_left="120" expect_top="35"></div>
          <div expect_left="180" expect_top="35"></div>
          <div expect_left="0" expect_top="70"></div>
          <div expect_left="60" expect_top="70"></div>
          <div expect_left="120" expect_top="70"></div>
          <div expect_left="180" expect_top="70"></div>
          <div expect_left="0" expect_top="105"></div>
          <div expect_left="60" expect_top="105"></div>
          <div expect_left="120" expect_top="105"></div>
          <div expect_left="180" expect_top="105"></div>
        </div>
    "#,
        true
    )
}

// Case: Gap with mixed fixed and auto columns
// Spec points:
//   - Gap is subtracted before distributing remaining space to auto columns
//   - Fixed columns get their specified size regardless of gap
// In this test:
//   - Container: width=300px, columns: 100px, auto, 100px; column-gap=20px
//   - Total gap: 20 * 2 = 40px
//   - Remaining for auto: 300 - 100 - 100 - 40 = 60px
//   - Column positions: 0, 120 (100+20), 200 (100+20+60+20)
#[test]
fn grid_gap_mixed_fixed_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px auto 100px; column-gap: 20px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="120" expect_width="60"></div>
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
        </div>
    "#,
        true
    )
}
