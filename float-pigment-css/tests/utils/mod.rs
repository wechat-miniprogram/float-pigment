use std::num::NonZeroUsize;

use float_pigment_css::query::{StyleNode, StyleNodeAttributeCaseSensitivity};
use float_pigment_css::sheet::PseudoElements;
use float_pigment_css::{
    length_num::LengthNum, property::*, MediaQueryStatus, StyleSheet, StyleSheetGroup,
};

pub struct StyleQueryTest<'a> {
    pub style_scope: Option<NonZeroUsize>,
    pub extra_style_scope: Option<NonZeroUsize>,
    pub host_style_scope: Option<NonZeroUsize>,
    pub tag_name: &'a str,
    pub id: &'a str,
    pub classes: &'a [(String, Option<NonZeroUsize>)],
    pub attributes: &'a [(String, String)],
    pub pseudo_element: Option<PseudoElements>,
}

impl<'a> StyleNode for StyleQueryTest<'a> {
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

    fn attribute(&self, name: &str) -> Option<(&str, StyleNodeAttributeCaseSensitivity)> {
        self.attributes
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| {
                (
                    v.as_str(),
                    match name {
                        "id" | "class" => StyleNodeAttributeCaseSensitivity::CaseSensitive,
                        "type" | "size" => StyleNodeAttributeCaseSensitivity::CaseInsensitive,
                        _ => StyleNodeAttributeCaseSensitivity::CaseSensitive,
                    },
                )
            })
    }

    fn pseudo_element(&self) -> Option<float_pigment_css::sheet::PseudoElements> {
        self.pseudo_element.clone()
    }
}

impl<'a> StyleQueryTest<'a> {
    pub fn single(
        style_scope: Option<NonZeroUsize>,
        extra_style_scope: Option<NonZeroUsize>,
        host_style_scope: Option<NonZeroUsize>,
        tag_name: &'a str,
        id: &'a str,
        classes: &'a [(String, Option<NonZeroUsize>)],
        attributes: &'a [(String, String)],
        pseudo_element: Option<PseudoElements>,
    ) -> Self {
        Self {
            style_scope,
            extra_style_scope,
            host_style_scope,
            tag_name,
            id,
            classes,
            attributes,
            pseudo_element,
        }
    }
}

#[macro_export]
macro_rules! test_parse_property {
    ($prop: ident, $prop_name: expr, $str_value: expr, $value: expr) => {{
        let name = $prop_name;
        let value = $value;
        let str_value = $str_value;
        let style_str = format!(".a{{{}:{};}}", name, str_value);
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(&style_str);
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.$prop(), value);
    }};
}

#[allow(dead_code)]
pub(super) fn style_sheets<const N: usize>(sources: [&str; N]) -> StyleSheetGroup {
    let mut ssg = StyleSheetGroup::new();
    for source in sources {
        let ss = StyleSheet::from_str(source);
        ssg.append(ss);
    }
    ssg
}

#[allow(dead_code)]
pub fn query<const N: usize, const M: usize>(
    ssg: &StyleSheetGroup,
    tag_name: &str,
    id: &str,
    classes: [&str; N],
    attributes: [(String, String); M],
) -> NodeProperties {
    query_with_media(
        ssg,
        tag_name,
        id,
        classes,
        attributes,
        &MediaQueryStatus::<f32>::default_screen(),
    )
}

#[allow(dead_code)]
pub(super) fn query_with_media<L: LengthNum, const N: usize, const M: usize>(
    ssg: &StyleSheetGroup,
    tag_name: &str,
    id: &str,
    classes: [&str; N],
    attributes: [(String, String); M],
    media_query_status: &MediaQueryStatus<L>,
) -> NodeProperties {
    let classes = classes
        .iter()
        .map(|x| (x.to_string(), None))
        .collect::<Vec<_>>();
    let query = StyleQueryTest::single(None, None, None, tag_name, id, &classes, &attributes, None);
    let mut node_properties = NodeProperties::new(None);
    ssg.query_single(query, media_query_status, &mut node_properties);
    node_properties
}

pub(super) struct StyleQueryWrapper {
    tag_name: String,
    id: String,
    classes: Vec<(String, Option<NonZeroUsize>)>,
    attributes: Vec<(String, String)>,
    pseudo_element: Option<PseudoElements>,
}

#[allow(dead_code)]
pub(super) fn query_item<'a, const N: usize, const M: usize>(
    tag_name: &'a str,
    id: &'a str,
    classes: [&'a str; N],
    attributes: [(String, String); M],
    pseudo_element: Option<PseudoElements>,
) -> StyleQueryWrapper {
    let classes = classes.iter().map(|x| (x.to_string(), None)).collect();
    StyleQueryWrapper {
        tag_name: tag_name.to_owned(),
        id: id.to_owned(),
        classes,
        attributes: attributes.to_vec(),
        pseudo_element,
    }
}

#[allow(dead_code)]
pub(super) struct QueryItem {
    w: StyleQueryWrapper,
}

impl QueryItem {
    #[allow(dead_code)]
    pub(super) fn new() -> Self {
        let w = StyleQueryWrapper {
            tag_name: String::new(),
            id: String::new(),
            classes: vec![],
            attributes: vec![],
            pseudo_element: None,
        };
        Self { w }
    }

    #[allow(dead_code)]
    pub(super) fn tag<T: ToString>(mut self, s: T) -> Self {
        self.w.tag_name = s.to_string();
        self
    }

    #[allow(dead_code)]
    pub(super) fn id<T: ToString>(mut self, s: T) -> Self {
        self.w.id = s.to_string();
        self
    }

    #[allow(dead_code)]
    pub(super) fn c<T: ToString>(self, class: T) -> Self {
        self.cs(class, 0)
    }

    #[allow(dead_code)]
    pub(super) fn cs<T: ToString, S: TryInto<NonZeroUsize>>(mut self, class: T, scope: S) -> Self {
        let scope = scope.try_into().ok();
        self.w.classes.push((class.to_string(), scope));
        self
    }

    #[allow(dead_code)]
    pub(super) fn attr<N: ToString, M: ToString>(mut self, name: N, value: M) -> Self {
        self.w
            .attributes
            .push((name.to_string(), value.to_string()));
        self
    }

    #[allow(dead_code)]
    pub(super) fn pe(mut self, pe: PseudoElements) -> Self {
        self.w.pseudo_element = Some(pe);
        self
    }

    #[allow(dead_code)]
    pub(super) fn end(self) -> StyleQueryWrapper {
        self.w
    }
}

#[allow(dead_code)]
pub(super) fn query_single(ssg: &StyleSheetGroup, item: StyleQueryWrapper) -> NodeProperties {
    query_list_with_media(ssg, [item], &MediaQueryStatus::<f32>::default_screen())
}

#[allow(dead_code)]
pub(super) fn query_list<const N: usize>(
    ssg: &StyleSheetGroup,
    list: [StyleQueryWrapper; N],
) -> NodeProperties {
    query_list_with_media(ssg, list, &MediaQueryStatus::<f32>::default_screen())
}

#[allow(dead_code)]
pub(super) fn query_list_with_media<L: LengthNum, const N: usize>(
    ssg: &StyleSheetGroup,
    list: [StyleQueryWrapper; N],
    media_query_status: &MediaQueryStatus<L>,
) -> NodeProperties {
    let list = Box::new(list);
    let query: Vec<_> = list
        .iter()
        .map(|sqw| {
            StyleQueryTest::single(
                None,
                None,
                None,
                &sqw.tag_name,
                &sqw.id,
                &sqw.classes,
                &sqw.attributes,
                sqw.pseudo_element.clone(),
            )
        })
        .collect();
    let mut node_properties = NodeProperties::new(None);
    ssg.query_ancestor_path(&query, media_query_status, &mut node_properties, None);
    node_properties
}

#[allow(dead_code)]
pub(super) fn query_list_with_parent<const N: usize>(
    ssg: &StyleSheetGroup,
    list: [StyleQueryWrapper; N],
    parent: &NodeProperties,
) -> NodeProperties {
    let list = Box::new(list);
    let query: Vec<_> = list
        .iter()
        .map(|sqw| {
            StyleQueryTest::single(
                None,
                None,
                None,
                &sqw.tag_name,
                &sqw.id,
                &sqw.classes,
                sqw.attributes.as_ref(),
                sqw.pseudo_element.clone(),
            )
        })
        .collect();
    let mut node_properties = NodeProperties::new(None);
    ssg.query_ancestor_path(
        &query,
        &MediaQueryStatus::<f32>::default_screen(),
        &mut node_properties,
        Some(parent),
    );
    node_properties
}
