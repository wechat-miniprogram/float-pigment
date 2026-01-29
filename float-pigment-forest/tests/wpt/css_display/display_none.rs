// WPT-style tests for the `display: none` property
// Inspired by WPT CSS Display tests:
// - `display:none` elements generate no box, take no space, and do not affect layout
// - all descendants of a `display:none` element are also not rendered

use crate::*;

// Case: basic `display: none` (element not rendered, takes no space)
// Spec meaning:
// - the display:none element has no layout box, so its width/height are effectively 0
// - subsequent siblings are laid out as if the element were not there
// In this test:
// - first child: display:none, expect_width=0, expect_height=0
// - second child: width=100, height=50 at top=0
#[test]
fn display_none() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: none; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `display: none` with children (children are also not rendered)
// Spec meaning:
// - descendants of a display:none element are also suppressed from layout
// In this test:
// - inner child under the display:none parent has expect_width=0, expect_height=0
// - visible sibling after the display:none parent is laid out normally at top=0
#[test]
fn display_none_with_children() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: none; width: 100px; height: 50px;">
                <div style="width: 50px; height: 30px;" expect_width="0" expect_height="0"></div>
            </div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: nested display:none inside a normal block
// Spec meaning:
// - the inner display:none element has no box
// - its sibling block element is laid out as if the display:none element were not present
#[test]
fn display_none_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="width: 100px; height: 50px;">
                <div style="display: none; width: 50px; height: 30px;" expect_width="0" expect_height="0"></div>
                <div style="width: 50px; height: 30px;" expect_width="50" expect_height="30" expect_top="0"></div>
            </div>
        </div>
    "#
    )
}

// Case: `display: none` with margin
// Spec meaning:
// - margin on a display:none element does not affect layout, as the box is not generated
// In this test:
// - we assert the display:none element has 0x0 size
// - the following visible block is at top=0, unaffected by the hidden element's margin
#[test]
fn display_none_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: none; width: 100px; height: 50px; margin: 20px;" expect_width="0" expect_height="0"></div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `display: none` with padding and border (not applied)
// Spec meaning:
// - padding and border have no effect when the element is display:none, since there is no box
// In this test:
// - we assert the hidden element has 0x0 size
// - the following visible block is laid out at top=0 with its specified size
#[test]
fn display_none_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: none; width: 100px; height: 50px; padding: 10px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="0" expect_height="0"></div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `display: none` in a flex container (doesn't affect flex layout)
// Spec meaning:
// - display:none items are removed from the flex items list
// In this test:
// - hidden flex child has 0x0 size
// - visible flex child is placed at left=0, width=50, unaffected by the hidden item
#[test]
fn display_none_in_flex() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="display: none; width: 50px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="width: 50px; height: 50px;" expect_width="50" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `display: none` with `position:absolute`
// Spec meaning:
// - position does not override display:none; absolutely positioned elements with display:none are also not rendered
// In this test:
// - we assert the absolutely positioned element with display:none has 0x0 size
#[test]
fn display_none_with_position_absolute() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; position: relative;">
            <div style="display: none; position: absolute; left: 10px; top: 20px; width: 50px; height: 50px;" expect_width="0" expect_height="0"></div>
        </div>
    "#
    )
}


// Case: `display: none` on multiple siblings
// Spec meaning:
// - multiple hidden siblings do not take space; only visible siblings contribute to layout
// In this test:
// - hidden siblings all have 0x0
// - visible siblings are laid out at top=0 and top=50 respectively
#[test]
fn display_none_multiple_siblings() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="display: none; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="0"></div>
            <div style="display: none; width: 100px; height: 50px;" expect_width="0" expect_height="0"></div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: `display: none` parent with a visible child in style
// Spec meaning:
// - a display:none ancestor suppresses all descendants regardless of their own display values
// In this test:
// - the inner block under the display:none parent has 0x0 size
// - the following visible sibling is laid out normally at top=0 with its specified size
#[test]
fn display_none_parent_visible_child() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="display: none; width: 100px; height: 50px;">
                <div style="display: block; width: 50px; height: 30px;" expect_width="0" expect_height="0"></div>
            </div>
            <div style="width: 100px; height: 50px;" expect_width="100" expect_height="50" expect_top="0"></div>
        </div>
    "#
    )
}
