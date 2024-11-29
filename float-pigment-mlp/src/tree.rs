use crate::{
    node::{element::Element, fragment::Fragment, text::Text, NodeType},
    utils::{multi_space_to_single, nl_filter},
};

#[cfg(feature = "htmlstream")]
use htmlstream::HTMLTagState;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Tree {
    root: Option<Rc<NodeType>>,
}

impl Tree {
    pub(crate) fn from(raw: &str) -> Self {
        let mut tree = Tree::default();
        tree.parse(raw);
        tree
    }
    pub fn root(&self) -> Option<&NodeType> {
        self.root.as_deref()
    }
    fn parse(&mut self, raw: &str) {
        let root = Rc::new(NodeType::Fragment(Fragment::new()));
        self.root = Some(Rc::clone(&root));
        let mut parent_stack: Vec<Rc<NodeType>> = vec![];
        parent_stack.push(root);
        {
            let mut element = None;
            for token in xmlparser::Tokenizer::from(raw).flatten() {
                match token {
                    xmlparser::Token::ElementStart { local, .. } => {
                        element = Some(Rc::new(NodeType::Element(Element::new(local.to_string()))));
                    }
                    xmlparser::Token::Attribute { local, value, .. } => {
                        if let Some(element) = element.as_ref() {
                            match element.as_ref() {
                                NodeType::Element(e) => {
                                    e.attributes().add(local.to_string(), value.to_string())
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                    xmlparser::Token::ElementEnd { end, .. } => match end {
                        xmlparser::ElementEnd::Close(_, _) => {
                            parent_stack.pop();
                        }
                        xmlparser::ElementEnd::Open | xmlparser::ElementEnd::Empty => {
                            if let Some(element) = element.as_ref() {
                                let parent = parent_stack.last().expect("parent_stack is empty!");
                                match parent.as_ref() {
                                    NodeType::Fragment(f) => f.children_mut().push(element.clone()),
                                    NodeType::Element(e) => e.children_mut().push(element.clone()),
                                    _ => unreachable!(),
                                }
                                if end == xmlparser::ElementEnd::Empty {
                                    parent_stack.pop();
                                } else {
                                    parent_stack.push(element.clone());
                                }
                            }
                        }
                    },
                    xmlparser::Token::Text { text } => {
                        let filtered_text = multi_space_to_single(&nl_filter(text.as_str()))
                            .trim()
                            .to_string();
                        if filtered_text.is_empty() {
                            continue;
                        }
                        let text = Rc::new(NodeType::Text(Text::new(filtered_text)));
                        let parent = parent_stack.last().expect("parent stack is empty!");
                        match parent.as_ref() {
                            NodeType::Fragment(f) => f.children_mut().push(text),
                            NodeType::Element(e) => e.children_mut().push(text),
                            _ => unreachable!(),
                        }
                    }
                    _ => {}
                }
            }
        }
        #[cfg(feature = "htmlstream")]
        {
            for (_, tag) in htmlstream::tag_iter(raw) {
                match tag.state {
                    HTMLTagState::Closing => {
                        parent_stack.pop();
                    }
                    HTMLTagState::Text => {
                        let filtered_text = multi_space_to_single(&nl_filter(&tag.html))
                            .trim()
                            .to_string();
                        if filtered_text.is_empty() {
                            continue;
                        }
                        let text = Rc::new(NodeType::Text(Text::new(filtered_text)));
                        let parent = parent_stack.last().expect("parent stack is empty!");
                        match parent.as_ref() {
                            NodeType::Fragment(f) => f.children_mut().push(text),
                            NodeType::Element(e) => e.children_mut().push(text),
                            _ => unreachable!(),
                        }
                    }
                    HTMLTagState::Opening | HTMLTagState::SelfClosing => {
                        // tag
                        let element = Element::new(tag.name);
                        // attrs
                        for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                            // other attrs
                            element.attributes().add(attr.name, attr.value);
                        }
                        let element = Rc::new(NodeType::Element(element));

                        let parent = parent_stack.last().expect("parent_stack is empty!");
                        match parent.as_ref() {
                            NodeType::Fragment(f) => f.children_mut().push(element.clone()),
                            NodeType::Element(e) => e.children_mut().push(element.clone()),
                            _ => unreachable!(),
                        }
                        if let HTMLTagState::Opening = tag.state {
                            parent_stack.push(element.clone());
                        }
                    }
                }
            }
        }
    }
}
