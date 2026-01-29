// WPT-style tests for the `display: inline` property
// Inspired by WPT CSS Display tests, but note:
// - inline layout in this engine relies on a simplified external text layout model (see `text-slot` and custom inline tests)
// - for pure `display:inline` elements without text measurement, width/height are generally reported as 0

use crate::*;

// Case: basic `display: inline` without text measurement
// Spec / engine behavior:
// - in this engine, pure inline boxes without a text measurement function report width/height as 0
// - this differs from browser engines, which measure actual glyphs; here we only validate the current engine behavior
// In this test:
// - child has `display:inline; width:100px; height:50px`, but as an inline box without text measurement it ends up 0x0
#[test]
fn display_inline() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
        </div>
    "#
    )
}

// Case: `display: inline` with text-slot (simulated text content)
// Text measurement model (engine-specific, not full CSS Text):
// - `text-slot` width = font-size * len, default font-size = 16px
// - `text-slot` height = font-size
// - an inline element that only contains a text-slot gets its width/height from the measured text
// In this test:
// - `<text-slot len="2">` → width = 16 * 2 = 32, height = 16
// - outer `display:inline` shrink-wraps to 32x16
#[test]
fn display_inline_with_text_slot() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline;" expect_width="32" expect_height="16">
                <text-slot len="2"></text-slot>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline` with text-slot inside a flex container
// Spec / engine behavior:
// - the inline element is treated as a flex item by the flex container
// - its intrinsic size comes from the text measurement (32x16), but as a flex item its cross-axis size is stretched to the container's cross size
// In this test:
// - flex container height=100px
// - inline child with `<text-slot len="2">` → width=32, height stretched to 100
#[test]
fn display_inline_text_in_flex() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="display: inline;" expect_width="32" expect_height="100">
                <text-slot len="2"></text-slot>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline` with multiple text-slot children on one line
// Text measurement model as above:
// - each `<text-slot len="2">` has size 32x16
// - both text slots are on the same line: total width=32+32=64, height=16
// - outer inline shrink-wraps to 64x16
#[test]
fn display_inline_multiple_text_slots() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline;" expect_width="64" expect_height="16">
                <text-slot len="2"></text-slot>
                <text-slot len="2"></text-slot>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline` with margin
// Engine behavior:
// - for inline elements without a text measurement function, this engine still reports width/height as 0
// - margin is present in style but does not change the reported size in these tests
// We only assert width/height=0, not the inline positioning, since inline layout is delegated to a simplified text model
#[test]
fn display_inline_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 100px; height: 50px; margin: 10px;" expect_width="0" expect_height="0"></div>
        </div>
    "#
    )
}

// Case: `display: inline` with padding/border and a block child
// Spec / engine behavior:
// - the inline element establishes an inline context; its block child participates in normal block layout inside
// - we don't assert the inline element's own box size here; instead we assert the block child layout remains correct (50x30 at top-left)
#[test]
fn display_inline_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;">
                <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30" expect_top="0" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: nested `display: inline` elements without text measurement
// Engine behavior:
// - without text measurement, inline boxes still report 0x0 regardless of specified width/height
// - we assert 0x0 on the inner inline to document this engine's simplification
#[test]
fn display_inline_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 150px; height: 80px;">
                <div style="display: inline; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: inline` with a block child
// Spec / engine behavior:
// - inline parent does not prevent block child from using its specified width/height
// - we assert the block child is sized 100x50 inside the inline parent
#[test]
fn display_inline_with_block_child() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 150px; height: 80px;">
                <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: multiple `display: inline` elements without text measurement
// Engine behavior:
// - each inline box reports width/height=0 as there is no text measurement
// - we assert 0x0 for both, documenting the current simplification
#[test]
fn display_inline_multiple() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 50px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="display: inline; width: 50px; height: 50px;" expect_width="0" expect_height="0"></div>
        </div>
    "#
    )
}

// Case: `display: inline` with min-width/max-width (non-applied in this engine)
// Engine behavior:
// - for this engine's simplified inline model, min/max-width do not affect the reported size of pure inline boxes without text
// - we assert width/height=0 to capture that current behavior (not full browser-compatible)
#[test]
fn display_inline_with_min_max_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; width: 50px; min-width: 100px; max-width: 150px; height: 50px;" expect_width="0" expect_height="0"></div>
        </div>
    "#
    )
}

// Case: `display: inline` with `box-sizing:border-box` and a block child
// Spec / engine behavior:
// - inline parent uses border-box sizing, but we focus on the block child
// - the child is laid out with width=50, height=30 at the top-left of the padding-box (0,0 here)
// - we assert child's size/position; the inline parent's box is not asserted due to the simplified inline model
#[test]
fn display_inline_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline; box-sizing: border-box; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;">
                <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30" expect_top="0" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}
