use std::num::NonZeroUsize;

use float_pigment_css::{
    property::*, typing::*, MediaQueryStatus, StyleQuery, StyleSheet, StyleSheetGroup,
    StyleSheetImportIndex, StyleSheetResource,
};

mod utils;
use utils::*;

const MIXED_MEDIA_RULES: &str = r#"
    .case { width: 1px; }
    @media (min-width: 300px) {
        @font-face { font-family: NestedFont; src: local(NestedFont); }
        .case { width: 2px; }
        @media (max-width: 900px) {
            @keyframes nested { to { opacity: 1; } }
            .case { width: 3px; }
        }
        .case { height: 4px; }
    }
    .case { height: 5px; }
"#;

fn animation_opacity(
    group: &StyleSheetGroup,
    name: &str,
    status: &MediaQueryStatus<f32>,
) -> Option<f32> {
    let animation = group.search_keyframes(None, name, status)?;
    animation.keyframes[0]
        .properties
        .iter()
        .find(|property| property.get_property_name() == "opacity")
        .map(|property| property.get_property_value_string().parse().unwrap())
}

#[test]
fn named_rules_follow_media_and_roundtrip() {
    let source = r#"
        @font-face { font-family: RootFont; src: local(RootFont); }
        @keyframes shared { to { opacity: 0.1; } }
        @media (min-width: 300px) {
            @font-face { font-family: MidFont; src: local(MidFont); }
            @keyframes shared { to { opacity: 0.2; } }
            @media (max-width: 900px) {
                @font-face { font-family: NestedFont; src: local(NestedFont); }
                @keyframes shared { to { opacity: 0.3; } }
                @keyframes nested { to { opacity: 1; } }
            }
        }
        @media (min-width: 1000px) {
            @keyframes shared { to { opacity: 0.4; } }
        }
    "#;
    let bytes = float_pigment_css::compile_style_sheet_to_bincode("named", source);
    let mut resource = StyleSheetResource::new();
    assert!(resource.add_bincode("named", bytes.clone()).is_empty());
    assert_eq!(resource.serialize_bincode("named").unwrap(), bytes);
    let mut owned = StyleSheetGroup::new();
    owned.append_from_resource(&resource, "named", None);
    let mut direct = StyleSheetGroup::new();
    direct.append(StyleSheet::from_str(source));
    for group in [
        direct,
        owned,
        float_pigment_css::style_sheet_from_bincode(bytes),
    ] {
        assert_eq!(group.get_font_face(0).unwrap().len(), 3);
        for (width, expected, fonts) in [
            (200., 0.1, vec!["RootFont"]),
            (700., 0.3, vec!["RootFont", "MidFont", "NestedFont"]),
            (1200., 0.4, vec!["RootFont", "MidFont"]),
        ] {
            let status = MediaQueryStatus::default_screen_with_size(width, 600.);
            assert_eq!(animation_opacity(&group, "shared", &status), Some(expected));
            assert_eq!(
                group.search_keyframes(None, "nested", &status).is_some(),
                width == 700.
            );
            let actual = group
                .get_font_face_with_media(0, &status)
                .unwrap()
                .iter()
                .map(|font| match &font.font_family {
                    FontFamilyName::Title(name) => name.to_string(),
                    name => panic!("unexpected family: {name:?}"),
                })
                .collect::<Vec<_>>();
            assert_eq!(actual, fonts);
        }
    }
}

#[test]
fn imported_named_rules_use_media_and_source_order() {
    for serialized in [false, true] {
        let mut resource = StyleSheetResource::new();
        for (path, source) in [
            ("a", "@keyframes shared { to { opacity: 0.1; } } @font-face { font-family: A; src: local(A); }"),
            ("b", "@media (min-width: 100px) { @keyframes shared { to { opacity: 0.2; } } @font-face { font-family: B; src: local(B); } }"),
            ("main", "@import 'a.wxss' (min-width: 300px); @import 'b.wxss' (max-width: 900px); @media (min-width: 1000px) { @keyframes shared { to { opacity: 0.9; } } } @font-face { font-family: Root; src: local(Root); }"),
        ] {
            let warnings = if serialized {
                resource.add_bincode(path, float_pigment_css::compile_style_sheet_to_bincode(path, source))
            } else { resource.add_source(path, source) };
            assert!(warnings.is_empty());
        }
        let mut group = StyleSheetGroup::new();
        group.append_from_resource(&resource, "main", None);
        for (width, expected, fonts) in [
            (50., None, vec!["Root"]),
            (200., Some(0.2), vec!["B", "Root"]),
            (700., Some(0.2), vec!["A", "B", "Root"]),
            (1200., Some(0.9), vec!["A", "Root"]),
        ] {
            let status = MediaQueryStatus::default_screen_with_size(width, 600.);
            assert_eq!(animation_opacity(&group, "shared", &status), expected);
            let actual = group
                .get_font_face_with_media(0, &status)
                .unwrap()
                .iter()
                .map(|font| match &font.font_family {
                    FontFamilyName::Title(name) => name.to_string(),
                    name => panic!("unexpected family: {name:?}"),
                })
                .collect::<Vec<_>>();
            assert_eq!(actual, fonts);
        }
        let mut scoped = StyleSheetGroup::new();
        let scope = NonZeroUsize::new(7);
        scoped.append_from_resource(&resource, "main", scope);
        let status = MediaQueryStatus::<f32>::default_screen();
        assert!(scoped.search_keyframes(None, "shared", &status).is_none());
        assert!(scoped.search_keyframes(scope, "shared", &status).is_some());
    }
}

#[test]
fn mixed_media_rules_preserve_order_and_roundtrip() {
    let bytes = float_pigment_css::compile_style_sheet_to_bincode("mixed", MIXED_MEDIA_RULES);
    let mut resource = StyleSheetResource::new();
    assert!(resource.add_bincode("mixed", bytes.clone()).is_empty());
    let mut owned = StyleSheetGroup::new();
    owned.append_from_resource(&resource, "mixed", None);
    let mut direct = StyleSheetGroup::new();
    direct.append(StyleSheet::from_str(MIXED_MEDIA_RULES));
    let zero_copy = float_pigment_css::style_sheet_from_bincode(bytes.clone());
    assert_eq!(resource.serialize_bincode("mixed").unwrap(), bytes);

    for group in [direct, owned, zero_copy] {
        let sheet = group.style_sheet(0).unwrap();
        assert_eq!(sheet.rules_count(None), Some(5));
        for (index, depth) in [0, 1, 2, 1, 0].into_iter().enumerate() {
            let rule = sheet.get_rule(index as u32).unwrap();
            assert_eq!(rule.get_selector_string(), ".case");
            assert_eq!(rule.get_media_query_string_list().len(), depth);
        }
        assert_eq!(group.get_font_face(0).unwrap().len(), 1);
        let status = MediaQueryStatus::<f32>::default_screen();
        assert!(group.search_keyframes(None, "nested", &status).is_some());
        for (width, expected) in [(200., 1.), (700., 3.), (1200., 2.)] {
            let status = MediaQueryStatus::default_screen_with_size(width, 600.);
            let props = query_with_media(&group, "", "", ["case"], [], &status);
            assert_eq!(props.width(), Length::Px(expected));
            assert_eq!(props.height(), Length::Px(5.));
        }
    }
}

#[test]
fn display_preserves_media_conditions_and_rule_order() {
    let mut resource = StyleSheetResource::new();
    assert!(resource.add_source("display", MIXED_MEDIA_RULES).is_empty());
    for serialized in [false, true] {
        let mut original = StyleSheetGroup::new();
        if serialized {
            let mut decoded = StyleSheetResource::new();
            assert!(decoded
                .add_bincode("display", resource.serialize_bincode("display").unwrap())
                .is_empty());
            original.append_from_resource(&decoded, "display", None);
        } else {
            original.append_from_resource(&resource, "display", None);
        }
        let text = original.style_sheet(0).unwrap().sheets()[0]
            .borrow()
            .to_string();
        let mut displayed = StyleSheetGroup::new();
        displayed.append(StyleSheet::from_str(&text));
        for index in 0..5 {
            let before = original.get_rule(0, index).unwrap();
            let after = displayed.get_rule(0, index).unwrap();
            assert_eq!(after.get_selector_string(), before.get_selector_string());
            assert_eq!(
                after.get_media_query_string_list(),
                before.get_media_query_string_list()
            );
        }
        for width in [200., 700., 1200.] {
            let status = MediaQueryStatus::default_screen_with_size(width, 600.);
            assert_eq!(
                animation_opacity(&displayed, "nested", &status),
                animation_opacity(&original, "nested", &status)
            );
            let fonts = |group: &StyleSheetGroup| {
                group
                    .get_font_face_with_media(0, &status)
                    .unwrap()
                    .into_iter()
                    .map(|font| font.font_family.clone())
                    .collect::<Vec<_>>()
            };
            assert_eq!(fonts(&displayed), fonts(&original));
            let before = query_with_media(&original, "", "", ["case"], [], &status);
            let after = query_with_media(&displayed, "", "", ["case"], [], &status);
            assert_eq!(after.width(), before.width());
            assert_eq!(after.height(), before.height());
        }
    }
}

#[cfg(all(
    feature = "serialize",
    feature = "serialize_json",
    feature = "deserialize",
    feature = "deserialize_json"
))]
#[test]
fn json_zero_copy_reads_sheets_and_releases_input() {
    use std::{cell::Cell, rc::Rc};

    for source in [
        "",
        ".empty {}",
        "@font-face { font-family: Alive; src: local(Alive); }",
    ] {
        let json = float_pigment_css::compile_style_sheet_to_json("json", source);
        let mut copied = StyleSheetResource::new();
        assert!(copied.add_json("json", &json).is_empty());
        let ptr = Box::into_raw(json.into_bytes().into_boxed_slice());
        let released = Rc::new(Cell::new(0));
        let drop_count = released.clone();
        let mut resource = StyleSheetResource::new();
        let warnings = unsafe {
            resource.add_json_zero_copy("json", ptr, move || {
                drop_count.set(drop_count.get() + 1);
                drop(Box::from_raw(ptr));
            })
        };
        assert!(warnings.is_empty(), "{warnings:?}");
        assert_eq!(released.get(), 1);
        let mut group = StyleSheetGroup::new();
        group.append_from_resource(&resource, "json", None);
        let mut expected = StyleSheetGroup::new();
        expected.append_from_resource(&copied, "json", None);
        assert_eq!(
            group.style_sheet(0).unwrap().rules_count(None),
            expected.style_sheet(0).unwrap().rules_count(None)
        );
        assert_eq!(group.get_font_face(0), expected.get_font_face(0));
        drop(resource);
        drop(group);
        assert_eq!(released.get(), 1);
    }

    for json in ["", "{", "\"None\""] {
        let ptr = Box::into_raw(json.as_bytes().to_vec().into_boxed_slice());
        let released = Rc::new(Cell::new(0));
        let drop_count = released.clone();
        let mut resource = StyleSheetResource::new();
        let warnings = unsafe {
            resource.add_json_zero_copy("json", ptr, move || {
                drop_count.set(drop_count.get() + 1);
                drop(Box::from_raw(ptr));
            })
        };
        assert_eq!(warnings.is_empty(), json == "\"None\"");
        assert_eq!(released.get(), 1);
    }
}

#[cfg(all(
    feature = "serialize",
    feature = "serialize_json",
    feature = "deserialize",
    feature = "deserialize_json"
))]
#[test]
fn json_zero_copy_import_index_preserves_dependencies() {
    use std::{cell::Cell, rc::Rc};

    let mut resource = StyleSheetResource::new();
    assert!(resource
        .add_source("main", "@import 'dep.wxss';")
        .is_empty());
    assert!(resource.add_source("dep", "").is_empty());
    let json = resource.generate_import_indexes().serialize_json();
    let expected = StyleSheetImportIndex::deserialize_json(&json);
    let ptr = Box::into_raw(json.into_bytes().into_boxed_slice());
    let released = Rc::new(Cell::new(0));
    let drop_count = released.clone();
    let index = unsafe {
        StyleSheetImportIndex::deserialize_json_zero_copy(ptr, move || {
            drop_count.set(drop_count.get() + 1);
            drop(Box::from_raw(ptr));
        })
    };
    assert_eq!(released.get(), 1);
    assert_eq!(
        index.list_dependencies("main", true),
        expected.list_dependencies("main", true)
    );
    drop(index);
    assert_eq!(released.get(), 1);
}

#[test]
fn replacing_nested_rule_preserves_index_and_updates_media() {
    let mut resource = StyleSheetResource::new();
    resource.add_source(
        "editable",
        r#"
        @media (min-width: 300px) {
            .before { width: 1px; }
            @media (max-width: 900px) {
                .target { width: 2px; }
                .after { width: 3px; }
            }
            .outside { width: 4px; }
        }
        "#,
    );
    let mut group = StyleSheetGroup::new();
    group.append_from_resource(&resource, "editable", None);
    let original = group.get_rule(0, 1).unwrap();
    let replacement = float_pigment_css::sheet::Rule::from_parts_str(
        ["(min-width: 700px)", "(max-width: 1100px)"],
        ".replacement",
    )
    .unwrap()
    .add_properties([float_pigment_css::sheet::PropertyMeta::Normal {
        property: Property::Width(Length::Px(20.).into()),
    }]);
    let removed = group.replace_rule(0, 1, replacement).unwrap();
    assert!(std::rc::Rc::ptr_eq(&original, &removed));
    assert_eq!(original.get_selector_string(), ".target");
    for (width, expected) in [
        (500., Length::Auto),
        (800., Length::Px(20.)),
        (1000., Length::Px(20.)),
        (1200., Length::Auto),
    ] {
        let status = MediaQueryStatus::default_screen_with_size(width, 600.);
        let props = query_with_media(&group, "", "", ["replacement"], [], &status);
        assert_eq!(props.width(), expected);
    }

    let replacement = float_pigment_css::sheet::Rule::from_parts_str([], ".unconditional")
        .unwrap()
        .add_properties([float_pigment_css::sheet::PropertyMeta::Normal {
            property: Property::Width(Length::Px(30.).into()),
        }]);
    group.replace_rule(0, 1, replacement).unwrap();
    let appended =
        float_pigment_css::sheet::Rule::from_parts_str(["(max-width: 100px)"], ".appended")
            .unwrap()
            .add_properties([float_pigment_css::sheet::PropertyMeta::Normal {
                property: Property::Width(Length::Px(40.).into()),
            }]);
    let mut sheet = group.style_sheet(0).unwrap().clone();
    assert_eq!(sheet.add_rule(appended), 4);

    let bytes = resource.serialize_bincode("editable").unwrap();
    let decoded = float_pigment_css::style_sheet_from_bincode(bytes);
    for group in [&group, &decoded] {
        assert_eq!(
            group.get_rule(0, 1).unwrap().get_selector_string(),
            ".unconditional"
        );
        assert_eq!(
            group.get_rule(0, 2).unwrap().get_selector_string(),
            ".after"
        );
        assert_eq!(
            group.get_rule(0, 3).unwrap().get_selector_string(),
            ".outside"
        );
        for width in [50., 200., 800., 1200.] {
            let status = MediaQueryStatus::default_screen_with_size(width, 600.);
            let props = query_with_media(group, "", "", ["unconditional"], [], &status);
            assert_eq!(props.width(), Length::Px(30.));
            let props = query_with_media(group, "", "", ["after"], [], &status);
            assert_eq!(
                props.width(),
                if width == 800. {
                    Length::Px(3.)
                } else {
                    Length::Auto
                }
            );
        }
    }
    let status = MediaQueryStatus::default_screen_with_size(50., 600.);
    for group in [&group, &decoded] {
        assert_eq!(
            group.get_rule(0, 4).unwrap().get_selector_string(),
            ".appended"
        );
        assert_eq!(
            query_with_media(group, "", "", ["appended"], [], &status).width(),
            Length::Px(40.)
        );
    }
}

#[test]
fn nested_media_tag_prefix_survives_serialization() {
    let source = "@media screen { @media (min-width: 300px) { box.item { width: 9px; } } }";
    let bytes = float_pigment_css::compile_style_sheet_to_bincode("prefix", source);
    for decoded in [false, true] {
        let mut resource = StyleSheetResource::new();
        if decoded {
            assert!(resource.add_bincode("prefix", bytes.clone()).is_empty());
        } else {
            assert!(resource.add_source("prefix", source).is_empty());
        }
        resource.add_tag_name_prefix("prefix", "custom-");
        let serialized = resource.serialize_bincode("prefix").unwrap();
        let mut direct = StyleSheetGroup::new();
        direct.append_from_resource(&resource, "prefix", None);
        let roundtrip = float_pigment_css::style_sheet_from_bincode(serialized);
        for group in [direct, roundtrip] {
            for (width, expected) in [(200., Length::Auto), (800., Length::Px(9.))] {
                let status = MediaQueryStatus::default_screen_with_size(width, 600.);
                assert_eq!(
                    query_with_media(&group, "custom-box", "", ["item"], [], &status).width(),
                    expected
                );
                assert_eq!(
                    query_with_media(&group, "box", "", ["item"], [], &status).width(),
                    Length::Auto
                );
            }
        }
    }
}

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
    assert_eq!(node_properties.width(), Length::Auto);
    assert_eq!(node_properties.height(), Length::Px(8.));
    let media = MediaQueryStatus::default_screen_with_size(800., 601.);
    let node_properties = query_with_media(&ssg, "", "", ["a"], [], &media);
    assert_eq!(node_properties.width(), Length::Auto);
    assert_eq!(node_properties.height(), Length::Auto);
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
            QueryItem::new().tag("xxx-hello").end(),
            QueryItem::new().c("world").end(),
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
        let query = StyleQuery::single(None, None, None, "view", "i", &classes);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(0.));
        assert_eq!(node_properties.height(), Length::Auto);
        let query = StyleQuery::single(NonZeroUsize::new(1), None, None, "view", "i", &classes);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(1.));
        assert_eq!(node_properties.height(), Length::Auto);
        let query = StyleQuery::single(NonZeroUsize::new(2), None, None, "view", "i", &classes);
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
        let query = StyleQuery::single(None, None, None, "", "", &classes);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(0.));
        assert_eq!(node_properties.height(), Length::Auto);
        let classes = vec![("a".into(), NonZeroUsize::new(1))];
        let query = StyleQuery::single(None, None, None, "", "", &classes);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(1.));
        assert_eq!(node_properties.height(), Length::Auto);
        let classes = vec![("a".into(), NonZeroUsize::new(2))];
        let query = StyleQuery::single(None, None, None, "", "", &classes);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(2.));
        assert_eq!(node_properties.height(), Length::Auto);
    }
}
