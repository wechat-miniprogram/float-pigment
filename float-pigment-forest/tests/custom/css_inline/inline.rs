use crate::*;

#[test]
fn inline() {
    assert_xml!(
        r#"
        <div>
          <div expect_height="16" expect_width="375">XX</div>
          <div style="display: inline;" expect_width="32" expect_height="16">
            <div style="display: inline" expect_height="16">XX</div>
          </div>
          <div style="display: inline;" expect_width="64">
            XX <div style="display: inline" expect_left="32">XX</div>
          </div>
        </div>
    "#
    )
}

#[test]
fn inline_set_size() {
    assert_xml!(
        r#"
        <div>
          <div style="display: flex">
            <div style="display: inline; height: 10px; width: 10px" expect_width="10" expect_height="10"></div>
          </div>
          <div style="display: block">
            <div style="display: inline; height: 10px; width: 10px" expect_width="0" expect_height="0"></div>
          </div>
          </div>
          "#
    )
}

#[test]
fn inline_block_with_padding() {
    assert_xml!(
        r#"
        <div style="display: inline-block; height: 40px; padding: 15px;" expect_width="60" expect_height="70" >
          <div style="height: 30px; width: 30px;"></div>
          <div style="height: 30px; width: 30px;"></div>
        </div>
    "#
    )
}

#[test]
fn inline_block_with_padding_2() {
    assert_xml!(
        r#"
        <div style="display: inline-block; height: 40px; padding: 15px;" expect_width="90" expect_height="70" >
          <div style="display: inline-block; height: 30px; width: 30px;"></div>
          <div style="display: inline-block; height: 30px; width: 30px;"></div>
        </div>
    "#
    )
}

#[test]
fn inline_in_block() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; padding: 50px; height: 200px;">
          <custom style="inline">
            <div style="width: 100%; height: 100%;" expect_width="275" expect_height="100"></div>
          </custom>
        </div>
    "#
    )
}

#[test]
fn inline_in_flexbox() {
    assert_xml!(
        r#"
          <div>
            <div style="display: flex; flex-direction: column; height: 100px" expect_height="100" expect_width="375">
              <div style="display: inline" expect_height="16" expect_width="375">XX</div>
            </div>
            <div style="display: flex;" expect_height="16" expect_width="375">
              <div style="display: inline" expect_height="16" expect_width="32">XX</div>
            </div>
          </div>
      "#
    )
}

// #[test]
// fn inline_wrap() {
//     assert_xml!(
//         r#"
//         <div>
//           <div style="display: inline;" expect_width="32" expect_height="16">
//             <div style="display: inline" expect_height="16">XX</div>
//           </div>
//         </div>
//     "#
//     )
// }

#[test]
fn inline_block() {
    assert_xml!(
        r#"
          <div expect_height="16" expect_width="375">
            <div style="display: inline-block" expect_height="16" expect_width="32">XX</div>
            <div style="display: inline-block" expect_height="16" expect_width="32" expect_left="32">XX</div>
          </div>
      "#
    )
}

#[test]
fn inline_block_vertical_align_1() {
    assert_xml!(
        r#"
          <div expect_height="50" expect_width="375">
            <div style="display: inline-block; height: 30px; width: 20px;" expect_height="30" expect_top="20"></div>
            <div style="display: inline-block; height: 50px; width: 20px" expect_height="50" expect_top="0" expect_left="20"></div>
          </div>
      "#
    )
}

#[test]
fn inline_block_vertical_align_2() {
    assert_xml!(
        r#"
          <div expect_height="50" expect_width="375">
            <div style="display: inline-block; height: 40px; width: 20px;" expect_height="40" expect_top="34">XX</div>
            <div style="display: inline-block; height: 50px; width: 20px" expect_height="50" expect_top="0" expect_left="20"></div>
          </div>
      "#
    )
}

#[test]
fn inline_block_wrap() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;">
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="0" expect_top="0"></div>
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="30" expect_top="0"></div>
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="60" expect_top="0"></div>
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="0" expect_top="10"></div>
        </div>
    "#
    )
}

#[test]
fn inline_block_wrap_2() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;">
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="0" expect_top="20"></div>
          <div style="display: inline-block; width: 30px; height: 20px;" expect_left="30" expect_top="10"></div>
          <div style="display: inline-block; width: 40px; height: 30px;" expect_left="60" expect_top="0"></div>
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="0" expect_top="30"></div>
        </div>
    "#
    )
}

#[test]
fn inline_block_wrap_3() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;">
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="0" expect_top="10"></div>
          <div style="display: inline-block; width: 30px; height: 20px;" expect_left="30" expect_top="0"></div>
          <div style="width: 100px; height: 10px" expect_left="0" expect_top="20"></div>
          <div style="display: inline-block; width: 40px; height: 30px;" expect_left="0" expect_top="30"></div>
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="40" expect_top="50"></div>
        </div>
    "#
    )
}

#[test]
fn inline_block_wrap_4() {
    assert_xml!(
        r#"
        <div style="width: 10px; height: 10px;">
          <div style="display: inline-block; width: 30px; height: 10px;" expect_left="0" expect_top="0"></div>
          <div style="display: inline-block; width: 30px; height: 20px;" expect_left="0" expect_top="10"></div>
        </div>
    "#
    )
}

#[test]
fn inline_block_margin() {
    assert_xml!(
        r#"
        <div expect_height="50">
          <div style="display: inline-block; width: 10px; height: 10px; margin: 20px;" expect_left="20" expect_height="10" expect_width="10"></div>
        </div>
      "#
    )
}

#[test]
fn inline_block_margin_1() {
    assert_xml!(
        r#"
        <div expect_height="50">
          <div style="display: inline-block; width: 10px; height: 10px; margin: 10px 20px 30px 40px;" expect_left="40" expect_height="10" expect_width="10"></div>
        </div>
      "#
    )
}

#[test]
fn inline_block_margin_2() {
    assert_xml!(
        r#"
        <div expect_height="60">
          <div style="display: inline-block; width: 10px; height: 10px; margin: 10px 20px 30px 40px;" expect_top="20" expect_left="40" expect_height="10" expect_width="10"></div>
          <div style="display: inline-block; width: 10px; height: 20px; margin: 10px 20px 30px 40px;" expect_top="10" expect_left="110" expect_height="20" expect_width="10"></div>
        </div>
      "#
    )
}

#[test]
fn inline_block_margin_3() {
    assert_xml!(
        r#"
        <div style="width: 100px;" expect_height="110">
          <div style="display: inline-block; width: 10px; height: 10px; margin: 10px 20px 30px 40px;" expect_left="40" expect_top="10" expect_height="10" expect_width="10"></div>
          <div style="display: inline-block; width: 10px; height: 20px; margin: 10px 20px 30px 40px;" expect_left="40" expect_top="60" expect_height="20" expect_width="10"></div>
        </div>
      "#
    )
}

#[test]
fn inline_block_in_flexbox() {
    assert_xml!(
        r#"
          <div>
            <div style="display: flex; flex-direction: column; height: 100px" expect_height="100" expect_width="375">
              <div style="display: inline-block" expect_height="16" expect_width="375">XX</div>
            </div>
            <div style="display: flex;" expect_height="16" expect_width="375">
              <div style="display: inline-block" expect_height="16" expect_width="32">XX</div>
            </div>
            <div style="display: flex; flex-direction: column; align-items: center; width: 96px;" expect_height="16" expect_width="96">
              <div style="display: inline-block" expect_left="32" expect_height="16" expect_width="32">
                XX
              </div>
            </div>
            <div style="display: flex; flex-direction: row; align-items: center; justify-content: center; width: 96px;" expect_height="16" expect_width="96">
              <div style="display: inline-block" expect_left="32" expect_height="16" expect_width="32">
                XX
              </div>
            </div>
          </div>
      "#
    )
}

#[test]
fn block_in_inline_block() {
    assert_xml!(
        r#"
          <div>
            <div expect_width="375">
              <div style="display: inline-block" expect_height="16" expect_width="32">
                <div expect_height="16" expect_width="32">XX</div>
              </div>
            </div>
          </div>
      "#
    )
}

use float_pigment_css::typing::Display;
use float_pigment_forest::{
    convert_node_ref_to_ptr, ChildOperation, DumpNode, DumpOptions, DumpStyleMode, Node,
    StyleSetter,
};
use float_pigment_layout::{DefLength, OptionNum, OptionSize, Size};

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

#[test]
pub fn inline_block_as_root() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::InlineBlock);
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(10.)));
        child.set_height(DefLength::Points(Len::from_f32(20.)));
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(container.layout_position().width, 10.);
    }
}

#[test]
pub fn inline_as_root() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Inline);
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Percent(1.));
        child.set_height(DefLength::Points(Len::from_f32(100.)));
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(container.layout_position().width, 375.);
        assert_eq!(child.layout_position().width, 375.);
    }
}

#[test]
pub fn measurable_inline_block_with_padding() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(25.)));
        child.set_height(DefLength::Points(Len::from_f32(25.)));
        child.set_display(Display::InlineBlock);
        child.set_padding_left(DefLength::Points(Len::from_f32(12.)));
        child.set_padding_right(DefLength::Points(Len::from_f32(12.)));
        child.set_is_measurable(true);
        set_node_measure_type(
            convert_node_ref_to_ptr(child),
            MeasureType::SpecifiedSize((20., 20.)),
        );
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(child.layout_position().left, 0.);
        assert_eq!(child.layout_position().width, 49.);
        assert_eq!(child.layout_position().height, 25.);
    }
}

#[test]
pub fn measurable_inline_block_with_margin() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(25.)));
        child.set_height(DefLength::Points(Len::from_f32(25.)));
        child.set_display(Display::InlineBlock);
        child.set_margin_left(DefLength::Points(Len::from_f32(12.)));
        child.set_margin_right(DefLength::Points(Len::from_f32(12.)));
        set_node_measure_type(
            convert_node_ref_to_ptr(child),
            MeasureType::SpecifiedSize((20., 20.)),
        );
        child.set_is_measurable(true);
        container.append_child(convert_node_ref_to_ptr(child));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(child.layout_position().left, 12.);
        assert_eq!(child.layout_position().width, 25.);
    }
}

#[test]
pub fn measurable_inline_block_with_margin_2() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(25.)));
        child.set_height(DefLength::Points(Len::from_f32(25.)));
        child.set_display(Display::InlineBlock);
        child.set_margin_left(DefLength::Points(Len::from_f32(12.)));
        child.set_margin_right(DefLength::Points(Len::from_f32(12.)));
        child.set_is_measurable(true);
        set_node_measure_type(
            convert_node_ref_to_ptr(child),
            MeasureType::SpecifiedSize((20., 20.)),
        );
        container.append_child(convert_node_ref_to_ptr(child));
        let child_b = as_ref(Node::new_ptr());
        child_b.set_width(DefLength::Points(Len::from_f32(25.)));
        child_b.set_height(DefLength::Points(Len::from_f32(25.)));
        child_b.set_display(Display::InlineBlock);
        child_b.set_margin_left(DefLength::Points(Len::from_f32(12.)));
        child_b.set_margin_right(DefLength::Points(Len::from_f32(12.)));
        child_b.set_is_measurable(true);
        set_node_measure_type(
            convert_node_ref_to_ptr(child_b),
            MeasureType::SpecifiedSize((25., 25.)),
        );
        container.append_child(convert_node_ref_to_ptr(child_b));
        container.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            container.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::None
                },
                0
            )
        );
        assert_eq!(child.layout_position().left, 12.);
        assert_eq!(child.layout_position().width, 25.);
        assert_eq!(child_b.layout_position().left, 61.);
        assert_eq!(child_b.layout_position().width, 25.);
    }
}

#[test]
fn inline_complex_1() {
    assert_xml!(
        r#"
          <span>
            <div style="position: relative; display: flex; flex-direction: row; height: 40px; width: 100%; box-sizing: border-box;" expect_height="40" expect_width="375">
              <div style="position: relative; height: 100%; box-sizing: border-box; flex-grow: 1; flex-basis: 0%;" expect_height="40" expect_width="375">
                <span style="position: relative; box-sizing: border-box;" expect_height="20" expect_width="375">
                  <div style="width: 100%; position: relative; box-sizing: border-box;" expect_height="20" expect_width="375">
                    <div style="position: relative; height: 20px; width: 100px; box-sizing: border-box;" expect_height="20" expect_width="100"></div>
                  </div>
                </span>
              </div>
            </div>
          </span>
      "#
    )
}

#[test]
fn inline_flex() {
    assert_xml!(
        r#"
          <div style="height: 100px; width: 60px;">
            <div style="display: inline-flex;" expect_width="30">
              <div style="height: 10px; width: 10px;"></div>
              <div style="height: 10px; width: 20px;" expect_left="10"></div>
            </div>
            <div style="display: inline-flex;" expect_width="30" expect_left="30">
              <div style="height: 10px; width: 10px;"></div>
              <div style="height: 10px; width: 20px;" expect_left="10"></div>
            </div>
            <div style="display: inline-flex; height: 50px;" expect_width="30" expect_left="0" expect_top="10">
              <div style="height: 10px; width: 10px;"></div>
              <div style="height: 10px; width: 20px;" expect_left="10"></div>
            </div>
          </div>
      "#
    )
}

#[test]
fn inline_block_wrap_precision() {
    assert_xml!(
        r#"
          <div style="width: 392.7px">
            <div style=" height: 800px; padding-left: 10.474036px; padding-right: 10.474036px;">
              <div>
                <div style="width: 100%;position: absolute;left: 0px;">
                  <div>
                    <div style="display: inline-block; width: 20%; height: 50px;"></div>
                    <div style="display: inline-block; width: 20%; height: 50px;"></div>
                    <div style="display: inline-block; width: 20%; height: 50px;"></div>
                    <div style="display: inline-block; width: 20%; height: 50px;"></div>
                    <div style="display: inline-block; width: 20%; height: 50px;"></div>
                    <div style="display: inline-block; width: 20%; height: 50px;" expect_left="0" expect_top="50"></div>

                  </div>
                </div>
              </div>
            </div>
            <div style="width: 100px;">
              <div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;"></div>
                <div style="display: inline-block; width: 10%; height: 50px;" expect_left="0" expect_top="50"></div>
              </div>
            </div>
          </div>
      "#
    )
}
