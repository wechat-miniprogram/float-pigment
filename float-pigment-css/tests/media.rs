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
                width: 4px;
            }
        }
        @media only unknown {
            .a {
                width: 5px;
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
    assert_eq!(node_properties.height(), Length::Auto);
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
    assert_eq!(node_properties.height(), Length::Auto);
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

#[test]
fn not_with_multiple_conditions() {
    // `not` negates the whole media query, not each condition
    let ssg = test_ss(
        r#"
        @media not screen and (min-width: 900px) {
            .a {
                width: 1px;
            }
        }
        @media not screen and (min-width: 700px) {
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
}

#[test]
fn resolution() {
    let ssg = test_ss(
        r#"
        @media (min-resolution: 2dppx) {
            .a {
                width: 1px;
            }
        }
        @media (max-resolution: 96dpi) {
            .a {
                width: 2px;
            }
        }
        @media (resolution: 3dppx) {
            .a {
                height: 1px;
            }
        }
        @media (min-device-pixel-ratio: 3) {
            .a {
                height: 2px;
            }
        }
        @media (-webkit-min-device-pixel-ratio: 3) {
            .a {
                height: 3px;
            }
        }
        @media (resolution: 192dpi) {
            .a {
                color: #123;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    // dpr = 1: only `max-resolution: 96dpi` (i.e. 1dppx) matches
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(2.));
    assert_eq!(node_properties.height(), Length::Auto);
    // dpr = 2: `min-resolution: 2dppx` and `resolution: 192dpi` (i.e. 2dppx) match
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.pixel_ratio = 2.;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Auto);
    assert_eq!(
        node_properties.color(),
        Color::Specified(0x11, 0x22, 0x33, 255)
    );
    // dpr = 3: `resolution: 3dppx` and `min-device-pixel-ratio: 3` match,
    // while the `-webkit-` prefixed one never matches
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.pixel_ratio = 3.;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn case_insensitive() {
    let ssg = test_ss(
        r#"
        @MEDIA SCREEN and (WIDTH: 800PX) {
            .a {
                width: 1px;
            }
        }
        @media screen and (ORIENTATION: LANDSCAPE) {
            .a {
                height: 1px;
            }
        }
        @media (Prefers-Color-Scheme: DARK) {
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
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(1.));
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.theme = Theme::Dark;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn not_unknown_feature() {
    // an unknown feature makes the query `not all`; an unknown type just evaluates to false
    let ssg = test_ss(
        r#"
        @media not (unknown-feature: 1) {
            .a {
                width: 1px;
            }
        }
        @media not unknown {
            .a {
                height: 2px;
            }
        }
        @media not (orientation: uuu) {
            .a {
                color: #123;
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
    assert_eq!(node_properties.width(), Length::Auto);
    assert_eq!(node_properties.height(), Length::Px(2.));
    assert_eq!(node_properties.color(), Color::Specified(0, 0, 0, 255));
}

#[test]
fn boolean_context() {
    let ssg = test_ss(
        r#"
        @media (width) {
            .a {
                width: 1px;
            }
        }
        @media (min-width) {
            .a {
                width: 2px;
            }
        }
        @media (orientation) {
            .a {
                height: 1px;
            }
        }
        @media (prefers-color-scheme) {
            .a {
                height: 2px;
            }
        }
        @media (resolution) {
            .a {
                color: #123;
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
fn resolution_robustness() {
    let ssg = test_ss(
        r#"
        @media (max-resolution: 37.8dpcm), screen {
            .a {
                width: 1px;
            }
        }
        @media (min-resolution: infinite) {
            .a {
                width: 2px;
            }
        }
        @media (max-resolution: infinite) {
            .a {
                height: 1px;
            }
        }
        @media (resolution: 2) {
            .a {
                width: 3px;
            }
        }
        @media (min-device-pixel-ratio: 2) {
            .a {
                height: 2px;
            }
        }
        @media (min-resolution: -1dppx) {
            .a {
                width: 4px;
            }
        }
        @media (min-resolution: calc(2dppx)), (min-width: 800px) {
            .a {
                color: #123;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    // dpr = 1: dpcm matches max-resolution; the calc() branch degrades to
    // not-all while the other comma-separated branch still applies
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(1.));
    assert_eq!(
        node_properties.color(),
        Color::Specified(0x11, 0x22, 0x33, 255)
    );
    // dpr = 2: dpcm exceeds max-resolution (the screen branch still matches)
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.pixel_ratio = 2.;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
}

#[test]
fn approximate_equality() {
    // computed values (dpi/96, em*base) equal literals within epsilon
    let ssg = test_ss(
        r#"
        @media (resolution: 201.6dpi) {
            .a {
                width: 1px;
            }
        }
        @media (width: 0.3em) {
            .a {
                height: 1px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    let mut media_status = MediaQueryStatus::default_screen_with_size(4.2, 600.);
    media_status.pixel_ratio = 2.1;
    media_status.base_font_size = 14.;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(1.));
}

#[test]
fn em_rem_length() {
    let ssg = test_ss(
        r#"
        @media (min-width: 50em) {
            .a {
                width: 1px;
            }
        }
        @media (max-width: 49.9em) {
            .a {
                width: 2px;
            }
        }
        @media (width: 50rem) {
            .a {
                height: 1px;
            }
        }
        @media (min-height: 37.5em) {
            .a {
                height: 2px;
            }
        }
    "#,
    );
    let classes = vec![("a".into(), None)];
    // base font size 16: 50em = 50rem = 800px, 49.9em = 798.4px, 37.5em = 600px
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        MediaQueryStatus::<f32>::default_screen(),
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Px(2.));
    // base font size 20: 50em = 50rem = 1000px, 49.9em = 998px, 37.5em = 750px
    let mut media_status = MediaQueryStatus::<f32>::default_screen();
    media_status.base_font_size = 20.;
    let node_properties = test_props(
        &ssg,
        StyleQuery::single(None, None, None, "", "", &classes),
        media_status,
    );
    assert_eq!(node_properties.width(), Length::Px(2.));
    assert_eq!(node_properties.height(), Length::Auto);
}
