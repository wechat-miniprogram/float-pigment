// WPT-style tests for the `flex-direction` property
// Inspired by WPT CSS Flexbox tests, covering the main axis direction control:
// - `flex-direction: row` (default): main axis is horizontal, items flow left-to-right
// - `flex-direction: row-reverse`: main axis is horizontal, items flow right-to-left
// - `flex-direction: column`: main axis is vertical, items flow top-to-bottom
// - `flex-direction: column-reverse`: main axis is vertical, items flow bottom-to-top
// The flex-direction determines which axis is the main axis (for flex-grow/shrink, justify-content)
// and which is the cross axis (for align-items, align-self)

use crate::*;

// Case: `flex-direction: row` (default behavior)
// Spec points:
// - The main axis is horizontal (left-to-right in LTR writing mode)
// - Flex items are laid out along the main axis in document order
// - The cross axis is vertical
// In this test:
// - Container: width=200, flex-direction=row
// - First item: width=50, expect_left=0 (starts at left edge)
// - Second item: width=50, expect_left=50 (placed immediately after first item)
#[test]
fn flex_direction_row() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `flex-direction: row-reverse`
// Spec points:
// - The main axis is horizontal, but items flow right-to-left
// - Flex items are laid out in reverse document order along the main axis
// - The cross axis is still vertical
// In this test:
// - Container: width=200, flex-direction=row-reverse
// - First item (in DOM order): expect_left=150 (positioned from right: 200 - 50 = 150)
// - Second item (in DOM order): expect_left=100 (positioned to the left of first item)
#[test]
fn flex_direction_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row-reverse;">
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
        </div>
    "#
    )
}

// Case: `flex-direction: column`
// Spec points:
// - The main axis is vertical (top-to-bottom)
// - Flex items are laid out along the main axis in document order
// - The cross axis is horizontal
// In this test:
// - Container: height=200, flex-direction=column
// - First item: height=50, expect_top=0 (starts at top)
// - Second item: height=50, expect_top=50 (placed immediately below first item)
#[test]
fn flex_direction_column() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; flex-direction: column;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: `flex-direction: column-reverse`
// Spec points:
// - The main axis is vertical, but items flow bottom-to-top
// - Flex items are laid out in reverse document order along the main axis
// - The cross axis is still horizontal
// In this test:
// - Container: height=200, flex-direction=column-reverse
// - First item (in DOM order): expect_top=150 (positioned from bottom: 200 - 50 = 150)
// - Second item (in DOM order): expect_top=100 (positioned above first item)
#[test]
fn flex_direction_column_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; flex-direction: column-reverse;">
          <div style="width: 50px; height: 50px;" expect_left="0" expect_top="150"></div>
          <div style="width: 50px; height: 50px;" expect_left="0" expect_top="100"></div>
        </div>
    "#
    )
}

// Case: `flex-direction: row` with `justify-content: center`
// Spec points:
// - `justify-content` aligns items along the main axis
// - With `flex-direction: row`, `justify-content: center` centers items horizontally
// - Available space is distributed as free space before and after the items
// In this test:
// - Container: width=200, flex-direction=row, justify-content=center
// - Total item width: 50 + 50 = 100
// - Free space: 200 - 100 = 100
// - Centered position: free space / 2 = 50
// - First item: expect_left=50
// - Second item: expect_left=100
#[test]
fn flex_direction_row_with_justify_content() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
        </div>
    "#
    )
}

// Case: `flex-direction: column` with `align-items: center`
// Spec points:
// - `align-items` aligns items along the cross axis
// - With `flex-direction: column`, the cross axis is horizontal
// - `align-items: center` centers items horizontally (cross-axis alignment)
// In this test:
// - Container: width=200, height=200, flex-direction=column, align-items=center
// - Item width: 50
// - Centered horizontally: (200 - 50) / 2 = 75
// - First item: expect_left=75, expect_top=0
// - Second item: expect_left=75, expect_top=50
#[test]
fn flex_direction_column_with_align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; width: 200px; flex-direction: column; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="75"></div>
          <div style="width: 50px; height: 50px;" expect_left="75" expect_top="50"></div>
        </div>
    "#
    )
}
