// Tests for node lifecycle behavior:
// - calc-handle resources registered by the FFI `*CalcHandle` setters are
//   released through the free-calc-handle callback when the node drops
// - a dropped parent clears the parent pointer of its surviving children, so
//   freeing the child later never observes a dangling parent

use crate::*;
use float_pigment_forest::ffi::{NodeSetFreeCalcHandle, NodeStyleSetWidthCalcHandle};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}

#[test]
pub fn drop_node_releases_registered_calc_handles() {
    let freed: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));
    let freed_sink = freed.clone();
    unsafe {
        let node = as_ref(Node::new_ptr());
        node.set_free_calc_handle(Some(Box::new(move |_node, handle| {
            freed_sink.borrow_mut().push(handle);
        })));
        node.add_calc_handle(1);
        node.add_calc_handle(2);
        drop(Box::from_raw(convert_node_ref_to_ptr(node)));
    }
    assert_eq!(*freed.borrow(), vec![1, 2]);
}

#[test]
pub fn drop_node_without_free_callback_is_noop() {
    unsafe {
        let node = as_ref(Node::new_ptr());
        node.add_calc_handle(7);
        drop(Box::from_raw(convert_node_ref_to_ptr(node)));
    }
}

#[test]
pub fn drop_parent_clears_surviving_children_parent() {
    unsafe {
        let parent = Node::new_ptr();
        let child = Node::new_ptr();
        as_ref(parent).append_child(child);
        assert!(as_ref(child).parent_ptr().is_some());
        drop(Box::from_raw(parent));
        assert!(as_ref(child).parent_ptr().is_none());
        drop(Box::from_raw(child));
    }
}

static FREED_HANDLES: Mutex<Vec<i32>> = Mutex::new(Vec::new());

extern "C" fn record_freed_handle(_node: *mut (), handle: i32) {
    FREED_HANDLES.lock().unwrap().push(handle);
}

#[test]
pub fn ffi_calc_handle_setter_registers_handle_for_release_on_drop() {
    unsafe {
        let node = as_ref(Node::new_ptr());
        let ptr = convert_node_ref_to_ptr(node) as *mut ();
        NodeSetFreeCalcHandle(ptr, record_freed_handle);
        NodeStyleSetWidthCalcHandle(ptr, 42);
        drop(Box::from_raw(ptr as *mut Node));
    }
    assert_eq!(*FREED_HANDLES.lock().unwrap(), vec![42]);
}

#[test]
pub fn reentrant_add_during_free_is_released_in_same_drop() {
    let freed: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));
    let freed_sink = freed.clone();
    unsafe {
        let node = as_ref(Node::new_ptr());
        let ptr = convert_node_ref_to_ptr(node);
        node.set_free_calc_handle(Some(Box::new(move |node, handle| {
            freed_sink.borrow_mut().push(handle);
            // Re-enter add_calc_handle from inside the free callback.
            if handle == 1 {
                (*node).add_calc_handle(2);
            }
        })));
        node.add_calc_handle(1);
        drop(Box::from_raw(ptr));
    }
    assert_eq!(*freed.borrow(), vec![1, 2]);
}

#[test]
pub fn for_each_child_node_allows_shared_queries_inside_closure() {
    unsafe {
        let parent = Node::new_ptr();
        let c1 = Node::new_ptr();
        let c2 = Node::new_ptr();
        as_ref(parent).append_child(c1);
        as_ref(parent).append_child(c2);
        let mut visited = 0;
        as_ref(parent).for_each_child_node(|child, idx| {
            // Shared queries during iteration must not conflict with the
            // ongoing traversal.
            assert_eq!(as_ref(parent).children_len(), 2);
            let got = as_ref(parent).get_child_at(idx).unwrap();
            assert!(std::ptr::eq(got, child));
            visited += 1;
        });
        assert_eq!(visited, 2);
        drop(Box::from_raw(parent));
        drop(Box::from_raw(c1));
        drop(Box::from_raw(c2));
    }
}
