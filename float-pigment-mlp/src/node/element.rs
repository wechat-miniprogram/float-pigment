use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use super::{attribute::Attribute, NodeType};

pub struct Element {
    tag: String,
    attributes: Attribute,
    children: RefCell<Vec<Rc<NodeType>>>,
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Element {{ tag: {:?}, attributes: {:?}, children: {:?} }}",
            self.tag,
            self.attributes,
            self.children.borrow()
        )
    }
}

impl Element {
    pub(crate) fn new(tag: String) -> Self {
        Self {
            attributes: Attribute::default(),
            tag,
            children: RefCell::new(vec![]),
        }
    }
    pub fn tag(&self) -> &str {
        self.tag.as_str()
    }
    pub fn children_mut(&self) -> RefMut<Vec<Rc<NodeType>>> {
        self.children.borrow_mut()
    }
    pub fn for_each_child(&self, f: Box<dyn Fn(&NodeType)>) {
        self.children
            .borrow()
            .iter()
            .for_each(|item| f(item.as_ref()))
    }
    pub fn attributes(&self) -> &Attribute {
        &self.attributes
    }
}
