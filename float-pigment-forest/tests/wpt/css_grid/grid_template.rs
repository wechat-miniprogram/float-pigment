// WPT-style tests for `grid-template-rows` and `grid-template-columns`
// Reference: CSS Grid Layout Module Level 1
// https://www.w3.org/TR/css-grid-1/#explicit-grids
//
// The `grid-template-columns` and `grid-template-rows` properties define
// the line names and track sizing functions of the explicit grid.
//
// Track sizing functions can include:
// - <length>: Fixed size in pixels, etc.
// - <percentage>: Percentage of the grid container's size
// - <flex> (fr): Flexible length, distributes remaining space
// - auto: Based on content size
// - min-content: Smallest content size
// - max-content: Largest content size

use crate::*;

// Case: Fixed pixel track sizes
// Spec points:
//   - Fixed <length> values define exact track sizes
//   - Track sizes are independent of content
// In this test:
//   - Container: 3 columns of 100px, 50px, 150px
//   - Items fill each column with exact sizes
//   - Column positions: 0, 100, 150
#[test]
fn grid_template_columns_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 50px 150px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="50"></div>
          <div style="height: 50px;" expect_left="150" expect_width="150"></div>
        </div>
    "#,
        true
    )
}

// Case: Fixed pixel row sizes
// Spec points:
//   - Fixed <length> values define exact row heights
//   - Row heights are independent of content
// In this test:
//   - Container: 3 rows of 30px, 50px, 40px
//   - Items fill each row with exact heights
//   - Row positions: 0, 30, 80
#[test]
fn grid_template_rows_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: 30px 50px 40px;">
          <div expect_top="0" expect_height="30"></div>
          <div expect_top="30" expect_height="50"></div>
          <div expect_top="80" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto track sizing with content
// Spec points:
//   - `auto` tracks size to fit their content
//   - Multiple `auto` tracks share remaining space equally
// In this test:
//   - Container: width=300px, 3 auto columns
//   - Each column: 300 / 3 = 100px
//   - Column positions: 0, 100, 200
#[test]
fn grid_template_columns_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto auto auto;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Mixed fixed and auto columns
// Spec points:
//   - Fixed tracks get their specified size first
//   - `auto` tracks share the remaining space
// In this test:
//   - Container: width=400px, columns: auto, 100px, auto
//   - Remaining for auto: 400 - 100 = 300px, each auto = 150px
//   - Column positions: 0, 150, 250
#[test]
fn grid_template_columns_mixed_fixed_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: auto 100px auto;">
          <div style="height: 50px;" expect_left="0" expect_width="150"></div>
          <div style="height: 50px;" expect_left="150" expect_width="100"></div>
          <div style="height: 50px;" expect_left="250" expect_width="150"></div>
        </div>
    "#,
        true
    )
}

// Case: Percentage column sizes
// Spec points:
//   - Percentage tracks are relative to the grid container's inline size
//   - 50% of 200px = 100px
// In this test:
//   - Container: width=200px, columns: 50%, 25%, 25%
//   - Column widths: 100px, 50px, 50px
//   - Column positions: 0, 100, 150
#[test]
fn grid_template_columns_percentage() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 50% 25% 25%;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="50"></div>
          <div style="height: 50px;" expect_left="150" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Percentage row sizes
// Spec points:
//   - Percentage rows are relative to the grid container's block size
//   - Container must have explicit height for percentage rows
// In this test:
//   - Container: height=200px, rows: 50%, 30%, 20%
//   - Row heights: 100px, 60px, 40px
//   - Row positions: 0, 100, 160
#[test]
fn grid_template_rows_percentage() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 200px; grid-template-columns: 100px; grid-template-rows: 50% 30% 20%;">
          <div expect_top="0" expect_height="100"></div>
          <div expect_top="100" expect_height="60"></div>
          <div expect_top="160" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto rows with content height
// Spec points:
//   - `auto` row height is determined by the tallest item in that row
//   - All items in the row share the same height
// In this test:
//   - Container: 2 columns, auto rows
//   - Row 1: items with height 30px and 50px, row height = 50px
//   - Row 2: items with height 20px, row height = 20px
#[test]
fn grid_template_rows_auto_with_content() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 30px;" expect_top="0" expect_height="30"></div>
          <div style="height: 50px;" expect_top="0" expect_height="50"></div>
          <div style="height: 20px;" expect_top="50" expect_height="20"></div>
          <div style="height: 20px;" expect_top="50" expect_height="20"></div>
        </div>
    "#,
        true
    )
}

// Case: Single column grid
// Spec points:
//   - Items stack vertically in a single column
//   - Each item forms its own row
// In this test:
//   - Container: 1 column of 200px
//   - 3 items with different heights
//   - Row positions: 0, 30, 80
#[test]
fn grid_template_single_column() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 200px;">
          <div style="height: 30px;" expect_top="0" expect_left="0" expect_width="200"></div>
          <div style="height: 50px;" expect_top="30" expect_left="0" expect_width="200"></div>
          <div style="height: 40px;" expect_top="80" expect_left="0" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: Single row grid
// Spec points:
//   - Items flow horizontally in a single row
//   - Each item forms its own column (if no explicit columns defined)
// In this test:
//   - Container: 1 row of 50px, 3 columns
//   - All items in same row with height 50px
#[test]
fn grid_template_single_row() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px 100px; grid-template-rows: 50px;">
          <div expect_left="0" expect_top="0" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_height="50"></div>
          <div expect_left="200" expect_top="0" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid with more items than explicit cells
// Spec points:
//   - Items exceeding explicit grid create implicit rows
//   - Implicit rows use `grid-auto-rows` sizing (default: auto)
// In this test:
//   - Container: 2 columns, 1 explicit row of 50px
//   - 4 items: first 2 in explicit row, last 2 in implicit row
//   - Implicit row height: auto (content-based)
#[test]
fn grid_template_overflow_to_implicit() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; grid-template-rows: 50px;">
          <div expect_left="0" expect_top="0" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_height="50"></div>
          <div style="height: 30px;" expect_left="0" expect_top="50" expect_height="30"></div>
          <div style="height: 30px;" expect_left="100" expect_top="50" expect_height="30"></div>
        </div>
    "#,
        true
    )
}

// Case: Empty grid container
// Spec points:
//   - Grid container with no items has zero content size
//   - Explicit rows/columns still define the grid structure
// In this test:
//   - Container: defined columns but no items
//   - Container height defaults to 0
#[test]
fn grid_template_empty_container() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px 100px 100px;" expect_height="0">
        </div>
    "#,
        true
    )
}

// Case: Grid with nested content affecting auto sizing
// Spec points:
//   - `auto` tracks consider the min-content and max-content of items
//   - Nested elements contribute to item sizing
// W3C §11.5 + §11.6:
//   - Auto tracks base_size = content min-content (§11.5)
//   - Maximize distributes free space equally (§11.6)
// In this test:
//   - Container: width=300px, 2 auto columns
//   - Track 1: base_size = 150px (child content)
//   - Track 2: base_size = 0px (no content width)
//   - Free space: 300 - 150 - 0 = 150px
//   - Maximize: each gets 75px (150 / 2)
//   - Final: track 1 = 225px, track 2 = 75px
#[test]
fn grid_template_auto_with_nested_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto auto;">
          <div expect_width="225">
            <div style="width: 150px; height: 50px;"></div>
          </div>
          <div expect_width="75">
            <div style="height: 30px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: Equal width columns with auto
// Spec points:
//   - Multiple `auto` columns share remaining space equally
//   - When no content forces different sizing, auto columns are equal
// In this test:
//   - Container: width=400px, 4 auto columns
//   - Each column: 400 / 4 = 100px
#[test]
fn grid_template_equal_auto_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: auto auto auto auto;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid with explicit rows and columns
// Spec points:
//   - Explicit grid is defined by grid-template-rows and grid-template-columns
//   - Items are placed into the grid in row-major order by default
// In this test:
//   - Container: 3x3 grid with fixed sizes
//   - 9 items fill the grid in order
#[test]
fn grid_template_explicit_3x3() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px 100px; grid-template-rows: 50px 50px 50px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="200" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="50" expect_width="100" expect_height="50"></div>
          <div expect_left="200" expect_top="50" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="100" expect_width="100" expect_height="50"></div>
          <div expect_left="200" expect_top="100" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}
