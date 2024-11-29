use alloc::borrow::Cow;
use core::fmt;

use super::*;
use crate::property::Property;

/// A CSS property with some metadata.
#[derive(Clone, Debug)]
pub enum PropertyMeta {
    /// A single normal property, e.g. `font-size: 16px`.
    Normal {
        /// The property body.
        property: Property,
    },
    /// A single property with `!important`, e.g. `font-size: 16px !important`.
    Important {
        /// The property body.
        property: Property,
    },
    /// A group of properties.
    ///
    /// It is designed for debugging only.
    /// In production environment, properties are well-normalized -
    /// shorthand properties (e.g. `font` `background`) are splitted in advance.
    /// However, we may add new shorthand properties in debugger -
    /// we can keep the shorthand properties as-is with `DebugGroup`s.
    DebugGroup {
        /// The original name-value string pair.
        original_name_value: Box<(String, String)>,
        /// The parsed property list.
        properties: Box<[Property]>,
        /// `!important` or not.
        important: bool,
        /// Disabled or not.
        disabled: bool,
    },
}

impl PropertyMeta {
    /// Generate a new property.
    ///
    /// Note that the property is in *debug* mode so that:
    /// * it cannot be serialized even if it has been inserted to a rule;
    /// * it has a little performance penalty.
    pub fn new_debug_properties(source: &str) -> Vec<Self> {
        parser::parse_inline_style(source, parser::StyleParsingDebugMode::Debug).0
    }

    /// Clone the property and set the disable state of the new property.
    ///
    /// Note that the new property is in *debug* mode so that:
    /// * it cannot be serialized even if it has been inserted to a rule;
    /// * it has a little performance penalty.
    pub fn to_debug_state(&self, disabled: bool) -> Self {
        match self {
            Self::Normal { property } => Self::DebugGroup {
                original_name_value: Box::new((
                    self.get_property_name().into(),
                    self.get_property_value_string(),
                )),
                properties: Box::new([property.clone()]),
                important: false,
                disabled,
            },
            Self::Important { property } => Self::DebugGroup {
                original_name_value: Box::new((
                    self.get_property_name().into(),
                    self.get_property_value_string(),
                )),
                properties: Box::new([property.clone()]),
                important: false,
                disabled,
            },
            Self::DebugGroup {
                original_name_value,
                properties,
                important,
                ..
            } => Self::DebugGroup {
                original_name_value: original_name_value.clone(),
                properties: properties.clone(),
                important: *important,
                disabled,
            },
        }
    }

    /// The property is `!important` or not.
    pub fn is_important(&self) -> bool {
        match self {
            Self::Normal { .. } => false,
            Self::Important { .. } => true,
            Self::DebugGroup { important, .. } => *important,
        }
    }

    /// Get the property name.
    pub fn get_property_name(&self) -> Cow<'static, str> {
        match self {
            Self::Normal { property } => property.get_property_name().into(),
            Self::Important { property } => property.get_property_name().into(),
            Self::DebugGroup {
                original_name_value,
                ..
            } => original_name_value.0.clone().into(),
        }
    }

    /// Get the property value as a string.
    ///
    /// Note that it may (and may not) be normalized.
    pub fn get_property_value_string(&self) -> String {
        match self {
            Self::Normal { property } => property.get_property_value_string(),
            Self::Important { property } => {
                let mut v = property.get_property_value_string();
                v.push_str(" !important");
                v
            }
            Self::DebugGroup {
                original_name_value,
                ..
            } => original_name_value.1.clone(),
        }
    }

    /// The property is disabled (only possible in debug state) or not.
    pub fn is_disabled(&self) -> bool {
        match self {
            Self::Normal { .. } => false,
            Self::Important { .. } => false,
            Self::DebugGroup { disabled, .. } => *disabled,
        }
    }

    /// The property is invalid (only possible in debug state) or not.
    pub fn is_invalid(&self) -> bool {
        match self {
            Self::Normal { .. } => false,
            Self::Important { .. } => false,
            Self::DebugGroup { properties, .. } => properties.len() == 0,
        }
    }

    /// The property is deprecated or not.
    pub fn is_deprecated(&self) -> bool {
        match self {
            Self::Normal { property, .. } | Self::Important { property, .. } => {
                property.is_deprecated()
            }
            Self::DebugGroup { .. } => false,
        }
    }

    /// Merge the property into a `NodeProperties`.
    pub fn merge_to_node_properties(
        &self,
        node_properties: &mut NodeProperties,
        parent_node_properties: Option<&NodeProperties>,
        current_font_size: f32,
    ) {
        match self {
            PropertyMeta::Normal { property: p } => {
                node_properties.merge_property(p, parent_node_properties, current_font_size)
            }
            PropertyMeta::Important { property: p } => {
                node_properties.merge_property(p, parent_node_properties, current_font_size)
            }
            PropertyMeta::DebugGroup {
                properties,
                disabled,
                ..
            } => {
                if !disabled {
                    for p in &**properties {
                        node_properties.merge_property(p, parent_node_properties, current_font_size)
                    }
                }
            }
        }
    }

    /// Get an iterate of the properties.
    pub fn iter(&self) -> PropertyMetaIter {
        PropertyMetaIter { pm: self, cur: 0 }
    }

    #[cfg(test)]
    pub fn property(&self) -> Option<Property> {
        match self {
            Self::Normal { property } | Self::Important { property } => Some(property.clone()),
            Self::DebugGroup { .. } => None,
        }
    }
}

impl fmt::Display for PropertyMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.is_disabled() {
            write!(
                f,
                "/* {}: {}; */",
                self.get_property_name(),
                self.get_property_value_string(),
            )
        } else {
            write!(
                f,
                "{}: {};",
                self.get_property_name(),
                self.get_property_value_string(),
            )
        }
    }
}

/// The iterator for `PropertyMeta` .
pub struct PropertyMetaIter<'a> {
    pm: &'a PropertyMeta,
    cur: usize,
}

impl<'a> Iterator for PropertyMetaIter<'a> {
    type Item = &'a Property;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pm {
            PropertyMeta::Normal { property } | PropertyMeta::Important { property } => {
                if self.cur == 0 {
                    self.cur = 1;
                    Some(property)
                } else {
                    None
                }
            }
            PropertyMeta::DebugGroup { properties, .. } => {
                if self.cur < properties.len() {
                    let ret = &properties[self.cur];
                    self.cur += 1;
                    Some(ret)
                } else {
                    None
                }
            }
        }
    }
}

/// A CSS rule, e.g. `.my-class { ... }`.
#[derive(Clone, Debug)]
pub struct Rule {
    pub(crate) selector: Selector,
    pub(crate) properties: Vec<PropertyMeta>,
    pub(crate) media: Option<Rc<Media>>,
    pub(super) index: u32,
    pub(crate) has_font_size: bool,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let media_queries = self.get_media_query_string_list();
        for media in media_queries.iter() {
            write!(f, "@media {} {{ ", media)?;
        }
        let selector = self.get_selector_string();
        write!(f, "{} {{ ", selector)?;
        for prop in self.properties() {
            write!(
                f,
                "{}: {}; ",
                prop.get_property_name(),
                prop.get_property_value_string()
            )?;
        }
        write!(f, "}}")?;
        for _ in media_queries.iter() {
            write!(f, " }}")?;
        }
        Ok(())
    }
}

impl Rule {
    /// Create an empty rule.
    pub fn new_empty() -> Box<Self> {
        Box::new(Self {
            selector: Selector::star_selector(),
            properties: Vec::with_capacity(0),
            media: None,
            index: 0,
            has_font_size: false,
        })
    }

    pub(crate) fn new(
        selector: Selector,
        properties: Vec<PropertyMeta>,
        media: Option<Rc<Media>>,
    ) -> Box<Self> {
        Self::new_with_index(selector, properties, media, 0)
    }

    pub(crate) fn new_with_index(
        selector: Selector,
        properties: Vec<PropertyMeta>,
        media: Option<Rc<Media>>,
        index: u32,
    ) -> Box<Self> {
        let mut has_font_size = false;
        for p in properties.iter() {
            match p {
                PropertyMeta::Normal { property } | PropertyMeta::Important { property } => {
                    match property {
                        Property::FontSize(..) => {
                            has_font_size = true;
                        }
                        _ => {}
                    }
                }
                PropertyMeta::DebugGroup { properties, .. } => {
                    for property in properties.iter() {
                        match property {
                            Property::FontSize(..) => {
                                has_font_size = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Box::new(Self {
            selector,
            properties,
            media,
            index,
            has_font_size,
        })
    }

    /// Construct a new rule with media query strings and a selector string
    ///
    /// The media query strings should not contain the leading `@media` segment (matches the output of `get_media_query_string_list` ).
    pub fn from_parts_str<'a>(
        media_query_str_list: impl IntoIterator<Item = &'a str>,
        selector_str: &str,
    ) -> Result<Box<Self>, Warning> {
        let mut media = None;
        for (index, media_str) in media_query_str_list.into_iter().enumerate() {
            let cur_media = parser::parse_media_expression_only(media_str).map_err(|mut w| {
                w.message = format!("{} (in media index {})", w.message.as_str(), index).into();
                w
            })?;
            media = Some(Rc::new(cur_media));
        }
        let selector = parser::parse_selector_only(selector_str)?;
        Ok(Self::new(selector, vec![], media))
    }

    /// Modify the rule with a different selector (and construct a new one as the result)
    pub fn modify_selector(&self, selector_str: &str) -> Result<Box<Self>, Warning> {
        let media = self.media.clone();
        let selector = parser::parse_selector_only(selector_str)?;
        let properties = self.properties.clone();
        Ok(Self::new(selector, properties, media))
    }

    /// Modify the rule by adding a new property (and construct a new one as the result)
    pub fn add_properties(&self, p: impl IntoIterator<Item = PropertyMeta>) -> Box<Self> {
        let media = self.media.clone();
        let selector = self.selector.clone();
        let mut properties = self.properties.clone();
        for p in p {
            properties.push(p);
        }
        Self::new(selector, properties, media)
    }

    /// Enable or disable the rule (and construct a new one as the result if success)
    pub fn set_property_disabled(&self, index: usize, disabled: bool) -> Option<Box<Self>> {
        let media = self.media.clone();
        let selector = self.selector.clone();
        let mut properties = self.properties.clone();
        if index < properties.len() {
            properties[index] = properties[index].to_debug_state(disabled);
            Some(Self::new(selector, properties, media))
        } else {
            None
        }
    }

    /// Modify the rule by removing a property (and construct a new one as the result if success)
    pub fn remove_property(&self, index: usize) -> Option<Box<Self>> {
        let media = self.media.clone();
        let selector = self.selector.clone();
        let mut properties = self.properties.clone();
        if index < properties.len() {
            properties.remove(index);
            Some(Self::new(selector, properties, media))
        } else {
            None
        }
    }

    /// Modify the rule by replacing properties (and construct a new one as the result if success)
    pub fn replace_properties(
        &self,
        range: impl core::ops::RangeBounds<usize>,
        p: impl IntoIterator<Item = PropertyMeta>,
    ) -> Option<Box<Self>> {
        use core::ops::Bound;
        let media = self.media.clone();
        let selector = self.selector.clone();
        let mut properties = self.properties.clone();
        let no_overflow = match range.end_bound() {
            Bound::Unbounded => true,
            Bound::Included(stp) => *stp < properties.len(),
            Bound::Excluded(stp) => *stp <= properties.len(),
        };
        let no_reversed = match range.start_bound() {
            Bound::Unbounded => true,
            Bound::Included(st) => match range.end_bound() {
                Bound::Unbounded => true,
                Bound::Included(stp) => *st <= *stp,
                Bound::Excluded(stp) => *st < *stp,
            },
            Bound::Excluded(st) => match range.end_bound() {
                Bound::Unbounded => true,
                Bound::Included(stp) => *st < *stp,
                Bound::Excluded(stp) => st + 1 < *stp,
            },
        };
        if no_overflow && no_reversed {
            properties.splice(range, p);
            Some(Self::new(selector, properties, media))
        } else {
            None
        }
    }

    /// Get the `@media` list.
    pub fn get_media_query_string_list(&self) -> Vec<String> {
        let mut list = vec![];
        if let Some(x) = &self.media {
            x.to_media_query_string_list(&mut list);
        }
        list
    }

    /// Get the selector as a string.
    pub fn get_selector_string(&self) -> String {
        format!("{}", self.selector)
    }

    /// Get an iterator of the properties.
    pub fn properties(&self) -> impl Iterator<Item = &PropertyMeta> {
        self.properties.iter()
    }

    pub(crate) fn match_query<L: LengthNum>(
        &self,
        query: &[StyleQuery],
        media_query_status: &MediaQueryStatus<L>,
        sheet_style_scope: Option<NonZeroUsize>,
    ) -> Option<u16> {
        match &self.media {
            Some(media) => {
                if !media.is_valid(media_query_status) {
                    return None;
                }
            }
            None => {}
        }
        self.selector.match_query(query, sheet_style_scope)
    }
}
