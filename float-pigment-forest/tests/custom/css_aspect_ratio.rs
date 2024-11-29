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
