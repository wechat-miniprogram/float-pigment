use crate::Len;

pub(crate) mod layout_impl;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LayoutPosition {
    pub left: Len,
    pub top: Len,
    pub width: Len,
    pub height: Len,
}
