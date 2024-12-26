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
pub(crate) use selector::{
    Attribute, AttributeFlags, AttributeOperator, PseudoClasses, PseudoElements, Selector,
    SelectorFragment, SelectorRelationType, SELECTOR_WHITESPACE,
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

#[derive(Debug, Clone)]
pub(crate) struct CompiledStyleSheet {
    imports: Vec<(String, Option<Rc<Media>>)>,
    linked: bool,
    ss: Rc<RefCell<StyleSheet>>,
}

impl CompiledStyleSheet {
    pub(crate) fn new() -> Self {
        Self {
            imports: Vec::with_capacity(0),
            linked: false,
            ss: Rc::new(RefCell::new(StyleSheet {
                rules: vec![],
                index: StyleSheetIndex::NeedUpdate,
                font_face: vec![],
                keyframes: vec![],
            })),
        }
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn new_with_config(
        imports: Vec<(String, Option<Rc<Media>>)>,
        rules: Vec<Rc<Rule>>,
        font_face: Vec<Rc<FontFace>>,
        keyframes: Vec<Rc<KeyFrames>>,
    ) -> Self {
        Self {
            imports,
            linked: false,
            ss: Rc::new(RefCell::new(StyleSheet {
                rules,
                index: StyleSheetIndex::NeedUpdate,
                font_face,
                keyframes,
            })),
        }
    }

    pub(crate) fn list_deps(&self) -> Vec<String> {
        self.imports.iter().map(|(s, _)| s.clone()).collect()
    }

    pub(crate) fn add_import(&mut self, path: String, media: Option<Rc<Media>>) {
        self.imports.push((path, media));
    }

    pub(crate) fn add_rule(&mut self, rule: Box<Rule>) {
        self.ss.borrow_mut().add_rule(rule);
    }

    pub(crate) fn add_font_face(&mut self, ff: FontFace) {
        self.ss.borrow_mut().add_font_face(ff)
    }

    pub(crate) fn add_keyframes(&mut self, keyframes: KeyFrames) {
        self.ss.borrow_mut().add_keyframes(keyframes)
    }

    pub(crate) fn add_tag_name_prefix(&mut self, prefix: &str) {
        let mut ss = self.ss.borrow_mut();
        for rule in ss.rules.iter_mut() {
            let rule = Rc::make_mut(rule);
            for frag in rule.selector.fragments.iter_mut() {
                frag.add_tag_name_prefix(prefix)
            }
        }
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
        for (target_path, media) in self.imports.iter() {
            if let Some(target) = res.refs.get(target_path) {
                if let Ok(mut target) = target.try_borrow_mut() {
                    let m = match media.clone() {
                        None => parent_media.clone(),
                        Some(mut m) => {
                            Rc::make_mut(&mut m).parent = parent_media.clone();
                            Some(m)
                        }
                    };
                    target.link_self(res, sheets, m, warnings);
                } else {
                    warnings.push(Warning {
                        kind: WarningKind::RecursiveImports,
                        message: format!(
                            "detected recursive style sheet import for {:?}",
                            target_path
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
                    message: format!(r#"target style sheet {:?} not found"#, target_path).into(),
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
            Ok(ss) => Ok(ss.into_sheet()),
            Err(err) => Err(format!(
                "Failed to deserialize bincode formatted style sheet: {}",
                err
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
                    Ok(ss) => Ok(ss.into_sheet()),
                    Err(err) => Err(format!(
                        "Failed to deserialize bincode formatted style sheet: {}",
                        err
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
            Ok(ss) => Ok(ss.into_sheet()),
            Err(err) => Err(format!(
                "Failed to deserialize json formatted style sheet: {}",
                err
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
                    Ok(ss) => Ok(ss.into_sheet()),
                    Err(err) => Err(format!(
                        "Failed to deserialize json formatted style sheet: {}",
                        err
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

    pub(crate) fn for_each_matched_rule<L: LengthNum>(
        &self,
        query: &[StyleQuery],
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
        for (sheet, media) in self.sheets.iter() {
            if let Some(media) = media {
                if !media.is_valid(media_query_status) {
                    continue;
                }
            }
            // TODO consider build a hashmap index
            for k in sheet.borrow().keyframes.iter().rev() {
                if k.ident.as_str() == name {
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
}

/// A style sheet body without `@import` information.
#[derive(Clone)]
pub struct StyleSheet {
    rules: Vec<Rc<Rule>>,
    index: StyleSheetIndex,
    font_face: Vec<Rc<FontFace>>,
    keyframes: Vec<Rc<KeyFrames>>,
}

#[derive(Clone)]
enum StyleSheetIndex {
    NeedUpdate,
    Updated {
        class_index: HashMap<String, Vec<Rc<Rule>>>,
        class_unindexed: Vec<Rc<Rule>>,
    },
}

impl core::fmt::Debug for StyleSheet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StyleSheet {{")?;
        for rule in self.rules.iter() {
            write!(f, " {:?}", rule)?;
        }
        for font_face in self.font_face.iter() {
            write!(f, " {:?}", font_face)?;
        }
        for keyframes in self.keyframes.iter() {
            write!(f, " {:?}", keyframes)?;
        }
        write!(f, " }}")
    }
}

impl core::fmt::Display for StyleSheet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for rule in self.rules.iter() {
            write!(f, " {}", rule)?;
        }
        for font_face in self.font_face.iter() {
            write!(f, " {}", font_face)?;
        }
        for keyframes in self.keyframes.iter() {
            write!(f, " {}", keyframes)?;
        }
        Ok(())
    }
}

impl StyleSheet {
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
        self.rules.len().min(u32::MAX as usize) as u32
    }

    fn get_rule(&self, rule_index: u32) -> Option<&Rc<Rule>> {
        self.rules.get(rule_index as usize)
    }

    fn add_rule(&mut self, mut rule: Box<Rule>) {
        rule.index = self.rules.len() as u32;
        self.rules.push(Rc::from(rule));
        self.index = StyleSheetIndex::NeedUpdate;
    }

    fn replace_rule(
        &mut self,
        rule_index: u32,
        mut rule: Box<Rule>,
    ) -> Result<Rc<Rule>, Box<Rule>> {
        let index = rule_index as usize;
        if index < self.rules.len() {
            rule.index = rule_index;
            let mut rule = Rc::from(rule);
            core::mem::swap(&mut self.rules[index], &mut rule);
            self.index = StyleSheetIndex::NeedUpdate;
            Ok(rule)
        } else {
            Err(rule)
        }
    }

    fn update_index(&mut self) {
        if let StyleSheetIndex::NeedUpdate = &self.index {
            let mut class_index: HashMap<String, Vec<Rc<Rule>>> = HashMap::default();
            let mut class_unindexed = vec![];
            for rule in self.rules.iter() {
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
            self.index = StyleSheetIndex::Updated {
                class_index,
                class_unindexed,
            };
        }
    }

    fn for_each_matched_rule<L: LengthNum>(
        &mut self,
        query: &[StyleQuery],
        media_query_status: &MediaQueryStatus<L>,
        sheet_style_scope: Option<NonZeroUsize>,
        sheet_index: u16,
        rule_index_offset: u32,
        mut f: impl FnMut(MatchedRuleRef),
    ) {
        self.update_index();
        if let StyleSheetIndex::Updated {
            class_index,
            class_unindexed,
        } = &self.index
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
            for (class, scope) in query_last.classes.iter() {
                if sheet_style_scope.is_none() || sheet_style_scope == *scope {
                    if let Some(rules) = class_index.get(class) {
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
        self.font_face.push(Rc::new(ff));
    }

    /// Get all font-face definitions.
    pub fn font_face(&self) -> &[Rc<FontFace>] {
        &self.font_face
    }

    pub(crate) fn add_keyframes(&mut self, keyframes: KeyFrames) {
        self.keyframes.push(Rc::new(keyframes));
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
