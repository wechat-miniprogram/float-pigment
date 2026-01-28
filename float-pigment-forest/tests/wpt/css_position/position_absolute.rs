// WPT-style tests for `position: absolute`
// Inspired by WPT CSS Position tests:
// - absolutely positioned elements are taken out of normal flow
// - they are positioned relative to the nearest positioned ancestor (or initial containing block)
// - percentages for width/height/offsets resolve against the containing block

use crate::*;

// Case: `position: absolute` with fixed size and offsets
// Spec meaning:
// - an absolutely positioned box with explicit width/height is placed at (left, top) relative to its containing block
// In this test:
// - parent: 100x200
// - child: width=10, height=10, left=10, top=10 → expect_left=10, expect_top=10, expect_width=10, expect_height=10
#[test]
fn position_absolute_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; left: 10px; top: 10px;" expect_left="10" expect_top="10" expect_width="10" expect_height="10"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with percentage width/height
// Spec meaning:
// - percentage width/height are resolved against the containing block size (here 100x200)
// In this test:
// - width:10% of 100px → 10
// - height:10% of 200px → 20
// - offset: left=10, top=10
#[test]
fn position_absolute_percentage_size() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10%; width: 10%; left: 10px; top: 10px;" expect_height="20" expect_width="10" expect_left="10" expect_top="10"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with percentage offsets
// Spec meaning:
// - left:50% and top:50% are resolved against containing block width/height
// In this test:
// - parent: 100x200 → left=50, top=100
// - child size is fixed at 10x10 but we only assert its position
#[test]
fn position_absolute_percentage_offsets() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; left: 50%; top: 50%;" expect_left="50" expect_top="100"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with right/bottom
// Spec meaning:
// - right:10 positions the right edge 10px from the containing block's right edge
// - bottom:10 similarly for the bottom edge
// In this test:
// - parent: 100x200, child 10x10
// - left = 100 - 10 (width) - 10 (right) = 80
// - top  = 200 - 10 (height) - 10 (bottom) = 180
#[test]
fn position_absolute_right_bottom() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; right: 10px; bottom: 10px;" expect_left="80" expect_top="180"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` filling container (left/top/right/bottom: 0)
// Spec meaning:
// - when all four inset properties (left, top, right, bottom) are 0 and width/height are auto,
//   the element stretches to fill the containing block
// In this test:
// - parent: 100x200, child fills it → expect_width=100, expect_height=200, top=0, left=0
#[test]
fn position_absolute_fill() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; left: 0; top: 0; right: 0; bottom: 0;" expect_width="100" expect_height="200" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` inside a flex container
// Spec meaning:
// - absolutely positioned elements are taken out of the flex layout
// - they do not affect the flex item distribution; flex items are laid out as if the abspos item wasn't there
// In this test:
// - container: 100x100, three children with flex-grow:1
// - first and third behave as flex items (50px each), abspos middle child is at (20,20) and does not affect widths
#[test]
fn position_absolute_in_flex() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px; display: flex;">
            <div style="flex-grow: 1;" expect_width="50" expect_left="0"></div>
            <div style="flex-grow: 1; position: absolute; left: 20px; top: 20px; width: 10px; height: 10px;" expect_top="20" expect_left="20"></div>
            <div style="flex-grow: 1;" expect_width="50" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with left/right and explicit width
// Spec meaning:
// - when width is specified, left/right insets do not stretch the element, they just constrain possible placement
// - this engine keeps the left offset and honors the specified width, ignoring the implied stretch
// In this test:
// - we assert left=10 and width=50 for the absolutely positioned box
#[test]
fn position_absolute_left_right_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; right: 10px; width: 50px;" expect_left="10" expect_width="50"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with top/bottom and explicit height
// Spec meaning:
// - similar to left/right: explicit height prevents stretching between top/bottom
// In this test:
// - we assert top=10 and height=50
#[test]
fn position_absolute_top_bottom_height() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; top: 10px; bottom: 10px; height: 50px;" expect_top="10" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with margin
// Spec meaning:
// - margin shifts the inset box: the element's border box is offset by margin from the inset positions
// In this test:
// - left=10, margin-left=5 → expect_left=15
// - top=10, margin-top=5  → expect_top=15
#[test]
fn position_absolute_with_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; width: 50px; height: 50px; margin: 5px;" expect_left="15" expect_top="15"></div>
        </div>
    "#
    )
}

// Case: `position: absolute` with padding and border
// Spec meaning:
// - padding and border expand the border box around the content
// - here we assert both the outer size and the inner content offset (padding+border)
// In this test:
// - content: 50x50, padding:5, border:2 → outer 64x64
// - inner child at padding+border = 7px → expect_top=7, expect_left=7
#[test]
fn position_absolute_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; width: 50px; height: 50px; padding: 5px; border-top-width: 2px; border-right-width: 2px; border-bottom-width: 2px; border-left-width: 2px;" expect_left="10" expect_top="10" expect_width="64" expect_height="64">
                <div style="width: 30px; height: 30px;" expect_top="7" expect_left="7"></div>
            </div>
        </div>
    "#
    )
}

// position: absolute with min-width/max-width
#[test]
fn position_absolute_with_min_max_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; width: 30px; min-width: 50px; max-width: 100px; height: 50px;" expect_left="10" expect_top="10" expect_width="50"></div>
        </div>
    "#
    )
}

// position: absolute nested
// Nested absolute positioning: child is positioned relative to parent (which is also absolute)
#[test]
fn position_absolute_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; position: relative;">
            <div style="position: absolute; left: 10px; top: 10px; width: 100px; height: 100px;">
                <div style="position: absolute; left: 20px; top: 20px; width: 50px; height: 50px;" expect_left="20" expect_top="20"></div>
            </div>
        </div>
    "#
    )
}
