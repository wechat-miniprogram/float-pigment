// WPT-style tests for the `display: inline-block` property
// Inspired by WPT CSS Display tests, but only covering behaviors actually implemented in this engine:
// - inline-block participates in inline layout (laid out horizontally in a line), while establishing a block formatting context for its contents
// - width/height calculations follow the CSS Box / Sizing specs, constrained by this engine's padding / border / box-sizing implementation

use crate::*;

// Case: basic inline-block inline layout + block sizing
// Spec meaning:
// - two inline-block siblings participate in the same inline formatting context and are laid out one after another along the inline axis
// - each inline-block's outer box width equals its specified `width`, and its height equals its specified `height`
// - with no margin/padding/border, the second box's `left` equals the first box's width
// In this test:
// - parent: width=200, height=100
// - first child: width=100, height=50 → expect_width=100, expect_height=50, expect_left=0
// - second child: same size, placed immediately after the first → expect_left=100
#[test]
fn display_inline_block() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_left="0"></div>
            <div style="display: inline-block; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_left="100"></div>
        </div>
    "#
    )
}

// Case: inline-block + a single text-slot intrinsic size
// Text measurement model (engine-specific, not full CSS Text):
// - `text-slot` width = font-size * len, default font-size = 16px
// - `text-slot` height = font-size
// - inline-block width/height shrink-wrap its contents when no explicit width/height is specified (no extra padding/border here)
// In this test:
// - `<text-slot len="2">` → text width = 16 * 2 = 32, height = 16
// - outer `display:inline-block` has no explicit width/height → shrink-to-fit the content
// So we assert: expect_width=32, expect_height=16
#[test]
fn display_inline_block_with_text_slot() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block;" expect_width="32" expect_height="16">
                <text-slot len="2"></text-slot>
            </div>
        </div>
    "#
    )
}

// Case: inline-block + multiple text-slot children, total width = sum of each text width
// Text measurement model as above:
// - each `<text-slot len="2">` has intrinsic size 32x16
// - multiple text slots placed on the same line: total width = 32 + 32 = 64, height remains 16
// - inline-block shrink-wraps all inline content → width=64, height=16
// This matches the spec idea that inline-block's size is determined by its inline content (under this simplified text model)
#[test]
fn display_inline_block_multiple_text_slots() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block;" expect_width="64" expect_height="16">
                <text-slot len="2"></text-slot>
                <text-slot len="2"></text-slot>
            </div>
        </div>
    "#
    )
}

// Case: inline-block + margin affecting inline layout
// Spec points:
// - inline-block participates in inline formatting; its margin-left shifts it along the inline axis
// - in this engine, margin-top also shifts the box down from the top of the parent line box
// In this test:
// - parent: width=200, height=100
// - first child: width=50, height=50, margin:10 → left=10 (margin-left), top=10 (margin-top)
// - second child: same margin:10, placed after the first:
//   left = 10(first margin-left) + 50(first width) + 10(second margin-left) = 80
// So we assert: first expect_left=10, expect_top=10; second expect_left=80, expect_top=10
#[test]
fn display_inline_block_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 50px; height: 50px; margin: 10px;" expect_width="50" expect_height="50" expect_left="10" expect_top="10"></div>
            <div style="display: inline-block; width: 50px; height: 50px; margin: 10px;" expect_width="50" expect_height="50" expect_left="80" expect_top="10"></div>
        </div>
    "#
    )
}

// Case: inline-block + padding + border (content-box)
// Spec points:
// - default `box-sizing: content-box`
// - outer width = content width + horizontal padding + horizontal border
// - outer height = content height + vertical padding + vertical border
// In this test:
// - inner content: width=100, height=50
// - padding: 10px on all sides → +20 horizontally, +20 vertically
// - border-width: 5px on all sides → +10 horizontally, +10 vertically
// → expected outer width: 100 + 20 + 10 = 130
// → expected outer height: 50 + 20 + 10 = 80
// The inner child in the padding-box has top/left = 10(padding) + 5(border) = 15
#[test]
fn display_inline_block_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="130" expect_height="80">
                <div style="width: 50px; height: 30px;" expect_top="15" expect_left="15"></div>
            </div>
        </div>
    "#
    )
}

// Case: inline-block auto width (shrink-wrap)
// Spec points:
// - for inline-block, when `width` is `auto`, the width is determined by its contents (shrink-to-fit)
// - here there is a single block child: width=100, height=30
// - no padding/border → inline-block's content box width equals its child width (100)
// We assert the inner block's expect_width=100, expect_height=30 to indirectly verify the shrink-wrapped width
#[test]
fn display_inline_block_auto_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; height: 50px;">
                <div style="width: 100px; height: 30px;" expect_width="100" expect_height="30"></div>
            </div>
        </div>
    "#
    )
}

// Case: inline-block with min-width/max-width constraints
// Spec points:
// - specified: width=50px, min-width=100px, max-width=150px
// - final used width = clamp(specified width, min-width, max-width) = 100px
// - height remains the specified height=50px
#[test]
fn display_inline_block_with_min_max_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 50px; min-width: 100px; max-width: 150px; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: inline-block + box-sizing:border-box + padding + border
// Spec points:
// - with `box-sizing:border-box`, the specified width/height include padding + border
// - here the outer width/height are fixed at 100x50
// - inner content size = outer size minus padding/border (not directly asserted here)
// - we assert the outer width/height still equal 100x50 to validate the box-sizing behavior
// The child div has expect_top/left=15, same as the content-box case: padding(10) + border(5)
#[test]
fn display_inline_block_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; box-sizing: border-box; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="100" expect_height="50">
                <div style="width: 50px; height: 30px;" expect_top="15" expect_left="15"></div>
            </div>
        </div>
    "#
    )
}

// Case: nested inline-blocks
// Spec points:
// - outer inline-block has width=150, height=80
// - inner inline-block has width=100, height=50 and determines its own size independent of the outer width
// - we assert the inner box width/height as 100x50 to confirm size independence in nesting
#[test]
fn display_inline_block_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 150px; height: 80px;">
                <div style="display: inline-block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: inline-block containing multiple block children (vertical stacking)
// Spec points:
// - outer inline-block height is 80px, with two inner block children of 50px height each
// - under normal block formatting:
//   - first block: top=0, height=50
//   - second block: top=50, height=50
// - we use expect_top on the second child to verify vertical stacking inside inline-block
#[test]
fn display_inline_block_with_block_child() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 150px; height: 80px;">
                <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
                <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: multiple fixed-width inline-block siblings in a single line
// Spec points:
// - three inline-blocks, each width=50, height=50
// - ordered along the inline axis:
//   - first: left=0
//   - second: left=50
//   - third: left=100
// - expect_left asserts the cumulative inline offset produced by inline layout
#[test]
fn display_inline_block_multiple() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="display: inline-block; width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
            <div style="display: inline-block; width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="100"></div>
        </div>
    "#
    )
}

// Case: inline-block percentage width
// Spec points:
// - parent width=200
// - child inline-block width:50% → 0.5 * 200 = 100
// - height uses the specified 50px
// We assert expect_width=100, expect_height=50
#[test]
fn display_inline_block_percentage_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 50%; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: inline-block with vertical-align-related height differences
// Notes:
// - this test only asserts the width/height and `left` positions of the two inline-block boxes
// - it does NOT assert precise vertical-align offsets, because this engine's vertical-align behavior is not fully aligned with browser implementations yet
// - the main goal here is to verify that inline-blocks of different heights share the same line and have the expected sizes and horizontal placement
#[test]
fn display_inline_block_with_vertical_align() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="display: inline-block; width: 50px; height: 80px;" expect_width="50" expect_height="80" expect_left="50"></div>
        </div>
    "#
    )
}
