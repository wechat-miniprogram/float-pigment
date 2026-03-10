// Tests for `flex-direction` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - flex-direction sets the main axis direction
// - Values: row (default), row-reverse, column, column-reverse

use crate::*;

// Case: flex-direction: row
// Spec points:
// - Main axis is horizontal, left to right
// - Items placed left to right
// In this test:
// - Two items of 50px width
// - First at left=0, second at left=50
#[test]
fn flex_direction_row() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>

        </div>
    "#
    )
}

// Case: flex-direction: row-reverse
// Spec points:
// - Main axis is horizontal, right to left
// - Items placed right to left
// In this test:
// - Two items of 50px width
// - First (in DOM) at left=50, second at left=0
#[test]
fn flex_direction_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row-reverse;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>

        </div>
    "#
    )
}

// Case: flex-direction: column
// Spec points:
// - Main axis is vertical, top to bottom
// - Items placed top to bottom
// In this test:
// - Two items of 50px height
// - First at top=0, second at top=50
#[test]
fn flex_direction_column() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: column; height: 100px;"  expect_width="100" expect_height="100">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: flex-direction: column-reverse
// Spec points:
// - Main axis is vertical, bottom to top
// - Items placed bottom to top
// In this test:
// - Two items of 50px height
// - First (in DOM) at top=50 (bottom), second at top=0 (top)
#[test]
fn flex_direction_column_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: column-reverse; height: 100px;"  expect_width="100" expect_height="100">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="50"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: flex-direction: row with padding
// Spec points:
// - Padding creates space inside container
// - Items start after padding-left
// In this test:
// - Container: border-box, 100px wide, padding 10px left/right
// - Child at left=10 (after padding)
#[test]
fn flex_direction_row_with_parent_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row; padding-left: 10px; padding-right: 10px; box-sizing: border-box;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="10"></div>
        </div>
      "#
    )
}

// Case: flex-direction: row-reverse with padding
// Spec points:
// - In row-reverse, items start from right
// - Item positioned accounting for padding
// In this test:
// - Container: 100px - 20px padding = 80px content
// - Item 50px wide at right edge: 100-10-50 = 40
#[test]
fn flex_direction_row_reverse_with_parent_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row-reverse; padding-left: 10px; padding-right: 10px; box-sizing: border-box;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="40"></div>
        </div>
      "#
    )
}
