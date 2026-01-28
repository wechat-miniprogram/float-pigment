// WPT-style tests for the `display: flex` property
// Note: detailed flexbox behavior (justify-content, align-items, etc.) is covered in `css_flexbox`.
// Here we only test basic `display:flex` container behavior and its interaction with box model properties.

use crate::*;

// Case: basic `display: flex` container with two fixed-size flex items
// Spec meaning:
// - flex container creates a flex formatting context
// - by default (`flex-direction:row`), items are laid out along the horizontal axis in document order
// In this test:
// - container: width=200, height=100
// - two children: each width=50, height=50, placed at left=0 and left=50 respectively
#[test]
fn display_flex() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `display: flex` with auto width
// Spec / engine behavior:
// - when `width:auto` and the container is in normal flow, width is determined by the parent
// - here we don't assert the container width itself, only that its child retains its fixed size
// - child: width=50, height=50
#[test]
fn display_flex_auto_width() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 100px;">
            <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `display: flex` with margin on the flex container
// Spec / engine behavior:
// - margin shifts the flex container within its parent
// - we assert the container's own width/height and its left offset (10px from the parent due to margin-left:10)
// - the inner flex item is laid out at left=0 inside the flex container
#[test]
fn display_flex_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: flex; width: 150px; height: 80px; margin: 10px;" expect_width="150" expect_height="80" expect_top="0" expect_left="10">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: flex` with padding and border (content-box)
// Spec points:
// - outer width = content width + horizontal padding + horizontal border
// - outer height = content height + vertical padding + vertical border
// In this test:
// - content: width=200, height=100
// - padding: 10px on all sides → +20 horizontally, +20 vertically
// - border-width: 5px on all sides → +10 horizontally, +10 vertically
// → expect outer width=230, height=130
// The inner flex item sits at padding+border: left=15, height=50
#[test]
fn display_flex_with_padding_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="230" expect_height="130">
            <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="15"></div>
        </div>
    "#
    )
}

// Case: nested `display: flex` containers
// Spec / engine behavior:
// - outer flex container 200x100, inner flex container 100x80
// - inner child flex item is 50x50 and laid out at the start of the inner flex container
#[test]
fn display_flex_nested() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="display: flex; width: 100px; height: 80px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: flex` with `box-sizing:border-box`
// Spec / engine behavior:
// - specified width/height (200x100) include padding and border
// - we assert the outer flex container remains 200x100 despite padding/border
// - inner flex item is positioned at padding+border (15px) with size 50x50
#[test]
fn display_flex_border_box() {
    assert_xml!(
        r#"
        <div style="display: flex; box-sizing: border-box; width: 200px; height: 100px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="15"></div>
        </div>
    "#
    )
}

// Case: `display: flex` with min-width/max-width on the flex container
// Spec points:
// - container: width=50, min-width=100, max-width=150 → used width=100
// - height=80 is used as specified
// - inner flex item retains its 50x50 size at left=0
#[test]
fn display_flex_with_min_max_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: flex; width: 50px; min-width: 100px; max-width: 150px; height: 80px;" expect_width="100" expect_height="80">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: flex` with percentage width
// Spec points:
// - parent width=200
// - flex container width:50% → 100
// - height=80
// - inner flex item remains 50x50 at left=0
#[test]
fn display_flex_percentage_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: flex; width: 50%; height: 80px;" expect_width="100" expect_height="80">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: flex` with `display:none` child
// Spec meaning:
// - `display:none` elements are removed from the flex layout and take no space
// In this test:
// - the first child with `display:none` has expect_width=0, expect_height=0
// - the second child becomes the first visible flex item at left=0 with width=50, height=50
#[test]
fn display_flex_with_none_child() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="display: none; width: 50px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `display: flex` with a block child (block becomes flex item)
// Spec meaning:
// - any element inside a flex container becomes a flex item regardless of its own `display` value (except `display:none`)
// In this test:
// - child has `display:block`, but is treated as a flex item
// - we assert it has width=50, height=50 at left=0 inside the flex container
#[test]
fn display_flex_with_block_child() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="display: block; width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
        </div>
    "#
    )
}
