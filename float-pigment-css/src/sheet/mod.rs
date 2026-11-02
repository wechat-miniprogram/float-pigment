//! The style sheet data structures.

use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::{cell::RefCell, num::NonZeroUsize};
use parser::WarningKind;

use hashbrown::HashMap;

use super::property::*;
use super::query::*;
use super::*;
use crate::group::StyleSheetResource;
use crate::length_num::LengthNum;
use crate::parser::Warning;

mod selector;
pub use selector::PseudoElements;
pub(crate) use selector::{
    Attribute, AttributeFlags, AttributeOperator, PseudoClasses, Selector, SelectorFragment,
    SelectorRelationType, SELECTOR_WHITESPACE,
};
mod rule;
pub use rule::Rule;
mod media;
pub use media::*;
pub(crate) mod keyframes;
pub use keyframes::*;
mod font_face;
pub use font_face::*;
pub mod borrow;
pub mod borrow_resource;
pub mod str_store;
pub use rule::PropertyMeta;

pub(crate) const MAX_RULE_NESTING_DEPTH: usize = 64;

#[derive(Clone)]
pub(crate) enum CssRule {
    Style(Rc<Rule>),
    Media(Rc<MediaRule>),
    KeyFrames(Rc<KeyFrames>),
    FontFace(Rc<FontFace>),
    Import(Rc<ImportRule>),
}

#[derive(Clone)]
pub(crate) struct MediaRule {
    pub(crate) media: Rc<Media>,
    pub(crate) rules: Vec<CssRule>,
}

#[derive(Clone)]
pub(crate) struct ImportRule {
    pub(crate) path: String,
    pub(crate) media: Option<Rc<Media>>,
}

#[cfg(feature = "deserialize")]
fn close_media_group(rules: &mut Vec<CssRule>, groups: &mut Vec<(Rc<Media>, Vec<CssRule>)>) {
    let (media, children) = groups.pop().unwrap();
    let rule = CssRule::Media(Rc::new(MediaRule {
        media,
        rules: children,
    }));
    groups
        .last_mut()
        .map_or(rules, |(_, rules)| rules)
        .push(rule);
}

fn wrap_style_rule(rule: Rc<Rule>) -> CssRule {
    let mut parent = rule.media.clone();
    let mut result = CssRule::Style(rule);
    while let Some(media) = parent {
        parent = media.parent.clone();
        result = CssRule::Media(Rc::new(MediaRule {
            media,
            rules: vec![result],
        }));
    }
    result
}

fn for_each_style_rule_mut(rules: &mut [CssRule], mut f: impl FnMut(&mut Rc<Rule>)) {
    let mut stack = vec![rules.iter_mut()];
    while let Some(iter) = stack.last_mut() {
        match iter.next() {
            Some(CssRule::Style(rule)) => f(rule),
            Some(CssRule::Media(rule)) => stack.push(Rc::make_mut(rule).rules.iter_mut()),
            Some(_) => {}
            None => {
                stack.pop();
            }
        }
    }
}

fn split_style_rule(rules: &mut Vec<CssRule>, index: u32) -> Option<(Vec<CssRule>, Rc<Rule>)> {
    for position in 0..rules.len() {
        match &mut rules[position] {
            CssRule::Style(rule) if rule.index == index => {
                let following = rules.split_off(position + 1);
                let CssRule::Style(rule) = rules.pop().unwrap() else {
                    unreachable!();
                };
                return Some((following, rule));
            }
            CssRule::Media(rule) => {
                let rule = Rc::make_mut(rule);
                if let Some((children, old_rule)) = split_style_rule(&mut rule.rules, index) {
                    let following_group = (!children.is_empty()).then(|| {
                        CssRule::Media(Rc::new(MediaRule {
                            media: rule.media.clone(),
                            rules: children,
                        }))
                    });
                    let empty = rule.rules.is_empty();
                    let mut following = rules.split_off(position + 1);
                    if empty {
                        rules.pop();
                    }
                    if let Some(group) = following_group {
                        following.insert(0, group);
                    }
                    return Some((following, old_rule));
                }
            }
            _ => {}
        }
    }
    None
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledStyleSheet {
    linked: bool,
    ss: Rc<RefCell<StyleSheet>>,
}

impl CompiledStyleSheet {
    pub(crate) fn new() -> Self {
        Self::from_rules(vec![])
    }

    pub(crate) fn from_rules(rules: Vec<CssRule>) -> Self {
        Self {
            linked: false,
            ss: Rc::new(RefCell::new(StyleSheet::from_rules(rules))),
        }
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn new_with_config(
        imports: Vec<(String, Option<Rc<Media>>)>,
        rules: Vec<Rc<Rule>>,
        font_face: Vec<Rc<FontFace>>,
        keyframes: Vec<Rc<KeyFrames>>,
    ) -> Self {
        let mut css_rules = imports
            .into_iter()
            .map(|(path, media)| CssRule::Import(Rc::new(ImportRule { path, media })))
            .collect::<Vec<_>>();
        let mut groups: Vec<(Rc<Media>, Vec<CssRule>)> = vec![];
        for rule in rules {
            let mut parents = vec![];
            let mut media = rule.media.clone();
            while let Some(parent) = media {
                media = parent.parent.clone();
                parents.push(parent);
            }
            parents.reverse();
            let shared = groups
                .iter()
                .zip(&parents)
                .take_while(|((media, _), parent)| Rc::ptr_eq(media, parent))
                .count();
            while groups.len() > shared {
                close_media_group(&mut css_rules, &mut groups);
            }
            groups.extend(
                parents
                    .into_iter()
                    .skip(shared)
                    .map(|media| (media, vec![])),
            );
            let children = groups.last_mut().map_or(&mut css_rules, |(_, rules)| rules);
            children.push(CssRule::Style(rule));
        }
        while !groups.is_empty() {
            close_media_group(&mut css_rules, &mut groups);
        }
        // V1 does not record media ownership for font-face or keyframes.
        css_rules.extend(font_face.into_iter().map(CssRule::FontFace));
        css_rules.extend(keyframes.into_iter().map(CssRule::KeyFrames));
        Self::from_rules(css_rules)
    }

    pub(crate) fn list_deps(&self) -> Vec<String> {
        self.ss
            .borrow()
            .index
            .imports
            .iter()
            .map(|rule| rule.path.clone())
            .collect()
    }

    pub(crate) fn add_tag_name_prefix(&mut self, prefix: &str) {
        let mut ss = self.ss.borrow_mut();
        ss.index = StyleSheetIndex::default();
        for_each_style_rule_mut(&mut ss.rules, |rule| {
            for frag in Rc::make_mut(rule).selector.fragments.iter_mut() {
                frag.add_tag_name_prefix(prefix);
            }
        });
        ss.rebuild_index();
    }

    pub(crate) fn link(
        &mut self,
        res: &StyleSheetResource,
        scope: Option<NonZeroUsize>,
    ) -> (LinkedStyleSheet, Vec<Warning>) {
        let mut sheets = vec![];
        let mut warnings = vec![];
        self.link_self(res, &mut sheets, None, &mut warnings);
        (LinkedStyleSheet { sheets, scope }, warnings)
    }

    #[allow(clippy::type_complexity)]
    fn link_self(
        &mut self,
        res: &StyleSheetResource,
        sheets: &mut Vec<(Rc<RefCell<StyleSheet>>, Option<Rc<Media>>)>,
        parent_media: Option<Rc<Media>>,
        warnings: &mut Vec<Warning>,
    ) {
        if !self.linked {
            self.ss.borrow_mut().update_index();
            self.linked = true;
        }
        let imports = self.ss.borrow().index.imports.clone();
        for rule in &imports {
            let target_path = &rule.path;
            if let Some(target) = res.refs.get(target_path) {
                if let Ok(mut target) = target.try_borrow_mut() {
                    let m = match rule.media.clone() {
                        None => parent_media.clone(),
                        Some(mut m) => {
                            Rc::make_mut(&mut m).parent.clone_from(&parent_media);
                            Some(m)
                        }
                    };
                    target.link_self(res, sheets, m, warnings);
                } else {
                    warnings.push(Warning {
                        kind: WarningKind::RecursiveImports,
                        message: format!(
                            "detected recursive style sheet import for {target_path:?}"
                        )
                        .into(),
                        start_line: 0,
                        start_col: 0,
                        end_line: 0,
                        end_col: 0,
                    });
                }
            } else {
                warnings.push(Warning {
                    kind: WarningKind::MissingImportTarget,
                    message: format!(r#"target style sheet {target_path:?} not found"#).into(),
                    start_line: 0,
                    start_col: 0,
                    end_line: 0,
                    end_col: 0,
                });
            }
        }
        sheets.push((self.ss.clone(), parent_media));
    }

    #[cfg(feature = "serialize")]
    pub(crate) fn serialize_bincode(&self) -> Vec<u8> {
        use float_pigment_consistent_bincode::Options;
        let s = borrow::StyleSheet::from_sheet(self);
        float_pigment_consistent_bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .serialize(&s)
            .unwrap()
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn deserialize_bincode(s: Vec<u8>) -> Result<Self, String> {
        use float_pigment_consistent_bincode::Options;
        let s: Result<borrow::StyleSheet, _> =
            float_pigment_consistent_bincode::DefaultOptions::new()
                .allow_trailing_bytes()
                .deserialize(&s);
        match s {
            Ok(ss) => ss.into_sheet(),
            Err(err) => Err(format!(
                "Failed to deserialize bincode formatted style sheet: {err}"
            )),
        }
    }

    #[cfg(feature = "deserialize")]
    pub(crate) unsafe fn deserialize_bincode_zero_copy(
        ptr: *const [u8],
        drop_callback: impl 'static + FnOnce(),
    ) -> Result<Self, String> {
        use float_pigment_consistent_bincode::Options;
        borrow::de_static_ref_zero_copy_env(
            ptr,
            |s| {
                let s: Result<borrow::StyleSheet, _> =
                    float_pigment_consistent_bincode::DefaultOptions::new()
                        .allow_trailing_bytes()
                        .deserialize(s);
                match s {
                    Ok(ss) => ss.into_sheet(),
                    Err(err) => Err(format!(
                        "Failed to deserialize bincode formatted style sheet: {err}"
                    )),
                }
            },
            drop_callback,
        )
    }

    #[cfg(all(feature = "serialize", feature = "serialize_json"))]
    pub(crate) fn serialize_json(&self) -> String {
        let s = borrow::StyleSheet::from_sheet(self);
        serde_json::to_string(&s).unwrap()
    }

    #[cfg(all(feature = "serialize", feature = "deserialize_json"))]
    pub(crate) fn deserialize_json(s: &str) -> Result<Self, String> {
        let s: Result<borrow::StyleSheet, _> = serde_json::from_str(s);
        match s {
            Ok(ss) => ss.into_sheet(),
            Err(err) => Err(format!(
                "Failed to deserialize json formatted style sheet: {err}"
            )),
        }
    }

    #[cfg(all(feature = "serialize", feature = "deserialize_json"))]
    pub(crate) unsafe fn deserialize_json_zero_copy(
        ptr: *mut [u8],
        drop_callback: impl 'static + FnOnce(),
    ) -> Result<Self, String> {
        borrow::de_static_ref_zero_copy_env(
            ptr,
            |s| {
                let s: Result<borrow::StyleSheet, _> =
                    serde_json::from_str(std::str::from_utf8_unchecked(s));
                match s {
                    Ok(ss) => ss.into_sheet(),
                    Err(err) => Err(format!(
                        "Failed to deserialize json formatted style sheet: {err}"
                    )),
                }
            },
            drop_callback,
        )
    }
}

/// A fully-parsed style sheet file.
///
/// A linked style sheet has a `scope` attached.
/// The scope can be used in style queries, to limit the style sheets which can be matched in the queries.
#[allow(clippy::type_complexity)]
#[derive(Debug, Clone)]
pub struct LinkedStyleSheet {
    sheets: Vec<(Rc<RefCell<StyleSheet>>, Option<Rc<Media>>)>,
    scope: Option<NonZeroUsize>,
}

impl LinkedStyleSheet {
    /// Create an empty style sheet file with no scope limits.
    pub fn new_empty() -> Self {
        let mut ss = CompiledStyleSheet::new();
        ss.link(&StyleSheetResource::new(), None).0
    }

    /// Get the scope of the style sheet file.
    pub fn scope(&self) -> Option<NonZeroUsize> {
        self.scope
    }

    /// Get all style sheets.
    ///
    /// A style sheet file can contain several `StyleSheet`.
    /// If the file has no `@import`, it has only one `StyleSheet`.
    /// Otherwise, other `StyleSheet` will be imported.
    ///
    /// All `StyleSheet`s are ordered based on the imported order.
    pub fn sheets(&self) -> Vec<Rc<RefCell<StyleSheet>>> {
        self.sheets.iter().map(|x| x.0.clone()).collect::<Vec<_>>()
    }

    #[doc(hidden)]
    pub fn rules_count(&self, sheet_index: Option<usize>) -> Option<u32> {
        if let Some(idx) = sheet_index {
            if self.sheets.len() > (idx + 1usize) {
                return None;
            }
            return Some(self.sheets.get(idx).unwrap().0.borrow().rules_count());
        }
        let mut count = 0;
        self.sheets.iter().for_each(|item| {
            count += item.0.borrow().rules_count();
        });
        Some(count)
    }

    /// Parse style sheet source to a style sheet directly.
    ///
    /// All `@import`s are ignored.
    /// It is a convinient way if there is no `@import` in the source.
    pub fn parse(source: &str, scope: Option<NonZeroUsize>) -> (Self, Vec<Warning>) {
        let (mut ss, mut warnings) = parser::parse_style_sheet("", source);
        let (ret, mut w2) = ss.link(&StyleSheetResource::new(), scope);
        warnings.append(&mut w2);
        (ret, warnings)
    }

    /// Get a rule by index.
    pub fn get_rule(&self, mut rule_index: u32) -> Option<Rc<Rule>> {
        for (sheet, _media) in self.sheets.iter() {
            let sheet = sheet.borrow();
            if rule_index < sheet.rules_count() {
                return sheet.get_rule(rule_index).cloned();
            }
            rule_index -= sheet.rules_count();
        }
        None
    }

    /// Append a new rule.
    ///
    /// Generally it is used for debugging.
    /// Re-query is needed when the style sheet is updated.
    pub fn add_rule(&mut self, rule: Box<Rule>) -> u32 {
        let mut rule_index = 0;
        for (sheet, _media) in self.sheets.iter() {
            rule_index += sheet.borrow().rules_count();
        }
        self.sheets
            .last_mut()
            .unwrap()
            .0
            .borrow_mut()
            .add_rule(rule);
        rule_index
    }

    /// Replace an existing rule with a new rule.
    ///
    /// The new rule is returned if success.
    /// Generally it is used for debugging.
    /// Re-query is needed when the style sheet is updated.
    pub fn replace_rule(
        &mut self,
        mut rule_index: u32,
        rule: Box<Rule>,
    ) -> Result<Rc<Rule>, Box<Rule>> {
        for (sheet, _media) in self.sheets.iter_mut() {
            let mut sheet = sheet.borrow_mut();
            if rule_index < sheet.rules_count() {
                return sheet.replace_rule(rule_index, rule);
            }
            rule_index -= sheet.rules_count();
        }
        Err(rule)
    }

    pub(crate) fn for_each_matched_rule<L: LengthNum, T: StyleNode>(
        &self,
        query: &[T],
        media_query_status: &MediaQueryStatus<L>,
        sheet_index: u16,
        mut f: impl FnMut(MatchedRuleRef),
    ) {
        // start from 1, so that computed weight of Matched rules is always non-zero
        let mut rule_index_offset = 1;
        for (sheet, media) in self.sheets.iter() {
            if let Some(media) = media {
                if !media.is_valid(media_query_status) {
                    continue;
                }
            }
            sheet.borrow_mut().for_each_matched_rule(
                query,
                media_query_status,
                self.scope,
                sheet_index,
                rule_index_offset,
                &mut f,
            );
            rule_index_offset += sheet.borrow().rules_count();
        }
    }

    pub(crate) fn search_keyframes<L: LengthNum>(
        &self,
        style_scope: Option<NonZeroUsize>,
        name: &str,
        media_query_status: &MediaQueryStatus<L>,
    ) -> Option<Rc<KeyFrames>> {
        if self.scope.is_some() && self.scope != style_scope {
            return None;
        }
        for (sheet, media) in self.sheets.iter().rev() {
            if let Some(media) = media {
                if !media.is_valid(media_query_status) {
                    continue;
                }
            }
            // TODO consider build a hashmap index
            for (k, media) in sheet.borrow().index.keyframes.iter().rev() {
                if k.ident.as_str() == name
                    && media
                        .as_ref()
                        .is_none_or(|media| media.is_valid(media_query_status))
                {
                    return Some(k.clone());
                }
            }
        }
        None
    }

    /// Get all `@font-face` definitions.
    pub fn get_font_face(&self) -> Vec<Rc<FontFace>> {
        let mut ret = vec![];
        for (sheet, _) in self.sheets.iter() {
            let sheet = sheet.borrow();
            sheet.font_face().iter().for_each(|ff| ret.push(ff.clone()));
        }
        ret
    }

    /// Get font-face definitions whose media conditions match the current environment.
    pub fn get_font_face_with_media<L: LengthNum>(
        &self,
        media_query_status: &MediaQueryStatus<L>,
    ) -> Vec<Rc<FontFace>> {
        let mut ret = vec![];
        for (sheet, media) in &self.sheets {
            if media
                .as_ref()
                .is_some_and(|media| !media.is_valid(media_query_status))
            {
                continue;
            }
            let sheet = sheet.borrow();
            for (font_face, media) in sheet
                .index
                .font_face
                .iter()
                .zip(&sheet.index.font_face_media)
            {
                if media
                    .as_ref()
                    .is_none_or(|media| media.is_valid(media_query_status))
                {
                    ret.push(font_face.clone());
                }
            }
        }
        ret
    }
}

/// A style sheet body with grouped CSS rules.
#[derive(Clone)]
pub struct StyleSheet {
    rules: Vec<CssRule>,
    index: StyleSheetIndex,
}

#[derive(Clone, Default)]
struct StyleSheetIndex {
    rules: Vec<Rc<Rule>>,
    font_face: Vec<Rc<FontFace>>,
    font_face_media: Vec<Option<Rc<Media>>>,
    keyframes: Vec<(Rc<KeyFrames>, Option<Rc<Media>>)>,
    imports: Vec<Rc<ImportRule>>,
    classes: ClassIndex,
}

#[derive(Clone, Default)]
enum ClassIndex {
    #[default]
    NeedUpdate,
    Updated {
        class_index: HashMap<String, Vec<Rc<Rule>>>,
        class_unindexed: Vec<Rc<Rule>>,
    },
}

impl StyleSheetIndex {
    fn from_rules(rules: &mut [CssRule]) -> Self {
        let mut index = Self::default();
        let mut stack = vec![(rules.iter_mut(), None::<Rc<Media>>)];
        while let Some((iter, parent)) = stack.last_mut() {
            match iter.next() {
                Some(CssRule::Style(rule)) => {
                    let media_matches = match (&rule.media, &*parent) {
                        (Some(media), Some(parent)) => Rc::ptr_eq(media, parent),
                        (None, None) => true,
                        _ => false,
                    };
                    let rule_index = index.rules.len() as u32;
                    if rule.index != rule_index || !media_matches {
                        let rule = Rc::make_mut(rule);
                        rule.index = rule_index;
                        rule.media = parent.clone();
                    }
                    index.rules.push(rule.clone());
                }
                Some(CssRule::Media(rule)) => {
                    let rule = Rc::make_mut(rule);
                    stack.push((rule.rules.iter_mut(), Some(rule.media.clone())));
                }
                Some(CssRule::KeyFrames(rule)) => {
                    index.keyframes.push((rule.clone(), parent.clone()))
                }
                Some(CssRule::FontFace(rule)) => {
                    index.font_face.push(rule.clone());
                    index.font_face_media.push(parent.clone());
                }
                Some(CssRule::Import(rule)) => index.imports.push(rule.clone()),
                None => {
                    stack.pop();
                }
            }
        }
        index
    }
}

impl core::fmt::Debug for StyleSheet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StyleSheet {{")?;
        for rule in self.index.rules.iter() {
            write!(f, " {rule:?}")?;
        }
        for font_face in self.index.font_face.iter() {
            write!(f, " {font_face:?}")?;
        }
        for (keyframes, _) in self.index.keyframes.iter() {
            write!(f, " {keyframes:?}")?;
        }
        write!(f, " }}")
    }
}

impl core::fmt::Display for StyleSheet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn fmt_rules(rules: &[CssRule], f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for rule in rules {
                match rule {
                    CssRule::Style(rule) => {
                        write!(f, " ")?;
                        rule.fmt_body(f)?;
                    }
                    CssRule::Media(rule) => {
                        let queries = rule
                            .media
                            .media_queries
                            .iter()
                            .map(|query| query.to_media_query_string())
                            .collect::<Box<[_]>>()
                            .join(", ");
                        write!(f, " @media {queries} {{")?;
                        fmt_rules(&rule.rules, f)?;
                        write!(f, " }}")?;
                    }
                    CssRule::FontFace(rule) => write!(f, " {rule}")?,
                    CssRule::KeyFrames(rule) => write!(f, " {rule}")?,
                    CssRule::Import(_) => {}
                }
            }
            Ok(())
        }
        fmt_rules(&self.rules, f)
    }
}

impl StyleSheet {
    fn from_rules(mut rules: Vec<CssRule>) -> Self {
        let index = StyleSheetIndex::from_rules(&mut rules);
        Self { rules, index }
    }

    fn rebuild_index(&mut self) {
        self.index = StyleSheetIndex::default();
        self.index = StyleSheetIndex::from_rules(&mut self.rules);
    }

    #[doc(hidden)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> LinkedStyleSheet {
        let (mut ss, warnings) = parser::parse_style_sheet("", s);
        for warning in warnings {
            warn!("{:?}", warning);
        }
        let ret = ss.link(&StyleSheetResource::new(), None);
        ret.0
    }

    #[doc(hidden)]
    pub fn from_str_with_path(path: &str, s: &str) -> LinkedStyleSheet {
        let (mut ss, warnings) = parser::parse_style_sheet(path, s);
        for warning in warnings {
            warn!("{:?}", warning);
        }
        let ret = ss.link(&StyleSheetResource::new(), None);
        ret.0
    }

    fn rules_count(&self) -> u32 {
        self.index.rules.len().min(u32::MAX as usize) as u32
    }

    fn get_rule(&self, rule_index: u32) -> Option<&Rc<Rule>> {
        self.index.rules.get(rule_index as usize)
    }

    fn add_rule(&mut self, mut rule: Box<Rule>) {
        rule.index = self.index.rules.len() as u32;
        self.rules.push(wrap_style_rule(Rc::from(rule)));
        self.rebuild_index();
    }

    fn replace_rule(
        &mut self,
        rule_index: u32,
        mut rule: Box<Rule>,
    ) -> Result<Rc<Rule>, Box<Rule>> {
        if rule_index as usize >= self.index.rules.len() {
            return Err(rule);
        }
        rule.index = rule_index;
        let (following, old_rule) = split_style_rule(&mut self.rules, rule_index).unwrap();
        self.rules.push(wrap_style_rule(Rc::from(rule)));
        self.rules.extend(following);
        self.rebuild_index();
        Ok(old_rule)
    }

    fn update_index(&mut self) {
        if let ClassIndex::NeedUpdate = &self.index.classes {
            let mut class_index: HashMap<String, Vec<Rc<Rule>>> = HashMap::default();
            let mut class_unindexed = vec![];
            for rule in self.index.rules.iter() {
                let index_classes = rule.selector.get_index_classes();
                for c in index_classes {
                    if !c.is_empty() {
                        let c = class_index.entry(c).or_default();
                        c.push(rule.clone());
                    } else {
                        class_unindexed.push(rule.clone());
                    }
                }
            }
            self.index.classes = ClassIndex::Updated {
                class_index,
                class_unindexed,
            };
        }
    }

    fn for_each_matched_rule<L: LengthNum, T: StyleNode>(
        &mut self,
        query: &[T],
        media_query_status: &MediaQueryStatus<L>,
        sheet_style_scope: Option<NonZeroUsize>,
        sheet_index: u16,
        rule_index_offset: u32,
        mut f: impl FnMut(MatchedRuleRef),
    ) {
        self.update_index();
        if let ClassIndex::Updated {
            class_index,
            class_unindexed,
        } = &self.index.classes
        {
            if sheet_style_scope.is_none()
                || query
                    .last()
                    .is_some_and(|x| x.contain_scope(sheet_style_scope))
            {
                for r in class_unindexed.iter() {
                    if let Some(selector_weight) =
                        r.match_query(query, media_query_status, sheet_style_scope)
                    {
                        let weight = RuleWeight::new(
                            selector_weight,
                            sheet_index,
                            rule_index_offset + r.index,
                        );
                        f(MatchedRuleRef { rule: r, weight });
                    }
                }
            }
            let query_last = match query.last() {
                Some(x) => x,
                None => return,
            };
            for class in query_last.classes() {
                if sheet_style_scope.is_none() || sheet_style_scope == class.scope() {
                    if let Some(rules) = class_index.get(class.name()) {
                        for r in rules {
                            if let Some(selector_weight) =
                                r.match_query(query, media_query_status, sheet_style_scope)
                            {
                                let weight = RuleWeight::new(
                                    selector_weight,
                                    sheet_index,
                                    rule_index_offset + r.index,
                                );
                                f(MatchedRuleRef { rule: r, weight });
                            }
                        }
                    }
                }
            }
        }
    }

    /// Add a font-face definition to the style sheet.
    pub fn add_font_face(&mut self, ff: FontFace) {
        self.rules.push(CssRule::FontFace(Rc::new(ff)));
        self.rebuild_index();
    }

    /// Get all font-face definitions.
    pub fn font_face(&self) -> &[Rc<FontFace>] {
        &self.index.font_face
    }
}

/// The weight of a rule (unique for each rule).
///
/// Weight of a rule is composed of multiple factors.
///
/// * High 16 bits is for the selector, while the detailed layout is `-MICCCCCCCCP--TT`:
///   * `M` - the important bit;
///   * `I` - the ID selector bit;
///   * `C` - the sum of the class selectors and the attribute selectors (max 255);
///   * `P` - the pseudo class bit;
///   * `T` - the sum of the tag name selector and the pseudo element selector.
/// * High 16th~31st bits is the style sheet index (0-based index).
/// * High 32nd~63rd bits is the rule index in the whole linked style sheet (1-based index).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleWeight(u64);

impl RuleWeight {
    pub(crate) fn new(selector_weight: u16, sheet_index: u16, rule_index: u32) -> Self {
        let weight =
            ((selector_weight as u64) << 48) + ((sheet_index as u64) << 32) + rule_index as u64;
        Self(weight)
    }

    pub(crate) fn inline() -> Self {
        Self(1 << 62)
    }

    /// Get the underlying weight number.
    pub fn normal(&self) -> u64 {
        self.0
    }

    /// Get the underlying weight number with `!important` added.
    pub fn important(&self) -> u64 {
        self.0 + (1 << 63)
    }

    /// Get the style sheet index.
    pub fn sheet_index(&self) -> u16 {
        (self.0 >> 32) as u16
    }

    /// Get the rule index.
    pub fn rule_index(&self) -> u32 {
        (self.0 as u32) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_retains_media_children_and_empty_groups() {
        let (sheet, warnings) = parser::parse_style_sheet(
            "tree",
            r#"
            @media screen {
                @font-face { font-family: Nested; src: local(Nested); }
                @keyframes nested { to { opacity: 1; } }
                @media (min-width: 300px) {}
                .inside { width: 1px; }
            }
            .outside { width: 2px; }
            "#,
        );
        assert!(warnings.is_empty());
        let decoded = CompiledStyleSheet::deserialize_bincode(sheet.serialize_bincode()).unwrap();
        for sheet in [sheet, decoded] {
            let sheet = sheet.ss.borrow();
            let CssRule::Media(media) = &sheet.rules[0] else {
                panic!("media group missing");
            };
            assert!(matches!(media.rules[0], CssRule::FontFace(_)));
            assert!(matches!(media.rules[1], CssRule::KeyFrames(_)));
            let CssRule::Media(empty) = &media.rules[2] else {
                panic!("empty media group missing");
            };
            assert!(empty.rules.is_empty());
            assert!(Rc::ptr_eq(
                empty.media.parent.as_ref().unwrap(),
                &media.media
            ));
            let CssRule::Style(style) = &media.rules[3] else {
                panic!("style rule missing");
            };
            assert!(Rc::ptr_eq(style, &sheet.index.rules[0]));
            assert!(Rc::ptr_eq(style.media.as_ref().unwrap(), &media.media));
            assert_eq!(sheet.index.rules.len(), 2);
            assert_eq!(sheet.index.font_face.len(), 1);
            assert_eq!(sheet.index.keyframes.len(), 1);
        }
    }

    #[test]
    fn media_group_depth_is_bounded() {
        for depth in [MAX_RULE_NESTING_DEPTH, MAX_RULE_NESTING_DEPTH + 1] {
            let source = format!(
                "{}@keyframes deep {{ to {{ opacity: 1; }} }}{}.outside {{ width: 1px; }}",
                "@media screen {".repeat(depth),
                "}".repeat(depth)
            );
            let (mut sheet, warnings) = parser::parse_style_sheet("depth", &source);
            assert_eq!(warnings.len(), usize::from(depth > MAX_RULE_NESTING_DEPTH));
            let bytes = sheet.serialize_bincode();
            let mut decoded = CompiledStyleSheet::deserialize_bincode(bytes).unwrap();
            for sheet in [&mut sheet, &mut decoded] {
                let (linked, _) = sheet.link(&StyleSheetResource::new(), None);
                assert_eq!(linked.rules_count(None), Some(1));
                assert_eq!(
                    linked
                        .search_keyframes(None, "deep", &MediaQueryStatus::<f32>::default_screen())
                        .is_some(),
                    depth == MAX_RULE_NESTING_DEPTH
                );
            }
            let parts = core::iter::repeat("screen").take(depth);
            assert_eq!(
                Rule::from_parts_str(parts, ".deep").is_ok(),
                depth == MAX_RULE_NESTING_DEPTH
            );
        }
    }

    #[test]
    fn editing_cloned_sheet_does_not_change_original() {
        let (sheet, _) = parser::parse_style_sheet(
            "clone",
            "@media screen { .a { width: 1px; } @media (min-width: 300px) { .b { width: 2px; } } }",
        );
        let original = sheet.ss.borrow().clone();
        let mut edited = original.clone();
        let replacement = edited
            .get_rule(1)
            .unwrap()
            .modify_selector(".renamed")
            .unwrap();
        edited.replace_rule(1, replacement).unwrap();
        edited.add_rule(Rule::new_empty());
        assert_eq!(original.rules_count(), 2);
        assert_eq!(original.get_rule(1).unwrap().get_selector_string(), ".b");
        assert_eq!(edited.rules_count(), 3);
        assert_eq!(
            edited.get_rule(1).unwrap().get_selector_string(),
            ".renamed"
        );
        assert_eq!(
            edited.get_rule(1).unwrap().get_media_query_string_list(),
            original.get_rule(1).unwrap().get_media_query_string_list()
        );
    }
}
