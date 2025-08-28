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
}

impl StyleSheet {
    #[cfg(feature = "serialize")]
    pub(crate) fn from_sheet(sheet: &sheet::CompiledStyleSheet) -> Self {
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
        for rule in sheet.ss.borrow().rules.iter() {
            collect_media(&mut media_arr, &mut media_index_map, &rule.media);
        }

        let (imports, imports_media) = sheet
            .imports
            .iter()
            .map(|(x, media)| {
                (
                    StrRef::from(x.clone()),
                    media
                        .as_ref()
                        .map(|media| Media::from_sheet(media, &media_index_map))
                        .into(),
                )
            })
            .unzip::<_, _, Vec<StrRef>, Vec<Nullable<Media>>>();
        let rules = sheet
            .ss
            .borrow()
            .rules
            .iter()
            .map(|x| Rule::from_sheet(x, &media_index_map))
            .collect::<Box<[_]>>();
        let version = Box::new(String::from(env!("CARGO_PKG_VERSION")).into());
        let font_face = sheet
            .ss
            .borrow()
            .font_face
            .iter()
            .map(|x| FontFace::from_sheet(x))
            .collect::<Box<[_]>>();
        let keyframes = sheet
            .ss
            .borrow()
            .keyframes
            .iter()
            .map(|x| KeyFrames::from_sheet(x))
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
        })
    }

    #[cfg(feature = "deserialize")]
    pub(crate) fn into_sheet(self) -> sheet::CompiledStyleSheet {
        match self {
            Self::None => sheet::CompiledStyleSheet::new(),
            Self::V1(StyleSheetV1 {
                buf: _,
                imports,
                imports_media,
                rules,
                media,
                version: _,
                font_face,
                keyframes,
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
                sheet::CompiledStyleSheet::new_with_config(imports, rules, font_face, keyframes)
            }
        }
    }
}

impl Serialize for StyleSheetV1 {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
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
                    let mut seq = ser.serialize_tuple(7)?;
                    seq.serialize_element(buf.whole_buffer())?;
                    seq.serialize_element(&imports)?;
                    seq.serialize_element(&imports_media)?;
                    seq.serialize_element(&rules)?;
                    seq.serialize_element(&media)?;
                    seq.serialize_element(&version)?;
                    seq.serialize_element(&font_face)?;
                    seq.serialize_element(&keyframes)?;
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
        struct StyleSheetVisitor;

        impl<'de> serde::de::Visitor<'de> for StyleSheetVisitor {
            type Value = StyleSheetV1;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "StyleSheet")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let buf = SerdeThreadGlobalState::get_de_optional(|mut de| {
                    if let Some(drop_callback) = de.as_mut().and_then(|de| de.zero_copy.take()) {
                        seq.next_element::<&[u8]>()?
                            .map(|x| {
                                let ptr = x as *const [u8] as *mut [u8];
                                unsafe { StrBuffer::new_static_borrowed(ptr, drop_callback) }
                            })
                            .ok_or_else(|| de::Error::invalid_length(0, &"StyleSheet"))
                    } else {
                        seq.next_element::<Vec<u8>>()?
                            .map(StrBuffer::new_with_buf)
                            .ok_or_else(|| de::Error::invalid_length(0, &"StyleSheet"))
                    }
                })?;
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
                Ok(StyleSheetV1 {
                    buf,
                    imports,
                    imports_media,
                    rules,
                    media,
                    version,
                    font_face,
                    keyframes,
                })
            }
        }

        de.deserialize_tuple(8, StyleSheetVisitor)
    }
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
