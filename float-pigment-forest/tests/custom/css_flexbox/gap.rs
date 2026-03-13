// Tests for `gap` property in CSS Flexbox
// Based on CSS Box Alignment Module Level 3:
// - gap sets spacing between flex items
// - row-gap and column-gap can be specified separately
// - Gap does not add space at container edges

use crate::*;

// Case: gap shorthand
// Spec points:
// - gap: 10px applies to both main and cross axis
// In this test:
// - Container: 100px, gap=10px
// - Two items with flex: 1
// - Available space after gap: 100 - 10 = 90px
// - Each item: 45px
// - Second item at left=55 (45 + 10)
#[test]
fn gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; gap: 10px;">
          <div style="height: 10px; flex: 1" expect_width="45"></div>
          <div style="height: 10px; flex: 1" expect_width="45" expect_left="55"></div>
        </div>
    "#
    )
}

// Case: column-gap in flex row
// Spec points:
// - column-gap affects spacing along main axis in row direction
// In this test:
// - Two items of 20px width, column-gap=10px
// - Second item at left=30
#[test]
fn column_gap_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: row; width: 100px; height: 100px; column-gap: 10px;">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="30"></div>
        </div>
    "#
    )
}

// Case: Percentage column-gap in flex row
// Spec points:
// - Percentage gap relative to container dimension
// In this test:
// - Container: 200px, column-gap=10% = 20px
// - Second item at left=40 (20px item + 20px gap)
#[test]
fn column_gap_with_percentage_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: row; width: 200px; height: 100px; column-gap: 10%; align-items: flex-start">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="40"></div>
        </div>
    "#
    )
}

// Case: column-gap in flex column with wrap
// Spec points:
// - In column wrap, column-gap affects cross axis between columns
// In this test:
// - Container: width=100px, height=20px, wrap
// - Items wrap to new column
// - Second item at left=55 (default stretch distributes remaining space)
#[test]
fn column_gap_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 20px; column-gap: 10px; flex-wrap: wrap;">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="55"></div>
        </div>
    "#
    )
}

// Case: column-gap in flex column with align-content: flex-start
// Spec points:
// - align-content: flex-start packs columns to start
// In this test:
// - Columns packed to left, second at left=30 (20px + 10px gap)
#[test]
fn column_gap_in_flex_column_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 20px; column-gap: 10px; flex-wrap: wrap; align-content: flex-start">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="30"></div>
        </div>
    "#
    )
}

// Case: Percentage column-gap in flex column
// Spec points:
// - Percentage gap in column direction still relative to width
// In this test:
// - Container: 100px, column-gap=10% = 10px
// - Second item at left=30
#[test]
fn column_gap_with_percentage_in_flex_column_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 20px; column-gap: 10%; flex-wrap: wrap; align-content: flex-start">
          <div style="width: 20px; height: 20px; flex-shrink: 0" expect_left="0"></div>
          <div style="width: 20px; height: 20px; flex-shrink: 0" expect_left="30"></div>
        </div>
    "#
    )
}

// Case: row-gap in flex row with wrap
// Spec points:
// - row-gap affects cross axis spacing between wrapped lines
// In this test:
// - Container: 100px wide, height=100px, wrap
// - Items 50px wide, 2 per row
// - Row gap with default stretch distributes space
#[test]
fn row_gap_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction:row; width: 100px; height: 100px; row-gap: 10px; flex-wrap: wrap;">
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="50"  expect_top="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="55"></div>
          <div style="height: 10px; width: 50px;" expect_left="50" expect_top="55"></div>
        </div>
    "#
    )
}

// Case: row-gap with align-content: flex-start
// Spec points:
// - Lines packed to top, gap applied between
// In this test:
// - First row at top=0, second at top=20 (10px + 10px gap)
#[test]
fn row_gap_in_flex_row_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction:row; width: 100px; height: 100px; row-gap: 10px; flex-wrap: wrap; align-content: flex-start;">
          <div style="height: 10px; width: 50px;" expect_left="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="50"></div>
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="20"></div>
          <div style="height: 10px; width: 50px;" expect_left="50" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: Percentage row-gap
// Spec points:
// - Percentage row-gap relative to container height (in cross axis)
// In this test:
// - Container: 100px high, row-gap=10% = 10px
#[test]
fn row_gap_with_percentage_in_flex_row_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction:row; width: 100px; height: 100px; row-gap: 10%; flex-wrap: wrap; align-content: flex-start;">
          <div style="height: 10px; width: 50px;" expect_left="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="50"></div>
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="20"></div>
          <div style="height: 10px; width: 50px;" expect_left="50" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: row-gap in flex column
// Spec points:
// - In column direction, row-gap affects main axis
// In this test:
// - Two items of 30px height, row-gap=10px
// - Second item at top=40
#[test]
fn row_gap_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 100px; row-gap: 10px;">
          <div style="width: 100px; height: 30px;"></div>
          <div style="width: 100px; height: 30px;"expect_top="40"></div>
        </div>
    "#
    )
}

// Case: Percentage row-gap in flex column
// Spec points:
// - Percentage row-gap relative to container dimension
// In this test:
// - Container: 100px, row-gap=10% = 10px
#[test]
fn row_gap_with_percentage_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 100px; row-gap: 10%;">
          <div style="width: 100px; height: 30px;"></div>
          <div style="width: 100px; height: 30px;"expect_top="40"></div>
        </div>
    "#
    )
}

// Case: Flex items with gap should shrink to fit
// Spec points:
// - Gap reduces available space for items
// - Items shrink proportionally when needed
// In this test:
// - Container: 50px height, gap=10px
// - Two items of 30px each = 60px, gap=10px, total needed=70px
// - Must shrink to fit: each item shrinks to 20px
#[test]
fn flex_item_with_gap_should_shrink_to_fit() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; height: 50px; flex-direction: column; gap: 10px;" expect_height="50">
          <div style="height: 30px;" expect_top="0" expect_height="20"></div>
          <div style="height: 30px;" expect_top="30" expect_height="20"></div>
        </div>
    "#
    )
}
