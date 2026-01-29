// WPT-style tests for combinations of `display` values and related layout behaviors
// Inspired by WPT CSS Display tests:
// - focus on interactions between display, flex/inline-block/block, and position/overflow/aspect-ratio
// - only cover behaviors actually implemented in this engine

use crate::*;

// Case: switching from `display:block` to `display:none` among siblings
// Spec meaning:
// - the display:none sibling generates no box, so the following visible block is placed directly after the first visible block
// In this test:
// - first block: 100x50 at top=0
// - second (none): 0x0
// - third block: 100x50 at top=50
#[test]
fn display_block_to_none() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
            <div style="display: none; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: switching from `display:flex` to `display:none`
// Spec meaning:
// - flex container is laid out as normal
// - subsequent display:none block does not affect layout or take space
#[test]
fn display_flex_to_none() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: flex; width: 100px; height: 50px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
            <div style="display: none; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
        </div>
    "#
    )
}


// Case: `display:flex` with an inline-block child (inline-block becomes a flex item)
// Spec meaning:
// - any non-none display value inside a flex container becomes a flex item
// - the child's own `display:inline-block` only affects its internal formatting context, not its flex participation
// We assert the child is sized 50x50 at left=0 as a flex item
#[test]
fn display_flex_with_inline_block_child() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="display: inline-block; width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `display:inline-block` with a flex child
// Spec meaning:
// - inline-block establishes a block formatting context for its contents
// - inside, the `display:flex` element lays out its children as flex items
// We assert that the inner flex child has the expected 50x50 size at left=0
#[test]
fn display_inline_block_with_flex_child() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 150px; height: 80px;">
                <div style="display: flex; width: 100px; height: 50px;">
                    <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
                </div>
            </div>
        </div>
    "#
    )
}

// Case: `display:block` with `position:absolute`
// Spec meaning:
// - absolutely positioned block is taken out of normal flow and positioned relative to the nearest positioned ancestor
// In this test:
// - parent: `position:relative`
// - child: `display:block; position:absolute; left:10; top:20; width:100; height:50`
// - we assert left=10, top=20 and size 100x50
#[test]
fn display_block_with_position_absolute() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; position: relative;">
            <div style="display: block; position: absolute; left: 10px; top: 20px; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_left="10" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: `display:flex` with `position:absolute` on the flex container
// Spec meaning:
// - the flex container itself is absolutely positioned relative to its positioned ancestor
// - its child still behaves as a flex item inside the absolutely positioned container
// We assert the child 50x50 at left=0 inside the positioned flex container
#[test]
fn display_flex_with_position_absolute() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; position: relative;">
            <div style="display: flex; position: absolute; left: 10px; top: 20px; width: 100px; height: 50px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display:inline-block` with `position:absolute`
// Spec meaning:
// - the inline-block itself is taken out of flow and positioned at (10,20)
// - we assert its outer box size (100x50) and position (left=10, top=20)
#[test]
fn display_inline_block_with_position_absolute() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; position: relative;">
            <div style="display: inline-block; position: absolute; left: 10px; top: 20px; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_left="10" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: `display:block` with `position:relative`
// Spec meaning:
// - a relatively positioned block is offset from its normal position by left/top, but still occupies its original space
// - this engine reports the visual box at the offset position (10,20)
// We assert the block's size 100x50 and position left=10, top=20
#[test]
fn display_block_with_position_relative() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; position: relative; left: 10px; top: 20px; width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_left="10" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: `display:flex` with `position:relative`
// Spec meaning:
// - the entire flex container is shifted by left/top relative to its normal position
// - the child flex item is laid out relative to the container's new origin (so still at left=0 inside)
#[test]
fn display_flex_with_position_relative() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: flex; position: relative; left: 10px; top: 20px; width: 100px; height: 50px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display:block` with `overflow:hidden`
// Spec meaning:
// - overflow affects painting/clipping, but not the intrinsic size of the overflowing child
// In this test:
// - child is 200x100 inside a 100x50 block with overflow:hidden
// - we assert the child's layout box remains 200x100 at (0,0); clipping is not modeled in these tests
#[test]
fn display_block_with_overflow() {
    assert_xml!(
        r#"
        <div style="display: block; width: 100px; height: 50px; overflow: hidden;">
            <div style="width: 200px; height: 100px;" expect_width="200" expect_height="100" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `display:flex` with `overflow:hidden`
// Engine behavior:
// - overflow does not change the flex container's layout algorithm
// - but the flex container's width (100px) constrains the width of its child flex item
// In this test:
// - child specified width=200, but is constrained to 100px (the container width) → expect_width=100
#[test]
fn display_flex_with_overflow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; height: 50px; overflow: hidden;">
            <div style="width: 200px; height: 100px;" expect_width="100" expect_height="100" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `display:block` with `aspect-ratio`
// Spec meaning:
// - when aspect-ratio is specified and one dimension is fixed (width here), the other is derived from the ratio
// In this test:
// - outer block: width=200, `aspect-ratio:2/1` → computed height=100
// - we assert expect_width=200, expect_height=100
#[test]
fn display_block_with_aspect_ratio() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 200px; aspect-ratio: 2/1;" expect_width="200" expect_height="100">
                <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display:flex` with `aspect-ratio`
// Spec meaning:
// - similar to the block case, aspect-ratio determines the cross size when main size is known
// In this test:
// - flex container: width=200, `aspect-ratio:2/1` → height=100
// - child 50x50 is laid out inside, we assert container size and child position
#[test]
fn display_flex_with_aspect_ratio() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: flex; width: 200px; aspect-ratio: 2/1;" expect_width="200" expect_height="100">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: complex nesting: block > flex > inline-block > block
// Spec / engine behavior:
// - we validate that nesting different display types still respects each intermediate formatting context
// - the innermost block should retain its specified 50x50 size within the nested contexts
#[test]
fn display_complex_nesting() {
    assert_xml!(
        r#"
        <div style="display: block; width: 200px; height: 200px;">
            <div style="display: flex; width: 150px; height: 150px;">
                <div style="display: inline-block; width: 100px; height: 100px;">
                    <div style="display: block; width: 50px; height: 50px;" expect_width="50" expect_height="50"></div>
                </div>
            </div>
        </div>
    "#
    )
}

// Case: `display:block` with empty content
// Spec meaning:
// - block with explicit width/height still has that size even if it has no children
// We assert the single block child has width=100, height=50
#[test]
fn display_block_empty() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `display:flex` with empty content
// Spec meaning:
// - a flex container with no flex items still has its own width/height
// We assert the container size 200x100
#[test]
fn display_flex_empty() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;" expect_width="200" expect_height="100"></div>
    "#
    )
}

// Case: `display:inline-block` with empty content
// Spec meaning:
// - inline-block with explicit width/height retains its size even with no children
// We assert the inline-block 100x50
#[test]
fn display_inline_block_empty() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: inline-block; width: 100px; height: 50px;" expect_width="100" expect_height="50"></div>
        </div>
    "#
    )
}
