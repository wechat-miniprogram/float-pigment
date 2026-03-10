// WPT-style tests for CSS Grid Layout basics
// Reference: CSS Grid Layout Module Level 1
// https://www.w3.org/TR/css-grid-1/
//
// Basic grid concepts covered:
// - `display: grid` and `display: inline-grid`
// - Grid container and grid item relationships
// - Grid cell, grid area, and grid track definitions
// - Basic item sizing and positioning within grid cells

use crate::*;

// Case: Basic grid display
// Spec points:
//   - `display: grid` creates a block-level grid container
//   - Grid items are laid out within the grid formatting context
// In this test:
//   - Container: display=grid, 2 columns
//   - Items flow into grid cells in row-major order
#[test]
fn grid_display_basic() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 50px;" expect_left="0" expect_top="0"></div>
          <div style="height: 50px;" expect_left="100" expect_top="0"></div>
        </div>
    "#,
        true
    )
}

// Case: inline-grid display
// Spec points:
//   - `display: inline-grid` creates an inline-level grid container
//   - Grid items are still laid out within the grid formatting context
// In this test:
//   - Container: display=inline-grid, 2 columns of 50px each
//   - Container width matches content (100px total)
#[test]
fn grid_display_inline() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 50px 50px;" expect_width="100">
          <div style="height: 30px;" expect_left="0" expect_top="0" expect_width="50"></div>
          <div style="height: 30px;" expect_left="50" expect_top="0" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item with explicit width and height
// Spec points:
//   - Grid items can have explicit dimensions
//   - Explicit dimensions are respected within the grid cell
// In this test:
//   - Container: 2 columns of 100px
//   - Item has explicit width=50px, height=30px
//   - Item positioned at cell start (0, 0)
#[test]
fn grid_item_explicit_size() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 30px;" expect_left="0" expect_top="0" expect_width="50" expect_height="30"></div>
          <div style="width: 60px; height: 40px;" expect_left="100" expect_top="0" expect_width="60" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item stretches to fill cell by default
// Spec points:
//   - By default, grid items stretch to fill the cell in inline direction
//   - Items without explicit width take full cell width
// In this test:
//   - Container: 2 columns of 100px
//   - Items without width stretch to 100px (cell width)
#[test]
fn grid_item_stretch_default() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item with margin
// Spec points:
//   - Item margins are applied within the grid cell
//   - Margins offset the item from cell edges
// In this test:
//   - Container: 2 columns of 100px
//   - First item: margin=10px on all sides
//   - Item positioned at (10, 10) within cell
#[test]
fn grid_item_with_margin() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="margin: 10px; width: 50px; height: 30px;" expect_left="10" expect_top="10"></div>
          <div style="width: 50px; height: 30px;" expect_left="100" expect_top="0"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item with padding
// Spec points:
//   - Padding is part of the item's box model
//   - Padding increases the item's total size
// In this test:
//   - Container: 2 columns of 100px
//   - Item: width=50px, padding=10px
//   - Total item width: 50 + 10 + 10 = 70px
#[test]
fn grid_item_with_padding() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 30px; padding: 10px;" expect_left="0" expect_width="70" expect_height="50"></div>
          <div style="width: 50px; height: 30px;" expect_left="100" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item with border
// Spec points:
//   - Border is part of the item's box model
//   - Border adds to the item's total size (content-box)
// In this test:
//   - Container: 2 columns of 100px
//   - Item: width=50px, border=2px
//   - Total item width: 50 + 2 + 2 = 54px
#[test]
fn grid_item_with_border() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 30px; border: 2px solid black;" expect_left="0" expect_width="54" expect_height="34"></div>
          <div style="width: 50px; height: 30px;" expect_left="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item with box-sizing: border-box
// Spec points:
//   - `box-sizing: border-box` includes padding and border in width/height
//   - Item's specified width/height is the total box size
// In this test:
//   - Container: 2 columns of 100px
//   - Item: width=50px, padding=10px, box-sizing=border-box
//   - Total width remains 50px (padding included)
#[test]
fn grid_item_border_box() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 30px; padding: 5px; box-sizing: border-box;" expect_left="0" expect_width="50" expect_height="30"></div>
          <div style="width: 50px; height: 30px;" expect_left="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Nested grid containers
// Spec points:
//   - Grid items can themselves be grid containers
//   - Nested grids have independent formatting contexts
// In this test:
//   - Outer grid: 2 columns of 150px
//   - First cell contains nested grid with 2 columns of 70px
#[test]
fn grid_nested() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 150px 150px;">
          <div style="display: grid; grid-template-columns: 70px 70px;">
            <div style="height: 30px;" expect_left="0" expect_width="70"></div>
            <div style="height: 30px;" expect_left="70" expect_width="70"></div>
          </div>
          <div style="height: 50px;" expect_left="150"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid with fewer items than cells
// Spec points:
//   - Grid cells without items remain empty
//   - Empty cells still contribute to grid structure
// In this test:
//   - Container: 3 columns, but only 2 items
//   - First two cells filled, third cell empty
#[test]
fn grid_fewer_items_than_cells() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px 100px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid item min-content affects auto track sizing
// W3C §11.5 + §11.6:
//   - Auto tracks base_size = content min-content (§11.5)
//   - Maximize distributes free space equally (§11.6)
// In this test:
//   - Container: width=300px, 2 auto columns
//   - Track 1: base_size = 100px (child content)
//   - Track 2: base_size = 0px (no content width)
//   - Free space: 300 - 100 - 0 = 200px
//   - Maximize: each gets 100px (200 / 2)
//   - Final: track 1 = 200px, track 2 = 100px
#[test]
fn grid_item_min_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto auto;">
          <div expect_width="200">
            <div style="width: 100px; height: 30px;"></div>
          </div>
          <div expect_width="100">
            <div style="height: 30px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: Grid container with explicit height
// Spec points:
//   - Grid container can have explicit height
//   - Row sizing can be affected by container height
// In this test:
//   - Container: height=200px, 1 column, 2 rows of 50%
//   - Each row: 200 * 50% = 100px
#[test]
fn grid_container_explicit_height() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 200px; grid-template-columns: 100px; grid-template-rows: 50% 50%;">
          <div expect_top="0" expect_height="100"></div>
          <div expect_top="100" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid with text content
// Spec points:
//   - Grid items can contain text
//   - Text establishes the item's intrinsic size
// In this test:
//   - Container: 2 columns
//   - Items contain text, establishing min-content
#[test]
fn grid_with_text_content() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: auto auto;">
          <div>Hello</div>
          <div>World</div>
        </div>
    "#,
        true
    )
}

// Case: Grid with mixed content types
// Spec points:
//   - Grid can contain items with different content types
//   - Row height accommodates the tallest item
// In this test:
//   - Container: 2 columns
//   - First item: explicit height
//   - Second item: nested content
#[test]
fn grid_mixed_content() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 60px;" expect_top="0" expect_height="60"></div>
          <div expect_top="0">
            <div style="height: 40px;"></div>
          </div>
        </div>
    "#,
        true
    )
}

// Case: Empty grid container has zero height
// Spec points:
//   - Grid container with no items has zero content height
//   - Container width still respects constraints
// In this test:
//   - Container: 100px width, no items
//   - Height is 0
#[test]
fn grid_empty_zero_height() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 100px; grid-template-columns: 50px 50px;" expect_height="0" expect_width="100">
        </div>
    "#,
        true
    )
}

// Case: Grid item positioned within tall row
// Spec points:
//   - Items in the same row start at the same top position
//   - Row height is determined by the tallest item
// In this test:
//   - Container: 2 columns
//   - Row 1: items with height 30px and 80px, row height = 80px
//   - Both items start at top=0
#[test]
fn grid_item_in_tall_row() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 30px;" expect_top="0" expect_height="30"></div>
          <div style="height: 80px;" expect_top="0" expect_height="80"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple rows with varying heights
// Spec points:
//   - Each row's height is determined independently
//   - Row positions are cumulative
// In this test:
//   - Container: 2 columns, 3 rows
//   - Row heights: 40px, 60px, 30px
//   - Row positions: 0, 40, 100
#[test]
fn grid_multiple_rows_varying_heights() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="height: 40px;" expect_top="0"></div>
          <div style="height: 40px;" expect_top="0"></div>
          <div style="height: 60px;" expect_top="40"></div>
          <div style="height: 60px;" expect_top="40"></div>
          <div style="height: 30px;" expect_top="100"></div>
          <div style="height: 30px;" expect_top="100"></div>
        </div>
    "#,
        true
    )
}
