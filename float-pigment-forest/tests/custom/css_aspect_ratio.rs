use crate::*;
use float_pigment_css::typing::*;

use float_pigment_layout::{DefLength, OptionNum, OptionSize};
unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

#[test]
pub fn aspect_ratio_in_flex_row() {
    assert_xml!(
        r#"
        <div style="width: 700px; height: 300px; display: flex; flex-direction: row;">
            <div style="width: 100px; height: 100px; aspect-ratio: 0.75 / 1;" expect_width="100" expect_height="100"></div>
            <div style="width: auto; height: 100px; aspect-ratio: 0.75 / 1;" expect_width="75" expect_height="100"></div>
            <div style="width: 100px; height: auto; aspect-ratio: 0.75 / 1;" expect_width="100" expect_height="300"></div>
            <div style="width: 100px; height: auto; aspect-ratio: 0.75 / 1; align-self: flex-start" expect_width="100" expect_height="133"></div>
        </div>
    "#
    )
}

#[test]
pub fn aspect_ratio_in_flex_column() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 600px; display: flex; flex-direction: column;">
            <div style="width: 100px; height: 100px; aspect-ratio: 0.75 / 1;" expect_width="100" expect_height="100"></div>
            <div style="width: auto; height: 100px; aspect-ratio: 0.75 / 1;" expect_width="300" expect_height="100"></div>
            <div style="width: auto; height: 100px; aspect-ratio: 0.75 / 1; align-self: flex-start" expect_width="75" expect_height="100"></div>
            <div style="width: 100px; height: auto; aspect-ratio: 0.75 / 1;" expect_width="100" expect_height="133"></div>
        </div>
    "#
    )
}

#[test]
pub fn aspect_ratio_with_flex_1() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(100.)));
        container.set_display(Display::Flex);
        container.set_flex_direction(FlexDirection::Column);
        root.append_child(convert_node_ref_to_ptr(container));

        let child_a = as_ref(Node::new_ptr());
        child_a.set_width(DefLength::Points(Len::from_f32(50.)));
        child_a.set_min_width(DefLength::Points(Len::from_f32(0.)));
        child_a.set_flex_basis(DefLength::Percent(0.));
        child_a.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child_a));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::None
                },
                0
            )
        );

        assert_eq!(child_a.layout_position().width, 50.);
        assert_eq!(child_a.layout_position().height, 50.);
    }
}

#[test]
pub fn aspect_ratio_with_flex_wrap() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_height(DefLength::Points(Len::from_f32(100.)));
        container.set_display(Display::Flex);
        container.set_flex_direction(FlexDirection::Column);
        container.set_flex_wrap(FlexWrap::Wrap);
        container.set_align_items(AlignItems::Stretch);
        root.append_child(convert_node_ref_to_ptr(container));

        let child_a = as_ref(Node::new_ptr());
        child_a.set_height(DefLength::Points(Len::from_f32(50.)));
        child_a.set_min_width(DefLength::Points(Len::from_f32(0.)));
        child_a.set_flex_basis(DefLength::Percent(0.));
        child_a.set_flex_shrink(1.);
        child_a.set_flex_grow(1.);
        child_a.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child_a));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::None
                },
                0
            )
        );

        assert_eq!(child_a.layout_position().width, 100.);
        assert_eq!(child_a.layout_position().height, 100.);
    }
}

#[test]
pub fn aspect_ratio_with_flex_no_wrap_1() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_height(DefLength::Points(Len::from_f32(50.)));
        container.set_display(Display::Flex);
        container.set_flex_direction(FlexDirection::Column);
        container.set_flex_wrap(FlexWrap::NoWrap);
        container.set_align_items(AlignItems::Stretch);
        root.append_child(convert_node_ref_to_ptr(container));

        let child_a = as_ref(Node::new_ptr());
        child_a.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child_a));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::None
                },
                0
            )
        );

        assert_eq!(child_a.layout_position().width, 375.);
        assert_eq!(child_a.layout_position().height, 50.);
    }
}

#[test]
pub fn aspect_ratio_with_flex_no_wrap_2() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(100.)));
        container.set_display(Display::Flex);
        container.set_flex_direction(FlexDirection::Column);
        container.set_flex_wrap(FlexWrap::NoWrap);
        container.set_align_items(AlignItems::Stretch);
        root.append_child(convert_node_ref_to_ptr(container));

        let child_a = as_ref(Node::new_ptr());
        child_a.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child_a));

        let child_b = as_ref(Node::new_ptr());
        child_b.set_height(DefLength::Percent(1.));
        child_a.append_child(convert_node_ref_to_ptr(child_b));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::None
                },
                0
            )
        );

        assert_eq!(child_a.layout_position().width, 100.);
        assert_eq!(child_a.layout_position().height, 100.);
    }
}

#[test]
pub fn aspect_ratio_in_block_width_fixed() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(100.)));
        child.set_height(DefLength::Auto);
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Points(Len::from_f32(100.)));
        child2.set_height(DefLength::Auto);
        child2.set_aspect_ratio(Some(0.5 / 1.));
        container.append_child(convert_node_ref_to_ptr(child2));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(400.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(child.layout_position().width, 100.);
        assert_eq!(child.layout_position().height, 50.);
        assert_eq!(child2.layout_position().width, 100.);
        assert_eq!(child2.layout_position().height, 200.);
    }
}

#[test]
pub fn aspect_ratio_in_block_height_fixed() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Auto);
        container.set_height(DefLength::Points(Len::from_f32(300.)));
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_height(DefLength::Points(Len::from_f32(100.)));
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Auto);
        child2.set_height(DefLength::Points(Len::from_f32(200.)));
        child2.set_aspect_ratio(Some(0.5 / 1.));
        container.append_child(convert_node_ref_to_ptr(child2));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(400.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(child.layout_position().width, 200.);
        assert_eq!(child.layout_position().height, 100.);
        assert_eq!(child2.layout_position().width, 100.);
        assert_eq!(child2.layout_position().height, 200.);
    }
}

// wpt:css/css-sizing/aspect-ratio/block-aspect-ratio-008.html
#[test]
pub fn aspect_ratio_in_parent_block_cross_size_fixed() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let container2 = as_ref(Node::new_ptr());
        container2.set_width(DefLength::Auto);
        container2.set_height(DefLength::Points(Len::from_f32(300.)));
        container2.set_writing_mode(WritingMode::VerticalLr);
        root.append_child(convert_node_ref_to_ptr(container2));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Auto);
        child2.set_aspect_ratio(Some(0.5 / 1.));
        container2.append_child(convert_node_ref_to_ptr(child2));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(400.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::Mutation
                },
                0
            )
        );

        assert_eq!(child.layout_position().width, 300.);
        assert_eq!(child.layout_position().height, 150.);
        assert_eq!(child2.layout_position().width, 150.);
        assert_eq!(child2.layout_position().height, 300.);
    }
}

#[test]
pub fn aspect_ratio_with_min_width_constraint() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_height(DefLength::Auto);
        child.set_min_width(DefLength::Points(Len::from_f32(400.)));
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(200.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::Mutation
                },
                0
            )
        );

        assert_eq!(child.layout_position().width, 400.);
        assert_eq!(child.layout_position().height, 200.);
    }
}

#[test]
pub fn aspect_ratio_with_max_width_constraint() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_height(DefLength::Auto);
        child.set_max_width(DefLength::Points(Len::from_f32(80.)));
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(200.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::Mutation
                },
                0
            )
        );

        assert_eq!(child.layout_position().width, 80.);
        assert_eq!(child.layout_position().height, 40.);
    }
}

#[test]
pub fn aspect_ratio_with_max_width_violating_min_height_constraint() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_height(DefLength::Auto);
        child.set_max_width(DefLength::Points(Len::from_f32(80.)));
        child.set_min_height(DefLength::Points(Len::from_f32(100.)));
        child.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(200.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::Mutation
                },
                0
            )
        );

        assert_eq!(child.layout_position().width, 80.);
        assert_eq!(child.layout_position().height, 100.);
    }
}

#[test]
pub fn aspect_ratio_block_size_with_box_sizing() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_height(DefLength::Auto);
        child.set_width(DefLength::Points(Len::from_f32(50.)));
        child.set_padding_left(DefLength::Points(Len::from_f32(30.)));
        child.set_border_left(DefLength::Points(Len::from_f32(20.)));
        child.set_box_sizing(BoxSizing::BorderBox);
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Auto);
        child2.set_height(DefLength::Auto);
        child2.set_width(DefLength::Points(Len::from_f32(50.)));
        child2.set_padding_left(DefLength::Points(Len::from_f32(30.)));
        child2.set_border_left(DefLength::Points(Len::from_f32(20.)));
        child2.set_box_sizing(BoxSizing::PaddingBox);
        child2.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child2));

        let child3 = as_ref(Node::new_ptr());
        child3.set_width(DefLength::Auto);
        child3.set_height(DefLength::Auto);
        child3.set_width(DefLength::Points(Len::from_f32(50.)));
        child3.set_padding_left(DefLength::Points(Len::from_f32(30.)));
        child3.set_border_left(DefLength::Points(Len::from_f32(20.)));
        child3.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child3));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(200.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::Mutation
                },
                0
            )
        );

        assert_eq!(child.layout_position().width, 50.);
        assert_eq!(child.layout_position().height, 25.);
        assert_eq!(child2.layout_position().width, 80.);
        assert_eq!(child2.layout_position().height, 25.);
        assert_eq!(child3.layout_position().width, 100.);
        assert_eq!(child3.layout_position().height, 25.);
    }
}

#[test]
pub fn aspect_ratio_block_size_with_box_sizing_and_writing_mode() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Auto);
        container.set_writing_mode(WritingMode::VerticalLr);
        root.append_child(convert_node_ref_to_ptr(container));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Auto);
        child.set_height(DefLength::Auto);
        child.set_height(DefLength::Points(Len::from_f32(50.)));
        child.set_padding_top(DefLength::Points(Len::from_f32(30.)));
        child.set_border_top(DefLength::Points(Len::from_f32(20.)));
        child.set_box_sizing(BoxSizing::BorderBox);
        child.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Auto);
        child2.set_height(DefLength::Auto);
        child2.set_height(DefLength::Points(Len::from_f32(50.)));
        child2.set_padding_top(DefLength::Points(Len::from_f32(30.)));
        child2.set_border_top(DefLength::Points(Len::from_f32(20.)));
        child2.set_box_sizing(BoxSizing::PaddingBox);
        child2.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child2));

        let child3 = as_ref(Node::new_ptr());
        child3.set_width(DefLength::Auto);
        child3.set_height(DefLength::Auto);
        child3.set_height(DefLength::Points(Len::from_f32(50.)));
        child3.set_padding_top(DefLength::Points(Len::from_f32(30.)));
        child3.set_border_top(DefLength::Points(Len::from_f32(20.)));
        child3.set_aspect_ratio(Some(2. / 1.));
        container.append_child(convert_node_ref_to_ptr(child3));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(200.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    layout: true,
                    style: DumpStyleMode::Mutation
                },
                0
            )
        );

        assert_eq!(child.layout_position().width, 100.);
        assert_eq!(child.layout_position().height, 50.);
        assert_eq!(child2.layout_position().width, 100.);
        assert_eq!(child2.layout_position().height, 80.);
        assert_eq!(child3.layout_position().width, 100.);
        assert_eq!(child3.layout_position().height, 100.);
    }
}

#[test]
pub fn aspect_ratio_writing_mode_stretched() {
    assert_xml!(
        r#"
        <div>
            <div style="height: 400px; width: 300px;">
              <div style="background: red; aspect-ratio: 3 / 1; min-width: 600px" expect_width="600" expect_height="200"></div>
              <div style="background: blue; aspect-ratio: 3 / 1; max-width: 60px" expect_width="60" expect_height="20"></div>
            </div>
            <div style="height: 300px; width: 400px; writing-mode: vertical-lr">
              <div style="background: red; aspect-ratio: 1 / 3; min-height: 600px" expect_width="200" expect_height="600"></div>
              <div style="background: blue; aspect-ratio: 1 / 3; max-height: 60px" expect_width="20" expect_height="60"></div>
            </div>
            <div style="height: 300px; width: 400px; writing-mode: vertical-lr">
              <div style="background: green; aspect-ratio: 3 / 1; min-height: 500px" expect_width="1500" expect_height="500"></div>
              <div style="background: blue; aspect-ratio: 3 / 1; max-height: 90px" expect_width="270" expect_height="90"></div>
            </div>
        </div>
    "#
    )
}
