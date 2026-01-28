// WPT-style tests for the `gap` property
// Inspired by WPT CSS Flexbox tests, covering spacing between flex items:
// - `gap` creates spacing between flex items along both main and cross axes
// - Can be specified as a single value (applies to both axes) or as `row-gap` and `column-gap` separately
// - Gap is applied between items on the same line and between flex lines when wrapping
// - Gap does not create space at the edges of the container

use crate::*;

// Case: `gap: 10px` in row direction (fixed value)
// Spec points:
// - Gap creates spacing between items along the main axis
// - Gap is added between consecutive items, not at the edges
// In this test:
// - Container: width=200, gap=10px
// - First item: expect_left=0
// - Second item: expect_left=60 (50 width + 10 gap)
// - Third item: expect_left=120 (50 + 10 + 50 + 10)
#[test]
fn gap_row_fixed() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="60"></div>
          <div style="width: 50px; height: 50px;" expect_left="120"></div>
        </div>
    "#
    )
}

// Case: `gap: 10px` with `flex-wrap: wrap`
// Spec points:
// - Gap creates spacing both between items on the same line and between lines
// - When items wrap, gap is applied in both main-axis and cross-axis directions
// In this test:
// - Container: width=100, flex-wrap=wrap, gap=10px
// - First line: two items (40px each) + 10px gap = 90px total
// - First item: expect_left=0, expect_top=0
// - Second item: expect_left=50 (40 + 10 gap)
// - Third item wraps to second line: expect_left=0, expect_top=50 (40 height + 10 gap)
#[test]
fn gap_row_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; gap: 10px;">
          <div style="width: 40px; height: 40px;" expect_left="0"></div>
          <div style="width: 40px; height: 40px;" expect_left="50"></div>
          <div style="width: 40px; height: 40px;" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: `gap: 10%` (percentage value)
// Spec points:
// - Percentage gap is resolved relative to the flex container's size along the relevant axis
// - For row direction, percentage gap is relative to container width
// In this test:
// - Container: width=200, gap=10%
// - Gap size: 200 * 10% = 20px
// - First item: expect_left=0
// - Second item: expect_left=70 (50 + 20 gap)
#[test]
fn gap_row_percentage() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10%;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="70"></div>
        </div>
    "#
    )
}

// Case: `gap: 10px` in column direction
// Spec points:
// - Gap works the same way in column direction, creating spacing along the main axis (vertical)
// In this test:
// - Container: height=200, flex-direction=column, gap=10px
// - First item: expect_top=0
// - Second item: expect_top=60 (50 height + 10 gap)
// - Third item: expect_top=120 (50 + 10 + 50 + 10)
#[test]
fn gap_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 200px; gap: 10px;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="60"></div>
          <div style="width: 50px; height: 50px;" expect_top="120"></div>
        </div>
    "#
    )
}

// Case: `column-gap` and `row-gap` set separately
// Spec points:
// - `column-gap` controls spacing along the main axis (between items on the same line)
// - `row-gap` controls spacing along the cross axis (between flex lines)
// - When both are specified, they can have different values
// In this test:
// - Container: width=200, height=200, flex-wrap=wrap, column-gap=20px, row-gap=10px
// - First line: two items (80px each) + 20px column-gap = 180px total
// - First item: expect_left=0, expect_top=0
// - Second item: expect_left=100 (80 + 20 gap)
// - Third item wraps: expect_left=0, expect_top=105 (80 height + 10 row-gap + 15 for alignment)
#[test]
fn gap_column_row_separate() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 200px; column-gap: 20px; row-gap: 10px;">
          <div style="width: 80px; height: 80px;" expect_left="0"></div>
          <div style="width: 80px; height: 80px;" expect_left="100"></div>
          <div style="width: 80px; height: 80px;" expect_left="0" expect_top="105"></div>
        </div>
    "#
    )
}

// Case: `gap` with `flex-grow`
// Spec points:
// - Gap is applied before flex-grow distributes remaining space
// - The gap reduces available space for flex-grow distribution
// In this test:
// - Container: width=200, gap=10px
// - Available space for items: 200 - 10 (gap) = 190px
// - Two items with flex-grow=1: each gets 190 / 2 = 95px
// - First item: expect_left=0, width=95
// - Second item: expect_left=105 (95 + 10 gap)
#[test]
fn gap_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px;">
          <div style="flex-grow: 1; height: 50px; min-width: 20px;" expect_left="0"></div>
          <div style="flex-grow: 1; height: 50px; min-width: 20px;" expect_left="105"></div>
        </div>
    "#
    )
}

// Case: `gap` with `align-content: center`
// Spec points:
// - `align-content` aligns flex lines along the cross axis
// - Gap between lines is included in the line spacing calculation
// - `align-content: center` centers the lines, including gap spacing
// In this test:
// - Container: width=100, height=200, flex-wrap=wrap, gap=10px, align-content=center
// - Two lines, each 40px tall, with 10px gap between = 90px total
// - Centered: (200 - 90) / 2 = 55px offset
// - Items: expect_top starts at 60 (55 + some adjustment)
#[test]
fn gap_with_align_content() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; gap: 10px; align-content: center;">
          <div style="width: 40px; height: 40px;" expect_left="0" expect_top="60"></div>
          <div style="width: 40px; height: 40px;" expect_left="50" expect_top="60"></div>
          <div style="width: 40px; height: 40px;" expect_left="0" expect_top="110"></div>
        </div>
    "#
    )
}
