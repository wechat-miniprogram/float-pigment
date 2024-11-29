use std::num::NonZeroUsize;

use float_pigment_css::{
    length_num::LengthNum, property::*, MediaQueryStatus, StyleQuery, StyleSheet, StyleSheetGroup,
};
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
    attributes: [&str; M],
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
    attributes: [&str; M],
    media_query_status: &MediaQueryStatus<L>,
) -> NodeProperties {
    let classes = classes
        .iter()
        .map(|x| (x.to_string(), None))
        .collect::<Vec<_>>();
    let attributes = attributes.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let query = StyleQuery::single(None, None, None, tag_name, id, &classes, &attributes);
    let mut node_properties = NodeProperties::new(None);
    ssg.query_single(&query, media_query_status, &mut node_properties);
    node_properties
}

pub(super) struct StyleQueryWrapper {
    tag_name: String,
    id: String,
    classes: Vec<(String, Option<NonZeroUsize>)>,
    attributes: Vec<String>,
}

#[allow(dead_code)]
pub(super) fn query_item<'a, const N: usize, const M: usize>(
    tag_name: &'a str,
    id: &'a str,
    classes: [&'a str; N],
    attributes: [&'a str; M],
) -> StyleQueryWrapper {
    let classes = classes.iter().map(|x| (x.to_string(), None)).collect();
    let attributes = attributes.iter().map(|x| x.to_string()).collect();
    StyleQueryWrapper {
        tag_name: tag_name.to_owned(),
        id: id.to_owned(),
        classes,
        attributes,
    }
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
            StyleQuery::single(
                None,
                None,
                None,
                &sqw.tag_name,
                &sqw.id,
                &sqw.classes,
                &sqw.attributes,
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
            StyleQuery::single(
                None,
                None,
                None,
                &sqw.tag_name,
                &sqw.id,
                &sqw.classes,
                sqw.attributes.as_ref(),
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
