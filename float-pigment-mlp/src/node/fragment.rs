use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

use super::NodeType;
pub struct Fragment(RefCell<Vec<Rc<NodeType>>>);

impl Debug for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fragment {{ children: {:?} }}", self.0.borrow())
    }
}

impl Fragment {
    pub(crate) fn new() -> Self {
        Self(RefCell::new(vec![]))
    }
    pub fn for_each_child(&self, f: Box<dyn Fn(&NodeType)>) {
        self.0.borrow().iter().for_each(|item| f(item.as_ref()))
    }
    pub fn children_mut(&self) -> RefMut<Vec<Rc<NodeType>>> {
        self.0.borrow_mut()
    }
}
