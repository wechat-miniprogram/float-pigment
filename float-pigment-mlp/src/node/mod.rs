use self::{element::Element, fragment::Fragment, text::Text};

pub mod attribute;
pub mod element;
pub mod fragment;
pub mod text;

pub enum NodeType {
    Fragment(Fragment),
    Element(Element),
    Text(Text),
}

impl std::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(e) => {
                write!(f, "{:?}", e)
            }
            Self::Fragment(e) => {
                write!(f, "{:?}", e)
            }
            Self::Text(e) => {
                write!(f, "{:?}", e)
            }
        }
    }
}
