// WPT-style tests for the `display: inline-flex` property
// Inspired by WPT CSS Display tests:
// - `display:inline-flex` participates in inline layout (like inline-block) but establishes a flex formatting context for its children
// - width/height calculations follow flexbox + box model rules as implemented in this engine

use crate::*;

// Case: basic `display: inline-flex` container with fixed size and a single flex item
// Spec meaning:
// - inline-flex container behaves like inline-block in the inline axis, but its children are flex items
// - here we only assert the child flex item size/position, not the inline positioning of the container itself
// In this test:
// - each inline-flex container is 100x50
// - inside, a single child with width=50, height=50 → expect_width=50, expect_height=50, expect_left=0
#[test]
fn display_inline_flex() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 100px; height: 50px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
            <div style="display: inline-flex; width: 100px; height: 50px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with a text-slot flex item
// Text measurement model (engine-specific):
// - `text-slot len=2` → intrinsic size 32x16 (font-size=16)
// Flex behavior:
// - the inline element containing the text-slot is a flex item
// - main-axis size = intrinsic width (32), cross-axis size is stretched to the container height (50)
// In this test:
// - container: height=50
// - child flex item: expect_width=32, expect_height=50
#[test]
fn display_inline_flex_with_text_slot() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; height: 50px;">
                <div style="display: inline;" expect_width="32" expect_height="50">
                    <text-slot len="2"></text-slot>
                </div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with two text-slot flex items
// Each `text-slot len=2` has intrinsic size 32x16
// In a 100px-wide container:
// - first inline flex item shrinks to 32px width, placed at left=0
// - second inline flex item also 32px width, placed immediately after at left=32
// - both items are stretched to the container height (50px) along the cross axis
#[test]
fn display_inline_flex_two_text_slots() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 100px; height: 50px;">
                <div style="display: inline;" expect_width="32" expect_height="50" expect_left="0">
                    <text-slot len="2"></text-slot>
                </div>
                <div style="display: inline;" expect_width="32" expect_height="50" expect_left="32">
                    <text-slot len="2"></text-slot>
                </div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with margin
// Spec / engine behavior:
// - margin affects the placement of the inline-flex container within its parent
// - here we only assert the inner flex item's size/position inside each container
//   to verify that margin does not change the local flex layout
#[test]
fn display_inline_flex_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 50px; height: 50px; margin: 10px;">
                <div style="width: 30px; height: 30px;" expect_width="30" expect_height="30" expect_left="0" expect_top="0"></div>
            </div>
            <div style="display: inline-flex; width: 50px; height: 50px; margin: 10px;">
                <div style="width: 30px; height: 30px;" expect_width="30" expect_height="30" expect_left="0" expect_top="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with padding and border (content-box)
// Spec points:
// - outer width = content width + horizontal padding + horizontal border
// - outer height = content height + vertical padding + vertical border
// In this test:
// - content: width=100, height=50
// - padding: 10px each side → +20 horizontally, +20 vertically
// - border-width: 5px each side → +10 horizontally, +10 vertically
// → expected outer width: 130, height: 80
// The inner flex item is positioned at padding+border: top=15, left=15
#[test]
fn display_inline_flex_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="130" expect_height="80">
                <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30" expect_left="15" expect_top="15"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with auto width (shrink-wrap)
// Spec / engine behavior:
// - when width is auto, the inline-flex container shrinks to fit its flex items
// - here the single child is 50x30, and we assert its size only (the container width is implied)
#[test]
fn display_inline_flex_auto_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; height: 50px;">
                <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with `box-sizing:border-box`
// Spec / engine behavior:
// - specified width/height include padding and border
// - the container's outer size remains 100x50 despite padding/border
// - we assert the inner flex item is positioned at padding+border: (15,15) with size 50x30
#[test]
fn display_inline_flex_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; box-sizing: border-box; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="100" expect_height="50">
                <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30" expect_left="15" expect_top="15"></div>
            </div>
        </div>
    "#
    )
}

// Case: multiple `display: inline-flex` siblings with simple flex items
// Spec / engine behavior:
// - we verify that each inline-flex container lays out its child flex item correctly (30x30 at left=0)
// - horizontal positioning of the containers themselves is not asserted here
#[test]
fn display_inline_flex_multiple() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 50px; height: 50px;">
                <div style="width: 30px; height: 30px;" expect_width="30" expect_height="30" expect_left="0"></div>
            </div>
            <div style="display: inline-flex; width: 50px; height: 50px;">
                <div style="width: 30px; height: 30px;" expect_width="30" expect_height="30" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline-flex` with flex-grow on the child
// Spec / engine behavior:
// - flex item with `flex-grow:1` expands to fill the main-axis size of the flex container
// In this test:
// - container width=100, height=50
// - child with `flex-grow:1; height:30px` → expect_width=100 (fills main axis), expect_height=30
#[test]
fn display_inline_flex_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 100px; height: 50px;">
                <div style="flex-grow: 1; height: 30px;" expect_width="100" expect_height="30" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: nested `display: inline-flex` containers
// Spec / engine behavior:
// - outer inline-flex 150x80, inner inline-flex 100x50
// - we assert the inner flex item inside the nested container has expected size/position 50x30 at left=0
#[test]
fn display_inline_flex_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-flex; width: 150px; height: 80px;">
                <div style="display: inline-flex; width: 100px; height: 50px;">
                    <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30" expect_left="0"></div>
                </div>
            </div>
        </div>
    "#
    )
}
