use std::cell::RefCell;

use rustc_hash::FxHashMap;
#[derive(Default)]
pub struct Attribute {
    inner: RefCell<FxHashMap<String, String>>,
}

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.borrow())
    }
}

impl Attribute {
    pub fn keys(&self) -> Vec<String> {
        self.inner
            .borrow()
            .keys()
            .map(|x| x.into())
            .collect::<Vec<String>>()
    }

    pub fn get(&self, k: &str) -> Option<String> {
        self.inner.borrow().get_key_value(k).map(|x| x.1.into())
    }

    pub fn add(&self, key: String, value: String) {
        self.inner.borrow_mut().insert(key, value);
    }
}
