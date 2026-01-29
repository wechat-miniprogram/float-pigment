// WPT-style tests for the `justify-content` property
// Inspired by WPT CSS Flexbox tests, covering main-axis alignment:
// - `justify-content` aligns flex items along the main axis
// - Values: flex-start, start, center, flex-end, end, space-between, space-around, space-evenly, left, right
// - The property distributes free space (container size minus sum of flex items' base sizes) along the main axis
// - Each value has specific rules for how free space is distributed

use crate::*;

// Case: `justify-content: flex-start` (default behavior)
// Spec points:
// - Items are packed toward the start of the main axis
// - No free space is distributed before the first item
// - All free space appears after the last item
// In this test:
// - Container: width=300, justify-content=flex-start
// - Items: 50px each, total 100px
// - First item: expect_left=0 (at start)
// - Second item: expect_left=50 (immediately after first)
#[test]
fn justify_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-start;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `justify-content: start`
// Spec points:
// - Behaves like `flex-start` in LTR writing mode
// - Items are packed toward the start of the main axis
// In this test:
// - Same behavior as flex-start: expect_left=0, 50
#[test]
fn justify_content_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: start;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `justify-content: center`
// Spec points:
// - Items are centered along the main axis
// - Free space is distributed equally before and after the items
// In this test:
// - Container: width=300, items: 50px each (total 100px)
// - Free space: 300 - 100 = 200px
// - Centered: free space / 2 = 100px offset
// - First item: expect_left=100
// - Second item: expect_left=150
#[test]
fn justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
        </div>
    "#
    )
}

// Case: `justify-content: flex-end`
// Spec points:
// - Items are packed toward the end of the main axis
// - All free space appears before the first item
// In this test:
// - Container: width=300, items: 50px each (total 100px)
// - Free space: 200px before items
// - First item: expect_left=200
// - Second item: expect_left=250
#[test]
fn justify_content_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-end;">
          <div style="width: 50px; height: 50px;" expect_left="200"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}

// Case: `justify-content: end`
// Spec points:
// - Behaves like `flex-end` in LTR writing mode
// - Items are packed toward the end of the main axis
// In this test:
// - Same behavior as flex-end: expect_left=200, 250
#[test]
fn justify_content_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: end;">
          <div style="width: 50px; height: 50px;" expect_left="200"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}

// Case: `justify-content: space-between`
// Spec points:
// - Items are evenly distributed with equal space between them
// - No space before the first item or after the last item
// In this test:
// - Container: width=200, items: 50px each (total 100px)
// - Free space: 100px
// - One gap between two items: 100px
// - First item: expect_left=0
// - Second item: expect_left=150 (50 + 100 gap)
#[test]
fn justify_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; justify-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
        </div>
    "#
    )
}

// Case: `justify-content: space-around`
// Spec points:
// - Items are evenly distributed with equal space around each item
// - Space before first and after last item is half the space between items
// In this test:
// - Container: width=200, items: 50px each (total 100px), free space: 100px
// - Space around each item: 100 / 2 = 50px
// - Half space at ends: 25px before first, 25px between, 25px after second
// - First item: expect_left=25
// - Second item: expect_left=125 (25 + 50 + 50)
#[test]
fn justify_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; justify-content: space-around;">
          <div style="width: 50px; height: 50px;" expect_left="25"></div>
          <div style="width: 50px; height: 50px;" expect_left="125"></div>
        </div>
    "#
    )
}

// Case: `justify-content: space-evenly`
// Spec points:
// - Items are evenly distributed with equal space between all items and at the ends
// - All gaps (before first, between items, after last) are equal
// In this test:
// - Container: width=200, items: 50px each (total 100px), free space: 100px
// - Three gaps (before first, between, after second): 100 / 3 ≈ 33.333px each
// - First item: expect_left=33 (rounded)
// - Second item: expect_left=117 (33.333 + 50 + 33.333, rounded)
#[test]
fn justify_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; justify-content: space-evenly;">
          <div style="width: 50px; height: 50px;" expect_left="33"></div>
          <div style="width: 50px; height: 50px;" expect_left="117"></div>
        </div>
    "#
    )
}

// Case: `justify-content: left`
// Spec points:
// - Behaves like `flex-start` in LTR writing mode
// - Items are aligned to the left edge
// In this test:
// - Same behavior as flex-start: expect_left=0, 50
#[test]
fn justify_content_left() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: left;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `justify-content: right`
// Spec points:
// - Behaves like `flex-end` in LTR writing mode
// - Items are aligned to the right edge
// In this test:
// - Same behavior as flex-end: expect_left=200, 250
#[test]
fn justify_content_right() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: right;">
          <div style="width: 50px; height: 50px;" expect_left="200"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}

// Case: `justify-content: center` with `flex-direction: column`
// Spec points:
// - `justify-content` works along the main axis, which is vertical in column direction
// - Items are centered vertically
// In this test:
// - Container: height=300, flex-direction=column, justify-content=center
// - Items: 50px each (total 100px)
// - Free space: 300 - 100 = 200px
// - Centered: free space / 2 = 100px offset
// - First item: expect_top=100
// - Second item: expect_top=150
#[test]
fn justify_content_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_top="150"></div>
        </div>
    "#
    )
}

// Case: `justify-content: space-between` with three items
// Spec points:
// - With multiple items, space-between distributes free space evenly between items only
// - No space at the ends
// In this test:
// - Container: width=300, items: 50px each (total 150px)
// - Free space: 150px
// - Two gaps between three items: 150 / 2 = 75px each
// - First item: expect_left=0
// - Second item: expect_left=125 (50 + 75)
// - Third item: expect_left=250 (125 + 50 + 75)
#[test]
fn justify_content_space_between_three_items() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="125"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}
