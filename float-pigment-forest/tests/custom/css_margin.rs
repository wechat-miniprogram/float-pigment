use crate::*;

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

// min-height < total-main-size
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

// min-height > total-main-size
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

//
//
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
