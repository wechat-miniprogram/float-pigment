// Tests for the is_measurable capability flag:
// - a Text node is implicitly measurable (backward compatible with the
//   legacy NodeType::Text cache gating)
// - an explicitly measurable node gets measure caching, and marking it dirty
//   clears the cache so the next layout re-measures

use crate::*;
use std::cell::Cell;
use std::rc::Rc;

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

#[test]
pub fn as_text_node_is_measurable_and_flag_can_be_turned_off() {
    unsafe {
        let node = as_ref(Node::new_ptr());
        assert!(!node.is_measurable());
        node.set_node_type(NodeType::Text);
        assert!(node.is_measurable());
        node.set_measurable(false);
        assert!(!node.is_measurable());
        drop(Box::from_raw(convert_node_ref_to_ptr(node)));
    }
}

#[test]
pub fn new_typed_text_node_is_measurable() {
    let node = Node::new_typed(NodeType::Text);
    assert!(node.is_measurable());
}

#[test]
pub fn measurable_node_caches_measure_until_dirty() {
    let count = Rc::new(Cell::new(0u32));
    let counter = count.clone();
    unsafe {
        let root = as_ref(Node::new_ptr());
        let child = as_ref(Node::new_ptr());
        child.set_measure_func(Some(Box::new(move |_, _, _, _, _, _, _, _, _| {
            counter.set(counter.get() + 1);
            Size::new(Len::from_f32(10.), Len::from_f32(10.))
        })));
        child.set_measurable(true);
        root.append_child(convert_node_ref_to_ptr(child));
        let layout = || {
            root.layout(
                OptionSize::new(
                    OptionNum::some(Len::from_f32(100.)),
                    OptionNum::some(Len::from_f32(100.)),
                ),
                Size::new(Len::from_f32(100.), Len::from_f32(100.)),
            );
        };
        layout();
        let after_first = count.get();
        assert_eq!(after_first, 1);
        layout();
        assert_eq!(count.get(), after_first, "cache hit: no re-measure");
        child.mark_self_dirty();
        layout();
        assert_eq!(
            count.get(),
            after_first + 1,
            "dirty cleared the cache: re-measured"
        );
        drop(Box::from_raw(convert_node_ref_to_ptr(root)));
        drop(Box::from_raw(convert_node_ref_to_ptr(child)));
    }
}
