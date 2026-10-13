#![doc(hidden)]
#[allow(unused_imports)]
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::ops::Index;

#[cfg(feature = "deserialize")]
use super::rule::PropertyMeta;
use crate::typing::{FontFamilyName, FontStyleType, FontWeightType, ImportantBitSet};
#[cfg(any(feature = "serialize", feature = "deserialize"))]
use bit_set::BitSet;

#[cfg(debug_assertions)]
use float_pigment_css_macro::{CompatibilityEnumCheck, CompatibilityStructCheck};

#[cfg(feature = "serialize")]
use hashbrown::HashMap;
use serde::{de, ser::SerializeTuple, Deserialize, Serialize};

use super::str_store::*;
use super::*;
use crate::sheet;

#[cfg(feature = "deserialize")]
pub(crate) unsafe fn de_static_ref_zero_copy_env<R>(
    buf: *const [u8],
    f: impl FnOnce(&[u8]) -> R,
    drop_callback: impl 'static + FnOnce(),
) -> R {
    SerdeThreadGlobalState::de_prepare(Box::new(drop_callback), || f(&*buf))
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum StyleSheet {
    None,
    V1(StyleSheetV1),
}

#[repr(C)]
#[derive(Debug)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct StyleSheetV1 {
    buf: StrBuffer,
    imports: Array<StrRef>,
    imports_media: Array<Nullable<Media>>,
    rules: Array<Rule>,
    media: Array<Media>,
    pub version: Box<StrRef>,
    font_face: Array<FontFace>,
    keyframes: Array<KeyFrames>,
    /// Grouped rules in source order; absent in legacy bincode data.
    rule_tree: Nullable<Array<RuleTreeNode>>,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum RuleTreeNode {
    Style(usize),
    Media {
        media_index: usize,
        children: Array<RuleTreeNode>,
    },
    KeyFrames(usize),
    FontFace(usize),
    Import(usize),
}

// Keep the existing preorder encoding on the wire.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
enum SerializedRuleTreeNode {
    Style(usize),
    Media(usize, usize),
    KeyFrames(usize),
    FontFace(usize),
    Import(usize),
}

#[cfg(debug_assertions)]
impl crate::CompatibilityCheck for RuleTreeNode {
    fn check() {
        <SerializedRuleTreeNode as crate::CompatibilityCheck>::check();
    }
}

impl StyleSheetV1 {
    fn validate(&self, tree: Option<&[SerializedRuleTreeNode]>) -> Result<(), String> {
        fn index(value: &Nullable<usize>) -> Option<usize> {
            match value {
                Nullable::None => None,
                Nullable::Some(index) => Some(*index),
            }
        }
        let sheet = self;
        let mut depths: Vec<usize> = Vec::with_capacity(sheet.media.len());
        for media in sheet.media.iter() {
            let depth = match index(&media.parent_index) {
                None => 1,
                Some(parent) => depths
                    .get(parent)
                    .map(|depth| depth + 1)
                    .ok_or_else(|| "Invalid media parent index".to_string())?,
            };
            if depth > MAX_RULE_NESTING_DEPTH {
                return Err("Media nesting limit exceeded".into());
            }
            depths.push(depth);
        }
        if sheet.imports.len() != sheet.imports_media.len() {
            return Err("Invalid import media list".into());
        }
        for import in sheet.imports_media.iter() {
            if let Nullable::Some(media) = import {
                if index(&media.parent_index).is_some_and(|parent| parent >= sheet.media.len()) {
                    return Err("Invalid import media parent index".into());
                }
            }
        }
        for rule in sheet.rules.iter() {
            if index(&rule.media_index).is_some_and(|media| media >= sheet.media.len()) {
                return Err("Invalid rule media index".into());
            }
        }
        let Some(tree) = tree else {
            return Ok(());
        };
        let lengths = [
            sheet.rules.len(),
            sheet.font_face.len(),
            sheet.keyframes.len(),
            sheet.imports.len(),
        ];
        let mut next = [0; 4];
        let mut groups: Vec<(usize, usize)> = vec![];
        for (position, node) in tree.iter().enumerate() {
            let parent = groups.last().map(|(media, _)| *media);
            let leaf = match node {
                SerializedRuleTreeNode::Style(rule) => {
                    let rules: &[Rule] = sheet.rules.as_ref();
                    let rule_media = rules
                        .get(*rule)
                        .ok_or_else(|| "Invalid rule tree style index".to_string())?;
                    if index(&rule_media.media_index) != parent {
                        return Err("Invalid style rule media parent".into());
                    }
                    Some((0, *rule))
                }
                SerializedRuleTreeNode::FontFace(rule) => Some((1, *rule)),
                SerializedRuleTreeNode::KeyFrames(rule) => Some((2, *rule)),
                SerializedRuleTreeNode::Import(rule) => {
                    if parent.is_some() {
                        return Err("Import inside a media rule".into());
                    }
                    Some((3, *rule))
                }
                SerializedRuleTreeNode::Media(media, children) => {
                    let media_list: &[Media] = sheet.media.as_ref();
                    let condition = media_list
                        .get(*media)
                        .ok_or_else(|| "Invalid rule tree media index".to_string())?;
                    if index(&condition.parent_index) != parent {
                        return Err("Invalid rule tree media parent".into());
                    }
                    if groups.len() >= MAX_RULE_NESTING_DEPTH
                        || *children > tree.len() - position - 1
                    {
                        return Err("Invalid rule tree nesting".into());
                    }
                    if let Some((_, remaining)) = groups.last_mut() {
                        *remaining -= 1;
                    }
                    groups.push((*media, *children));
                    None
                }
            };
            if let Some((kind, rule)) = leaf {
                if rule != next[kind] || rule >= lengths[kind] {
                    return Err("Invalid rule tree order or index".into());
                }
                next[kind] += 1;
                if let Some((_, remaining)) = groups.last_mut() {
                    *remaining -= 1;
                }
            }
            while groups.last().is_some_and(|(_, remaining)| *remaining == 0) {
                groups.pop();
            }
        }
        if !groups.is_empty() || next != lengths {
            return Err("Incomplete rule tree".into());
        }
        Ok(())
    }
}

fn flatten_rule_tree(tree: &[RuleTreeNode]) -> Vec<SerializedRuleTreeNode> {
    let mut result = vec![];
    let mut stack = vec![tree.iter()];
    while let Some(iter) = stack.last_mut() {
        match iter.next() {
            Some(RuleTreeNode::Style(index)) => result.push(SerializedRuleTreeNode::Style(*index)),
            Some(RuleTreeNode::Media {
                media_index,
                children,
            }) => {
                result.push(SerializedRuleTreeNode::Media(*media_index, children.len()));
                stack.push(children.iter());
            }
            Some(RuleTreeNode::KeyFrames(index)) => {
                result.push(SerializedRuleTreeNode::KeyFrames(*index))
            }
            Some(RuleTreeNode::FontFace(index)) => {
                result.push(SerializedRuleTreeNode::FontFace(*index))
            }
            Some(RuleTreeNode::Import(index)) => {
                result.push(SerializedRuleTreeNode::Import(*index))
            }
            None => {
                stack.pop();
            }
        }
    }
    result
}

fn expand_rule_tree(tree: Vec<SerializedRuleTreeNode>) -> Array<RuleTreeNode> {
    let mut result = vec![];
    let mut groups: Vec<(usize, usize, Vec<RuleTreeNode>)> = vec![];
    for node in tree {
        let rule = match node {
            SerializedRuleTreeNode::Media(media_index, children) => {
                if let Some((_, remaining, _)) = groups.last_mut() {
                    *remaining -= 1;
                }
                groups.push((media_index, children, vec![]));
                None
            }
            SerializedRuleTreeNode::Style(index) => Some(RuleTreeNode::Style(index)),
            SerializedRuleTreeNode::KeyFrames(index) => Some(RuleTreeNode::KeyFrames(index)),
            SerializedRuleTreeNode::FontFace(index) => Some(RuleTreeNode::FontFace(index)),
            SerializedRuleTreeNode::Import(index) => Some(RuleTreeNode::Import(index)),
        };
        if let Some(rule) = rule {
            if let Some((_, remaining, children)) = groups.last_mut() {
                *remaining -= 1;
                children.push(rule);
            } else {
                result.push(rule);
            }
        }
        while groups
            .last()
            .is_some_and(|(_, remaining, _)| *remaining == 0)
        {
            let (media_index, _, children) = groups.pop().unwrap();
            let rule = RuleTreeNode::Media {
                media_index,
                children: children.into(),
            };
            groups
                .last_mut()
                .map_or(&mut result, |(_, _, rules)| rules)
                .push(rule);
        }
    }
    result.into()
}

impl StyleSheet {
    #[cfg(feature = "serialize")]
    pub(crate) fn from_sheet(sheet: &sheet::CompiledStyleSheet) -> Self {
        let sheet = sheet.ss.borrow();
        let index = &sheet.index;
        // collect media into a list
        fn collect_media(
            arr: &mut Vec<Media>,
            index_map: &mut HashMap<*const sheet::Media, usize>,
            media: &Option<Rc<sheet::Media>>,
        ) {
            if let Some(media) = media {
                collect_media(arr, index_map, &media.parent);
                let media: &sheet::Media = media;
                let media_key = media as *const _;
                if !index_map.contains_key(&media_key) {
                    let index = arr.len();
                    arr.push(Media::from_sheet(media, index_map));
                    index_map.insert(media_key, index);
                }
            }
        }
        let mut media_arr = vec![];
        let mut media_index_map = HashMap::default();
        for rule in index.rules.iter() {
            collect_media(&mut media_arr, &mut media_index_map, &rule.media);
        }

        let mut stack = vec![(None, sheet.rules.iter(), vec![])];
        let (mut font_face, mut keyframes, mut import) = (0, 0, 0);
        let rule_tree = loop {
            let (_, iter, result) = stack.last_mut().unwrap();
            match iter.next() {
                Some(CssRule::Style(rule)) => result.push(RuleTreeNode::Style(rule.index as usize)),
                Some(CssRule::Media(rule)) => {
                    collect_media(
                        &mut media_arr,
                        &mut media_index_map,
                        &Some(rule.media.clone()),
                    );
                    let media = media_index_map[&Rc::as_ptr(&rule.media)];
                    stack.push((Some(media), rule.rules.iter(), vec![]));
                }
                Some(CssRule::KeyFrames(_)) => {
                    result.push(RuleTreeNode::KeyFrames(keyframes));
                    keyframes += 1;
                }
                Some(CssRule::FontFace(_)) => {
                    result.push(RuleTreeNode::FontFace(font_face));
                    font_face += 1;
                }
                Some(CssRule::Import(_)) => {
                    result.push(RuleTreeNode::Import(import));
                    import += 1;
                }
                None => {
                    let (media_index, _, children) = stack.pop().unwrap();
                    match media_index {
                        None => break children,
                        Some(media_index) => {
                            stack.last_mut().unwrap().2.push(RuleTreeNode::Media {
                                media_index,
                                children: children.into(),
                            });
                        }
                    }
                }
            }
        };

        let (imports, imports_media) = index
            .imports
            .iter()
            .map(|rule| {
                (
                    StrRef::from(rule.path.clone()),
                    rule.media
                        .as_ref()
                        .map(|media| Media::from_sheet(media, &media_index_map))
                        .into(),
                )
            })
            .unzip::<_, _, Vec<StrRef>, Vec<Nullable<Media>>>();
        let rules = index
            .rules
            .iter()
            .map(|x| Rule::from_sheet(x, &media_index_map))
            .collect::<Box<[_]>>();
        let version = Box::new(String::from(env!("CARGO_PKG_VERSION")).into());
        let font_face = index
            .font_face
            .iter()
            .map(|x| FontFace::from_sheet(x))
            .collect::<Box<[_]>>();
        let keyframes = index
            .keyframes
            .iter()
            .map(|(x, _)| KeyFrames::from_sheet(x))
            .collect::<Box<[_]>>();
        let mut str_store = StrBuffer::new();
        str_store.freeze();
        Self::V1(StyleSheetV1 {
            imports: imports.into(),
            imports_media: imports_media.into(),
            rules: rules.into(),
            media: media_arr.into(),
            buf: str_store,
            version,
            font_face: font_face.into(),
            keyframes: keyframes.into(),
            rule_tree: Nullable::Some(rule_tree.into()),
        })
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn into_sheet(self) -> Result<sheet::CompiledStyleSheet, String> {
        match self {
            Self::None => Ok(sheet::CompiledStyleSheet::new()),
            Self::V1(StyleSheetV1 {
                buf: _,
                imports,
                imports_media,
                rules,
                media,
                version: _,
                font_face,
                keyframes,
                rule_tree,
            }) => {
                let mut media_vec = Vec::with_capacity(media.arr.len());
                for m in media.into_iter() {
                    let m = m.into_sheet(&media_vec);
                    media_vec.push(m);
                }
                let imports = imports
                    .into_iter()
                    .zip(imports_media)
                    .map(|(s, media)| (s.to_string(), media.map(|x| x.into_sheet(&media_vec))))
                    .collect();
                let rules = rules
                    .into_iter()
                    .enumerate()
                    .map(|(index, x)| x.into_sheet(&media_vec, index))
                    .collect();
                let font_face: Vec<_> = font_face.into_iter().map(|ff| ff.into_sheet()).collect();
                let keyframes: Vec<_> = keyframes.into_iter().map(|kf| kf.into_sheet()).collect();
                match rule_tree {
                    Nullable::None => Ok(sheet::CompiledStyleSheet::new_with_config(
                        imports, rules, font_face, keyframes,
                    )),
                    Nullable::Some(tree) => {
                        Ok(sheet::CompiledStyleSheet::from_rules(restore_rule_tree(
                            tree.into_vec(),
                            imports,
                            rules,
                            font_face,
                            keyframes,
                            media_vec,
                        )))
                    }
                }
            }
        }
    }
}

#[cfg(feature = "deserialize")]
fn restore_rule_tree(
    tree: Vec<RuleTreeNode>,
    imports: Vec<(String, Option<Rc<sheet::Media>>)>,
    rules: Vec<Rc<sheet::Rule>>,
    font_face: Vec<Rc<sheet::FontFace>>,
    keyframes: Vec<Rc<sheet::KeyFrames>>,
    media: Vec<Rc<sheet::Media>>,
) -> Vec<CssRule> {
    let mut imports = imports
        .into_iter()
        .map(|(path, media)| Rc::new(ImportRule { path, media }));
    let mut rules = rules.into_iter();
    let mut font_face = font_face.into_iter();
    let mut keyframes = keyframes.into_iter();
    let mut stack = vec![(None, tree.into_iter(), vec![])];
    loop {
        let (_, iter, result) = stack.last_mut().unwrap();
        match iter.next() {
            Some(RuleTreeNode::Media {
                media_index,
                children,
            }) => stack.push((
                Some(media[media_index].clone()),
                children.into_iter(),
                vec![],
            )),
            Some(RuleTreeNode::Style(_)) => result.push(CssRule::Style(rules.next().unwrap())),
            Some(RuleTreeNode::KeyFrames(_)) => {
                result.push(CssRule::KeyFrames(keyframes.next().unwrap()))
            }
            Some(RuleTreeNode::FontFace(_)) => {
                result.push(CssRule::FontFace(font_face.next().unwrap()))
            }
            Some(RuleTreeNode::Import(_)) => result.push(CssRule::Import(imports.next().unwrap())),
            None => {
                let (media, _, rules) = stack.pop().unwrap();
                match media {
                    None => return rules,
                    Some(media) => stack
                        .last_mut()
                        .unwrap()
                        .2
                        .push(CssRule::Media(Rc::new(MediaRule { media, rules }))),
                }
            }
        }
    }
}

impl Serialize for StyleSheetV1 {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tree = match &self.rule_tree {
            Nullable::None => None,
            Nullable::Some(tree) => Some(flatten_rule_tree(tree.arr.as_ref())),
        };
        self.serialize_with_rules(ser, tree.as_deref())
    }
}

impl StyleSheetV1 {
    fn serialize_with_rules<S>(
        &self,
        ser: S,
        tree: Option<&[SerializedRuleTreeNode]>,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self {
            buf: _,
            imports,
            imports_media,
            rules,
            media,
            version,
            font_face,
            keyframes,
            rule_tree: _,
        } = self;
        str_buffer_ser_env(
            || {
                float_pigment_consistent_bincode::serialized_size(&(
                    imports,
                    imports_media,
                    rules,
                    media,
                    version,
                    font_face,
                    keyframes,
                ))
            },
            |r, buf| match r {
                Ok(_) => {
                    let mut seq = ser.serialize_tuple(if tree.is_some() { 10 } else { 8 })?;
                    seq.serialize_element(buf.whole_buffer())?;
                    seq.serialize_element(&imports)?;
                    seq.serialize_element(&imports_media)?;
                    seq.serialize_element(&rules)?;
                    seq.serialize_element(&media)?;
                    seq.serialize_element(&version)?;
                    seq.serialize_element(&font_face)?;
                    seq.serialize_element(&keyframes)?;
                    if let Some(tree) = tree {
                        seq.serialize_element(&1u8)?;
                        seq.serialize_element(tree)?;
                    }
                    seq.end()
                }
                Err(_) => {
                    use serde::ser::Error;
                    Err(S::Error::custom("Failed preprocessing StrRef"))
                }
            },
        )
    }
}

impl<'de> Deserialize<'de> for StyleSheetV1 {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_style_sheet_v1(de)
    }
}

fn deserialize_style_sheet_v1<'de, D>(de: D) -> Result<StyleSheetV1, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StyleSheetVisitor {
        zero_copy: bool,
    }

    impl<'de> serde::de::Visitor<'de> for StyleSheetVisitor {
        type Value = StyleSheetV1;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(formatter, "StyleSheet")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let buf = deserialize_str_buffer(&mut seq, self.zero_copy)?;
            let (imports, imports_media, rules, media, version, font_face, keyframes) =
                str_buffer_de_env(&buf, || {
                    let imports = seq
                        .next_element::<Array<_>>()?
                        .ok_or_else(|| de::Error::invalid_length(1, &"StyleSheet"))?;
                    let imports_media = seq
                        .next_element::<Array<_>>()?
                        .ok_or_else(|| de::Error::invalid_length(2, &"StyleSheet"))?;
                    let rules = seq
                        .next_element::<Array<Rule>>()?
                        .ok_or_else(|| de::Error::invalid_length(3, &"StyleSheet"))?;
                    let media = seq
                        .next_element::<Array<Media>>()?
                        .ok_or_else(|| de::Error::invalid_length(4, &"StyleSheet"))?;
                    let version = seq
                        .next_element::<_>()
                        .unwrap_or_default()
                        .unwrap_or_default();
                    let font_face = seq
                        .next_element::<Array<FontFace>>()
                        .unwrap_or_default()
                        .unwrap_or_else(|| vec![].into());
                    let keyframes = seq
                        .next_element::<Array<KeyFrames>>()
                        .unwrap_or_default()
                        .unwrap_or_else(|| vec![].into());
                    Ok((
                        imports,
                        imports_media,
                        rules,
                        media,
                        version,
                        font_face,
                        keyframes,
                    ))
                })?;
            let tree = match seq.next_element::<u8>()? {
                None => None,
                Some(1) => Some(
                    seq.next_element::<Vec<SerializedRuleTreeNode>>()?
                        .ok_or_else(|| de::Error::invalid_length(9, &"rule tree"))?,
                ),
                Some(_) => return Err(de::Error::custom("Unsupported rule tree version")),
            };
            let mut sheet = StyleSheetV1 {
                buf,
                imports,
                imports_media,
                rules,
                media,
                version,
                font_face,
                keyframes,
                rule_tree: Nullable::None,
            };
            sheet.validate(tree.as_deref()).map_err(de::Error::custom)?;
            if let Some(tree) = tree {
                sheet.rule_tree = Nullable::Some(expand_rule_tree(tree));
            }
            Ok(sheet)
        }
    }

    let visitor = StyleSheetVisitor {
        zero_copy: !de.is_human_readable(),
    };
    de.deserialize_tuple(10, visitor)
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct Rule {
    selector: Selector,
    properties: Array<Property>,
    media_index: Nullable<usize>,
    important: ImportantBitSet,
}

impl Rule {
    #[cfg(feature = "serialize")]
    fn from_sheet(
        rule: &sheet::Rule,
        media_index_map: &HashMap<*const sheet::Media, usize>,
    ) -> Self {
        let rule = rule.clone();
        // let mut important = vec![];
        let mut important = BitSet::new();
        let mut bs_empty = true;
        let properties: Vec<Property> = rule
            .properties
            .into_iter()
            .enumerate()
            .map(|(index, x)| match x {
                PropertyMeta::Normal { property } => property,
                PropertyMeta::Important { property } => {
                    important.insert(index);
                    bs_empty = false;
                    property
                }
                PropertyMeta::DebugGroup { .. } => Property::Unknown,
            })
            .collect();
        let important: ImportantBitSet = if bs_empty {
            ImportantBitSet::None
        } else {
            let arr = important.into_bit_vec().to_bytes().into();
            ImportantBitSet::Array(arr)
        };
        Self {
            selector: Selector::from_sheet(rule.selector),
            properties: properties.into(),
            media_index: match &rule.media {
                Some(parent) => {
                    let p: &sheet::Media = parent;
                    media_index_map.get(&(p as *const _)).cloned().into()
                }
                None => Nullable::None,
            },
            important,
        }
    }

    #[cfg(feature = "deserialize")]
    fn into_sheet(self, media_list: &[Rc<sheet::Media>], index: usize) -> Rc<sheet::Rule> {
        let selector = self.selector.into_sheet();
        let important = match self.important {
            ImportantBitSet::Array(important) => {
                let v = important.into_vec();
                BitSet::from_bytes(&v)
            }
            _ => BitSet::new(),
        };
        let properties = self
            .properties
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(index, property)| {
                let is_important = important.contains(index);
                match is_important {
                    false => PropertyMeta::Normal { property },
                    true => PropertyMeta::Important { property },
                }
            })
            .collect();
        let media = self.media_index.map(|x| media_list[x].clone());
        Rc::from(sheet::Rule::new_with_index(
            selector,
            properties,
            media,
            index as u32,
        ))
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct InlineRule {
    properties: Array<Property>,
    important: ImportantBitSet,
}

impl InlineRule {
    #[allow(dead_code)]
    pub(crate) fn new(properties: Vec<Property>, important: ImportantBitSet) -> Self {
        let important = match important {
            ImportantBitSet::Array(x) => ImportantBitSet::Array(x),
            _ => ImportantBitSet::None,
        };
        Self {
            properties: properties.into(),
            important,
        }
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct Selector {
    fragments: Array<SelectorFragment>,
    max_weight: u16,
}

impl Selector {
    fn from_sheet(sel: sheet::Selector) -> Self {
        Self {
            fragments: sel
                .fragments
                .into_iter()
                .map(SelectorFragment::from_sheet)
                .collect::<Box<_>>()
                .into(),
            max_weight: sel.max_weight,
        }
    }

    #[cfg(feature = "deserialize")]
    fn into_sheet(self) -> sheet::Selector {
        sheet::Selector {
            fragments: self.fragments.into_iter().map(|x| x.into_sheet()).collect(),
            max_weight: self.max_weight,
        }
    }

    pub fn from_string(sel_text: &str) -> Self {
        let sel = sheet::Selector::from_string(sel_text);
        Self::from_sheet(sel)
    }
}

#[repr(C)]
#[derive(Debug, Serialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct SelectorFragment {
    tag_name: StrRef,
    id: StrRef,
    classes: Array<StrRef>,
    relation: SelectorRelationType,
    pseudo_classes: PseudoClassesType,
    pseudo_elements: PseudoElementsType,
    attributes: NullablePtr<Array<AttributeType>>,
}

impl<'de> Deserialize<'de> for SelectorFragment {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SelectorFragmentVisitor;

        impl<'de> serde::de::Visitor<'de> for SelectorFragmentVisitor {
            type Value = SelectorFragment;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "SelectorFragment")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let tag_name = seq
                    .next_element::<_>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &"SelectorFragment"))?;
                let id = seq
                    .next_element::<_>()?
                    .ok_or_else(|| de::Error::invalid_length(1, &"SelectorFragment"))?;
                let classes = seq
                    .next_element::<Array<StrRef>>()?
                    .ok_or_else(|| de::Error::invalid_length(2, &"SelectorFragment"))?;
                let relation = seq
                    .next_element::<_>()?
                    .ok_or_else(|| de::Error::invalid_length(3, &"SelectorFragment"))?;
                let pseudo_classes = seq
                    .next_element::<_>()
                    .unwrap_or_default()
                    .unwrap_or(PseudoClassesType::None);
                let pseudo_elements = seq
                    .next_element::<_>()
                    .unwrap_or_default()
                    .unwrap_or(PseudoElementsType::None);
                let attributes = seq
                    .next_element::<NullablePtr<Array<AttributeType>>>()
                    .unwrap_or_default()
                    .unwrap_or(NullablePtr::None);
                Ok(SelectorFragment {
                    tag_name,
                    id,
                    classes,
                    relation,
                    pseudo_classes,
                    pseudo_elements,
                    attributes,
                })
            }
        }
        const FIELDS: &[&str] = &[
            "tag_name",
            "id",
            "classes",
            "parent",
            "pseudo_classes",
            "pseudo_elements",
            "attributes",
        ];
        de.deserialize_struct("SelectorFragment", FIELDS, SelectorFragmentVisitor)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum SelectorRelationType {
    None,
    #[serde(rename = "_")] // empty slot (leave for compatibilities, should not be used)
    Invalid,
    Ancestor(Box<SelectorFragment>),
    DirectParent(Box<SelectorFragment>),
    NextSibling(Box<SelectorFragment>),
    SubsequentSibling(Box<SelectorFragment>),
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum PseudoClassesType {
    None,
    #[serde(rename = "_")] // empty slot (leave for compatibilities, should not be used)
    Invalid,
    Host,
    FirstChild,
    LastChild,
    Empty,
    NotExpr(Array<SelectorFragment>),
    OnlyChild,
    NthChild(i32, i32, Nullable<Box<Array<SelectorFragment>>>),
    NthOfType(i32, i32),
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum PseudoElementsType {
    None,
    #[serde(rename = "_")] // empty slot (leave for compatibilities, should not be used)
    Invalid,
    Before,
    After,
    Selection,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct AttributeType {
    operator: AttributeOperator,
    case_insensitive: AttributeFlags,
    never_matches: bool,
    name: StrRef,
    value: Nullable<StrRef>,
}

impl SelectorFragment {
    fn from_sheet(frag: sheet::SelectorFragment) -> Self {
        Self {
            tag_name: frag.tag_name.into(),
            id: frag.id.into(),
            classes: frag
                .classes
                .into_iter()
                .map(|x| x.into())
                .collect::<Box<[StrRef]>>()
                .into(),
            relation: match frag.relation {
                None => SelectorRelationType::None,
                Some(x) => match *x {
                    sheet::SelectorRelationType::None => SelectorRelationType::None,
                    sheet::SelectorRelationType::Ancestor(x) => {
                        SelectorRelationType::Ancestor(Box::new(SelectorFragment::from_sheet(x)))
                    }
                    sheet::SelectorRelationType::DirectParent(x) => {
                        SelectorRelationType::DirectParent(Box::new(SelectorFragment::from_sheet(
                            x,
                        )))
                    }
                    sheet::SelectorRelationType::NextSibling(x) => {
                        SelectorRelationType::NextSibling(Box::new(SelectorFragment::from_sheet(x)))
                    }
                    sheet::SelectorRelationType::SubsequentSibling(x) => {
                        SelectorRelationType::SubsequentSibling(Box::new(
                            SelectorFragment::from_sheet(x),
                        ))
                    }
                },
            },
            pseudo_classes: match frag.pseudo_classes {
                None => PseudoClassesType::None,
                Some(x) => match *x {
                    selector::PseudoClasses::Host => PseudoClassesType::Host,
                    selector::PseudoClasses::FirstChild => PseudoClassesType::FirstChild,
                    selector::PseudoClasses::LastChild => PseudoClassesType::LastChild,
                    selector::PseudoClasses::Empty => PseudoClassesType::Empty,
                    selector::PseudoClasses::Not(v) => {
                        let a = v
                            .into_iter()
                            .map(|item| SelectorFragment::from_sheet(item))
                            .collect::<Vec<SelectorFragment>>()
                            .into();
                        PseudoClassesType::NotExpr(a)
                    }
                    selector::PseudoClasses::OnlyChild => PseudoClassesType::OnlyChild,
                    selector::PseudoClasses::NthChild(a, b, selector_list) => {
                        PseudoClassesType::NthChild(
                            a,
                            b,
                            Nullable::from(selector_list.map(|list| {
                                Box::new(
                                    list.into_iter()
                                        .map(|selector| SelectorFragment::from_sheet(selector))
                                        .collect::<Vec<_>>()
                                        .into(),
                                )
                            })),
                        )
                    }
                    selector::PseudoClasses::NthOfType(a, b) => PseudoClassesType::NthOfType(a, b),
                },
            },
            pseudo_elements: match frag.pseudo_elements {
                None => PseudoElementsType::None,
                Some(x) => match *x {
                    selector::PseudoElements::Before => PseudoElementsType::Before,
                    selector::PseudoElements::After => PseudoElementsType::After,
                    selector::PseudoElements::Selection => PseudoElementsType::Selection,
                },
            },
            attributes: frag
                .attributes
                .map(|v| {
                    Box::new(
                        v.into_iter()
                            .map(|attribute| AttributeType {
                                operator: attribute.operator,
                                case_insensitive: attribute.case_insensitive,
                                never_matches: attribute.never_matches,
                                name: attribute.name.into(),
                                value: attribute.value.map(|v| v.into()).into(),
                            })
                            .collect::<Vec<AttributeType>>()
                            .into(),
                    )
                })
                .into(),
        }
    }

    #[cfg(feature = "deserialize")]
    fn into_sheet(self) -> sheet::SelectorFragment {
        let mut frag = match self.relation {
            SelectorRelationType::None | SelectorRelationType::Invalid => {
                sheet::SelectorFragment::new()
            }
            SelectorRelationType::Ancestor(x) => sheet::SelectorFragment::with_relation(
                sheet::SelectorRelationType::Ancestor(x.into_sheet()),
            ),
            SelectorRelationType::DirectParent(x) => sheet::SelectorFragment::with_relation(
                sheet::SelectorRelationType::DirectParent(x.into_sheet()),
            ),
            SelectorRelationType::NextSibling(x) => sheet::SelectorFragment::with_relation(
                sheet::SelectorRelationType::NextSibling(x.into_sheet()),
            ),
            SelectorRelationType::SubsequentSibling(x) => sheet::SelectorFragment::with_relation(
                sheet::SelectorRelationType::SubsequentSibling(x.into_sheet()),
            ),
        };
        frag.set_basics(
            self.tag_name.to_string(),
            self.id.to_string(),
            self.classes.into_iter().map(|x| x.to_string()).collect(),
        );
        match self.pseudo_classes {
            PseudoClassesType::Host => frag.set_pseudo_classes(PseudoClasses::Host),
            PseudoClassesType::FirstChild => frag.set_pseudo_classes(PseudoClasses::FirstChild),
            PseudoClassesType::LastChild => frag.set_pseudo_classes(PseudoClasses::LastChild),
            PseudoClassesType::Empty => frag.set_pseudo_classes(PseudoClasses::Empty),
            PseudoClassesType::NotExpr(a) => frag.set_pseudo_classes({
                let v = a
                    .into_vec()
                    .into_iter()
                    .map(|item| item.into_sheet())
                    .collect();
                PseudoClasses::Not(v)
            }),
            PseudoClassesType::OnlyChild => frag.set_pseudo_classes(PseudoClasses::OnlyChild),
            PseudoClassesType::NthChild(a, b, selector_list) => {
                frag.set_pseudo_classes(PseudoClasses::NthChild(
                    a,
                    b,
                    Option::from(selector_list).map(
                        |list: Box<Array<sheet::borrow::SelectorFragment>>| {
                            Box::new(
                                list.into_iter()
                                    .map(|selector| selector.into_sheet())
                                    .collect(),
                            )
                        },
                    ),
                ))
            }
            PseudoClassesType::NthOfType(a, b) => {
                frag.set_pseudo_classes(PseudoClasses::NthOfType(a, b))
            }
            PseudoClassesType::None | PseudoClassesType::Invalid => {}
        }
        match self.pseudo_elements {
            PseudoElementsType::Before => frag.set_pseudo_elements(PseudoElements::Before),
            PseudoElementsType::After => frag.set_pseudo_elements(PseudoElements::After),
            _ => {}
        }
        if let Some(attributes) = self.attributes.into_option() {
            attributes.into_iter().for_each(|attr| {
                frag.add_attribute(Attribute {
                    operator: attr.operator,
                    case_insensitive: attr.case_insensitive,
                    never_matches: attr.never_matches,
                    name: attr.name.to_string(),
                    value: attr.value.map(|v| v.to_string()),
                })
            });
        }
        frag
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct KeyFrames {
    ident: StrRef,
    keyframes: Array<KeyFrameRule>,
}

impl KeyFrames {
    #[cfg(feature = "serialize")]
    pub(crate) fn from_sheet(keyframes: &keyframes::KeyFrames) -> Self {
        Self {
            ident: keyframes.ident.clone().into(),
            keyframes: keyframes
                .keyframes
                .iter()
                .map(|x| x.clone().into())
                .collect::<Vec<_>>()
                .into(),
        }
    }
    #[cfg(feature = "deserialize")]
    pub(crate) fn into_sheet(self) -> Rc<keyframes::KeyFrames> {
        Rc::new(keyframes::KeyFrames::new(
            self.ident.to_string(),
            self.keyframes
                .into_iter()
                .map(|x: KeyFrameRule| x.into())
                .collect(),
        ))
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub(crate) struct KeyFrameRule {
    keyframe: Array<KeyFrame>,
    properties: Array<Property>,
}

impl From<keyframes::KeyFrameRule> for KeyFrameRule {
    fn from(keyframe_rule: keyframes::KeyFrameRule) -> Self {
        Self {
            keyframe: keyframe_rule.keyframe.into(),
            properties: keyframe_rule
                .properties
                .iter()
                .map(|p| match p.clone() {
                    PropertyMeta::Normal { property } => property,
                    _ => Property::Unknown,
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl From<KeyFrameRule> for keyframes::KeyFrameRule {
    fn from(keyframe_rule: KeyFrameRule) -> keyframes::KeyFrameRule {
        keyframes::KeyFrameRule::new(
            keyframe_rule.keyframe.into_iter().collect(),
            keyframe_rule
                .properties
                .into_iter()
                .map(|x: Property| PropertyMeta::Normal { property: x })
                .collect(),
        )
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct FontFace {
    font_family: FontFamilyName,
    src: Array<FontSrc>,
    font_style: Nullable<FontStyleType>,
    font_weight: Nullable<FontWeightType>,
    font_display: Nullable<FontDisplay>,
}

impl FontFace {
    pub fn from_sheet(ff: &font_face::FontFace) -> Self {
        Self {
            font_family: ff.font_family.clone(),
            src: ff
                .src
                .iter()
                .map(|x| x.clone().into())
                .collect::<Vec<_>>()
                .into(),
            font_style: ff.font_style.clone().into(),
            font_weight: ff.font_weight.clone().into(),
            font_display: ff.font_display.clone().into(),
        }
    }
    #[cfg(feature = "deserialize")]
    pub fn into_sheet(self) -> Rc<font_face::FontFace> {
        let mut ff = font_face::FontFace::new();
        ff.with_font_display(self.font_display.into())
            .with_font_family(self.font_family)
            .with_font_style(self.font_style.into())
            .with_font_weight(self.font_weight.into())
            .with_src(self.src.into_iter().map(|x: FontSrc| x.into()).collect());
        Rc::new(ff)
    }
}

#[repr(C)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum FontSrc {
    Local(FontFamilyName),
    Url(FontUrl),
}

impl From<font_face::FontSrc> for FontSrc {
    fn from(fs: font_face::FontSrc) -> Self {
        match fs {
            font_face::FontSrc::Local(v) => FontSrc::Local(v),
            font_face::FontSrc::Url(v) => FontSrc::Url(FontUrl {
                url: Box::new(StrRef::from(v.url.clone())),
                format: v
                    .format
                    .map(|arr| {
                        arr.iter()
                            .map(|x| StrRef::from(x.clone()))
                            .collect::<Vec<_>>()
                            .into()
                    })
                    .into(),
            }),
        }
    }
}

impl From<FontSrc> for font_face::FontSrc {
    fn from(x: FontSrc) -> font_face::FontSrc {
        match x {
            FontSrc::Local(v) => font_face::FontSrc::Local(v),
            FontSrc::Url(v) => {
                let format_option: Option<_> = v.format.into();
                let format = format_option
                    .map(|arr: Array<_>| arr.iter().map(|x| x.to_string()).collect::<Vec<_>>());
                font_face::FontSrc::Url(font_face::FontUrl {
                    url: v.url.to_string(),
                    format,
                })
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct FontUrl {
    pub(crate) url: Box<StrRef>,
    pub(crate) format: Nullable<Array<StrRef>>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct Media {
    parent_index: Nullable<usize>,
    media_queries: Array<MediaQuery>,
}

impl Media {
    #[cfg(feature = "serialize")]
    fn from_sheet(
        media: &sheet::Media,
        media_index_map: &HashMap<*const sheet::Media, usize>,
    ) -> Self {
        Self {
            parent_index: match &media.parent {
                Some(parent) => {
                    let p: &sheet::Media = parent;
                    media_index_map.get(&(p as *const _)).cloned().into()
                }
                None => Nullable::None,
            },
            media_queries: media
                .media_queries
                .iter()
                .map(MediaQuery::from_sheet)
                .collect::<Box<_>>()
                .into(),
        }
    }

    #[cfg(feature = "deserialize")]
    fn into_sheet(self, list: &[Rc<sheet::Media>]) -> Rc<sheet::Media> {
        let parent = self.parent_index.map(|x| list[x].clone());
        let media_queries = self
            .media_queries
            .into_iter()
            .map(|x| x.into_sheet())
            .collect();
        Rc::new(sheet::Media {
            parent,
            media_queries,
        })
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub(crate) struct MediaQuery {
    decorator: MediaTypeDecorator,
    cond: Array<MediaExpression>,
}

impl MediaQuery {
    #[cfg(feature = "serialize")]
    fn from_sheet(media_query: &sheet::MediaQuery) -> Self {
        Self {
            decorator: media_query.decorator,
            cond: media_query.cond.clone().into(),
        }
    }

    #[cfg(feature = "deserialize")]
    fn into_sheet(self) -> sheet::MediaQuery {
        sheet::MediaQuery {
            decorator: self.decorator,
            cond: self.cond.arr.into(),
        }
    }
}

/// cbindgen:ignore
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Nullable<T> {
    None,
    Some(T),
}

impl<T> Nullable<T> {
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(x) => Some(f(x)),
            Self::None => None,
        }
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(x: Option<T>) -> Self {
        match x {
            Some(x) => Nullable::Some(x),
            None => Nullable::None,
        }
    }
}

impl<T> From<Nullable<T>> for Option<T> {
    fn from(x: Nullable<T>) -> Option<T> {
        match x {
            Nullable::None => None,
            Nullable::Some(x) => Some(x),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum NullablePtr<T> {
    None,
    Some(Box<T>),
}

impl<T> From<Option<Box<T>>> for NullablePtr<T> {
    fn from(x: Option<Box<T>>) -> Self {
        match x {
            Some(x) => NullablePtr::Some(x),
            None => NullablePtr::None,
        }
    }
}

impl<T> NullablePtr<T> {
    #[allow(unused)]
    fn into_option(self) -> Option<Box<T>> {
        match self {
            Self::Some(x) => Some(x),
            Self::None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct Array<T> {
    arr: Box<[T]>,
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(x: Vec<T>) -> Self {
        Self {
            arr: x.into_boxed_slice(),
        }
    }
}

impl<T> From<Box<[T]>> for Array<T> {
    fn from(x: Box<[T]>) -> Self {
        Self { arr: x }
    }
}

impl<T> Array<T> {
    pub fn empty() -> Self {
        Self { arr: Box::new([]) }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.arr.into_vec()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.arr.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.arr.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    pub fn arr(self) -> Box<[T]> {
        self.arr
    }
}

impl<T> AsRef<[T]> for Array<T> {
    fn as_ref(&self) -> &[T] {
        &self.arr
    }
}

impl<T> AsRef<Box<[T]>> for Array<T> {
    fn as_ref(&self) -> &Box<[T]> {
        &self.arr
    }
}

impl<T> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Self { arr: Box::new([]) }
    }
}

#[cfg(all(test, feature = "serialize", feature = "deserialize"))]
mod tests {
    use super::*;
    use float_pigment_consistent_bincode::{DefaultOptions, Options};

    #[derive(Serialize)]
    enum EncodedStyleSheet<'a> {
        None,
        V1(EncodedStyleSheetV1<'a>),
    }

    struct EncodedStyleSheetV1<'a> {
        sheet: &'a StyleSheetV1,
        tree: Option<&'a [SerializedRuleTreeNode]>,
    }

    impl Serialize for EncodedStyleSheetV1<'_> {
        fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.sheet.serialize_with_rules(ser, self.tree)
        }
    }

    #[test]
    fn rule_tree_extension_preserves_legacy_fields() {
        assert_eq!(
            DefaultOptions::new()
                .serialize(&EncodedStyleSheet::None)
                .unwrap(),
            DefaultOptions::new().serialize(&StyleSheet::None).unwrap(),
        );
        let (compiled, warnings) = crate::parser::parse_style_sheet(
            "compat",
            r#"
            @media (min-width: 300px) {
                @keyframes nested { to { opacity: 1; } }
                @font-face { font-family: Nested; src: local(Nested); }
                @media screen {}
            }
            .plain { width: 3px; }
        "#,
        );
        assert!(warnings.is_empty());
        let current = compiled.serialize_bincode();
        let current: StyleSheet = DefaultOptions::new()
            .allow_trailing_bytes()
            .deserialize(&current)
            .unwrap();
        let StyleSheet::V1(current) = current else {
            panic!("V1 body missing");
        };
        assert_eq!(current.rules.len(), 1);
        assert_eq!(current.font_face.len(), 1);
        assert_eq!(current.keyframes.len(), 1);
        assert_eq!(current.media.len(), 2);
        let Nullable::Some(tree) = &current.rule_tree else {
            panic!("rule tree missing");
        };
        assert_eq!(
            tree.arr.as_ref(),
            &[
                RuleTreeNode::Media {
                    media_index: 0,
                    children: vec![
                        RuleTreeNode::KeyFrames(0),
                        RuleTreeNode::FontFace(0),
                        RuleTreeNode::Media {
                            media_index: 1,
                            children: Array::empty(),
                        },
                    ]
                    .into(),
                },
                RuleTreeNode::Style(0),
            ],
        );

        let old = DefaultOptions::new()
            .serialize(&EncodedStyleSheet::V1(EncodedStyleSheetV1 {
                sheet: &current,
                tree: None,
            }))
            .unwrap();
        let restored: StyleSheet = DefaultOptions::new().deserialize(&old).unwrap();
        let StyleSheet::V1(data) = &restored else {
            unreachable!();
        };
        assert!(matches!(data.rule_tree, Nullable::None));
        let mut restored = restored.into_sheet().unwrap();
        let (linked, _) = restored.link(&crate::StyleSheetResource::new(), None);
        assert_eq!(linked.rules_count(None), Some(1));
        let status = crate::MediaQueryStatus::<f32>::default_screen_with_size(100., 600.);
        assert!(linked.search_keyframes(None, "nested", &status).is_some());
        assert_eq!(linked.get_font_face_with_media(&status).len(), 1);
    }

    #[cfg(feature = "ffi")]
    #[test]
    fn ffi_exposes_rule_tree_from_source_and_bincode() {
        use crate::ffi;
        use alloc::ffi::CString;
        use core::ptr::null_mut;

        const SOURCE: &str = r#"
            @import 'base.css' screen;
            .root { width: 1px; }
            @media screen {
                @font-face { font-family: Nested; src: local(Nested); }
                @keyframes anim { to { opacity: 1; } }
                .inside { width: 2px; }
                @media (min-width: 300px) { .nested { width: 3px; } }
                @media (min-height: 200px) {}
            }
            .after { width: 4px; }
            @keyframes anim { to { opacity: 0.5; } }
        "#;

        fn check(sheet: &StyleSheet) {
            let StyleSheet::V1(sheet) = sheet else {
                panic!("V1 body missing");
            };
            assert_eq!(sheet.rules.len(), 4);
            assert_eq!(sheet.media.len(), 3);
            assert_eq!(sheet.font_face.len(), 1);
            assert_eq!(sheet.keyframes.len(), 2);
            assert_eq!(sheet.imports.len(), 1);
            let Nullable::Some(tree) = &sheet.rule_tree else {
                panic!("rule tree missing");
            };
            assert_eq!(
                tree.arr.as_ref(),
                &[
                    RuleTreeNode::Import(0),
                    RuleTreeNode::Style(0),
                    RuleTreeNode::Media {
                        media_index: 0,
                        children: vec![
                            RuleTreeNode::FontFace(0),
                            RuleTreeNode::KeyFrames(0),
                            RuleTreeNode::Style(1),
                            RuleTreeNode::Media {
                                media_index: 1,
                                children: vec![RuleTreeNode::Style(2)].into(),
                            },
                            RuleTreeNode::Media {
                                media_index: 2,
                                children: Array::empty(),
                            },
                        ]
                        .into(),
                    },
                    RuleTreeNode::Style(3),
                    RuleTreeNode::KeyFrames(1),
                ],
            );
        }

        unsafe extern "C" fn release(args: ffi::RawMutPtr) {
            drop(Box::from_raw(args as *mut Vec<u8>));
        }

        unsafe {
            let source = CString::new(SOURCE).unwrap();
            let parsed = ffi::parse_style_sheet_from_string(source.as_ptr());
            assert!(matches!(parsed.err, ffi::FfiErrorCode::None));
            check(&*parsed.value);
            ffi::style_sheet_free(parsed.value);

            let mut bytes = Box::new(crate::compile_style_sheet_to_bincode("tree", SOURCE));
            let buffer_ptr = bytes.as_mut_ptr();
            let buffer_len = bytes.len();
            let args = Box::into_raw(bytes) as ffi::RawMutPtr;
            let index = ffi::style_sheet_import_index_new().value;
            let path = CString::new("tree.css").unwrap();
            let result = ffi::style_sheet_import_index_add_bincode(
                index,
                path.as_ptr(),
                buffer_ptr,
                buffer_len,
                Some(release),
                args,
                null_mut(),
            );
            assert!(matches!(result.err, ffi::FfiErrorCode::None));
            let restored = ffi::style_sheet_import_index_get_style_sheet(index, path.as_ptr());
            assert!(matches!(restored.err, ffi::FfiErrorCode::None));
            check(&*restored.value);
            ffi::style_sheet_import_index_free(index);
        }
    }

    #[test]
    fn invalid_rule_tree_rejects_and_releases_zero_copy_buffer() {
        let (compiled, _) = crate::parser::parse_style_sheet(
            "invalid",
            r#"
            .first { width: 1px; }
            @media screen { @keyframes nested { to { opacity: 1; } } .last { width: 2px; } }
        "#,
        );
        for nodes in [
            vec![],
            vec![SerializedRuleTreeNode::Media(usize::MAX, 0)],
            vec![
                SerializedRuleTreeNode::Style(0),
                SerializedRuleTreeNode::Style(0),
            ],
            vec![
                SerializedRuleTreeNode::Style(0),
                SerializedRuleTreeNode::Media(0, usize::MAX),
            ],
            vec![
                SerializedRuleTreeNode::Style(0),
                SerializedRuleTreeNode::Style(1),
            ],
        ] {
            let data = StyleSheet::from_sheet(&compiled);
            let StyleSheet::V1(body) = &data else {
                unreachable!();
            };
            let bytes = DefaultOptions::new()
                .serialize(&EncodedStyleSheet::V1(EncodedStyleSheetV1 {
                    sheet: body,
                    tree: Some(&nodes),
                }))
                .unwrap();
            assert!(CompiledStyleSheet::deserialize_bincode(bytes.clone()).is_err());
            let ptr = Box::into_raw(bytes.into_boxed_slice());
            let released = Rc::new(core::cell::Cell::new(0));
            let drop_count = released.clone();
            let result = unsafe {
                CompiledStyleSheet::deserialize_bincode_zero_copy(ptr, move || {
                    drop_count.set(drop_count.get() + 1);
                    drop(Box::from_raw(ptr));
                })
            };
            assert!(result.is_err());
            assert_eq!(released.get(), 1);
        }
    }

    #[test]
    fn rule_tree_rejects_overdeep_media_table_and_truncated_data() {
        let (compiled, _) =
            crate::parser::parse_style_sheet("deep", "@keyframes nested { to { opacity: 1; } }");
        let mut data = StyleSheet::from_sheet(&compiled);
        let depth = MAX_RULE_NESTING_DEPTH + 1;
        let StyleSheet::V1(body) = &mut data else {
            unreachable!();
        };
        body.media = (0..depth)
            .map(|index| Media {
                parent_index: index.checked_sub(1).into(),
                media_queries: vec![].into(),
            })
            .collect::<Vec<_>>()
            .into();
        let mut nodes = (0..depth)
            .map(|index| SerializedRuleTreeNode::Media(index, 1))
            .collect::<Vec<_>>();
        nodes.push(SerializedRuleTreeNode::KeyFrames(0));
        let bytes = DefaultOptions::new()
            .serialize(&EncodedStyleSheet::V1(EncodedStyleSheetV1 {
                sheet: body,
                tree: Some(&nodes),
            }))
            .unwrap();
        assert!(CompiledStyleSheet::deserialize_bincode(bytes)
            .unwrap_err()
            .contains("Media nesting limit exceeded"));

        let bytes = compiled.serialize_bincode();
        for removed in [1, 2, 4] {
            assert!(CompiledStyleSheet::deserialize_bincode(
                bytes[..bytes.len() - removed].to_vec()
            )
            .is_err());
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn nested_rule_tree_depth_is_bounded_on_small_stack() {
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let (compiled, _) = crate::parser::parse_style_sheet(
                    "depth",
                    "@keyframes nested { to { opacity: 1; } }",
                );
                for depth in [MAX_RULE_NESTING_DEPTH, MAX_RULE_NESTING_DEPTH + 1] {
                    let mut data = StyleSheet::from_sheet(&compiled);
                    let StyleSheet::V1(body) = &mut data else {
                        unreachable!();
                    };
                    body.media = (0..depth)
                        .map(|index| Media {
                            parent_index: index.checked_sub(1).into(),
                            media_queries: Array::empty(),
                        })
                        .collect::<Vec<_>>()
                        .into();
                    let mut nodes = (0..depth)
                        .map(|index| SerializedRuleTreeNode::Media(index, 1))
                        .collect::<Vec<_>>();
                    nodes.push(SerializedRuleTreeNode::KeyFrames(0));
                    let bytes = DefaultOptions::new()
                        .serialize(&EncodedStyleSheet::V1(EncodedStyleSheetV1 {
                            sheet: body,
                            tree: Some(&nodes),
                        }))
                        .unwrap();
                    let restored = DefaultOptions::new().deserialize::<StyleSheet>(&bytes);
                    if depth > MAX_RULE_NESTING_DEPTH {
                        assert!(restored
                            .unwrap_err()
                            .to_string()
                            .contains("Media nesting limit"));
                    } else {
                        let restored = restored.unwrap();
                        let StyleSheet::V1(sheet) = &restored else {
                            unreachable!();
                        };
                        let Nullable::Some(tree) = &sheet.rule_tree else {
                            panic!("rule tree missing");
                        };
                        let mut level = tree;
                        for index in 0..depth {
                            assert_eq!(level.len(), 1);
                            let RuleTreeNode::Media {
                                media_index,
                                children,
                            } = &level[0]
                            else {
                                panic!("media node missing");
                            };
                            assert_eq!(*media_index, index);
                            level = children;
                        }
                        assert_eq!(level.arr.as_ref(), &[RuleTreeNode::KeyFrames(0)]);
                        assert_eq!(DefaultOptions::new().serialize(&restored).unwrap(), bytes);
                        drop(restored);
                    }

                    let ptr = Box::into_raw(bytes.into_boxed_slice());
                    let released = Rc::new(core::cell::Cell::new(0));
                    let drop_count = released.clone();
                    let restored = unsafe {
                        CompiledStyleSheet::deserialize_bincode_zero_copy(ptr, move || {
                            drop_count.set(drop_count.get() + 1);
                            drop(Box::from_raw(ptr));
                        })
                    };
                    assert_eq!(restored.is_ok(), depth <= MAX_RULE_NESTING_DEPTH);
                    drop(restored);
                    assert_eq!(released.get(), 1);
                }
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn zero_copy_input_lives_until_its_last_string_reference() {
        let (compiled, _) = crate::parser::parse_style_sheet(
            "lifetime",
            "@media screen { @font-face { font-family: Alive; src: local(Alive); } }",
        );
        let bytes = compiled.serialize_bincode().into_boxed_slice();
        let expected = bytes.to_vec();
        let start = bytes.as_ptr() as usize;
        let end = start + bytes.len();
        let ptr = Box::into_raw(bytes);
        let released = Rc::new(core::cell::Cell::new(0));
        let drop_count = released.clone();
        let restored = unsafe {
            CompiledStyleSheet::deserialize_bincode_zero_copy(ptr, move || {
                drop_count.set(drop_count.get() + 1);
                drop(Box::from_raw(ptr));
            })
        }
        .unwrap();
        assert_eq!(released.get(), 0);
        assert_eq!(restored.serialize_bincode(), expected);
        let font = restored.ss.borrow().font_face()[0].clone();
        let FontFamilyName::Title(name) = &font.font_family else {
            panic!("missing font family");
        };
        let name_ptr = name.as_slice().as_ptr() as usize;
        assert!(name_ptr >= start && name_ptr + name.as_slice().len() <= end);
        drop(restored);
        assert_eq!(released.get(), 0);
        assert_eq!(font.font_family, FontFamilyName::Title("Alive".into()));
        drop(font);
        assert_eq!(released.get(), 1);

        for bytes in [
            vec![],
            DefaultOptions::new().serialize(&StyleSheet::None).unwrap(),
        ] {
            let ptr = Box::into_raw(bytes.into_boxed_slice());
            let drop_count = released.clone();
            let result = unsafe {
                CompiledStyleSheet::deserialize_bincode_zero_copy(ptr, move || {
                    drop_count.set(drop_count.get() + 1);
                    drop(Box::from_raw(ptr));
                })
            };
            drop(result);
        }
        assert_eq!(released.get(), 3);
    }

    #[cfg(feature = "ffi")]
    #[test]
    fn ffi_version_owns_its_string_after_input_is_freed() {
        let mut bytes = crate::compile_style_sheet_to_bincode("version", ".a { width: 1px; }");
        let result =
            unsafe { crate::ffi::style_sheet_bincode_version(bytes.as_mut_ptr(), bytes.len()) };
        assert!(matches!(result.err, crate::ffi::FfiErrorCode::None));
        let version = unsafe { Box::from_raw(result.value) };
        drop(bytes);
        assert_eq!(version.as_str(), env!("CARGO_PKG_VERSION"));
    }

    #[cfg(feature = "ffi")]
    #[test]
    fn ffi_bincode_add_owns_path_when_input_is_released() {
        use crate::ffi;
        use core::cell::Cell;
        use core::ptr::null_mut;

        struct Input {
            bytes: Box<[u8]>,
            released: Rc<Cell<usize>>,
        }

        unsafe extern "C" fn release(args: ffi::RawMutPtr) {
            let input = Box::from_raw(args as *mut Input);
            input.released.set(input.released.get() + 1);
            drop(input);
        }

        let path = b"shared.css\0";
        for (import_index, bytes) in [
            (false, crate::compile_style_sheet_to_bincode("shared", "")),
            (
                true,
                DefaultOptions::new().serialize(&StyleSheet::None).unwrap(),
            ),
            (true, vec![255]),
        ] {
            let len = bytes.len();
            let mut allocation = path.to_vec();
            allocation.extend_from_slice(&bytes);
            let released = Rc::new(Cell::new(0));
            let mut input = Box::new(Input {
                bytes: allocation.into_boxed_slice(),
                released: released.clone(),
            });
            let path_ptr = input.bytes.as_ptr().cast();
            let buffer = unsafe { input.bytes.as_mut_ptr().add(path.len()) };
            let args = Box::into_raw(input).cast();

            unsafe {
                if import_index {
                    let index = ffi::style_sheet_import_index_new().value;
                    let result = ffi::style_sheet_import_index_add_bincode(
                        index,
                        path_ptr,
                        buffer,
                        len,
                        Some(release),
                        args,
                        null_mut(),
                    );
                    assert!(matches!(result.err, ffi::FfiErrorCode::None));
                    assert_eq!(released.get(), 1);
                    let found =
                        ffi::style_sheet_import_index_get_style_sheet(index, path.as_ptr().cast());
                    assert!(matches!(found.err, ffi::FfiErrorCode::None));
                    assert!(!found.value.is_null());
                    ffi::style_sheet_import_index_free(index);
                } else {
                    let mut resource = crate::StyleSheetResource::new();
                    let result = ffi::style_sheet_resource_add_bincode(
                        (&mut resource as *mut crate::StyleSheetResource).cast(),
                        path_ptr,
                        buffer,
                        len,
                        Some(release),
                        args,
                        null_mut(),
                    );
                    assert!(matches!(result.err, ffi::FfiErrorCode::None));
                    assert_eq!(released.get(), 1);
                    let (linked, warnings) = resource.link("shared.css", None);
                    assert!(warnings.is_empty());
                    assert_eq!(linked.rules_count(None), Some(0));
                }
            }
            assert_eq!(released.get(), 1);
        }
    }
}
