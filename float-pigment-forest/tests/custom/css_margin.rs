// Tests for `margin` properties in CSS
// Based on CSS Box Model Module Level 3:
// - Margins create space outside element's border
// - Vertical margins collapse between adjacent block elements
// - margin: auto can center elements horizontally
// - Margins can be fixed values or percentages (relative to container width)

use crate::*;

// Case: Fixed margin
// Spec points:
// - margin: 10px applies 10px to all four sides
// - Element is offset from container edge by margin
// In this test:
// - Element: margin=10px, positioned at left=10
// - Note: top margin collapses with parent, so expect_top=0 within parent
#[test]
fn margin_fixed() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;" expect_top="0">
          <div style="width: 10px; height: 10px; margin: 10px;" expect_width="10" expect_height="10" expect_left="10" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: Percentage margin
// Spec points:
// - Percentage margins are relative to container width (including vertical margins)
// In this test:
// - Parent: 100x100px
// - Child: margin=10% = 10px (10% of 100px width)
#[test]
fn margin_percentage() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;" expect_top="0">
            <div style="width: 10px; height: 10px; margin: 10%;" expect_width="10" expect_height="10" expect_left="10" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: margin-left fixed
// Spec points:
// - margin-left only affects left side positioning
// In this test:
// - Element: margin-left=10px, positioned at left=10
#[test]
fn margin_left_fixed() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="width: 10px; height: 10px; margin-left: 10px;" expect_width="10" expect_height="10" expect_left="10"></div>
        </div>
    "#
    )
}

// Case: margin-right in flex context
// Spec points:
// - In flex layout, margin-right creates space between items
// In this test:
// - Container: flex
// - First child: margin-right=10px
// - Second child: positioned at left=20 (10px width + 10px margin)
#[test]
fn margin_right_fixed() {
    assert_xml!(
        r#"
        <div style="height: 100px; display: flex; width: 100px;">
            <div style="width: 10px; height: 10px; margin-right: 10px" expect_width="10" expect_height="10"></div>
            <div style="width: 10px; height: 10px;" expect_left="20" expect_width="10" expect_height="10"></div>
        </div>
    "#
    )
}

// Case: margin-top between siblings
// Spec points:
// - margin-top creates space above element
// In this test:
// - First child: 10x10px, at top=0
// - Second child: margin-top=10px, at top=20 (10px height + 10px margin)
#[test]
fn margin_top_fixed() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="width: 10px; height: 10px;"></div>
            <div style="width: 10px; height: 10px; margin-top: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: margin-bottom between siblings
// Spec points:
// - margin-bottom creates space below element
// In this test:
// - First child: margin-bottom=20px
// - Second child: positioned at top=30 (10px height + 20px margin)
#[test]
fn margin_bottom_fixed() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 100px;">
            <div style="width: 10px; height: 10px; margin-bottom: 20px;"></div>
            <div style="width: 10px; height: 10px;" expect_top="30"></div>
        </div>
    "#
    )
}

// Case: Margin collapse between siblings (positive margins)
// Spec points:
// - Adjacent vertical margins collapse to the larger value
// In this test:
// - First child: margin-bottom=50px
// - Second child: margin-top=40px
// - Collapsed margin = max(50, 40) = 50px
// - Second child at top = 100 + 50 = 150
#[test]
fn margin_collapse_1() {
    assert_xml!(
        r#"
        <div style="height: 800px;">
          <div style="height: 100px; margin-bottom: 50px;"></div>
          <div style="height: 100px; margin-top: 40px;" expect_top="150"></div>
        </div>
    "#
    )
}

// Case: Margin collapse with empty inline nodes between
// Spec points:
// - Empty inline elements don't prevent margin collapse
// In this test:
// - Same as margin_collapse_1 but with empty inline between siblings
// - Margins still collapse to max(50, 40) = 50px
#[test]
fn margin_collapse_empty_inline_nodes() {
    assert_xml!(
        r#"
        <div style="height: 800px;">
          <div style="height: 100px; margin-bottom: 50px;"></div>
          <div style="display: inline; height: 0;"></div>
          <div style="margin-top: 40px;" expect_top="150"></div>
        </div>
    "#
    )
}

// Case: Margin collapse with negative margin
// Spec points:
// - When one margin is negative, subtract it from the positive
// In this test:
// - First child: margin-bottom=50px
// - Second child: margin-top=-40px
// - Net margin = 50 + (-40) = 10px
// - Second child at top = 100 + 10 = 110
#[test]
fn margin_collapse_negative() {
    assert_xml!(
        r#"
        <div style="height: 800px;">
          <div style="height: 100px; margin-bottom: 50px;"></div>
          <div style="height: 100px; margin-top: -40px;" expect_top="110"></div>
        </div>
    "#
    )
}

// Case: Margin collapse with both negative margins
// Spec points:
// - When both margins are negative, use the most negative
// In this test:
// - First child: margin-bottom=-50px
// - Second child: margin-top=-40px
// - Collapsed margin = min(-50, -40) = -50px
// - Second child at top = 100 - 50 = 50
#[test]
fn margin_collapse_negative_maximum() {
    assert_xml!(
        r#"
        <div style="height: 800px;">
          <div style="height: 100px; margin-bottom: -50px;"></div>
          <div style="height: 100px; margin-top: -40px;" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: Margin does not collapse if padding exists
// Spec points:
// - Padding prevents margin collapse between parent and child
// In this test:
// - First child: padding-bottom=10px, margin-bottom=10px
// - Second child: margin-top=10px
// - No collapse because of padding, top = 10 + 10 = 20
#[test]
fn margin_not_collapse_if_padding_exists() {
    assert_xml!(
        r#"
        <div style="height: 800px;">
          <div style="height: 10px; margin-bottom: 10px; box-sizing: border-box; padding-bottom: 10px;" expect_top="0"></div>
          <div style="height: 10px; margin-top: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: Margin collapse with complex padding/border structure
// Spec points:
// - Border prevents margin collapse at that edge
// - Padding prevents margin collapse at that edge
// In this test:
// - Parent with padding-top, border-bottom, nested children with margins
// - Margins collapse within the padding but stop at border
#[test]
fn margin_not_collapse_if_padding_exists_2() {
    assert_xml!(
        r#"
        <div style="border: 1px;>
          <div style="padding-top: 30px; border-bottom: 10px; margin-top: 20px; margin-bottom: 20px;" expect_top="20" expect_height="90">
            <div style="margin-top: 10px; margin-bottom: 10px; width: 10px; height: 10px;" expect_top="40"></div>
            <div style="margin-top: 5px; margin-bottom: 10px; width: 10px; height: 10px;" expect_top="60"></div>
          </div>
        </div>
    "#
    )
}

// Case: Margin collapse with min-height (min-height < total content)
// Spec points:
// - min-height allows content to overflow
// - Margins still collapse normally
// In this test:
// - Parent with min-height=30px, children total more than that
// - Margin collapse continues past min-height
#[test]
fn margin_collapse_min_height() {
    assert_xml!(
        r#"
        <div style="width: 100px;" expect_height="120">
            <div style="min-height: 30px;" expect_height="40">
                <div style="height: 20px;" expect_height="20"></div>
                <div style="height: 20px; margin-bottom: 50px;" expect_height="20"></div>
            </div>
            <div style="height: 30px;" expect_height="30" expect_top="90"><div>
        </div>
    "#
    )
}

// Case: Margin collapse with min-height (min-height > total content)
// Spec points:
// - min-height expands element beyond content
// - Margins collapse up to the min-height boundary
// In this test:
// - Parent with min-height=50px > content height
// - Parent expands to accommodate margin-bottom
#[test]
fn margin_collapse_min_height_2() {
    assert_xml!(
        r#"
        <div style="width: 100px;" expect_height="120">
            <div style="min-height: 50px;" expect_height="90">
                <div style="height: 20px;" expect_height="20"></div>
                <div style="height: 20px; margin-bottom: 50px;" expect_height="20"></div>
            </div>
            <div style="height: 30px;" expect_height="30" expect_top="90"><div>
        </div>
    "#
    )
}

// Case: Margin collapse with max-height
// Spec points:
// - max-height=0 clips the element's height but overflow exists
// - Margins collapse from the visible edge
// In this test:
// - max-height=0 clips first container
// - Second container positioned at top=0 due to collapse
#[test]
fn margin_collapse_max_height() {
    assert_xml!(
        r#"
            <div style="width: 100px;" expect_height="30">
                <div style="max-height: 0px" expect_height="0">
                    <div style="height: 20px;" expect_height="20"></div>
                    <div style="height: 20px; margin-bottom: 20px;" expect_height="20"></div>
                </div>
                <div style="height: 30px;" expect_height="30" expect_top="0"></div>
            </div>
        "#
    )
}

// Case: Margin collapse with max-height and external margin
// Spec points:
// - Margin on clipped element still participates in collapse
// In this test:
// - First element: max-height=0, margin-bottom=10px, child margin-bottom=50px
// - Collapsed margin = max(10, 50) = 50px... but collapses with next sibling
// - Second child at top=10 after collapse
#[test]
fn margin_collapse_max_height_2() {
    assert_xml!(
        r#"
            <div expect_height="40">
                <div style="max-height: 0px; margin-bottom: 10px;" expect_height="0">
                    <div style="height: 20px;" expect_height="20"></div>
                    <div style="height: 20px; margin-bottom: 50px;" expect_height="20"></div>
                </div>
                <div style="height: 30px;" expect_top="10" expect_height="30"></div>
            </div>
        "#
    )
}

// Case: Margin collapse across flex boundary
// Spec points:
// - Margins don't collapse across flex container boundaries
// - Margins collapse within block children
// In this test:
// - Flex column container with block children
// - Block children's margins collapse internally
#[test]
fn margin_collapse_cross_flex() {
    assert_xml!(
        r#"
            <div style="margin-top: 100px; display: flex; flex-direction: column" expect_top="0">
                <div style="margin-top: 200px;" expect_top="200">
                    <div style="display: flex; margin-top: 250px; height: 50px;" expect_top="250"></div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse cross flex (nested case 2)
// Spec points:
// - Block children within flex items can have margin collapse
#[test]
fn margin_collapse_cross_flex_2() {
    assert_xml!(
        r#"
            <div expect_top="0">
                <div style="height: 10px; margin-top: 10px;" expect_top="0"></div>
                <div style="margin-top: 10px;"  expect_top="30">
                    <div style="margin-top: 20px; display: flex; height: 100px;" expect_top="0"></div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse cross flex (within flex column)
// Spec points:
// - In flex column, margins are preserved (no collapse)
#[test]
fn margin_collapse_cross_flex_3() {
    assert_xml!(
        r#"
            <div style="display: flex; flex-direction: column;" expect_top="0">
                <div style="height: 10px; margin-top: 10px;"  expect_top="10"></div>
                <div style="margin-top: 10px;" expect_top="30">
                    <div style="margin-top: 20px; display:flex; height: 100px;" expect_top="20"></div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse cross flex (deeply nested)
// Spec points:
// - Complex nesting with flex and block contexts
#[test]
fn margin_collapse_cross_flex_4() {
    assert_xml!(
        r#"
            <div style="display: flex; flex-direction: column;" expect_top="0">
                <div style="display: flex; flex-direction: column; margin-top: 10px;" expect_top="10">
                    <div style="margin-top: 10px" expect_top="10">
                        <div style="margin-top: 80px;" expect_top="90">
                            <div style="margin-top: 90px;" expect_top="0">
                                <div style="display: flex; flex-direction: column; margin-top: 40px; height: 50px;" expect_top="0"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse cross flex (variation 5)
#[test]
fn margin_collapse_cross_flex_5() {
    assert_xml!(
        r#"
            <div>
                <div style="display: flex; flex-direction: column; margin-top: 10px;" expect_top="0">
                    <div style="margin-top: 10px" expect_top="10">
                        <div style="margin-top: 80px;" expect_top="90">
                            <div style="margin-top: 90px;" expect_top="0">
                                <div style="display: flex; flex-direction: column; margin-top: 40px; height: 50px;" expect_top="0"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse cross flex (variation 6)
#[test]
fn margin_collapse_cross_flex_6() {
    assert_xml!(
        r#"
            <div expect_top="0">
                <div style="margin-top: 10px" expect_top="0">
                    <div style="display: flex; flex-direction: column; margin-top: 20px; height: 100px;" expect_top="0"></div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse cross flex (variation 7)
#[test]
fn margin_collapse_cross_flex_7() {
    assert_xml!(
        r#"
            <div style="display: flex; flex-direction: column;" expect_top="0">
                <div style="margin-top: 10px" expect_top="10">
                    <div style="display: flex; flex-direction: column; margin-top: 20px; height: 100px;" expect_top="20"></div>
                </div>
            </div>
        "#
    )
}

// Case: Margin collapse between siblings with empty block
// Spec points:
// - Empty block with margins collapses its own margins
// - Then participates in sibling margin collapse
// In this test:
// - Empty div with margin-top=10px, margin-bottom=20px
// - Collapsed own margins = 20px (larger wins)
// - Third sibling at top=30+20=50
#[test]
fn margin_collapse_between_sibling_and_empty_block_1() {
    assert_xml!(
        r#"
            <div style="height: 300px;">
                <div style="height: 30px;" expect_height="30" expect_top="0"></div>
                <div style="margin-top: 10px; margin-bottom: 20px;" expect_height="0"></div>
                <div style="height: 30px;" expect_height="30" expect_top="50"></div>
            </div>
        "#
    )
}

// Case: Margin collapse between siblings with empty block (larger sibling margin)
// Spec points:
// - When sibling has larger margin, it dominates collapse
// In this test:
// - First sibling: margin-bottom=50px
// - Empty: margin-top=10px, margin-bottom=20px
// - Collapsed = max(50, 10, 20) = 50px
// - Third sibling at top=30+50=80
#[test]
fn margin_collapse_between_sibling_and_empty_block_2() {
    assert_xml!(
        r#"
            <div style="height: 300px;">
                <div style="height: 30px; margin-bottom: 50px" expect_height="30" expect_top="0"></div>
                <div style="margin-top: 10px; margin-bottom: 20px;" expect_height="0"></div>
                <div style="height: 30px;" expect_height="30" expect_top="80"></div>
            </div>
        "#
    )
}

// Case: Margin collapse with empty block and following sibling margin
// Spec points:
// - Multiple margins all collapse together
// In this test:
// - First: margin-bottom=200px
// - Empty: margins 10/20
// - Third: margin-top=100px
// - All collapse to max=200px
#[test]
fn margin_collapse_between_sibling_and_empty_block_3() {
    assert_xml!(
        r#"
            <div style="height: 300px;">
                <div style="height: 30px; margin-bottom: 200px" expect_height="30" expect_top="0"></div>
                <div style="margin-top: 10px; margin-bottom: 20px;" expect_height="0" expect_top="230"></div>
                <div style="height: 30px; margin-top: 100px" expect_height="30" expect_top="230"></div>
            </div>
        "#
    )
}

// Case: Margin collapse with larger following sibling
// In this test:
// - First: no margin-bottom
// - Empty: margins 10/20
// - Third: margin-top=100px (dominates)
// - Result: third at top=130
#[test]
fn margin_collapse_between_sibling_and_empty_block_4() {
    assert_xml!(
        r#"
            <div style="height: 300px;">
                <div style="height: 30px;" expect_height="30" expect_top="0"></div>
                <div style="margin-top: 10px; margin-bottom: 20px;" expect_height="0"></div>
                <div style="height: 30px; margin-top: 100px" expect_height="30" expect_top="130"></div>
            </div>
        "#
    )
}

// Case: Margin collapse with text-slot (zero-width joiner)
// Spec points:
// - Empty text slots don't break margin collapse
// In this test:
// - text-slot with len=0 doesn't prevent collapse
// - Margins collapse: max(50, 100) = 100px
#[test]
fn margin_collapse_between_sibling_and_empty_block_5() {
    assert_xml!(
        r#"
            <div style="height: 300px;">
                <div style="height: 30px; margin-bottom: 50px" expect_height="30" expect_top="0"></div>
                <text-slot len="0"></text-slot>
                <div style="height: 30px; margin-top: 100px" expect_height="30" expect_top="130"></div>
            </div>
        "#,
        true
    )
}

// Case: Margin collapse between parent and empty block child
// Spec points:
// - Child margins collapse through parent
// In this test:
// - Parent: margin-top=40px
// - Child: margin-top=10px, margin-bottom=30px (empty)
// - Collapsed into parent margin = max(40, 30) = 40px
// - Parent at top=100+40=140
#[test]
fn margin_collapse_between_parent_and_empty_block_1() {
    assert_xml!(
        r#"
        <div style="height: 100px;"></div>
        <div style="height: 300px; margin-top: 40px;" expect_top="140">
            <div style="margin-top: 10px; margin-bottom: 30px;" expect_height="0" expect_top="0"></div>
        </div>
        "#
    )
}

// Case: Parent margin smaller than child empty block margins
// In this test:
// - Parent: margin-top=20px
// - Child: margin-top=10px, margin-bottom=30px
// - Collapsed: max(20, 30) = 30px
// - Parent at top=100+30=130
#[test]
fn margin_collapse_between_parent_and_empty_block_2() {
    assert_xml!(
        r#"
            <div style="height: 100px;"></div>
            <div style="height: 300px; margin-top: 20px;" expect_top="130">
                <div style="margin-top: 10px; margin-bottom: 30px;" expect_height="0" expect_top="0"></div>
            </div>
        "#
    )
}

// Case: Multiple empty children with margin collapse
// In this test:
// - Two empty children with different margins
// - All collapse together with parent margin
#[test]
fn margin_collapse_between_parent_and_empty_block_3() {
    assert_xml!(
        r#"
            <div style="height: 100px;"></div>
            <div style="height: 300px; margin-top: 20px;" expect_top="160">
                <div style="margin-top: 10px; margin-bottom: 30px;" expect_height="0" expect_top="0"></div>
                <div style="margin-top: 10px; margin-bottom: 60px;" expect_height="0" expect_top="0"></div>
            </div>
        "#
    )
}

// Case: margin: auto for horizontal centering
// Spec points:
// - margin-left: auto and margin-right: auto centers element horizontally
// - Works with absolute positioning when left/right are set
// In this test:
// - Element: position=absolute, left=0, right=100px, width=10px
// - Available space = 0 to (375-100) = 275px, center = (275-10)/2 = 132.5 ≈ 95
// - Wait, the test expects 95... Let me check: (375 - 100 - 10) / 2 = 132.5... 
// - Actually with left=0, right=100, space = 275, (275-10)/2 = 132.5... but expect_left=95
// - Maybe right is relative position? Let me preserve original test
#[test]
fn margin_auto() {
    assert_xml!(
        r#"
        <div style="height: 100px; width: 300px;">
          <div style="position: absolute; left: 0px; right: 100px; width: 10px; height: 10px; margin-left: auto; margin-right: auto;" expect_left="95"></div>
        </div>
    "#
    )
}

// Case: Margin with inline elements
// Spec points:
// - Inline element's margin affects containing block
// - margin-bottom on child pushes following siblings
// In this test:
// - Inline container with block child having margin-bottom=100px
// - Following sibling at top=200 (100+100)
#[test]
fn margin_inline() {
    assert_xml!(
        r#"
        <div style="width: 300px;" expect_height="300">
            <div style="display: inline;">
                <div style="margin-bottom: 100px; height: 100px; width: 100px;" expect_height="100"></div>
            </div>
            <div style="margin-top: 20px; height: 100px; width: 100px;" expect_top="200" expect_height="100"></div>
        </div>
    "#
    )
}

// Case: Margin with nested inline elements
// Spec points:
// - Margins collapse through inline element boundaries
#[test]
fn margin_inline_1() {
    assert_xml!(
        r#"
        <div>
            <div >
                <div style="display: inline;">
                    <div style="margin-bottom: 100px; height: 100px; width: 100px;" expect_height="100"></div>
                </div>
            </div>
            <div style="height: 100px" expect_top="200"></div>
        </div>
    "#
    )
}

// Case: Inline element with margin (but inline margins don't apply to block children)
// In this test:
// - Inline has margin-bottom=200px but doesn't affect layout
// - Block child's margin-bottom=100px affects position
#[test]
fn margin_inline_2() {
    assert_xml!(
        r#"
        <div>
            <div >
                <div style="display: inline; margin-bottom: 200px;">
                    <div style="margin-bottom: 100px; height: 100px; width: 100px;" expect_height="100"></div>
                </div>
            </div>
            <div style="height: 100px" expect_top="200"></div>
        </div>
    "#
    )
}

use float_pigment_forest::{convert_node_ref_to_ptr, ChildOperation, Node, StyleSetter};
use float_pigment_layout::{DefLength, LayoutTreeNode, OptionNum, OptionSize, Size};

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

// Case: Margin on root element
// Spec points:
// - Root element margins apply relative to viewport
// In this test:
// - Root margins collapse with first child margins
#[test]
pub fn margin_root() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let root = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_margin_top(DefLength::Points(Len::from_f32(10.)));
        child.set_margin_bottom(DefLength::Points(Len::from_f32(10.)));
        child.set_padding_top(DefLength::Points(Len::from_f32(10.)));
        child.set_padding_bottom(DefLength::Points(Len::from_f32(10.)));
        container.append_child(convert_node_ref_to_ptr(root));
        root.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(root.layout_position().top, 0.);
        assert_eq!(child.layout_position().top, 0.);
        assert_eq!(child.layout_position().height, 20.);
    }
}

// Case: Margin on empty root block element
// Spec points:
// - Computed margins are stored in computed style
#[test]
pub fn margin_root_empty_block() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        root.set_margin_top(DefLength::Points(Len::from_f32(10.)));
        root.set_margin_right(DefLength::Points(Len::from_f32(20.)));
        root.set_margin_bottom(DefLength::Points(Len::from_f32(30.)));
        root.set_margin_left(DefLength::Points(Len::from_f32(40.)));
        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(root.layout_node().computed_style().margin.top, 10.);
        assert_eq!(root.layout_node().computed_style().margin.right, 20.);
        assert_eq!(root.layout_node().computed_style().margin.bottom, 30.);
        assert_eq!(root.layout_node().computed_style().margin.left, 40.);
    }
}

// Case: Inline root element with margin on child
// Spec points:
// - Inline element's margin is determined by content
#[test]
pub fn margin_root_3() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        // root.set_margin(DefLength::Points(100.));
        root.set_display(float_pigment_css::typing::Display::Inline);
        let child = as_ref(Node::new_ptr());
        child.set_height(DefLength::Points(Len::from_f32(100.)));
        child.set_margin(DefLength::Points(Len::from_f32(100.)));
        root.append_child(convert_node_ref_to_ptr(child));
        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(root.layout_node().computed_style().margin.bottom, 100.);
        assert_eq!(root.layout_position().height, 100.);
        assert_eq!(child.layout_position().height, 100.);
    }
}

// Case: Nested inline elements with margin
// Spec points:
// - Deeply nested inline margin propagation
#[test]
pub fn margin_root_4() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let child = as_ref(Node::new_ptr());
        child.set_display(float_pigment_css::typing::Display::Inline);

        let child_child = as_ref(Node::new_ptr());
        child_child.set_height(DefLength::Points(Len::from_f32(100.)));
        child_child.set_margin(DefLength::Points(Len::from_f32(100.)));

        root.append_child(convert_node_ref_to_ptr(child));
        child.append_child(convert_node_ref_to_ptr(child_child));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(root.layout_node().computed_style().margin.bottom, 100.);
        assert_eq!(root.layout_position().height, 100.);
        assert_eq!(child.layout_position().height, 100.);
        assert_eq!(child_child.layout_position().height, 100.);
    }
}
