// Tests for margin collapsing aligned with CSS 2.1 §8.3.1
// Focus: layout-entry node (no parent) plays the role of the CSS root element
// (`<html>`) — its margins do not collapse with its children.

use crate::*;

use float_pigment_forest::{convert_node_ref_to_ptr, ChildOperation, Node, StyleSetter};
use float_pigment_layout::{DefLength, OptionNum, OptionSize, Size};

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

// CSS 2.1 §8.3.1: "Margins of the root element's box do not collapse."
// The layout-entry node (no parent) plays the root-element role. Its margin
// stays independent — the child's margin does not merge into it.
//
// entry margin-top:30, child margin-top:20:
// - entry positioned at (0, 30) — root margin pushes entry down
// - child.top = 20 relative to entry's content origin (child margin NOT absorbed)
#[test]
fn entry_node_margin_does_not_collapse_with_child() {
    unsafe {
        let entry = as_ref(Node::new_ptr());
        entry.set_margin_top(DefLength::Points(Len::from_f32(30.)));
        let child = as_ref(Node::new_ptr());
        child.set_height(DefLength::Points(Len::from_f32(50.)));
        child.set_margin_top(DefLength::Points(Len::from_f32(20.)));
        entry.append_child(convert_node_ref_to_ptr(child));

        entry.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        assert_eq!(entry.layout_position().top, 30.);
        assert_eq!(child.layout_position().top, 20.);
    }
}

// Counter-case: non-entry parent (has a parent itself) DOES collapse with first child.
// container is entry (no parent, no margin — nothing to compare).
// parent has margin-top:30, child has margin-top:20 — they collapse to max(30,20)=30.
// child.top relative to `parent`'s content origin should be 0 (child margin absorbed).
#[test]
fn non_entry_parent_collapses_with_first_child() {
    unsafe {
        let container = as_ref(Node::new_ptr());
        let parent = as_ref(Node::new_ptr());
        parent.set_margin_top(DefLength::Points(Len::from_f32(30.)));
        let child = as_ref(Node::new_ptr());
        child.set_height(DefLength::Points(Len::from_f32(50.)));
        child.set_margin_top(DefLength::Points(Len::from_f32(20.)));
        parent.append_child(convert_node_ref_to_ptr(child));
        container.append_child(convert_node_ref_to_ptr(parent));

        container.layout(
            OptionSize::new(OptionNum::some(Len::from_f32(375.)), OptionNum::none()),
            Size::new(Len::from_f32(0.), Len::from_f32(0.)),
        );

        // parent is not the layout entry (container is) — parent-child collapse
        // proceeds normally: max(30, 20) = 30 absorbed into parent's outside.
        // child.top relative to parent's content origin = 0.
        assert_eq!(child.layout_position().top, 0.);
    }
}

// Edge case: [block, BFC, block] sequence. The third child (non-BFC) comes
// after a BFC sibling. Its margin should NOT propagate up to the parent's
// collapsed_margin_start — the BFC broke the collapse chain, so the third
// child's top margin acts as its own offset.
//
// Layout: container(root) > parent(no margin) > [a(h:10,mb:20), b(flex,mt:30,h:40), c(mt:50,h:60)]
// Standard:
//   - parent.top = 0 (no own margin, container does not collapse with it)
//   - a.top = 0 (first child, no mt)
//   - b.top = 10(a height) + 20(a mb, not collapsed with BFC b) + 30(b mt) = 60
//   - c.top = 60(b top) + 40(b height) + 50(c mt, not collapsed with BFC b) = 150
// If the implementation wrongly treats c as "first child" after BFC reset,
// c.mt:50 would propagate to parent_collapsed_margin_start, making parent.top=50.

// display: flow-root — CSS Display Module Level 3 §2.7:
// "block container box, lays out contents using flow layout, always
// establishes a new BFC".
// Behaviorally identical to display:block except it builds a BFC.
//
// As parent: flow-root's margin does NOT collapse with its child's margin.

// As child: flow-root's margin does NOT collapse with its parent's margin.

// flow-root as sibling: its margin does not collapse with adjacent block siblings.
