use float_pigment_css::{
    length_num::LengthNum, property::*, sheet::Theme, typing::*, MediaQueryStatus, StyleQuery,
    StyleSheet, StyleSheetGroup,
};

fn test_ss(ss: &str) -> StyleSheetGroup {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(ss);
    ssg.append(ss);
    ssg
}

fn test_props<L: LengthNum>(
    ssg: &StyleSheetGroup,
    query: StyleQuery,
    media: MediaQueryStatus<L>,
) -> NodeProperties {
    let query = vec![query];
    let matched_rules = ssg.query_matched_rules(&query, &media);
    {
        // try stringify and re-parse
        for mr in matched_rules.rules.iter() {
            let mut ss = ".m { opacity: 0.5; }".to_string();
            for s in mr.rule.get_media_query_string_list() {
                ss = format!("@media {s} {{ {ss} }}");
            }
            let ssg = test_ss(&ss);
            let classes = vec![("m".into(), None)];
            let query = vec![StyleQuery::single(None, None, None, "", "", &classes)];
            let matched_rules = ssg.query_matched_rules(&query, &media);
            assert_eq!(matched_rules.rules.len(), 1);
        }
    }
    let mut node_properties = NodeProperties::new(None);
    matched_rules.merge_node_properties(&mut node_properties, None, 16., &[]);
    node_properties
}

#[test]
fn media_type() {
    let ssg = test_ss(
        r#"
        @media screen {
            .a {
                width: 1px;
            }
        }
        @media screen and (width: 800px) {
            .a {
                height: 2px;
            }
        }
        @media screen and (width: 799px) {
            .a {
                width: 3px;
            }
        }
        @media unknown {
            .a {
                color: #123;
            }
        }
        @media only unknown {
            .a {
                color: #456;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
    assert_eq!(
        node_properties.color(),
        Color::Specified(0x11, 0x22, 0x33, 255)
    );
}

#[test]
fn nested() {
    let ssg = test_ss(
        r#"
        .a {
            width: 1px;
        }
        @media (height: 600px) {
            .a {
                width: 2px;
            }
            @media (width: 800px) {
                .a {
                    width: 3px;
                }
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(800., 600.),
    );
    assert_eq!(node_properties.width(), Length::Px(3.));
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(801., 600.),
    );
    assert_eq!(node_properties.width(), Length::Px(2.));
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(800., 601.),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(801., 601.),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
}

#[test]
fn decorator() {
    let ssg = test_ss(
        r#"
        @media not (height: 599px) {
            .a {
                width: 1px;
            }
        }
        @media not (height: 600px) {
            .a {
                width: 2px;
            }
        }
        @media (unknown: xxx) {
            .a {
                height: 3px;
            }
        }
        @media only (unknown: xxx) {
            .a {
                height: 4px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(3.));
}

#[test]
fn min_width() {
    let ssg = test_ss(
        r#"
        @media (min-width: 800px) {
            .a {
                width: 1px;
            }
        }
        @media (min-width: 799px) {
            .a {
                height: 2px;
            }
        }
        @media (min-width: 801px) {
            .a {
                height: 3px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn max_width() {
    let ssg = test_ss(
        r#"
        @media (max-width: 800px) {
            .a {
                width: 1px;
            }
        }
        @media (max-width: 801px) {
            .a {
                height: 2px;
            }
        }
        @media (max-width: 799px) {
            .a {
                height: 3px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn min_height() {
    let ssg = test_ss(
        r#"
        @media (min-height: 600px) {
            .a {
                width: 1px;
            }
        }
        @media (min-height: 599px) {
            .a {
                height: 2px;
            }
        }
        @media (min-height: 601px) {
            .a {
                height: 3px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn max_height() {
    let ssg = test_ss(
        r#"
        @media (max-height: 600px) {
            .a {
                width: 1px;
            }
        }
        @media (max-height: 601px) {
            .a {
                height: 2px;
            }
        }
        @media (max-height: 599px) {
            .a {
                height: 3px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn width() {
    let ssg = test_ss(
        r#"
        @media (width: 800px) {
            .a {
                width: 1px;
            }
        }
        @media (width: 801px) {
            .a {
                width: 2px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(801., 600.),
    );
    assert_eq!(node_properties.width(), Length::Px(2.));
}

#[test]
fn height() {
    let ssg = test_ss(
        r#"
        @media (height: 600px) {
            .a {
                height: 1px;
            }
        }
        @media (height: 601px) {
            .a {
                height: 2px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.height(), Length::Px(1.));
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(800., 601.),
    );
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn prefers_color_scheme() {
    let ssg = test_ss(
        r#"
        @media (prefers-color-scheme: dark) {
            .a {
                height: 1px;
            }
        }
        @media (prefers-color-scheme: light) {
            .a {
                height: 2px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.theme = Theme::Light;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.height(), Length::Px(2.));
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.theme = Theme::Dark;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.height(), Length::Px(1.));
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.theme = Theme::None;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.height(), Length::Undefined);
}

#[test]
fn orientation() {
    let ssg = test_ss(
        r#"
        @media (orientation: landscape) {
            .a {
                height: 1px;
            }
        }
        @media (orientation: portrait) {
            .a {
                height: 2px;
            }
        }
        @media (orientation: uuu) {
            .a {
                height: 3px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(800., 600.),
    );
    assert_eq!(node_properties.height(), Length::Px(1.));
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::default_screen_with_size(600., 800.),
    );
    assert_eq!(node_properties.height(), Length::Px(2.));
}
