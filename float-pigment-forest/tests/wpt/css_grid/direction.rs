// WPT-style tests for CSS direction property in Grid Layout
// Reference: CSS Grid Layout Module Level 1 §10
// https://www.w3.org/TR/css-grid-1/#grid-align
//
// CSS Writing Modes Level 3 §2.1
// https://www.w3.org/TR/css-writing-modes-3/#direction
//
// The `direction` property affects the inline axis direction:
// - `direction: ltr`: inline axis goes left-to-right
// - `direction: rtl`: inline axis goes right-to-left
//
// In Grid layout, this affects:
// 1. The starting edge of the inline axis (justify-items/justify-self `start`/`end`)
// 2. The placement order along the inline axis

use crate::*;

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl - Basic Layout
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with fixed columns
// W3C Spec:
//   - In RTL, the inline axis flows right-to-left
//   - Grid tracks are placed from the inline-start (right in RTL)
//   - With justify-content: normal (default = start), tracks align to right
//
// Layout calculation:
//   - Container: width=400px, direction=rtl
//   - Columns: 100px + 100px = 200px total
//   - Free space: 400 - 200 = 200px
//   - justify-content: normal (start in RTL = right)
//   - Column 1 starts at right: left = 400 - 100 = 300
//   - Column 2 follows: left = 400 - 200 = 200
//
// Items:
//   - Item 1 (column 1): left = 300
//   - Item 2 (column 2): left = 200
#[test]
fn grid_direction_rtl_basic() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px;">
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: direction: ltr (default) - for comparison
// Layout calculation:
//   - Container: width=400px, direction=ltr (default)
//   - Columns: 100px + 100px = 200px
//   - justify-content: normal (start in LTR = left)
//   - Column 1 starts at left: left = 0
//   - Column 2 follows: left = 100
#[test]
fn grid_direction_ltr_basic() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: 100px 100px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl with justify-items
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with justify-items: start
// W3C Spec §10.3:
//   - justify-items aligns items within their grid area
//   - `start` aligns to the inline-start edge (right in RTL)
//
// Layout calculation:
//   - Container: width=400px, direction=rtl
//   - Columns: 100px + 100px
//   - justify-items: start (aligns item to right edge of cell in RTL)
//   - Item 1 in cell [0,0]: cell starts at left=300, item with width=50 at right of cell
//     → item left = 300 + (100 - 50) = 350
//   - Item 2 in cell [0,1]: cell starts at left=200, item at right of cell
//     → item left = 200 + (100 - 50) = 250
#[test]
fn grid_direction_rtl_justify_items_start() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px; justify-items: start;">
          <div style="width: 50px; height: 50px;" expect_left="350" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="250" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: direction: rtl with justify-items: end
// W3C Spec §10.3:
//   - `end` aligns to the inline-end edge (left in RTL)
//
// Layout calculation:
//   - Item 1 in cell [0,0]: cell left=300, item at left edge of cell
//     → item left = 300
//   - Item 2 in cell [0,1]: cell left=200, item at left edge of cell
//     → item left = 200
#[test]
fn grid_direction_rtl_justify_items_end() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px; justify-items: end;">
          <div style="width: 50px; height: 50px;" expect_left="300" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="200" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: direction: rtl with justify-items: center
// W3C Spec §10.3:
//   - `center` centers items within the grid area
//   - Direction does not affect centering
//
// Layout calculation:
//   - Item 1 in cell [0,0]: cell left=300, cell width=100, item width=50
//     → item left = 300 + (100 - 50) / 2 = 325
//   - Item 2 in cell [0,1]: cell left=200
//     → item left = 200 + (100 - 50) / 2 = 225
#[test]
fn grid_direction_rtl_justify_items_center() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px; justify-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="325" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="225" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl with justify-content
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with justify-content: center
// W3C Spec §10.5:
//   - justify-content distributes space among grid tracks
//   - `center` centers all tracks within the container
//
// Layout calculation:
//   - Container: width=400px
//   - Columns: 100px + 100px = 200px
//   - Free space: 400 - 200 = 200px
//   - Centering offset: 200 / 2 = 100px from each side
//   - Column 1 (in RTL, rightmost): left = 100 + 100 = 200
//   - Column 2 (in RTL, leftmost): left = 100
#[test]
fn grid_direction_rtl_justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px; justify-content: center;">
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: direction: rtl with justify-content: space-between
// W3C Spec §10.5:
//   - `space-between` distributes free space evenly between tracks
//   - First and last tracks align to container edges
//
// Layout calculation:
//   - Container: width=400px, direction=rtl
//   - Columns: 100px + 100px = 200px
//   - Free space: 400 - 200 = 200px (all goes between tracks)
//   - In RTL: Column 1 at right edge, Column 2 at left edge
//   - Column 1: left = 400 - 100 = 300
//   - Column 2: left = 0
#[test]
fn grid_direction_rtl_justify_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px; justify-content: space-between;">
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl with physical properties (left/right)
// W3C CSS Box Alignment §4.1: Physical keywords (left/right) are NOT affected
// by direction, unlike logical keywords (start/end).
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with justify-self: left (physical)
// W3C Spec CSS Box Alignment §4.1:
//   - `left` is a physical keyword, NOT affected by direction
//   - Item aligns to left edge regardless of RTL
//
// Layout calculation:
//   - Cell [0,0]: left=300, width=100
//   - justify-self: left → item at left edge of cell
//   - Item left = 300 (cell left) + 0 = 300
#[test]
fn grid_direction_rtl_justify_self_left_physical() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 50px; justify-self: left;" expect_left="300" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: left;" expect_left="200" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: direction: rtl with justify-self: right (physical)
// W3C Spec CSS Box Alignment §4.1:
//   - `right` is a physical keyword, NOT affected by direction
//   - Item aligns to right edge regardless of RTL
//
// Layout calculation:
//   - Cell [0,0]: left=300, width=100, right_edge=400
//   - justify-self: right → item at right edge of cell
//   - Item left = 300 + (100 - 50) = 350
#[test]
fn grid_direction_rtl_justify_self_right_physical() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 50px; justify-self: right;" expect_left="350" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: right;" expect_left="250" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl with multiple rows
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with 2x2 grid
// W3C Spec:
//   - direction only affects inline (horizontal) axis
//   - block (vertical) axis remains top-to-bottom
//
// Layout calculation:
//   - Container: 400x200, direction=rtl
//   - Columns: 100px 100px from right
//   - Rows: 100px 100px from top
//   - Item 1 (row 0, col 0): left=300, top=0
//   - Item 2 (row 0, col 1): left=200, top=0
//   - Item 3 (row 1, col 0): left=300, top=100
//   - Item 4 (row 1, col 1): left=200, top=100
#[test]
fn grid_direction_rtl_multirow() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 200px; direction: rtl; grid-template-columns: 100px 100px; grid-template-rows: 100px 100px;">
          <div expect_left="300" expect_top="0" expect_width="100" expect_height="100"></div>
          <div expect_left="200" expect_top="0" expect_width="100" expect_height="100"></div>
          <div expect_left="300" expect_top="100" expect_width="100" expect_height="100"></div>
          <div expect_left="200" expect_top="100" expect_width="100" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl with fr units
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with fr units
// W3C Spec §7.2.4:
//   - fr units divide remaining space proportionally
//   - Direction affects track placement order, not fr calculation
//
// Layout calculation:
//   - Container: width=400px, direction=rtl
//   - Columns: 1fr 1fr = 200px each
//   - In RTL: Column 1 at right, Column 2 at left
//   - Column 1: left = 400 - 200 = 200
//   - Column 2: left = 0
#[test]
fn grid_direction_rtl_with_fr() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 1fr 1fr;">
          <div style="height: 50px;" expect_left="200" expect_width="200"></div>
          <div style="height: 50px;" expect_left="0" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: direction: rtl with mixed fr and fixed
// Layout calculation:
//   - Container: width=400px, direction=rtl
//   - Columns: 100px + 1fr
//   - 1fr = 400 - 100 = 300px
//   - In RTL: Column 1 (100px) at right, Column 2 (300px) at left
//   - Column 1: left = 400 - 100 = 300
//   - Column 2: left = 0
#[test]
fn grid_direction_rtl_mixed_fr_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 1fr;">
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
          <div style="height: 50px;" expect_left="0" expect_width="300"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// direction: rtl with gap
// ═══════════════════════════════════════════════════════════════════════════

// Case: direction: rtl with column-gap
// W3C Spec §10.1:
//   - Gaps are inserted between tracks
//   - Direction affects track order, not gap size
//
// Layout calculation:
//   - Container: width=400px, direction=rtl, column-gap=20px
//   - Columns: 100px + 20px (gap) + 100px = 220px
//   - Free space: 400 - 220 = 180px (at inline-end = left in RTL)
//   - Column 1: left = 400 - 100 = 300
//   - Gap: 20px
//   - Column 2: left = 400 - 100 - 20 - 100 = 180
#[test]
fn grid_direction_rtl_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; direction: rtl; grid-template-columns: 100px 100px; column-gap: 20px;">
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
          <div style="height: 50px;" expect_left="180" expect_width="100"></div>
        </div>
    "#,
        true
    )
}
