//! Utilities for style queries.

use alloc::{rc::Rc, string::String, vec::Vec};
use core::{hash::Hasher, num::NonZeroUsize};

use crate::length_num::LengthNum;
use crate::property::{
    NodeProperties, NodePropertiesOrder, Property, PropertyMeta, PropertyValueWithGlobal,
};
use crate::sheet::{PseudoElements, Rule};
use crate::sheet::{RuleWeight, Theme};
use crate::typing::{Length, LengthType};

/// The status of media query, i.e. screen size, screen type, etc.
///
/// This also contains some global environment values, such as `env(...)` values in CSS.
///
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQueryStatus<L: LengthNum> {
    /// The viewport is a `screen` media type.
    pub is_screen: bool,
    /// The viewport width.
    pub width: L,
    /// The viewport height.
    pub height: L,
    /// The viewport pixel ratio.
    pub pixel_ratio: f32,
    /// The global font-size.
    pub base_font_size: L,
    /// The current theme, i.e. dark mode or not.
    pub theme: Theme,
    /// The `env(...)` expression value.
    pub env: EnvValues<L>,
}

/// The values used in CSS `env()` functions
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct EnvValues<L: LengthNum> {
    pub safe_area_inset_left: L,
    pub safe_area_inset_top: L,
    pub safe_area_inset_right: L,
    pub safe_area_inset_bottom: L,
}

impl<L: LengthNum> Default for EnvValues<L> {
    fn default() -> Self {
        Self {
            safe_area_inset_left: L::zero(),
            safe_area_inset_top: L::zero(),
            safe_area_inset_right: L::zero(),
            safe_area_inset_bottom: L::zero(),
        }
    }
}

impl<L: LengthNum> Default for MediaQueryStatus<L> {
    fn default() -> Self {
        Self::default_screen()
    }
}

impl<L: LengthNum> MediaQueryStatus<L> {
    /// Default screen settings (800x600).
    pub fn default_screen() -> Self {
        Self {
            is_screen: true,
            width: L::from_i32(800),
            height: L::from_i32(600),
            pixel_ratio: 1.,
            base_font_size: L::from_i32(16),
            theme: Theme::Light,
            env: Default::default(),
        }
    }

    /// Default screen settings with size specified.
    pub fn default_screen_with_size(width: L, height: L) -> Self {
        Self {
            is_screen: true,
            width,
            height,
            pixel_ratio: 1.,
            base_font_size: L::from_i32(16),
            theme: Theme::Light,
            env: Default::default(),
        }
    }
}

/// The class for a `StyleNode`.
pub trait StyleNodeClass {
    /// The name of the class.
    fn name(&self) -> &str;

    /// The style scope of the class.
    fn scope(&self) -> Option<NonZeroUsize>;
}

impl StyleNodeClass for (String, Option<NonZeroUsize>) {
    fn name(&self) -> &str {
        &self.0
    }

    fn scope(&self) -> Option<NonZeroUsize> {
        self.1
    }
}

/// The case-sensitivity for attribute matching.
pub enum StyleNodeAttributeCaseSensitivity {
    /// Case-sensitive.
    CaseSensitive,

    /// Case-insensitive.
    CaseInsensitive,
}

impl StyleNodeAttributeCaseSensitivity {
    /// Matches two strings with this case-sensitivity.
    pub fn eq(&self, a: &str, b: &str) -> bool {
        match self {
            Self::CaseSensitive => a == b,
            Self::CaseInsensitive => a.eq_ignore_ascii_case(b),
        }
    }

    /// Check if `a` starts with `b` in this case-sensitivity.
    pub fn starts_with(&self, a: &str, b: &str) -> bool {
        // FIXME: reduce memory allocation
        match self {
            Self::CaseSensitive => a.starts_with(b),
            Self::CaseInsensitive => a.to_ascii_lowercase().starts_with(&b.to_ascii_lowercase()),
        }
    }

    /// Check if `a` ends with `b` in this case-sensitivity.
    pub fn ends_with(&self, a: &str, b: &str) -> bool {
        // FIXME: reduce memory allocation
        match self {
            Self::CaseSensitive => a.ends_with(b),
            Self::CaseInsensitive => a.to_ascii_lowercase().ends_with(&b.to_ascii_lowercase()),
        }
    }

    /// Check if `a` contains `b` in this case-sensitivity.
    pub fn contains(&self, a: &str, b: &str) -> bool {
        // FIXME: reduce memory allocation
        match self {
            Self::CaseSensitive => a.contains(b),
            Self::CaseInsensitive => a.to_ascii_lowercase().contains(&b.to_ascii_lowercase()),
        }
    }
}

/// A node descriptor for a style query.
pub trait StyleNode {
    /// The type for a class.
    type Class: StyleNodeClass;

    /// The type for classes iteration.
    type ClassIter<'a>: Iterator<Item = &'a Self::Class>
    where
        Self: 'a;

    /// The style scope of the node itself.
    fn style_scope(&self) -> Option<NonZeroUsize>;

    /// The extra style scope of the node.
    fn extra_style_scope(&self) -> Option<NonZeroUsize>;

    /// The extra style scope for the `:host` selector.
    fn host_style_scope(&self) -> Option<NonZeroUsize>;

    /// The tag name of the node.
    fn tag_name(&self) -> &str;

    /// The id of the node.
    fn id(&self) -> Option<&str>;

    /// The classes of the node.
    fn classes(&self) -> Self::ClassIter<'_>;

    /// Get an attribute of the node.
    fn attribute(&self, name: &str) -> Option<(&str, StyleNodeAttributeCaseSensitivity)>;

    /// The pseudo element to query.
    fn pseudo_element(&self) -> Option<PseudoElements>;

    /// Check if the node has a specified scope.
    fn contain_scope(&self, scope: Option<NonZeroUsize>) -> bool {
        scope.is_none()
            || self.style_scope() == scope
            || self.extra_style_scope() == scope
            || self.host_style_scope() == scope
    }
}

/// Represents node information, used for matching rules.
#[derive(Debug)]
pub struct StyleQuery<'a> {
    pub(super) style_scope: Option<NonZeroUsize>,
    pub(super) extra_style_scope: Option<NonZeroUsize>,
    pub(super) host_style_scope: Option<NonZeroUsize>,
    pub(super) tag_name: &'a str,
    pub(super) id: &'a str,
    pub(super) classes: &'a [(String, Option<NonZeroUsize>)],
}

impl Clone for StyleQuery<'_> {
    fn clone(&self) -> Self {
        Self {
            style_scope: self.style_scope,
            extra_style_scope: self.extra_style_scope,
            host_style_scope: self.host_style_scope,
            tag_name: self.tag_name,
            id: self.id,
            classes: self.classes,
        }
    }
}

impl<'a> StyleQuery<'a> {
    /// Constructs a style query from tag name, id, classes, etc.
    pub fn single(
        style_scope: Option<NonZeroUsize>,
        extra_style_scope: Option<NonZeroUsize>,
        host_style_scope: Option<NonZeroUsize>,
        tag_name: &'a str,
        id: &'a str,
        classes: &'a [(String, Option<NonZeroUsize>)],
    ) -> Self {
        Self {
            style_scope,
            extra_style_scope,
            host_style_scope,
            tag_name,
            id,
            classes,
        }
    }
}

impl<'a> StyleNode for StyleQuery<'a> {
    type Class = (String, Option<NonZeroUsize>);
    type ClassIter<'c>
        = core::slice::Iter<'c, Self::Class>
    where
        'a: 'c;

    fn style_scope(&self) -> Option<NonZeroUsize> {
        self.style_scope
    }

    fn extra_style_scope(&self) -> Option<NonZeroUsize> {
        self.extra_style_scope
    }

    fn host_style_scope(&self) -> Option<NonZeroUsize> {
        self.host_style_scope
    }

    fn tag_name(&self) -> &str {
        self.tag_name
    }

    fn id(&self) -> Option<&str> {
        Some(self.id)
    }

    fn classes(&self) -> Self::ClassIter<'_> {
        self.classes.iter()
    }

    fn attribute(&self, _name: &str) -> Option<(&str, StyleNodeAttributeCaseSensitivity)> {
        None
    }

    fn pseudo_element(&self) -> Option<PseudoElements> {
        None
    }
}

impl<'b, 'a: 'b> StyleNode for &'b StyleQuery<'a> {
    type Class = (String, Option<NonZeroUsize>);
    type ClassIter<'c>
        = core::slice::Iter<'c, Self::Class>
    where
        'b: 'c;

    fn style_scope(&self) -> Option<NonZeroUsize> {
        self.style_scope
    }

    fn extra_style_scope(&self) -> Option<NonZeroUsize> {
        self.extra_style_scope
    }

    fn host_style_scope(&self) -> Option<NonZeroUsize> {
        self.host_style_scope
    }

    fn tag_name(&self) -> &str {
        self.tag_name
    }

    fn id(&self) -> Option<&str> {
        Some(self.id)
    }

    fn classes(&self) -> Self::ClassIter<'_> {
        self.classes.iter()
    }

    fn attribute(&self, _name: &str) -> Option<(&str, StyleNodeAttributeCaseSensitivity)> {
        None
    }

    fn pseudo_element(&self) -> Option<PseudoElements> {
        None
    }
}

/// Represents a matched rule (borrowed form).
#[derive(Debug, Clone)]
pub struct MatchedRuleRef<'a> {
    /// The rule body.
    pub rule: &'a Rc<Rule>,
    /// The weight of the rule.
    pub weight: RuleWeight,
}

/// Represents a matched rule.
#[derive(Debug, Clone)]
pub struct MatchedRule {
    /// The rule body.
    pub rule: Rc<Rule>,
    /// The weight of the rule.
    pub weight: RuleWeight,
    /// The style scope of the rule.
    pub style_scope: Option<NonZeroUsize>,
}

impl PartialEq for MatchedRule {
    fn eq(&self, other: &Self) -> bool {
        self.weight.normal() == other.weight.normal()
    }
}

impl PartialOrd for MatchedRule {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MatchedRule {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.weight.normal().cmp(&other.weight.normal())
    }
}

impl Eq for MatchedRule {}

/// Represents the matched rule list.
#[derive(Debug, Clone)]
pub struct MatchedRuleList {
    /// The matched rules.
    pub rules: Vec<MatchedRule>,
}

impl MatchedRuleList {
    /// Create an empty list.
    pub fn new_empty() -> Self {
        Self {
            rules: Vec::with_capacity(0),
        }
    }

    /// Calculate the font size.
    ///
    /// Some methods like `merge_node_properties` requires it to resolve `em` values.
    pub fn get_current_font_size<L: LengthNum>(
        &self,
        parent_font_size: f32,
        parent_node_properties: Option<&NodeProperties>,
        extra_styles: &[PropertyMeta],
        media_query_status: &MediaQueryStatus<L>,
    ) -> f32 {
        // find font-size properties
        let mut font_size_p = None;
        let mut font_size_w: u64 = 0;
        fn handle_property_meta<'a>(
            font_size_p: &mut Option<&'a LengthType>,
            font_size_w: &mut u64,
            pm: &'a PropertyMeta,
            rw: RuleWeight,
        ) {
            match pm {
                PropertyMeta::Normal { property: p } => {
                    if let Property::FontSize(x) = p {
                        let w = rw.normal();
                        if w >= *font_size_w {
                            *font_size_w = w;
                            *font_size_p = Some(x);
                        }
                    }
                }
                PropertyMeta::Important { property: p } => {
                    if let Property::FontSize(x) = p {
                        let w = rw.important();
                        if w >= *font_size_w {
                            *font_size_w = w;
                            *font_size_p = Some(x);
                        }
                    }
                }
                PropertyMeta::DebugGroup {
                    properties,
                    important,
                    disabled,
                    ..
                } => {
                    if !disabled {
                        let w = if *important {
                            rw.important()
                        } else {
                            rw.normal()
                        };
                        if w >= *font_size_w {
                            for p in &**properties {
                                if let Property::FontSize(x) = p {
                                    *font_size_w = w;
                                    *font_size_p = Some(x);
                                }
                            }
                        }
                    }
                }
            }
        }
        for pm in extra_styles.iter() {
            handle_property_meta(&mut font_size_p, &mut font_size_w, pm, RuleWeight::inline());
        }
        for matched_rule in self.rules.iter() {
            let rw = matched_rule.weight;
            if !matched_rule.rule.has_font_size {
                continue;
            };
            for pm in matched_rule.rule.properties.iter() {
                handle_property_meta(&mut font_size_p, &mut font_size_w, pm, rw);
            }
        }

        // get current font-size
        let default_font_size = media_query_status.base_font_size.to_f32();
        let parent_font_size_p = parent_node_properties.map(|x| x.font_size_ref());
        let parent_font_size = parent_font_size.to_f32();
        let current_font_size = if let Some(p) = font_size_p {
            p.to_inner(parent_font_size_p, Length::Px(default_font_size), true)
                .and_then(|x| x.resolve_to_f32(media_query_status, parent_font_size, true))
                .unwrap_or(parent_font_size)
        } else {
            parent_font_size
        };

        current_font_size
    }

    /// Merge the rule list into specified `NodeProperties` .
    pub fn merge_node_properties(
        &self,
        node_properties: &mut NodeProperties,
        parent_node_properties: Option<&NodeProperties>,
        current_font_size: f32,
        extra_styles: &[PropertyMeta],
    ) {
        let mut order = NodePropertiesOrder::new();
        let mut merge_property_meta = |pm: &PropertyMeta, rw: RuleWeight| match pm {
            PropertyMeta::Normal { property: p } => {
                if order.compare_property(p, rw.normal()) {
                    node_properties.merge_property(p, parent_node_properties, current_font_size)
                }
            }
            PropertyMeta::Important { property: p } => {
                if order.compare_property(p, rw.important()) {
                    node_properties.merge_property(p, parent_node_properties, current_font_size)
                }
            }
            PropertyMeta::DebugGroup {
                properties,
                important,
                disabled,
                ..
            } => {
                if !disabled {
                    let w = if *important {
                        rw.important()
                    } else {
                        rw.normal()
                    };
                    for p in &**properties {
                        if order.compare_property(p, w) {
                            node_properties.merge_property(
                                p,
                                parent_node_properties,
                                current_font_size,
                            )
                        }
                    }
                }
            }
        };
        for pm in extra_styles.iter() {
            merge_property_meta(pm, RuleWeight::inline());
        }
        for matched_rule in self.rules.iter() {
            for pm in matched_rule.rule.properties.iter() {
                merge_property_meta(pm, matched_rule.weight);
            }
        }
    }

    /// Iterate properties with weights.
    pub fn for_each_property(&self, mut f: impl FnMut(&Property, u64)) {
        for matched_rule in self.rules.iter() {
            let weight = matched_rule.weight;
            for pm in matched_rule.rule.properties.iter() {
                if pm.is_disabled() {
                    continue;
                }
                let w = if pm.is_important() {
                    weight.important()
                } else {
                    weight.normal()
                };
                for p in pm.iter() {
                    f(p, w);
                }
            }
        }
    }

    /// Find the style scope of the rule which contains the applied `animation-name` property.
    ///
    /// This call is designed for the search of keyframes with style scopes.
    /// Returns `None` if there is no `animation-name` property or it is inside inline styles.
    pub fn animation_name_style_scope(&self) -> Option<NonZeroUsize> {
        let mut w = u64::MIN;
        let mut ret = None;
        let mut check_property_meta = |pm: &PropertyMeta, rw: RuleWeight, scope| {
            for p in pm.iter() {
                if let Property::AnimationName(..) = p {
                    let self_w = if pm.is_important() {
                        rw.important()
                    } else {
                        rw.normal()
                    };
                    if self_w >= w {
                        w = self_w;
                        ret = scope;
                    }
                }
            }
        };
        for matched_rule in self.rules.iter() {
            for pm in matched_rule.rule.properties.iter() {
                check_property_meta(pm, matched_rule.weight, matched_rule.style_scope);
            }
        }
        ret
    }

    /// Get a fast hash value of the list.
    ///
    /// The hash value can be used to identify the rule list is the same as the other one or not.
    pub fn fast_hash_value(&self) -> u64 {
        let mut hasher = ahash::AHasher::default();
        for matched_rule in self.rules.iter() {
            let rule: &Rule = &matched_rule.rule;
            hasher.write_usize(rule as *const Rule as usize);
            hasher.write_u64(matched_rule.weight.normal());
        }
        hasher.finish()
    }
}
