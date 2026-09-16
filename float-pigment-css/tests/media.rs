use float_pigment_css::{
    length_num::LengthNum, property::*, sheet::Theme, typing::*, MediaQueryStatus, StyleQuery,
    StyleSheet, StyleSheetGroup, StyleSheetResource,
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

fn matches(group: &StyleSheetGroup, status: &MediaQueryStatus<f32>) -> bool {
    let classes = vec![("a".into(), None)];
    let query = [StyleQuery::single(None, None, None, "", "", &classes)];
    !group.query_matched_rules(&query, status).rules.is_empty()
}

fn assert_media_matches(condition: &str, expected: bool, status: &MediaQueryStatus<f32>) {
    let source = format!("@media {condition} {{ .a {{ width: 1px }} }}");
    let group = test_ss(&source);
    assert_eq!(matches(&group, status), expected, "{condition}");
    let rule = float_pigment_css::sheet::Rule::from_parts_str([condition], ".a").unwrap();
    let normalized = rule.get_media_query_string_list().join(", ");
    let roundtrip = test_ss(&format!("@media {normalized} {{ .a {{ width: 1px }} }}"));
    assert_eq!(
        matches(&roundtrip, status),
        expected,
        "stringify: {condition} -> {normalized}"
    );
    let bytes = float_pigment_css::compile_style_sheet_to_bincode("a", &source);
    let mut resource = StyleSheetResource::new();
    assert!(resource.add_bincode("a", bytes).is_empty());
    let mut decoded = StyleSheetGroup::new();
    decoded.append_from_resource(&resource, "a", None);
    assert_eq!(matches(&decoded, status), expected, "bincode: {condition}");
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
    // Negation preserves an unknown feature, but reverses an unknown media type.
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

// W3C Media Queries 4 §§2.4.3, 2.5, 3.1 and 3.2.
#[test]
fn mq4_conditions() {
    let status = MediaQueryStatus::default_screen();
    for condition in [
        "not screen and (unknown: 1) and (min-width: 900px)",
        "not screen and (min-width: 900px) and (unknown: 1)",
        "(unknown: 1) or (width: 800px)",
        "not ((unknown: 1) and (width: 900px))",
        "screen and ((width: 800px) or (height: 900px))",
        "screen and not (width: 900px)",
        "(width >= 800px)",
        "(800px <= width)",
        "(799px < width <= 800px)",
        "(801px > width >= 800px)",
        "(800px = width)",
        "(0dppx < resolution <= 1dppx)",
        "(height = 600px)",
        "(min-width: -1px)",
        "(orientation: 1), screen",
        "(prefers-color-scheme: 2), screen",
        "&bad, screen",
        "screen,",
        ",screen",
        "screen and, screen",
        "(width: 800px bogus), screen",
        "not unknown",
        "(width: 50em)",
        "(width: 50rem)",
        "(width: 100vw)",
        "(width >= 50em)",
        "(25rem < width <= 50em)",
        "(100vw >= width > 50vmin)",
        "(width >= 8in)",
        "(height <= 6.25in)",
        "(height >= 450pt)",
    ] {
        assert_media_matches(condition, true, &status);
    }
    for condition in [
        "not (unknown: 1)",
        "not screen and (unknown: 1)",
        "(unknown: 1) and (width: 800px)",
        "(width: 900px) or (unknown: 1)",
        "(width > 800px)",
        "(width > 50em)",
        "(width > 100vw)",
        "(800px > width)",
        "(800px < width < 900px)",
        "(400px < width > 200px)",
        "(width > = 799px)",
        "(width: 800px) or (height: 600px) and (width)",
        "not (width: 900px) and (height: 600px)",
        "screen and (width: 900px) or (height: 600px)",
        "only (width)",
        "not and",
        "not layer",
        "(screen)",
        "(resolution: 1dppx bogus)",
        "not (resolution: 1dppx bogus)",
        "(width: 800px bogus)",
        "not (orientation: 1)",
        "(resolution: 1)",
        "(max-width: -1px)",
        "(width: 800.0005px)",
        "(resolution: 1.0005dppx)",
    ] {
        assert_media_matches(condition, false, &status);
    }
}

#[test]
fn resolution_zero_and_infinity() {
    let mut status = MediaQueryStatus::default_screen();
    status.pixel_ratio = 0.;
    assert_media_matches("(resolution)", false, &status);
    status.pixel_ratio = f32::INFINITY;
    assert_media_matches("(resolution: infinite)", true, &status);
    assert_media_matches("(resolution >= infinite)", true, &status);
    assert_media_matches("(resolution > infinite)", false, &status);
}

#[test]
fn range_boundaries_and_converted_units() {
    for width in [799., 800., 801.] {
        let status = MediaQueryStatus::default_screen_with_size(width, 600.);
        for (condition, expected) in [
            ("(width < 800px)", width < 800.),
            ("(width <= 800px)", width <= 800.),
            ("(width = 800px)", width == 800.),
            ("(width >= 800px)", width >= 800.),
            ("(width > 800px)", width > 800.),
            ("(799px < width < 801px)", width == 800.),
        ] {
            assert_media_matches(condition, expected, &status);
        }
    }
    let mut status = MediaQueryStatus::default_screen_with_size(4.2, 600.);
    status.base_font_size = 14.;
    status.pixel_ratio = 2.1;
    for condition in [
        "(width = 0.3em)",
        "(width >= 0.3em)",
        "(width <= 0.3em)",
        "(resolution = 201.6dpi)",
        "(resolution >= 201.6dpi)",
        "(resolution <= 201.6dpi)",
    ] {
        assert_media_matches(condition, true, &status);
    }
    for condition in [
        "(width < 0.3em)",
        "(width > 0.3em)",
        "(resolution < 201.6dpi)",
        "(resolution > 201.6dpi)",
    ] {
        assert_media_matches(condition, false, &status);
    }
}

#[test]
fn nested_media_and_conditional_import() {
    let mut resource = StyleSheetResource::new();
    resource.add_source(
        "child",
        "@media not ((unknown: 1) and (height > 600px)) { .a { width: 1px } }",
    );
    resource.add_source(
        "parent",
        "@import \"child\" screen and (799px < width <= 800px);",
    );
    let mut group = StyleSheetGroup::new();
    group.append_from_resource(&resource, "parent", None);
    assert!(matches(&group, &MediaQueryStatus::default_screen()));
    assert!(!matches(
        &group,
        &MediaQueryStatus::default_screen_with_size(799., 600.)
    ));
    assert!(!matches(
        &group,
        &MediaQueryStatus::default_screen_with_size(800., 601.)
    ));
}

#[test]
fn malformed_general_enclosed_and_depth_limit() {
    let status = MediaQueryStatus::default_screen();
    assert_media_matches("(width: \"bad\n) or (width: 800px)", false, &status);
    assert_media_matches("(width: \"bad\n), screen", true, &status);
    let deep = format!("{}width{}", "(".repeat(200), ")".repeat(200));
    assert_media_matches(&deep, false, &status);
    assert_media_matches(&format!("{deep}, screen"), true, &status);
}

#[test]
fn invalid_media_values_report_warnings() {
    use float_pigment_css::parser::WarningKind;
    for condition in [
        "(min-width: abc)",
        "(width: 800px bogus)",
        "screen and ((width: 800px bogus) or (height: 600px))",
        "((width: abc) or (height) and (width))",
        "(width: abc) and",
        "(orientation: 1)",
        "(resolution: 2)",
        "not (resolution: -300dpi)",
        "(min-resolution: -1dppx)",
        "(prefers-color-scheme: bogus)",
    ] {
        let mut resource = StyleSheetResource::new();
        let source = format!("@media {condition}, screen {{ .a {{ width: 1px }} }}");
        let warnings = resource.add_source("warnings", &source);
        assert_eq!(warnings.len(), 1, "{condition}: {warnings:?}");
        assert_eq!(
            warnings[0].kind,
            WarningKind::InvalidMediaExpression,
            "{condition}"
        );
        assert!(warnings[0].end_col > warnings[0].start_col, "{warnings:?}");
        let mut group = StyleSheetGroup::new();
        group.append_from_resource(&resource, "warnings", None);
        assert!(matches(&group, &MediaQueryStatus::default_screen()));
    }
    for condition in [
        "screen and (width: 800px)",
        "screen and ((width >= 50em) or (height < 1px))",
    ] {
        let mut resource = StyleSheetResource::new();
        assert!(
            resource
                .add_source(
                    "valid",
                    &format!("@media {condition} {{ .a {{ width: 1px }} }}")
                )
                .is_empty(),
            "{condition}"
        );
    }
}

#[test]
fn negative_resolution_values_are_rejected() {
    let status = MediaQueryStatus::<f32>::default_screen();
    for condition in [
        "(min-resolution: -1dppx)",
        "not (resolution: -300dpi)",
        "(resolution > -1dppx)",
        "(-1dpi < resolution)",
        "(-1dppx < resolution < 2dppx)",
        "(min-resolution: -1dpcm)",
        "(min-resolution: -1x)",
        "(min-resolution: -1e-44dpi)",
        "(min-device-pixel-ratio: -1)",
        "not (device-pixel-ratio: -1)",
    ] {
        assert_media_matches(condition, false, &status);
        assert_media_matches(&format!("{condition}, screen"), true, &status);
    }
    for condition in [
        "(min-width: -1px)",
        "(height > -1px)",
        "not (width: -1px)",
        "(min-resolution: 0dppx)",
        "(min-resolution: -0dpi)",
        "(min-device-pixel-ratio: 0)",
    ] {
        assert_media_matches(condition, true, &status);
    }
}
