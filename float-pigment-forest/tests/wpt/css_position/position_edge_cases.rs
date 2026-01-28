// WPT-style edge-case tests for CSS positioning
// Focused on combinations and corner cases for `position:absolute` and `position:relative`
// where they interact with box-sizing, inline/inline-block, flexbox, overflow, and aspect-ratio.

use crate::*;

// Case: `position: absolute` in a border-box container
// Spec meaning:
// - with `box-sizing:border-box`, the specified width/height include border
// - absolute insets are measured from the padding edge of the containing block
// In this test:
// - outer: border-box 200x100 with 2px border → content area 196x96
// - child: left=10, right=10, height=100%:
//   - width = content_width - 10 - 10 = 176
//   - top = border-top = 2, height = content_height = 96
#[test]
fn position_absolute_in_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; border-top-width: 2px; border-right-width: 2px; border-bottom-width: 2px; border-left-width: 2px;">
            <div style="position: absolute; left: 10px; height: 100%; right: 10px;" expect_top="2" expect_height="96" expect_left="12" expect_width="176"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with flex align-items on the container
// Spec / engine behavior:
// - the flex container has `align-items:center`, but the absolutely positioned child is taken out of flex layout
// - height:100% of the inner box (40px container - 2px borders top/bottom = 36px)
// - top equals the border-top width (2px)
#[test]
fn position_absolute_flex_align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; box-sizing: border-box; width: 100%; height: 40px; border-top-width: 2px; border-right-width: 2px; border-bottom-width: 2px; border-left-width: 2px; align-items: center;">
            <div style="position: absolute; right: 0; width: 60px; height: 100%; align-items: center;" expect_top="2" expect_height="36"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` inside `display:inline` (edge behavior)
// Engine behavior:
// - inline boxes in this engine report 0x0 layout size when they only contain abspos children
// - abspos children do not contribute to the inline wrapper's width/height
// In this test:
// - outer div: width=200, height remains 0 because its inline child has 0x0
// - first abspos child: width=100% of inline box (here reported as 0), height=20
// - second abspos child: width=30, height=100% (0) → height=0
#[test]
fn position_absolute_in_inline() {
    assert_xml!(
        r#"
        <div style="width: 200px;" expect_width="200" expect_height="0">
            <div style="display: inline" expect_width="0" expect_height="0">
                <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="0"></div>
                <div style="position: absolute; width: 30px; height: 100%;" expect_height="0" expect_width="30"></div>
            </div>
        </div>
    "#
    )
}

// Case: `position: relative` inside `display:inline`
// Engine behavior:
// - the outer absolutely positioned container 200px wide contains two inline wrappers
// - each inline wrapper has a flex child with `position:relative`
// - we assert only the sizes/offsets of the flex children (300x300, and second shifted by 100,100)
#[test]
fn position_relative_in_inline() {
    assert_xml!(
        r#"
        <div style="width: 200px; position: absolute;" expect_width="200" expect_height="600">
            <div style="display: inline" expect_width="300" expect_height="300">
                <div style="display: flex; position: relative; width: 300px; height: 300px;" expect_height="300" expect_width="300"></div>
            </div>
            <div style="display: inline" expect_width="300" expect_height="300">
                <div style="display: flex; position: relative; left: 100px; top: 100px; width: 300px; height: 300px;" expect_left="100" expect_top="100" expect_height="300" expect_width="300"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display:inline` treated as relative (inline with `position:relative`)
// Engine behavior:
// - similar to the previous case, but `position:relative` is applied on the inline wrappers
// - we assert the inline wrappers' top offsets (100, 400) and the inner flex children's offsets (including 100,100 shift)
#[test]
fn position_inline_as_relative() {
    assert_xml!(
        r#"
        <div style="width: 200px; position: absolute;" expect_width="200" expect_height="600">
            <div style="display: inline; position: relative; top: 100px;" expect_width="300" expect_height="300" expect_top="100">
                <div style="display: flex; position: relative; width: 300px; height: 300px;" expect_height="300" expect_width="300"></div>
            </div>
            <div style="display: inline; position: relative; top: 100px;" expect_width="300" expect_height="300" expect_top="400">
                <div style="display: flex; position: relative; left: 100px; top: 100px; width: 300px; height: 300px;" expect_left="100" expect_top="100" expect_height="300" expect_width="300"></div>
            </div>
        </div>
    "#
    )
}

// Case: `position: absolute` with `max-width` constraint
// Spec meaning:
// - max-width applies to the border box of the absolutely positioned element
// In this test:
// - abspos element: width:30px, max-width:20px → used width=20
// - it contains a 30x10 child (used width unaffected inside)
// - positioned at left=0, top=100
#[test]
fn position_absolute_with_max_width() {
    assert_xml!(
        r#"
        <div style="position: absolute; max-width: 20px; left: 0; top: 100px; width: 30px;" expect_width="20" expect_height="10" expect_left="0" expect_top="100">
            <div style="height: 10px; width: 30px;" expect_width="30" expect_height="10"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with `overflow:hidden` on ancestor
// Spec meaning:
// - overflow:hidden clips painting but not layout of the abspos child
// In this test:
// - child keeps its full 300x300 layout box at (10,10); clipping is not modeled in these asserts
#[test]
fn position_absolute_with_overflow() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative; overflow: hidden;">
            <div style="position: absolute; left: 10px; top: 10px; width: 300px; height: 300px;" expect_left="10" expect_top="10" expect_width="300" expect_height="300"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with `box-sizing:border-box` and padding/border
// Spec meaning:
// - with border-box, specified width/height (50x50) include padding and border
// - inner content sits inside padding+border
// In this test:
// - child border-box remains 50x50 at (10,10)
// - inner box at padding+border = 7px from top/left
#[test]
fn position_absolute_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; box-sizing: border-box; width: 50px; height: 50px; padding: 5px; border-top-width: 2px; border-right-width: 2px; border-bottom-width: 2px; border-left-width: 2px;" expect_left="10" expect_top="10" expect_width="50" expect_height="50">
                <div style="width: 30px; height: 30px;" expect_top="7" expect_left="7"></div>
            </div>
        </div>
    "#
    )
}

// Case: `position: absolute` with `aspect-ratio`
// Note:
// - in general, aspect-ratio may require content or additional constraints to resolve height
// - here we only assert that the absolutely positioned container is created and the child retains its 50x50 size
#[test]
fn position_absolute_with_aspect_ratio() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; width: 100px; aspect-ratio: 2/1;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: multiple `position: absolute` elements
// Spec meaning:
// - multiple abspos siblings are each positioned independently relative to the same containing block
// In this test:
// - two 50x50 boxes at (10,10) and (70,70)
#[test]
fn position_multiple_absolute() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; width: 50px; height: 50px;" expect_left="10" expect_top="10"></div>
            <div style="position: absolute; left: 70px; top: 70px; width: 50px; height: 50px;" expect_left="70" expect_top="70"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with auto width/height
// Spec meaning:
// - auto width/height of an abspos element shrink-wrap to its content
// In this test:
// - inner content: 50x50
// - outer abspos, with no explicit size, becomes 50x50 at (10,10)
#[test]
fn position_absolute_auto_size() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px;">
                <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}
