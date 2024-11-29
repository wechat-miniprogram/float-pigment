use std::fmt::Debug;

#[derive(Debug)]
pub struct Text(String);

impl Text {
    pub fn text(&self) -> &str {
        &self.0
    }
    pub(crate) fn new(text: String) -> Self {
        Self(text)
    }
}
