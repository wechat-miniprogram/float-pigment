use crate::*;
use float_pigment_css::typing::*;

use float_pigment_layout::{DefLength, OptionNum, OptionSize};
unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}
#[test]
pub fn aspect_ratio_in_flex_row() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(700.)));
        container.set_height(DefLength::Points(Len::from_f32(300.)));
        container.set_display(Display::Flex);
        container.set_flex_direction(FlexDirection::Row);
        root.append_child(convert_node_ref_to_ptr(container));

        let child_a = as_ref(Node::new_ptr());
        child_a.set_width(DefLength::Points(Len::from_f32(100.)));
        child_a.set_height(DefLength::Points(Len::from_f32(100.)));
        child_a.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_a));

        let child_b = as_ref(Node::new_ptr());
        child_b.set_width(DefLength::Auto);
        child_b.set_height(DefLength::Points(Len::from_f32(100.)));
        child_b.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_b));

        let child_c = as_ref(Node::new_ptr());
        child_c.set_width(DefLength::Points(Len::from_f32(100.)));
        child_c.set_height(DefLength::Auto);
        child_c.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_c));

        let child_d = as_ref(Node::new_ptr());
        child_d.set_width(DefLength::Points(Len::from_f32(100.)));
        child_d.set_height(DefLength::Auto);
        child_d.set_align_self(AlignSelf::FlexStart);
        child_d.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_d));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(800.)), OptionNum::none()),
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

        assert_eq!(child_b.layout_position().width, 75.);
        assert_eq!(child_b.layout_position().height, 100.);

        assert_eq!(child_c.layout_position().width, 100.);
        assert_eq!(child_c.layout_position().height, 300.);

        assert_eq!(child_d.layout_position().width, 100.);
        assert_eq!(child_d.layout_position().height.round(), 133.);
    }
}

#[test]
pub fn aspect_ratio_in_flex_column() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Points(Len::from_f32(600.)));
        container.set_display(Display::Flex);
        container.set_flex_direction(FlexDirection::Column);
        root.append_child(convert_node_ref_to_ptr(container));

        let child_a = as_ref(Node::new_ptr());
        child_a.set_width(DefLength::Points(Len::from_f32(100.)));
        child_a.set_height(DefLength::Points(Len::from_f32(100.)));
        child_a.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_a));

        let child_b = as_ref(Node::new_ptr());
        child_b.set_width(DefLength::Auto);
        child_b.set_height(DefLength::Points(Len::from_f32(100.)));
        child_b.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_b));

        let child_c = as_ref(Node::new_ptr());
        child_c.set_width(DefLength::Auto);
        child_c.set_height(DefLength::Points(Len::from_f32(100.)));
        child_c.set_aspect_ratio(Some(0.75 / 1.));
        child_c.set_align_self(AlignSelf::FlexStart);
        container.append_child(convert_node_ref_to_ptr(child_c));

        let child_d = as_ref(Node::new_ptr());
        child_d.set_width(DefLength::Points(Len::from_f32(100.)));
        child_d.set_height(DefLength::Auto);
        child_d.set_aspect_ratio(Some(0.75 / 1.));
        container.append_child(convert_node_ref_to_ptr(child_d));

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

        assert_eq!(child_b.layout_position().width, 300.);
        assert_eq!(child_b.layout_position().height, 100.);

        assert_eq!(child_c.layout_position().width, 75.);
        assert_eq!(child_c.layout_position().height, 100.);

        assert_eq!(child_d.layout_position().width, 100.);
        assert_eq!(child_d.layout_position().height.round(), 133.);
    }
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

// #[test]
// pub fn aspect_ratio_1() {
//     unsafe {
//         let root = as_ref(Node::new_ptr(Some(0), None));

//         let container = as_ref(Node::new_ptr(Some(1), None));
//         container.set_height(DefLength::Points(50.));
//         container.set_display(Display::Flex);
//         container.set_flex_direction(FlexDirection::Column);
//         container.set_flex_wrap(FlexWrap::NoWrap);
//         container.set_align_items(AlignItems::Stretch);
//         root.append_child(container.self_ptr().unwrap());

//         let child_a = as_ref(Node::new_ptr(Some(2), None));
//         child_a.set_aspect_ratio(AspectRatio::Ratio(Number::F32(1.), Number::F32(1.)));
//         container.append_child(child_a.self_ptr().unwrap());

//         root.layout(OptionNum::some(375.), OptionNum::none());
//         println!(
//             "{}",
//             root.dump_to_html(
//                 DumpOptions {
//                     recursive: true,
//                     layout: true,
//                     style: DumpStyleMode::None
//                 },
//                 0
//             )
//         );

//         assert_eq!(child_a.layout_position().width, 375.);
//         assert_eq!(child_a.layout_position().height, 50.);
//     }
// }

#[test]
pub fn aspect_ratio_2() {
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
        child.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(400.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(child.layout_position().width, 100.);
        assert_eq!(child.layout_position().height, 100.);
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
        child.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(400.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(child.layout_position().width, 100.);
        assert_eq!(child.layout_position().height, 100.);
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
        child.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

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
        assert_eq!(child.layout_position().height, 300.);
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

        assert_eq!(child.layout_position().width, 400.);
        assert_eq!(child.layout_position().height, 400.);
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
        assert_eq!(child.layout_position().height, 80.);
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
        child.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Auto);
        child2.set_height(DefLength::Auto);
        child2.set_width(DefLength::Points(Len::from_f32(50.)));
        child2.set_padding_left(DefLength::Points(Len::from_f32(30.)));
        child2.set_border_left(DefLength::Points(Len::from_f32(20.)));
        child2.set_box_sizing(BoxSizing::PaddingBox);
        child2.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child2));

        let child3 = as_ref(Node::new_ptr());
        child3.set_width(DefLength::Auto);
        child3.set_height(DefLength::Auto);
        child3.set_width(DefLength::Points(Len::from_f32(50.)));
        child3.set_padding_left(DefLength::Points(Len::from_f32(30.)));
        child3.set_border_left(DefLength::Points(Len::from_f32(20.)));
        child3.set_aspect_ratio(Some(1. / 1.));
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
        assert_eq!(child.layout_position().height, 50.);
        assert_eq!(child2.layout_position().width, 80.);
        assert_eq!(child2.layout_position().height, 50.);
        assert_eq!(child3.layout_position().width, 100.);
        assert_eq!(child3.layout_position().height, 50.);
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
        child.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child));

        let child2 = as_ref(Node::new_ptr());
        child2.set_width(DefLength::Auto);
        child2.set_height(DefLength::Auto);
        child2.set_height(DefLength::Points(Len::from_f32(50.)));
        child2.set_padding_top(DefLength::Points(Len::from_f32(30.)));
        child2.set_border_top(DefLength::Points(Len::from_f32(20.)));
        child2.set_box_sizing(BoxSizing::PaddingBox);
        child2.set_aspect_ratio(Some(1. / 1.));
        container.append_child(convert_node_ref_to_ptr(child2));

        let child3 = as_ref(Node::new_ptr());
        child3.set_width(DefLength::Auto);
        child3.set_height(DefLength::Auto);
        child3.set_height(DefLength::Points(Len::from_f32(50.)));
        child3.set_padding_top(DefLength::Points(Len::from_f32(30.)));
        child3.set_border_top(DefLength::Points(Len::from_f32(20.)));
        child3.set_aspect_ratio(Some(1. / 1.));
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
        assert_eq!(child.layout_position().height, 50.);
        assert_eq!(child2.layout_position().width, 50.);
        assert_eq!(child2.layout_position().height, 80.);
        assert_eq!(child3.layout_position().width, 50.);
        assert_eq!(child3.layout_position().height, 100.);
    }
}
