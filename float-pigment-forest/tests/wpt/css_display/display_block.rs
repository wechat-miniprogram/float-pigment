// WPT-style tests for the `display: block` property
// Inspired by WPT CSS Display tests, but only covering behaviors actually implemented in this engine:
// - block elements create a block formatting context and are laid out vertically one after another
// - width/height calculations follow the CSS Box / Sizing specs (content-box / border-box, padding, border, min/max, percentage sizes)

use crate::*;

// Case: basic `display: block` vertical stacking and explicit sizing
// Spec meaning:
// - block-level elements are laid out one below another in the block formatting context
// - each block's used width/height follow its specified width/height when there are no conflicting constraints
// In this test:
// - first block: width=100, height=50 → expect_width=100, expect_height=50, top=0
// - second block: same size, placed directly below the first → expect_top=50
#[test]
fn display_block() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
            <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: `display: block` with auto width (fills the containing block)
// Spec meaning:
// - a non-replaced block-level element in normal flow with `width:auto` expands to fill the remaining inline space of its containing block
// In this test:
// - parent width=200
// - child: `display:block; height:50px; width:auto` → expect_width=200, expect_height=50
#[test]
fn display_block_auto_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; height: 50px;" expect_width="200" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `display: block` with margin
// Spec meaning:
// - horizontal margins shift the block within the containing block's width
// - here we only assert that margin-left=10 moves the block 10px from the left edge;
//   vertical margin-top is not asserted in this simple case
#[test]
fn display_block_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 100px; height: 50px; margin: 10px;" expect_width="100" expect_height="50" expect_top="0" expect_left="10"></div>
        </div>
    "#
    )
}

// Case: `display: block` with padding and border (content-box)
// Spec points:
// - default `box-sizing: content-box`
// - outer width = content width + horizontal padding + horizontal border
// - outer height = content height + vertical padding + vertical border
// In this test:
// - content: width=100, height=50
// - padding: 10px on all sides → +20 horizontally, +20 vertically
// - border-width: 5px on all sides → +10 horizontally, +10 vertically
// → expected outer width: 100 + 20 + 10 = 130
// → expected outer height: 50 + 20 + 10 = 80
// The inner child is positioned at padding+border: top=15, left=15
#[test]
fn display_block_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="130" expect_height="80">
                <div style="width: 50px; height: 30px;" expect_top="15" expect_left="15"></div>
            </div>
        </div>
    "#
    )
}

// Case: nested `display: block` elements
// Spec meaning:
// - inner blocks follow the same vertical stacking rules within the outer block's content box
// - outer block's width/height do not change the inner blocks' explicit sizes
// In this test:
// - outer: width=150, height=100
// - inner1: width=100, height=50
// - inner2: width=100, height=50, vertically stacked below inner1 → top=50
#[test]
fn display_block_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="display: block; width: 150px; height: 100px;">
                <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
                <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: block` with `min-width`
// Spec points:
// - specified: width=50px, min-width=100px
// - used width = max(specified width, min-width) = 100px
// - height stays at the specified 50px
#[test]
fn display_block_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 50px; min-width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `display: block` with `max-width`
// Spec points:
// - specified: width=200px, max-width=100px
// - used width = min(specified width, max-width) = 100px
// - height stays at the specified 50px
#[test]
fn display_block_with_max_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 200px; max-width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `display: block` with percentage width
// Spec points:
// - parent width=200
// - child: width:50% → 0.5 * 200 = 100
// - height is the specified 50px
#[test]
fn display_block_percentage_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 50%; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `display: block` with vertical margin collapse between siblings
// Spec points:
// - adjacent vertical margins between block siblings collapse
// - collapsed margin = max(20px (bottom of first), 30px (top of second)) = 30px
// - second block's vertical position: 50 (first height) + 30 (collapsed margin) = 80
#[test]
fn display_block_margin_collapse() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="display: block; height: 50px; margin-bottom: 20px;" expect_height="50"></div>
            <div style="display: block; height: 50px; margin-top: 30px;" expect_top="80" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `display: block` with `box-sizing:border-box`
// Spec points:
// - specified width/height include padding + border
// - here: width=100, height=50 with padding=10 and border-width=5 on all sides
// - we assert the outer box width/height remain 100x50, verifying border-box behavior
// The inner child is positioned at padding+border: top=15, left=15
#[test]
fn display_block_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; box-sizing: border-box; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="100" expect_height="50">
                <div style="width: 50px; height: 30px;" expect_top="15" expect_left="15"></div>
            </div>
        </div>
    "#
    )
}
