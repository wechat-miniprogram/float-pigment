use super::*;

    #[test]
    fn font() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a { font: italic bold 10px / 14px sans-serif; }
                .b { font: 32px :-), sans-serif; }
                .c { font: 32px , sans-serif; }
                .d { font: 32px (), sans-serif; }
                .e { font: 32px {}, sans-serif; }
                .f { font: 32px [], sans-serif; }
                .g { font: 32px a(), sans-serif; }
                .h { font: 32px a{}, sans-serif; }
                .i { font: 32px a[], sans-serif; }
                .j { font: 32px; }
                .k { font: 32px \"", sans-serif; }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.font_style(), FontStyle::Italic);
        assert_eq!(np.font_size(), Length::Px(10.));
        assert_eq!(np.font_weight(), FontWeight::Bold);
        assert_eq!(np.line_height(), LineHeight::Length(Length::Px(14.)));
        assert_eq!(
            np.font_family(),
            FontFamily::Names(vec![FontFamilyName::SansSerif].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["i"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["j"], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["k"], []);
        assert_eq!(np.font_size(), Length::Undefined);
    }

    // 0x90
    #[test]
    fn font_size() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { font-size: 100px; }
            .b { font-size: 20rem; }
            .c { font-size: 80% }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.font_size(), Length::Undefined);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.font_size(), Length::Px(100.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.font_size(), Length::Px(320.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.font_size(), Length::Px(16. * 0.8));
        test_parse_property!(font_size, "font-size", "-10px", Length::Undefined);
        test_parse_property!(font_size, "font-size", "0", Length::Px(0.));
    }

    // 0x91
    #[test]
    fn direction() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { direction: ltr }
            .b { direction: rtl }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [], []);
        assert_eq!(np.direction(), Direction::Auto);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.direction(), Direction::LTR);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.direction(), Direction::RTL);
    }

    // 0x92
    #[test]
    fn writing_mode() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { writing-mode: horizontal-tb }
            .b { writing-mode: vertical-rl }
            .c { writing-mode: vertical-lr }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.writing_mode(), WritingMode::HorizontalTb);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.writing_mode(), WritingMode::HorizontalTb);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.writing_mode(), WritingMode::VerticalRl);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.writing_mode(), WritingMode::VerticalLr);
    }

    // 0x93
    #[test]
    fn line_height() {
        test_parse_property!(line_height, "line-height", "-100px", LineHeight::Normal);
        test_parse_property!(line_height, "line-height", "-100", LineHeight::Normal);
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { line-height: normal; }
            .b { line-height: 20px; }
            .c { line-height: 1.2; }            
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.line_height(), LineHeight::Normal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.line_height(), LineHeight::Length(Length::Px(20.)));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.line_height(), LineHeight::Num(Number::F32(1.2)));
    }

    // 0x94
    #[test]
    fn text_align() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { text-align: left }
            .b { text-align: right }
            .c { text-align: center }
            .d { text-align: justify }
            .e { text-align: justify-all }
            .f { text-align: start }
            .g { text-align: end }
            .h { text-align: match-parent }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.text_align(), TextAlign::Start);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.text_align(), TextAlign::Left);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.text_align(), TextAlign::Right);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.text_align(), TextAlign::Center);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.text_align(), TextAlign::Justify);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.text_align(), TextAlign::JustifyAll);
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(np.text_align(), TextAlign::Start);
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(np.text_align(), TextAlign::End);
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(np.text_align(), TextAlign::MatchParent);
    }

    // 0x95
    #[test]
    fn font_weight() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { font-weight: normal }
            .b { font-weight: bold }
            .c { font-weight: lighter}
            .d { font-weight: bolder }
            .e { font-weight: 500 }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.font_weight(), FontWeight::Normal);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.font_weight(), FontWeight::Normal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.font_weight(), FontWeight::Bold);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.font_weight(), FontWeight::Lighter);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.font_weight(), FontWeight::Bolder);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.font_weight(), FontWeight::Num(Number::F32(500.)));
    }

    // 0x96
    #[test]
    fn word_break() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { word-break: normal }
            .b { word-break: break-word }
            .c { word-break: break-all }
            .d { word-break: keep-all }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.word_break(), WordBreak::BreakWord);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.word_break(), WordBreak::BreakWord);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.word_break(), WordBreak::BreakWord);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.word_break(), WordBreak::BreakAll);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.word_break(), WordBreak::KeepAll);
    }

    // 0x97
    #[test]
    fn white_space() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { white-space: normal }
            .b { white-space: nowrap }
            .c { white-space: pre }
            .d { white-space: pre-wrap }
            .e { white-space: pre-line }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.white_space(), WhiteSpace::Normal);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.white_space(), WhiteSpace::Normal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.white_space(), WhiteSpace::NoWrap);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.white_space(), WhiteSpace::Pre);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.white_space(), WhiteSpace::PreWrap);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.white_space(), WhiteSpace::PreLine);
    }

    // 0x98
    #[test]
    fn text_overflow() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { text-overflow: clip }
            .b { text-overflow: ellipsis }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.text_overflow(), TextOverflow::Clip);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.text_overflow(), TextOverflow::Ellipsis);
    }

    // 0x99
    #[test]
    fn text_indent() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { text-indent: 40px; }
            .b { text-indent: 3em; }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.text_indent(), Length::Px(40.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.text_indent(), Length::Px(48.));
    }

    // 0x9a
    #[test]
    fn vertical_align() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { vertical-align: baseline }
            .b { vertical-align: top }
            .c { vertical-align: middle }
            .d { vertical-align: bottom }
            .e { vertical-align: text-top }
            .f { vertical-align: text-bottom }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.vertical_align(), VerticalAlign::Baseline);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.vertical_align(), VerticalAlign::Top);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.vertical_align(), VerticalAlign::Middle);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.vertical_align(), VerticalAlign::Bottom);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.vertical_align(), VerticalAlign::TextTop);
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(np.vertical_align(), VerticalAlign::TextBottom);
    }

    // 0x9b
    #[test]
    fn letter_spacing() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { letter-spacing: normal; }
            .b { letter-spacing: 3em; }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.letter_spacing(), LetterSpacing::Normal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.letter_spacing(), LetterSpacing::Length(Length::Px(48.)));
    }

    // 0x9c
    #[test]
    fn word_spacing() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { word-spacing: normal; }
            .b { word-spacing: 3em; }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.word_spacing(), WordSpacing::Normal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.word_spacing(), WordSpacing::Length(Length::Px(48.)));
    }

    // 0x9d
    #[test]
    fn font_family() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { font-family: "Gill Sans Extrabold", sans-serif, sans-serif }
            .b { font-family: Courier          New, sans-serif; }
            .c { font-family:iconfont!important; }
            .d { font-family:iconfont !important }
            .e { font-family:iconfont, }
            .f { font-family:iconfont, !important }
            .g { font-family:iconfont,!important }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.font_family(),
            FontFamily::Names(
                vec![
                    FontFamilyName::Title("Gill Sans Extrabold".to_string().into()),
                    FontFamilyName::SansSerif,
                    FontFamilyName::SansSerif
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.font_family(),
            FontFamily::Names(
                vec![
                    FontFamilyName::Title("Courier New".to_string().into()),
                    FontFamilyName::SansSerif
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.font_family(),
            FontFamily::Names(vec![FontFamilyName::Title("iconfont".to_string().into())].into())
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.font_family(),
            FontFamily::Names(vec![FontFamilyName::Title("iconfont".to_string().into())].into())
        );
    }

    // 0x9e
    #[test]
    fn font_style() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { font-style: normal }
            .b { font-style: italic }
            .c { font-style: oblique }
            .d { font-style: oblique 10deg; }
        "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [""], []);
            assert_eq!(np.font_style(), FontStyle::Normal);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.font_style(), FontStyle::Normal);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.font_style(), FontStyle::Italic);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.font_style(), FontStyle::Oblique(Angle::Deg(14.)));
        }
        {
            let np = query(&ssg, "", "", ["d"], []);
            assert_eq!(np.font_style(), FontStyle::Oblique(Angle::Deg(10.)));
        }
    }

    // 0x9f
    #[test]
    fn text_shadow() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a {
                text-shadow: 1px 1px 2px black;
            }
            .b {
                text-shadow: none;
            }
            .c {
                text-shadow: white 2px 5px;
            }
            .d {
                text-shadow: 4px;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.text_shadow(),
            TextShadow::List(
                vec![TextShadowItem::TextShadowValue(
                    Length::Px(1.),
                    Length::Px(1.),
                    Length::Px(2.),
                    Color::Specified(0, 0, 0, 255)
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.text_shadow(), TextShadow::None);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.text_shadow(),
            TextShadow::List(
                vec![TextShadowItem::TextShadowValue(
                    Length::Px(2.),
                    Length::Px(5.),
                    Length::Undefined,
                    Color::Specified(255, 255, 255, 255)
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.text_shadow(), TextShadow::None);
    }

    // 0xa0
    #[test]
    fn text_decoration_line() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a {
                text-decoration-line: underline overline line-through;
            }
            .b {
                text-decoration-line: none;
            }
            .c {
              text-decoration-line: underline asdasdasdasd;
            }
            .d {
                text-decoration-line: none dasdad;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.text_decoration_line(),
            TextDecorationLine::List(
                vec![
                    TextDecorationLineItem::Underline,
                    TextDecorationLineItem::Overline,
                    TextDecorationLineItem::LineThrough,
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.text_decoration_line(), TextDecorationLine::None);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.text_decoration_line(), TextDecorationLine::None);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.text_decoration_line(), TextDecorationLine::None);
    }

    // 0xa1
    #[test]
    fn text_decoration_style() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            text-decoration-style: dashed;
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.text_decoration_style(), TextDecorationStyle::Dashed);
    }

    // 0xa2
    #[test]
    fn text_decoration_color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            text-decoration-color: red;
        }
        .b {
            text-decoration-color: rgba(123, 22, 1, 0);
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.text_decoration_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.text_decoration_color(), Color::Specified(123, 22, 1, 0));
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.text_decoration_color(), Color::CurrentColor);
    }

    // 0xa3
    #[test]
    fn text_decoration_thickness() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a {
                    text-decoration-thickness: from-font
                }
                .b {
                    text-decoration-thickness: 10%;
                }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.text_decoration_thickness(),
            TextDecorationThickness::FromFont
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.text_decoration_thickness(),
            TextDecorationThickness::Length(Length::Px(1.6))
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.text_decoration_thickness(),
            TextDecorationThickness::Auto
        );
    }

    // 0xab
    #[test]
    fn text_underline_offset() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a {
                    text-underline-offset: auto;
                }
                .b {
                    text-underline-offset: 10%;
                }
                .c {
                    text-underline-offset: 3px;
                }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.text_underline_offset(), TextUnderlineOffset::Auto);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.text_underline_offset(),
            TextUnderlineOffset::Length(Length::Px(1.6))
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.text_underline_offset(),
            TextUnderlineOffset::Length(Length::Px(3.))
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.text_underline_offset(), TextUnderlineOffset::Auto);
    }

    #[test]
    fn text_decoration() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            text-decoration: underline;
        }
        .b {
            text-decoration: underline dotted;
        }
        .c {
            text-decoration: underline overline red;
        }
        .d {
            text-decoration: green wavy underline;
        }
    "#,
        );
        // println!("{:?}", ss);
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.text_decoration_line(),
            TextDecorationLine::List(vec![TextDecorationLineItem::Underline].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.text_decoration_line(),
            TextDecorationLine::List(vec![TextDecorationLineItem::Underline].into()),
        );
        assert_eq!(np.text_decoration_style(), TextDecorationStyle::Dotted);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.text_decoration_line(),
            TextDecorationLine::List(
                vec![
                    TextDecorationLineItem::Underline,
                    TextDecorationLineItem::Overline
                ]
                .into()
            ),
        );
        assert_eq!(np.text_decoration_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.text_decoration_line(),
            TextDecorationLine::List(vec![TextDecorationLineItem::Underline,].into()),
        );
        assert_eq!(np.text_decoration_color(), Color::Specified(0, 128, 0, 255));
        assert_eq!(np.text_decoration_style(), TextDecorationStyle::Wavy);
    }

    // 0xa4
    #[test]
    fn font_feature_settings() {
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            "normal",
            FontFeatureSettings::Normal
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#""liga""#,
            FontFeatureSettings::FeatureTags(
                vec![FeatureTag {
                    opentype_tag: "liga".into(),
                    value: Number::F32(1.),
                }]
                .into()
            )
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#""smcp" on"#,
            FontFeatureSettings::FeatureTags(
                vec![FeatureTag {
                    opentype_tag: "smcp".into(),
                    value: Number::F32(1.),
                }]
                .into()
            )
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#""swsh" off"#,
            FontFeatureSettings::FeatureTags(
                vec![FeatureTag {
                    opentype_tag: "swsh".into(),
                    value: Number::F32(0.),
                }]
                .into()
            )
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#""swsh" 2"#,
            FontFeatureSettings::FeatureTags(
                vec![FeatureTag {
                    opentype_tag: "swsh".into(),
                    value: Number::F32(2.),
                }]
                .into()
            )
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#""smcp", "swsh" 2"#,
            FontFeatureSettings::FeatureTags(
                vec![
                    FeatureTag {
                        opentype_tag: "smcp".into(),
                        value: Number::F32(1.),
                    },
                    FeatureTag {
                        opentype_tag: "swsh".into(),
                        value: Number::F32(2.),
                    }
                ]
                .into()
            )
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#""swsh" -1"#,
            FontFeatureSettings::Normal
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#"0 "swsh""#,
            FontFeatureSettings::Normal
        );
        test_parse_property!(
            font_feature_settings,
            "font-feature-settings",
            r#"xxxxxxx xxxx"#,
            FontFeatureSettings::Normal
        );
    }
