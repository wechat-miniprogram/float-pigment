// WPT-style tests for the `flex-wrap` property
// Inspired by WPT CSS Flexbox tests, covering multi-line flex container behavior:
// - `flex-wrap: nowrap` (default): all items stay on a single line, may be compressed
// - `flex-wrap: wrap`: items wrap to new lines when they don't fit
// - `flex-wrap: wrap-reverse`: items wrap to new lines in reverse order
// The wrap property controls whether flex items can create multiple flex lines

use crate::*;

// Case: `flex-wrap: nowrap` (default behavior)
// Spec points:
// - All flex items are forced onto a single flex line
// - If items don't fit, they are compressed (flex-shrink applies) or overflow
// - Items maintain their order and stay on one line
// Engine behavior:
// - Items with specified width that exceeds container are shrunk proportionally
// In this test:
// - Container: width=200, flex-wrap=nowrap
// - Three items, each width=100 (total 300px > 200px)
// - Each item is compressed: 200 / 3 ≈ 66.7px, rounded to 67px
// - All items remain on the same line (expect_top=0)
#[test]
fn flex_wrap_nowrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: nowrap; width: 200px;">
          <div style="width: 100px; height: 50px;" expect_width="67" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="67" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="67" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` - basic wrapping behavior
// Spec points:
// - Flex items can wrap to new lines when they don't fit on the current line
// - Each flex line is laid out independently
// - Items wrap in document order
// In this test:
// - Container: width=200, flex-wrap=wrap
// - First two items (100px each) fit on first line: expect_top=0
// - Third item (100px) wraps to second line: expect_top=50 (below first line's height)
#[test]
fn flex_wrap_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap-reverse` - reverse wrapping order
// Spec points:
// - Flex items wrap to new lines, but lines are stacked in reverse order
// - The first line (in document order) appears at the bottom
// - Items within each line maintain their order
// In this test:
// - Container: width=200, height=100, flex-wrap=wrap-reverse
// - First two items (in DOM) wrap to bottom line: expect_top=50
// - Third item (in DOM) wraps to top line: expect_top=0
#[test]
fn flex_wrap_wrap_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap-reverse; width: 200px; height: 100px;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` with `flex-direction: column`
// Spec points:
// - When main axis is vertical (column), wrapping occurs horizontally
// - Items flow top-to-bottom, then wrap to the next column
// In this test:
// - Container: height=200, width=375, flex-direction=column, flex-wrap=wrap
// - First two items (height=100 each) fit in first column: expect_left=0
// - Third item wraps to second column: expect_left=188 (approximately centered in remaining width)
#[test]
fn flex_wrap_wrap_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; flex-direction: column; height: 200px; width: 375px;">
          <div style="width: 50px; height: 100px;" expect_width="50" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 100px;" expect_width="50" expect_left="0" expect_top="100"></div>
          <div style="width: 50px; height: 100px;" expect_width="50" expect_left="188" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` with `gap` property
// Spec points:
// - `gap` creates spacing between flex items (both main-axis and cross-axis)
// - Gap is applied between items on the same line and between lines
// In this test:
// - Container: width=200, flex-wrap=wrap, gap=10px
// - First line: two items (90px each) + 10px gap = 190px total
// - First item: expect_left=0, expect_top=0
// - Second item: expect_left=100 (90 + 10 gap)
// - Third item wraps to second line: expect_left=0, expect_top=60 (50 height + 10 gap)
#[test]
fn flex_wrap_wrap_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; gap: 10px;">
          <div style="width: 90px; height: 50px;" expect_width="90" expect_left="0" expect_top="0"></div>
          <div style="width: 90px; height: 50px;" expect_width="90" expect_left="100" expect_top="0"></div>
          <div style="width: 90px; height: 50px;" expect_width="90" expect_left="0" expect_top="60"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` with `align-content: center`
// Spec points:
// - `align-content` aligns flex lines along the cross axis when there are multiple lines
// - `align-content: center` centers the lines within the flex container
// In this test:
// - Container: width=200, height=150, flex-wrap=wrap, align-content=center
// - Two lines, each 50px tall, total 100px
// - Free space: 150 - 100 = 50px
// - Centered: free space / 2 = 25px offset
// - All items: expect_top starts at 25 (instead of 0)
#[test]
fn flex_wrap_wrap_with_align_content() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 150px; align-content: center;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="25"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="25"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="75"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` with `justify-content: center`
// Spec points:
// - `justify-content` aligns items along the main axis within each flex line
// - Each flex line is aligned independently
// Engine behavior:
// - In this test, all items fit on one line (100 + 100 + 100 = 300px, container width=300px)
// - Since items exactly fill the container, justify-content has no visible effect
// - All items remain on the same line (expect_top=0)
#[test]
fn flex_wrap_wrap_with_justify_content() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 300px; justify-content: center;">
          <div style="width: 100px; height: 50px;" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` with items wider than container
// Spec points:
// - Items with width exceeding the container's main-axis size are clamped
// - The item's used width becomes the container's width (or min-width if larger)
// - Then wrapping behavior applies normally
// In this test:
// - Container: width=150, flex-wrap=wrap
// - First item: width=200px → clamped to 150px (container width), expect_width=150, expect_top=0
// - Second item: width=100px → fits on second line, expect_top=50
#[test]
fn flex_wrap_wrap_overflow() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 150px;">
          <div style="width: 200px; height: 50px;" expect_width="150" expect_left="0" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap` with `flex-grow`
// Spec points:
// - `flex-grow` distributes available space within each flex line
// - Each flex line's space distribution is independent
// Engine behavior:
// - In this test, all items fit on one line (min-width=50 each, container width=200)
// - With flex-grow=1, space is distributed equally: each item gets ~67px width
// - No wrapping occurs (expect_top=0 for all items)
#[test]
fn flex_wrap_wrap_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px;">
          <div style="flex-grow: 1; height: 50px; min-width: 50px;" expect_top="0"></div>
          <div style="flex-grow: 1; height: 50px; min-width: 50px;" expect_top="0"></div>
          <div style="flex-grow: 1; height: 50px; min-width: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `flex-wrap: wrap-reverse` with `flex-direction: row-reverse`
// Spec points:
// - `flex-direction: row-reverse` reverses item order within each line
// - `flex-wrap: wrap-reverse` reverses the line order
// - Combined: items flow right-to-left, and lines stack bottom-to-top
// In this test:
// - Container: width=200, height=100, flex-direction=row-reverse, flex-wrap=wrap-reverse
// - First item (DOM): expect_left=100 (right-aligned due to row-reverse), expect_top=50 (bottom line)
// - Second item (DOM): expect_left=0 (to the left of first), expect_top=50
// - Third item (DOM): expect_left=100, expect_top=0 (top line)
#[test]
fn flex_wrap_wrap_reverse_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap-reverse; flex-direction: row-reverse; width: 200px; height: 100px;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="0"></div>
        </div>
    "#
    )
}
