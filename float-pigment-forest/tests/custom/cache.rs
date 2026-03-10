// Tests for layout cache invalidation
// These tests ensure the layout engine correctly invalidates and recalculates
// positions when style properties change. Key scenarios:
// - order property changes in flex container
// - flex-direction changes
// - display property changes
// - Style mutations requiring position recalculation

use crate::*;
use float_pigment_css::typing::*;

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

// Case: Position cache invalidation when order changes
// Spec points:
// - Changing order property reorders flex items visually
// - Positions must be recalculated after order change
// In this test:
// - Initial: items 1,2,3,4 with widths 1,2,3,4 and order 1,2,3,4
// - After: order becomes 4,3,2,1 (reversed)
// - Items should be repositioned in reverse order
#[test]
pub fn position_cache_if_order_changed() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Flex);
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Points(Len::from_f32(100.)));
        root.append_child(convert_node_ref_to_ptr(container));
        for i in 0..4 {
            let item = as_ref(Node::new_ptr());
            item.set_width(DefLength::Points(Len::from_f32(1. * (i + 1) as f32)));
            item.set_height(DefLength::Points(Len::from_f32(1. * (i + 1) as f32)));
            container.append_child(convert_node_ref_to_ptr(item));
        }
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            0.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            1.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            3.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            6.
        );

        for i in 0..4 {
            if let Some(item) = container.get_child_at(i) {
                item.set_order((4 - i) as i32);
            }
        }
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            9.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            7.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            4.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            0.
        );
    }
}

// Case: Position cache with equal-width items and order change
// Spec points:
// - Order change with equal-width items still requires recalculation
// In this test:
// - Four items of width=1px each
// - Initial positions: 0, 1, 2, 3
// - After order reversal: 3, 2, 1, 0
#[test]
pub fn position_cache_if_order_changed_2() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Flex);
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Points(Len::from_f32(100.)));
        root.append_child(convert_node_ref_to_ptr(container));
        for _ in 0..4 {
            let item = as_ref(Node::new_ptr());
            item.set_width(DefLength::Points(Len::from_f32(1.)));
            item.set_height(DefLength::Points(Len::from_f32(1.)));
            container.append_child(convert_node_ref_to_ptr(item));
        }
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            0.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            1.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            2.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            3.
        );

        for i in 0..4 {
            if let Some(item) = container.get_child_at(i) {
                item.set_order((4 - i) as i32);
            }
        }
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            3.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            2.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            1.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            0.
        );
    }
}

// Case: Position cache invalidation when flex-direction changes
// Spec points:
// - Changing flex-direction from row to row-reverse
// - Items repositioned from left-to-right to right-to-left
// In this test:
// - Initial: row direction, items at 0, 1, 3, 6
// - After: row-reverse, items positioned from right edge
#[test]
pub fn position_cache_if_flex_direction_changed() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Flex);
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Points(Len::from_f32(100.)));
        root.append_child(convert_node_ref_to_ptr(container));
        for i in 0..4 {
            let item = as_ref(Node::new_ptr());
            item.set_width(DefLength::Points(Len::from_f32(1. * (i + 1) as f32)));
            item.set_height(DefLength::Points(Len::from_f32(1. * (i + 1) as f32)));
            container.append_child(convert_node_ref_to_ptr(item));
        }
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            0.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            1.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            3.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            6.
        );
        container.set_flex_direction(FlexDirection::RowReverse);
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            299.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            297.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            294.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            290.
        );
    }
}

// Case: Position cache with equal-width items and flex-direction change
// Spec points:
// - Direction change with equal-width items
// In this test:
// - Four items of width=1px
// - Initial: 0, 1, 2, 3
// - After row-reverse: 299, 298, 297, 296
#[test]
pub fn position_cache_if_flex_direction_changed_2() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Flex);
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Points(Len::from_f32(100.)));
        root.append_child(convert_node_ref_to_ptr(container));
        for _ in 0..4 {
            let item = as_ref(Node::new_ptr());
            item.set_width(DefLength::Points(Len::from_f32(1.)));
            item.set_height(DefLength::Points(Len::from_f32(1.)));
            container.append_child(convert_node_ref_to_ptr(item));
        }
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            0.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            1.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            2.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            3.
        );
        container.set_flex_direction(FlexDirection::RowReverse);
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(
            container.get_child_at(0).unwrap().layout_position().left,
            299.
        );
        assert_eq!(
            container.get_child_at(1).unwrap().layout_position().left,
            298.
        );
        assert_eq!(
            container.get_child_at(2).unwrap().layout_position().left,
            297.
        );
        assert_eq!(
            container.get_child_at(3).unwrap().layout_position().left,
            296.
        );
    }
}

// Case: Position cache with absolute child and mark_dirty_propagate
// Spec points:
// - Absolute positioned children need recalculation on dirty
// - Sibling positions should remain stable
// In this test:
// - Nested absolute container with two children
// - After mark_dirty_propagate, positions should remain correct
#[test]
pub fn layout() {
    unsafe {
        let root = as_ref(Node::new_ptr());

        let container = as_ref(Node::new_ptr());
        container.set_height(DefLength::Points(Len::from_f32(700.)));

        let parent = as_ref(Node::new_ptr());
        parent.set_height(DefLength::Points(Len::from_f32(300.)));
        parent.set_width(DefLength::Points(Len::from_f32(300.)));
        parent.set_position(Position::Absolute);
        parent.set_top(DefLength::Points(Len::from_f32(10.)));
        parent.set_left(DefLength::Points(Len::from_f32(10.)));

        let child = as_ref(Node::new_ptr());
        child.set_width(DefLength::Points(Len::from_f32(50.)));
        child.set_position(Position::Absolute);
        child.set_top(DefLength::Points(Len::from_f32(10.)));
        child.set_left(DefLength::Points(Len::from_f32(10.)));

        let item = as_ref(Node::new_ptr());
        item.set_height(DefLength::Points(Len::from_f32(50.)));

        let item_b = as_ref(Node::new_ptr());
        item_b.set_height(DefLength::Points(Len::from_f32(50.)));

        root.append_child(convert_node_ref_to_ptr(container));
        container.append_child(convert_node_ref_to_ptr(parent));
        parent.append_child(convert_node_ref_to_ptr(child));
        child.append_child(convert_node_ref_to_ptr(item));
        child.append_child(convert_node_ref_to_ptr(item_b));

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(item_b.layout_position().left, 0.);
        assert_eq!(item_b.layout_position().top, 50.);

        root.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        item.mark_dirty_propagate();
        assert_eq!(item_b.layout_position().left, 0.);
        assert_eq!(item_b.layout_position().top, 50.);
    }
}

// Case: Clear position cache when parent display changes
// Spec points:
// - display: none removes element from layout (zero size)
// - Restoring display: flex restores original layout
// In this test:
// - Initial: flex container with absolute child at left=10
// - After display=none: child position becomes 0x0x0
// - After display=flex restored: child position restored
#[test]
pub fn clear_position_cache_if_parent_display_changed() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_display(Display::Flex);
        container.set_width(DefLength::Points(Len::from_f32(300.)));
        container.set_height(DefLength::Points(Len::from_f32(100.)));
        root.append_child(convert_node_ref_to_ptr(container));
        let item = as_ref(Node::new_ptr());
        item.set_width(DefLength::Points(Len::from_f32(10.)));
        item.set_height(DefLength::Points(Len::from_f32(10.)));
        item.set_position(Position::Absolute);
        item.set_left(DefLength::Points(Len::from_f32(10.)));
        container.append_child(convert_node_ref_to_ptr(item));

        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(item.layout_position().left, 10.);
        assert_eq!(item.layout_position().width, 10.);
        assert_eq!(item.layout_position().height, 10.);
        container.set_display(Display::None);
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(item.layout_position().left, 0.);
        assert_eq!(item.layout_position().width, 0.);
        assert_eq!(item.layout_position().height, 0.);

        container.set_display(Display::Flex);
        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        assert_eq!(item.layout_position().left, 10.);
        assert_eq!(item.layout_position().width, 10.);
        assert_eq!(item.layout_position().height, 10.);
    }
}

// Case: Complex nested layout with inline elements
// Spec points:
// - Inline elements within flex items require careful cache management
// - Changing child height should trigger proper relayout
// In this test:
// - Nested structure with flex, block, and inline elements
// - After changing first child height, inline position should update correctly
#[test]
pub fn test() {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let container = as_ref(Node::new_ptr());
        container.set_width(DefLength::Percent(1.0));
        container.set_height(DefLength::Percent(1.0));
        root.append_child(convert_node_ref_to_ptr(container));

        let flex_box = as_ref(Node::new_ptr());
        flex_box.set_display(Display::Flex);
        flex_box.set_flex_direction(FlexDirection::Column);
        container.append_child(convert_node_ref_to_ptr(flex_box));

        let block_box = as_ref(Node::new_ptr());
        flex_box.append_child(convert_node_ref_to_ptr(block_box));

        let first = as_ref(Node::new_ptr());
        first.set_height(DefLength::Points(Len::from_f32(10.)));
        block_box.append_child(convert_node_ref_to_ptr(first));

        let second = as_ref(Node::new_ptr());
        block_box.append_child(convert_node_ref_to_ptr(second));

        let item = as_ref(Node::new_ptr());
        item.set_height(DefLength::Points(Len::from_f32(10.)));
        second.append_child(convert_node_ref_to_ptr(item));

        let item = as_ref(Node::new_ptr());
        item.set_height(DefLength::Points(Len::from_f32(10.)));
        second.append_child(convert_node_ref_to_ptr(item));

        let inline = as_ref(Node::new_ptr());
        inline.set_display(Display::Inline);
        second.append_child(convert_node_ref_to_ptr(inline));

        let inline_child = as_ref(Node::new_ptr());
        inline_child.set_height(DefLength::Points(Len::from_f32(24.)));
        inline_child.set_width(DefLength::Points(Len::from_f32(34.)));
        inline.append_child(convert_node_ref_to_ptr(inline_child));

        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    style: float_pigment_forest::DumpStyleMode::Mutation,
                    layout: true,
                },
                2
            )
        );

        assert_eq!(inline.layout_position().top, 20.);
        first.set_height(DefLength::Points(Len::from_f32(20.)));

        root.layout(
            OptionSize::new(
                OptionNum::some(Len::from_f32(375.)),
                OptionNum::some(Len::from_f32(750.)),
            ),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );
        println!(
            "{}",
            root.dump_to_html(
                DumpOptions {
                    recursive: true,
                    style: float_pigment_forest::DumpStyleMode::Mutation,
                    layout: true,
                },
                2
            )
        );
        assert_eq!(inline.layout_position().top, 20.);
    }
}
