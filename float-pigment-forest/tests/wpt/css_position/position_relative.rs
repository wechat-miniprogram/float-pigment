// WPT-style tests for `position: relative`
// Inspired by WPT CSS Position tests:
// - `position:relative` offsets the element visually by left/top/right/bottom
// - the element still occupies space at its original position in normal flow

use crate::*;

// Case: basic `position: relative` positive offsets
// Spec meaning:
// - a relatively positioned element is shifted by (left, top) from its normal-flow position
// - the space it occupies in the flow is unchanged; here we only assert the visual offset
// In this test:
// - first child: left=10, top=10 → expect_left=10, expect_top=10
// - second child: normal-flow top would be 50 (after first's height), plus top=10 → expect_top=60, left=10
#[test]
fn position_relative_basic() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="10"></div>
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="60"></div>
        </div>
    "#
    )
}

// Case: `position: relative` with negative offsets
// Spec meaning:
// - negative left/top move the element in the opposite direction
// In this test:
// - first child: offset (10,10) as a reference
// - second child: normal-flow top would be 50, plus top=-10 → 40; left=-10
#[test]
fn position_relative_negative() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="10"></div>
            <div style="position: relative; height: 50px; width: 50px; left: -10px; top: -10px;" expect_left="-10" expect_top="40"></div>
        </div>
    "#
    )
}

// Case: `position: relative` inside a flex container
// Spec / engine behavior:
// - flex container lays out items along the main axis (row) with flex-grow
// - the relatively positioned item is shifted additionally by its left/top offsets
// In this test:
// - container: width=200, two flex items with flex-grow:1 → each 100px base width
// - first item: width=100, left=0
// - second item: base left=100, plus left=10 → expect_left=110; top offset=10
#[test]
fn position_relative_in_flex() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="flex-grow: 1; height: 50px;" expect_width="100" expect_left="0"></div>
            <div style="position: relative; flex-grow: 1; height: 50px; left: 10px; top: 10px;" expect_width="100" expect_left="110" expect_top="10"></div>
        </div>
    "#
    )
}

// Case: `position: relative` with right/bottom offsets
// Spec meaning:
// - for relative positioning, right/bottom shift the box in the opposite directions of left/top
// - effectively: right:10 behaves like left:-10, bottom:10 like top:-10
// In this test:
// - element is moved -10 on both axes → expect_left=-10, expect_top=-10
#[test]
fn position_relative_right_bottom() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; right: 10px; bottom: 10px;" expect_left="-10" expect_top="-10"></div>
        </div>
    "#
    )
}

// Case: `position: relative` with margin
// Spec meaning:
// - margin is applied to the normal-flow position first
// - then relative offsets (left/top) are added on top
// In this test:
// - margin-left:5 moves the normal position to left=5
// - left:10 then shifts it to 15 → expect_left=15
// - top=10 shifts downward from the original line, which is asserted as expect_top=10 in this engine
#[test]
fn position_relative_with_margin() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px; margin: 5px;" expect_left="15" expect_top="10"></div>
        </div>
    "#
    )
}

// Case: `position: relative` with padding and border
// Spec meaning:
// - padding and border enlarge the element's box, but relative offsets still apply to the border box
// - inner content sits inside padding+border
// In this test:
// - outer box: content 50x50, padding:5, border:2 → 50 + (5+2)*2 = 64 in both width/height
// - outer is shifted by left/top=10 → expect_left=10, expect_top=10, expect_width=64, expect_height=64
// - inner child is at padding+border: top=7, left=7
#[test]
fn position_relative_with_padding_border() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px; padding: 5px; border-top-width: 2px; border-right-width: 2px; border-bottom-width: 2px; border-left-width: 2px;" expect_left="10" expect_top="10" expect_width="64" expect_height="64">
                <div style="width: 30px; height: 30px;" expect_top="7" expect_left="7"></div>
            </div>
        </div>
    "#
    )
}

// Case: nested `position: relative`
// Spec meaning:
// - each relatively positioned element is offset relative to its own normal-flow position
// - relative positioning is not cumulative across ancestors for the child offsets
// In this test:
// - parent is shifted by (10,10) but we only assert the child's own offsets (20,20) inside the parent
#[test]
fn position_relative_nested() {
    assert_xml!(
        r#"
        <div style="height: 200px; width: 200px;">
            <div style="position: relative; height: 100px; width: 100px; left: 10px; top: 10px;">
                <div style="position: relative; height: 50px; width: 50px; left: 20px; top: 20px;" expect_left="20" expect_top="20"></div>
            </div>
        </div>
    "#
    )
}
