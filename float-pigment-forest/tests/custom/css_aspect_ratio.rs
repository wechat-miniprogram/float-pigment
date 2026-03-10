// Tests for `aspect-ratio` property in CSS
// Based on CSS Box Sizing Module Level 4 specification:
// - `aspect-ratio` sets a preferred aspect ratio for the box
// - The ratio is width / height
// - When one dimension is auto, it is computed from the other using the aspect ratio
// - Constraints like min/max-width/height are applied after aspect-ratio computation

use crate::*;
use float_pigment_css::typing::*;

use float_pigment_layout::{DefLength, OptionNum, OptionSize};
unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

// Case: aspect-ratio in flex row container
// Spec points:
// - In flex context, aspect-ratio affects cross-size calculation
// - Explicit width/height takes precedence over aspect-ratio
// In this test:
// - Container: flex row, 700x300px
// - Child 1: both width and height explicit (100x100), aspect-ratio has no effect
// - Child 2: width=auto, height=100px, ratio=0.75/1, width = 100 * 0.75 = 75px
// - Child 3: width=100px, height=auto, stretches to container height (300px) in flex
// - Child 4: width=100px, height=auto, align-self=flex-start, height = 100 / 0.75 ≈ 133px
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

// Case: aspect-ratio in flex column container
// Spec points:
// - In flex column context, main axis is vertical
// - Cross-axis (width) auto-sizing respects aspect-ratio
// In this test:
// - Container: flex column, 300x600px
// - Child 1: both explicit (100x100), aspect-ratio has no effect
// - Child 2: width=auto, height=100px, stretches to container width (300px) in flex
// - Child 3: width=auto, height=100px, align-self=flex-start, width = 100 * 0.75 = 75px
// - Child 4: width=100px, height=auto, height = 100 / 0.75 ≈ 133px
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

// Case: aspect-ratio with flex-basis: 0%
// Spec points:
// - flex-basis: 0% gives zero main size initially
// - aspect-ratio computes the cross size from the final width
// In this test:
// - Child: width=50px, flex-basis=0%, aspect-ratio=1/1
// - Expected: 50x50 (width determines height via aspect-ratio)
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

// Case: aspect-ratio in flex container with wrap
// Spec points:
// - With flex-wrap and flex-grow, items can grow
// - aspect-ratio affects final sizing after flex layout
// In this test:
// - Container: flex column wrap, height=100px
// - Child: height=50px, flex-grow=1, aspect-ratio=1/1
// - With align-items=stretch, child grows to fill container
// - Expected: 100x100 (stretched in both dimensions)
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

// Case: aspect-ratio in flex container without wrap (stretch behavior)
// Spec points:
// - In flex column, stretch affects width
// - With aspect-ratio=1/1, height capped by container height
// In this test:
// - Container: flex column nowrap, height=50px, width stretches to viewport (375px)
// - Child: aspect-ratio=1/1, no explicit size
// - Expected: width=375 (stretched), height=50 (capped by container)
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

// Case: aspect-ratio in flex container nowrap with percentage child
// Spec points:
// - aspect-ratio determines height from width
// - Percentage height children can reference the computed height
// In this test:
// - Container: flex column nowrap, width=100px
// - Child: aspect-ratio=1/1, stretches to 100px width, height = 100px
// - Grandchild: height=100%, gets parent's computed height
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

// Case: aspect-ratio in block context with fixed width
// Spec points:
// - In block layout, explicit width + aspect-ratio determines height
// - ratio > 1 means wider than tall, height = width / ratio
// - ratio < 1 means taller than wide, height = width / ratio
// In this test:
// - Child 1: width=100px, ratio=2/1, height = 100 / 2 = 50px
// - Child 2: width=100px, ratio=0.5/1, height = 100 / 0.5 = 200px
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

// Case: aspect-ratio in block context with fixed height
// Spec points:
// - With explicit height + aspect-ratio, width = height * ratio
// In this test:
// - Child 1: height=100px, ratio=2/1, width = 100 * 2 = 200px
// - Child 2: height=200px, ratio=0.5/1, width = 200 * 0.5 = 100px
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
// Case: aspect-ratio uses parent cross-size when auto in block/vertical mode
// Spec points:
// - When width=auto, block child stretches to parent width
// - aspect-ratio then computes height from stretched width
// - In vertical writing mode, the roles are swapped
// In this test:
// - Container 1: width=300px, child stretches to 300px, height = 300/2 = 150px
// - Container 2: vertical-lr, height=300px, child stretches to 300px height, width = 300*0.5 = 150px
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

// Case: aspect-ratio with min-width constraint
// Spec points:
// - min-width overrides the natural width (even if from aspect-ratio)
// - height is then computed from the clamped width
// In this test:
// - Container: width=300px (available), viewport=200px
// - Child: min-width=400px, aspect-ratio=2/1
// - Final width = max(300, 400) = 400px, height = 400/2 = 200px
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

// Case: aspect-ratio with max-width constraint
// Spec points:
// - max-width clamps the natural width
// - height is computed from the clamped width
// In this test:
// - Container: width=300px
// - Child: max-width=80px, aspect-ratio=2/1
// - Final width = min(300, 80) = 80px, height = 80/2 = 40px
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

// Case: aspect-ratio with max-width violating min-height
// Spec points:
// - When max-width conflicts with min-height via aspect-ratio
// - min-height takes precedence, but max-width still caps width
// In this test:
// - Child: max-width=80px, min-height=100px, aspect-ratio=1/1
// - Width clamped to 80px by max-width
// - Height would be 80px from ratio, but min-height=100px forces 100px
// - Final: 80x100 (aspect ratio violated due to constraints)
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

// Case: aspect-ratio with box-sizing
// Spec points:
// - aspect-ratio applies to the sizing box determined by box-sizing
// - border-box: ratio applies to border box (includes padding/border)
// - padding-box: ratio applies to padding box (includes padding)
// - content-box: ratio applies to content box only
// In this test:
// - All children: width=50px, padding-left=30px, border-left=20px, ratio=2/1
// - Child 1 (border-box): 50px total, height = 50/2 = 25px
// - Child 2 (padding-box): 50px padding-box + 20px border = 80px, height = 50/2 = 25px
// - Child 3 (content-box): 50px + 30px + 20px = 100px, height = 50/2 = 25px
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

// Case: aspect-ratio with box-sizing in vertical writing mode
// Spec points:
// - In vertical writing mode, width and height roles are swapped
// - box-sizing still affects which box the ratio applies to
// In this test:
// - Container: vertical-lr writing mode
// - All children: height=50px, padding-top=30px, border-top=20px, ratio=2/1
// - Child 1 (border-box): 50px total height, width = 50*2 = 100px
// - Child 2 (padding-box): 50px padding-box + 20px border = 80px height, width = 50*2 = 100px
// - Child 3 (content-box): 50px + 30px + 20px = 100px height, width = 50*2 = 100px
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

// Case: aspect-ratio with min/max constraints in stretched context
// Spec points:
// - min-width/max-width constraints override stretched size
// - aspect-ratio computes height from the constrained width
// In this test:
// - Container 1: width=300px
//   - Child with min-width=600px, ratio=3/1: width=600px, height=200px
//   - Child with max-width=60px, ratio=3/1: width=60px, height=20px
// - Container 2: vertical-lr, height=300px
//   - Child with min-height=600px, ratio=1/3: height=600px, width=200px
//   - Child with max-height=60px, ratio=1/3: height=60px, width=20px
// - Container 3: vertical-lr, width=400px
//   - Child with min-height=500px, ratio=3/1: height=500px, width=1500px
//   - Child with max-height=90px, ratio=3/1: height=90px, width=270px
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
