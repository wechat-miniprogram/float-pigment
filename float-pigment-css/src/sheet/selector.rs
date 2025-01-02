use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::{cell::Cell, fmt, num::NonZeroUsize};

use cssparser::{Parser, ParserInput};
#[cfg(debug_assertions)]
use float_pigment_css_macro::{compatibility_enum_check, compatibility_struct_check};

use crate::parser::{parse_selector, ParseState};
use crate::query::{StyleNode, StyleNodeClass};

#[cfg_attr(debug_assertions, compatibility_enum_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) enum SelectorRelationType {
    None,
    Ancestor(SelectorFragment),
    DirectParent(SelectorFragment),
    NextSibling(SelectorFragment),
    SubsequentSibling(SelectorFragment),
}

#[cfg_attr(debug_assertions, compatibility_enum_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[allow(clippy::box_collection)] // TODO optimize here
pub(crate) enum PseudoClasses {
    Host,
    FirstChild,
    LastChild,
    Empty,
    Not(Vec<SelectorFragment>),
    OnlyChild,
    NthChild(i32, i32, Option<Box<Vec<SelectorFragment>>>),
    NthOfType(i32, i32),
}

impl core::fmt::Display for PseudoClasses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Host => "host".to_string(),
            Self::FirstChild => "first-child".to_string(),
            Self::LastChild => "last-child".to_string(),
            Self::Empty => "empty".to_string(),
            Self::Not(selectors) => {
                let selectors_str = selectors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("not({})", selectors_str)
            }
            Self::OnlyChild => "only-child".to_string(),
            Self::NthChild(a, b, selector_list) => {
                if let Some(selectors) = selector_list {
                    format!(
                        "nth-child({}n + {} of {})",
                        a,
                        b,
                        selectors
                            .iter()
                            .map(|selector| selector.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                } else {
                    format!("nth-child({}n + {})", a, b)
                }
            }
            Self::NthOfType(a, b) => format!("nth-of-type({}n + {})", a, b),
        };
        write!(f, "{}", s)
    }
}

#[cfg_attr(debug_assertions, compatibility_enum_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) enum PseudoElements {
    Before,
    After,
}

impl core::fmt::Display for PseudoElements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Before => "before",
            Self::After => "after",
        };
        write!(f, "{}", s)
    }
}

pub(crate) static SELECTOR_WHITESPACE: &[char] = &[' ', '\t', '\n', '\r', '\x0C'];

#[repr(C)]
#[cfg_attr(debug_assertions, compatibility_enum_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum AttributeFlags {
    CaseSensitivityDependsOnName, // no flag
    CaseSensitive,                // 's' flag
    CaseInsensitive,              // 'i' flag
}

#[repr(C)]
#[cfg_attr(debug_assertions, compatibility_enum_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) enum AttributeOperator {
    Set,
    Exact,
    List,
    Hyphen,
    Begin,
    End,
    Contain,
}

#[cfg_attr(debug_assertions, compatibility_struct_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Attribute {
    pub(crate) operator: AttributeOperator,
    pub(crate) case_insensitive: AttributeFlags,
    pub(crate) never_matches: bool,
    pub(crate) name: String,
    pub(crate) value: Option<String>,
}

impl Attribute {
    pub(crate) fn new_set(name: String) -> Self {
        Self {
            operator: AttributeOperator::Set,
            case_insensitive: AttributeFlags::CaseSensitivityDependsOnName,
            never_matches: false,
            name,
            value: None,
        }
    }
}

impl core::fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self.name.to_string();
        match self.operator {
            AttributeOperator::Set => {
                return write!(f, "{}", s);
            }
            AttributeOperator::Exact => s.push('='),
            AttributeOperator::List => s.push_str("~="),
            AttributeOperator::Begin => s.push_str("^="),
            AttributeOperator::End => s.push_str("$="),
            AttributeOperator::Contain => s.push_str("*="),
            AttributeOperator::Hyphen => s.push_str("|="),
        };
        s.push_str(&format!("\"{}\"", self.value.as_ref().unwrap()));
        match self.case_insensitive {
            AttributeFlags::CaseInsensitive => s.push_str(" i"),
            AttributeFlags::CaseSensitive => s.push_str(" s"),
            AttributeFlags::CaseSensitivityDependsOnName => {}
        }
        write!(f, "{}", s)
    }
}

// TODO consider change String to StrRef
#[cfg_attr(debug_assertions, compatibility_struct_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[allow(clippy::box_collection)] // TODO optimize here
pub(crate) struct SelectorFragment {
    pub(crate) tag_name: String,
    pub(crate) id: String,
    pub(crate) classes: Vec<String>,
    pub(crate) relation: Option<Box<SelectorRelationType>>,
    weight: Cell<u16>,
    pub(crate) pseudo_classes: Option<Box<PseudoClasses>>,
    pub(crate) pseudo_elements: Option<Box<PseudoElements>>,
    pub(crate) attributes: Option<Box<Vec<Attribute>>>,
}

impl fmt::Display for SelectorFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(relation) = &self.relation {
            match &**relation {
                SelectorRelationType::None => {}
                SelectorRelationType::Ancestor(x) => write!(f, "{} ", x)?,
                SelectorRelationType::DirectParent(x) => write!(f, "{} > ", x)?,
                SelectorRelationType::NextSibling(x) => write!(f, "{} + ", x)?,
                SelectorRelationType::SubsequentSibling(x) => write!(f, "{} ~ ", x)?,
            }
        }
        if !self.tag_name.is_empty() {
            write!(f, "{}", self.tag_name)?;
        }
        if !self.id.is_empty() {
            write!(f, "#{}", self.id)?;
        }
        for class in self.classes.iter() {
            write!(f, ".{}", class)?;
        }
        if self.pseudo_classes.is_some() {
            write!(f, ":{}", self.pseudo_classes.as_ref().unwrap())?;
        }
        if self.pseudo_elements.is_some() {
            write!(f, "::{}", self.pseudo_elements.as_ref().unwrap())?;
        }
        if let Some(attributes) = self.attributes.as_ref() {
            for attr in attributes.iter() {
                write!(f, "[{}]", attr)?;
            }
        }
        Ok(())
    }
}

impl SelectorFragment {
    pub(crate) fn new() -> Self {
        Self {
            tag_name: String::new(),
            id: String::new(),
            classes: vec![],
            relation: None,
            weight: Cell::new(0),
            pseudo_classes: None,
            pseudo_elements: None,
            attributes: None,
        }
    }
    pub(crate) fn with_relation(parent: SelectorRelationType) -> Self {
        Self {
            tag_name: String::new(),
            id: String::new(),
            classes: vec![],
            relation: Some(Box::new(parent)),
            weight: Cell::new(0),
            pseudo_classes: None,
            pseudo_elements: None,
            attributes: None,
        }
    }
    pub(crate) fn weight(&self) -> u16 {
        let mut weight = self.weight.get();
        if weight > 0 {
            return weight;
        }
        if !self.tag_name.is_empty() {
            weight += 1 << 0;
        }
        if !self.id.is_empty() {
            weight += 1 << 13;
        }
        let class_and_attr_count = self.classes.len()
            + self
                .attributes
                .as_ref()
                .map(|a| a.len())
                .unwrap_or_default();
        weight += (class_and_attr_count.min(0xff) << 5) as u16;
        if let Some(ref relation) = self.relation {
            weight += match &**relation {
                SelectorRelationType::None => 0,
                SelectorRelationType::Ancestor(x) => x.weight(),
                SelectorRelationType::DirectParent(x) => x.weight(),
                SelectorRelationType::NextSibling(x) => x.weight(),
                SelectorRelationType::SubsequentSibling(x) => x.weight(),
            }
        }
        if self.pseudo_classes.as_ref().is_some() {
            weight += 1 << 5;
        }
        self.weight.set(weight);
        weight
    }
    pub(crate) fn set_tag_name(&mut self, tag_name: &str) {
        self.tag_name = tag_name.into();
    }
    pub(crate) fn set_id(&mut self, id: &str) {
        self.id = id.into();
    }
    pub(crate) fn add_class(&mut self, class: &str) {
        self.classes.push(class.into())
    }
    #[cfg(feature = "deserialize")]
    pub(crate) fn set_basics(&mut self, tag_name: String, id: String, classes: Vec<String>) {
        self.tag_name = tag_name;
        self.id = id;
        self.classes = classes;
    }
    pub(crate) fn set_pseudo_classes(&mut self, pseudo_classes: PseudoClasses) {
        self.pseudo_classes = Some(Box::new(pseudo_classes));
    }
    pub(crate) fn set_pseudo_elements(&mut self, pseudo_elements: PseudoElements) {
        self.pseudo_elements = Some(Box::new(pseudo_elements));
    }
    pub(crate) fn add_attribute(&mut self, attribute: Attribute) {
        if self.attributes.is_none() {
            self.attributes.replace(Box::new(Vec::with_capacity(1)));
        }
        if let Some(ref mut attributes) = self.attributes {
            attributes.push(attribute);
        }
    }
    pub(crate) fn add_tag_name_prefix(&mut self, prefix: &str) {
        if !self.tag_name.is_empty() {
            self.tag_name = format!("{}{}", prefix, self.tag_name);
        }
        if let Some(parent) = self.relation.as_mut() {
            match parent.as_mut() {
                SelectorRelationType::Ancestor(frag) => {
                    frag.add_tag_name_prefix(prefix);
                }
                SelectorRelationType::DirectParent(frag) => {
                    frag.add_tag_name_prefix(prefix);
                }
                _ => {}
            }
        }
    }
}

#[cfg_attr(debug_assertions, compatibility_struct_check(selector))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct Selector {
    pub(crate) fragments: Vec<SelectorFragment>,
    pub(crate) max_weight: u16,
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.fragments
                .iter()
                .map(|f| f.to_string())
                .collect::<Box<[String]>>()
                .join(", ")
        )
    }
}

impl Selector {
    pub(crate) fn star_selector() -> Self {
        Self {
            fragments: vec![SelectorFragment::new()],
            max_weight: 0,
        }
    }
    pub(crate) fn from_fragments(fragments: Vec<SelectorFragment>) -> Self {
        let mut max_weight = 0;
        for f in fragments.iter() {
            let w = f.weight();
            if w > max_weight {
                max_weight = w;
            }
        }
        Self {
            fragments,
            max_weight,
        }
    }
    pub(crate) fn get_index_classes(&self) -> Vec<String> {
        let mut ret = vec![];
        for frag in self.fragments.iter() {
            let s = if !frag.classes.is_empty() {
                frag.classes[0].clone()
            } else {
                String::new()
            };
            if !ret.contains(&s) {
                ret.push(s)
            }
        }
        ret
    }
    pub(crate) fn match_query<T: StyleNode>(
        &self,
        query: &[T],
        sheet_style_scope: Option<NonZeroUsize>,
    ) -> Option<u16> {
        let mut cur_weight = 0;
        'f: for frag in self.fragments.iter() {
            let mut query = query.iter();
            match query.next_back() {
                Some(mut cur_query) => {
                    let same_scope = sheet_style_scope.is_none()
                        || sheet_style_scope == cur_query.style_scope()
                        || sheet_style_scope == cur_query.extra_style_scope();
                    let mut allow_ancestor = false;
                    let mut cur_frag = frag;
                    loop {
                        let mut matches = true;
                        if (!cur_frag.id.is_empty()
                            && (!same_scope || Some(cur_frag.id.as_str()) != cur_query.id()))
                            || (!cur_frag.tag_name.is_empty()
                                && (!same_scope || cur_frag.tag_name != cur_query.tag_name()))
                        {
                            matches = false
                        } else if let Some(pc) = cur_frag.pseudo_classes.as_ref() {
                            match &**pc {
                                PseudoClasses::Host => {
                                    if sheet_style_scope.is_some()
                                        && sheet_style_scope != cur_query.host_style_scope()
                                    {
                                        matches = false
                                    }
                                }
                                _ => matches = false,
                            }
                        } else {
                            for class_name in cur_frag.classes.iter() {
                                if !cur_query.classes().any(|x| {
                                    (sheet_style_scope.is_none() || sheet_style_scope == x.scope())
                                        && x.name() == class_name
                                }) {
                                    matches = false;
                                }
                            }

                            if matches {
                                if let Some(selector_attributes) = &cur_frag.attributes {
                                    for attribute in selector_attributes.iter() {
                                        let selector_attr_value =
                                            attribute.value.as_deref().unwrap_or_default();
                                        if let Some(element_attr_value) =
                                            cur_query.attribute(&attribute.name)
                                        {
                                            if !match attribute.operator {
                                                AttributeOperator::Set => true,
                                                AttributeOperator::Exact => {
                                                    element_attr_value == selector_attr_value
                                                }
                                                AttributeOperator::List => {
                                                    if selector_attr_value.is_empty() {
                                                        false
                                                    } else {
                                                        element_attr_value
                                                            .split(SELECTOR_WHITESPACE)
                                                            .any(|x| x == selector_attr_value)
                                                    }
                                                }
                                                AttributeOperator::Hyphen => {
                                                    if element_attr_value.len()
                                                        < selector_attr_value.len()
                                                    {
                                                        false
                                                    } else if element_attr_value.len()
                                                        == selector_attr_value.len()
                                                    {
                                                        element_attr_value == selector_attr_value
                                                    } else {
                                                        element_attr_value.starts_with(
                                                            &alloc::format!(
                                                                "{}-",
                                                                selector_attr_value
                                                            ),
                                                        )
                                                    }
                                                }
                                                AttributeOperator::Begin => element_attr_value
                                                    .starts_with(selector_attr_value),
                                                AttributeOperator::End => element_attr_value
                                                    .ends_with(selector_attr_value),
                                                AttributeOperator::Contain => {
                                                    element_attr_value.contains(selector_attr_value)
                                                }
                                            } {
                                                matches = false;
                                                break;
                                            }
                                        } else {
                                            matches = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if !matches {
                            if allow_ancestor {
                                cur_query = match query.next_back() {
                                    Some(x) => x,
                                    None => continue 'f,
                                }
                            } else {
                                continue 'f;
                            }
                            continue;
                        }
                        if let Some(ref relation) = cur_frag.relation {
                            cur_query = match query.next_back() {
                                Some(x) => x,
                                None => continue 'f,
                            };
                            match &**relation {
                                SelectorRelationType::None => {
                                    // empty
                                }
                                SelectorRelationType::Ancestor(x) => {
                                    cur_frag = x;
                                    allow_ancestor = true;
                                }
                                SelectorRelationType::DirectParent(x) => {
                                    cur_frag = x;
                                    allow_ancestor = false;
                                }
                                SelectorRelationType::NextSibling(x) => {
                                    cur_frag = x;
                                    allow_ancestor = false;
                                }
                                SelectorRelationType::SubsequentSibling(x) => {
                                    cur_frag = x;
                                    allow_ancestor = false
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
                None => break,
            }
            let w = frag.weight();
            if w == self.max_weight {
                return Some(w);
            }
            cur_weight = w;
        }
        if cur_weight > 0 {
            Some(cur_weight)
        } else {
            None
        }
    }
    pub(crate) fn from_string(selector_str: &str) -> Selector {
        let mut parser_input = ParserInput::new(selector_str);
        let mut parser = Parser::new(&mut parser_input);
        let mut st = ParseState::new(None, crate::parser::StyleParsingDebugMode::None, None);
        let selector = parse_selector(&mut parser, &mut st);
        if let Ok(ret) = selector {
            return ret;
        }
        Selector::default()
    }
}
