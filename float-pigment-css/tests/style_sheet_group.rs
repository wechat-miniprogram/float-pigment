use std::num::NonZeroUsize;

use float_pigment_css::{
    property::*, typing::*, MediaQueryStatus, StyleQuery, StyleSheet, StyleSheetGroup,
    StyleSheetImportIndex, StyleSheetResource,
};

mod utils;
use utils::*;

#[test]
fn multi_style_sheets() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
            .a { width: 1px; }
        ",
    );
    ssg.append(ss);
    let ss = StyleSheet::from_str(
        "
            .a { width: 2px; }
        ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
}

#[test]
fn style_sheet_resource_basic() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/a.wxss",
        r#"
        .a { width: 1px }
    "#,
    );
    ssr.add_source(
        "my/style/sheet/b.wxss",
        r#"
        @import "./a";
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append_from_resource(&ssr, "my/style/sheet/b.wxss", None);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.width(), Length::Px(1.));
}

#[test]
fn style_sheet_resource_recursive_import() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/a.wxss",
        r#"
        .a { width: 2px }
    "#,
    );
    ssr.add_source(
        "my/style/b.wxss",
        r#"
        @import "sheet/a";
    "#,
    );
    ssr.add_source(
        "my/style/sheet/c.wxss",
        r#"
        @import "../b";
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append_from_resource(&ssr, "my/style/sheet/c.wxss", None);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
}

#[test]
fn style_sheet_resource_priority() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/a.wxss",
        r#"
        .a { width: 3px }
    "#,
    );
    ssr.add_source(
        "my/style/sheet/b.wxss",
        r#"
        @import "../sheet/a.wxss";
        .a { width: 4px; height: 5px }
    "#,
    );
    ssr.add_source(
        "my/style/sheet/c.wxss",
        r#"
        @import "/my/style/sheet/b";
        @import "a";
        .a { height: 6px }
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append_from_resource(&ssr, "my/style/sheet/c.wxss", None);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.width(), Length::Px(3.));
    assert_eq!(node_properties.height(), Length::Px(6.));
}

#[test]
fn style_sheet_resource_media_import() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/a.wxss",
        r#"
        .a { width: 7px }
    "#,
    );
    ssr.add_source(
        "my/style/sheet/b.wxss",
        r#"
        @import "a" (width: 800px);
        .a { height: 8px }
    "#,
    );
    ssr.add_source(
        "my/style/sheet/c.wxss",
        r#"
        @import "b" (height: 600px);
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append_from_resource(&ssr, "my/style/sheet/c.wxss", None);
    let media = MediaQueryStatus::default_screen_with_size(800., 600.);
    let node_properties = query_with_media(&ssg, "", "", ["a"], [], &media);
    assert_eq!(node_properties.width(), Length::Px(7.));
    assert_eq!(node_properties.height(), Length::Px(8.));
    let media = MediaQueryStatus::default_screen_with_size(801., 600.);
    let node_properties = query_with_media(&ssg, "", "", ["a"], [], &media);
    assert_eq!(node_properties.width(), Length::Undefined);
    assert_eq!(node_properties.height(), Length::Px(8.));
    let media = MediaQueryStatus::default_screen_with_size(800., 601.);
    let node_properties = query_with_media(&ssg, "", "", ["a"], [], &media);
    assert_eq!(node_properties.width(), Length::Undefined);
    assert_eq!(node_properties.height(), Length::Undefined);
}

#[test]
fn style_sheet_resource_tag_name_prefix() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/a.wxss",
        r#"
            a { width: 1vw }
            hello .world {
                width: 1vw;
            }
        "#,
    );
    ssr.add_tag_name_prefix("my/style/sheet/a.wxss", "xxx-");
    let mut ssg = StyleSheetGroup::new();
    ssg.append_from_resource(&ssr, "my/style/sheet/a.wxss", None);
    let node_properties = query(&ssg, "xxx-a", "", [""], []);
    assert_eq!(node_properties.width(), Length::Vw(1.));
    let node_properties = query_list(
        &ssg,
        [
            query_item("xxx-hello", "", [""], []),
            query_item("", "", ["world"], []),
        ],
    );
    assert_eq!(node_properties.width(), Length::Vw(1.));
}

#[test]
fn style_sheet_import_index_query() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source("my/style/sheet/a.wxss", r#""#);
    ssr.add_source(
        "my/style/sheet/b.wxss",
        r#"
        @import "a" (width: 800px);
    "#,
    );
    ssr.add_source(
        "my/style/sheet/c.wxss",
        r#"
        @import "b" (height: 600px);
        @import "a";
    "#,
    );
    ssr.add_source(
        "my/style/sheet/d.wxss",
        r#"
        @import "c";
    "#,
    );
    let mut ii = ssr.generate_import_indexes();
    let dep_b = ii.query_and_mark_dependencies("my/style/sheet/b.wxss");
    assert_eq!(dep_b, vec!["my/style/sheet/a", "my/style/sheet/b"]);
    let dep_b = ii.list_dependencies("my/style/sheet/b.wxss", true);
    assert_eq!(dep_b, vec!["my/style/sheet/a", "my/style/sheet/b"]);
    let dep_b = ii.query_and_mark_dependencies("my/style/sheet/d.wxss");
    assert_eq!(dep_b, vec!["my/style/sheet/c", "my/style/sheet/d"]);
    let dep_b = ii.list_dependencies("my/style/sheet/d.wxss", true);
    assert_eq!(
        dep_b,
        vec![
            "my/style/sheet/a",
            "my/style/sheet/b",
            "my/style/sheet/a",
            "my/style/sheet/c",
            "my/style/sheet/d",
        ]
    );
}

#[test]
fn style_sheet_import_index_serde() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source("my/style/sheet/a.wxss", r#""#);
    ssr.add_source("my/style/sheet/b.wxss", r#""#);
    ssr.add_source(
        "my/style/sheet/c.wxss",
        r#"
        @import "b" (height: 600px);
        @import "a";
    "#,
    );
    ssr.add_source(
        "my/style/sheet/d.wxss",
        r#"
        @import "c";
    "#,
    );
    let ii = ssr.generate_import_indexes();
    let bincode = ii.serialize_bincode();
    let mut ii = StyleSheetImportIndex::deserialize_bincode(bincode);
    let dep_b = ii.list_dependencies("my/style/sheet/d.wxss", true);
    assert_eq!(
        dep_b,
        vec![
            "my/style/sheet/b",
            "my/style/sheet/a",
            "my/style/sheet/c",
            "my/style/sheet/d",
        ]
    );
    let dep_b = ii.query_and_mark_dependencies("my/style/sheet/d.wxss");
    assert_eq!(
        dep_b,
        vec![
            "my/style/sheet/b",
            "my/style/sheet/a",
            "my/style/sheet/c",
            "my/style/sheet/d",
        ]
    );
}

#[test]
fn style_sheet_import_index_serde_merge() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/a.wxss",
        r#"
        @import "b";
    "#,
    );
    ssr.add_source("my/style/sheet/b.wxss", r#""#);
    let ii1 = ssr.generate_import_indexes();
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "my/style/sheet/c.wxss",
        r#"
            @import "a" (height: 600px);
        "#,
    );
    ssr.add_source(
        "my/style/sheet/d.wxss",
        r#"
            @import "c";
        "#,
    );
    let ii2 = ssr.generate_import_indexes();
    let bincode1 = ii1.serialize_bincode();
    let bincode2 = ii2.serialize_bincode();
    let mut ii = StyleSheetImportIndex::deserialize_bincode(bincode1);
    ii.merge_bincode(bincode2);
    let dep_b = ii.list_dependencies("my/style/sheet/a.wxss", true);
    assert_eq!(dep_b, vec!["my/style/sheet/b", "my/style/sheet/a",]);
    let dep_b = ii.query_and_mark_dependencies("my/style/sheet/d.wxss");
    assert_eq!(
        dep_b,
        vec![
            "my/style/sheet/b",
            "my/style/sheet/a",
            "my/style/sheet/c",
            "my/style/sheet/d",
        ]
    );
}

#[test]
fn scopes_in_tag_name_and_id() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "file0",
        r#"
            * { width: 0px }
        "#,
    );
    ssr.add_source(
        "file1",
        r#"
            #i { width: 1px }
        "#,
    );
    ssr.add_source(
        "file2",
        r#"
            view { width: 2px }
            * { height: 2px }
        "#,
    );
    {
        let mut ssg = StyleSheetGroup::new();
        ssg.append_from_resource(&ssr, "file0", NonZeroUsize::new(0));
        ssg.append_from_resource(&ssr, "file1", NonZeroUsize::new(1));
        ssg.append_from_resource(&ssr, "file2", NonZeroUsize::new(2));
        let classes = vec![];
        let query = StyleQuery::single(None, None, None, "view", "i", &classes, &[]);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(0.));
        assert_eq!(node_properties.height(), Length::Undefined);
        let query =
            StyleQuery::single(NonZeroUsize::new(1), None, None, "view", "i", &classes, &[]);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(1.));
        assert_eq!(node_properties.height(), Length::Undefined);
        let query =
            StyleQuery::single(NonZeroUsize::new(2), None, None, "view", "i", &classes, &[]);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(2.));
        assert_eq!(node_properties.height(), Length::Px(2.));
    }
}

#[test]
fn scopes_in_classes() {
    let mut ssr = StyleSheetResource::new();
    ssr.set_panic_on_warning(true);
    ssr.add_source(
        "file0",
        r#"
            * { width: 0px }
        "#,
    );
    ssr.add_source(
        "file1",
        r#"
            .a { width: 1px }
            * { height: 1px }
        "#,
    );
    ssr.add_source(
        "file2",
        r#"
            .a { width: 2px }
            * { height: 2px }
        "#,
    );
    {
        let mut ssg = StyleSheetGroup::new();
        ssg.append_from_resource(&ssr, "file0", NonZeroUsize::new(0));
        ssg.append_from_resource(&ssr, "file1", NonZeroUsize::new(1));
        ssg.append_from_resource(&ssr, "file2", NonZeroUsize::new(2));
        let classes = vec![("a".into(), NonZeroUsize::new(0))];
        let query = StyleQuery::single(None, None, None, "", "", &classes, &[]);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(0.));
        assert_eq!(node_properties.height(), Length::Undefined);
        let classes = vec![("a".into(), NonZeroUsize::new(1))];
        let query = StyleQuery::single(None, None, None, "", "", &classes, &[]);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(1.));
        assert_eq!(node_properties.height(), Length::Undefined);
        let classes = vec![("a".into(), NonZeroUsize::new(2))];
        let query = StyleQuery::single(None, None, None, "", "", &classes, &[]);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(2.));
        assert_eq!(node_properties.height(), Length::Undefined);
    }
}
