use super::*;


    // 0xd0
    #[test]
    fn list_style_type() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            list-style-type: decimal;
        }
        .b {
            list-style-type: cjk-decimal;
        }
        .c {
            list-style-type: "hello",
        }
        .d {
            list-style-type: hello,
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.list_style_type(), ListStyleType::Disc);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.list_style_type(), ListStyleType::Decimal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.list_style_type(), ListStyleType::CjkDecimal);
        // let query = StyleQuery::single("", "", Box::new(["d"]));
        // let mut np = NodeProperties::new(None);
        // ssg.query_single(&query, None, &MediaQueryStatus::default_screen(), &mut np);
        // assert_eq!(
        //     np.list_style_type(),
        //     ListStyleType::CustomIdent("hello".to_string().into())
        // );
    }

    // 0xd1
    #[test]
    fn list_style_image() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            list-style-image: url("wechat.gif");
        }
        .b {
            list-style-image: url(wechat.gif);
        }
        
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.list_style_image(), ListStyleImage::None);

        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.list_style_image(),
            ListStyleImage::Url("wechat.gif".to_string().into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.list_style_image(),
            ListStyleImage::Url("wechat.gif".to_string().into())
        );
    }
    // 0xd2
    #[test]
    fn list_style_position() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a {
                list-style-position: outside;
            }
            .b {
                list-style-position: inside;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.list_style_position(), ListStylePosition::Outside);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.list_style_position(), ListStylePosition::Outside);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.list_style_position(), ListStylePosition::Inside);
    }

    #[test]
    fn list_style() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            list-style: decimal outside url("wechat.gif");
        }
        .b {
            list-style: outside decimal url(wechat.gif);
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.list_style_position(), ListStylePosition::Outside);
        assert_eq!(np.list_style_type(), ListStyleType::Decimal);
        assert_eq!(
            np.list_style_image(),
            ListStyleImage::Url("wechat.gif".to_string().into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.list_style_position(), ListStylePosition::Outside);
        assert_eq!(np.list_style_type(), ListStyleType::Decimal);
        assert_eq!(
            np.list_style_image(),
            ListStyleImage::Url("wechat.gif".to_string().into())
        );
    }

    // 0xd3
    #[test]
    fn backdrop_filter() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            backdrop-filter: url(commonfilters.svg#filter);
        }
        .b {
            backdrop-filter: contrast(40%);
        }
        .c {
            backdrop-filter: hue-rotate(120deg);
        }
        .d {
            backdrop-filter: url(filters.svg#filter) blur(4px) saturate(150%);
        }
        .e {
            backdrop-filter: url(123) ssdad(10%);
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.backdrop_filter(),
            BackdropFilter::List(
                vec![FilterFunc::Url(
                    "commonfilters.svg#filter".to_string().into()
                ),]
                .into(),
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.backdrop_filter(),
            BackdropFilter::List(vec![FilterFunc::Contrast(Length::Ratio(0.4))].into(),)
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.backdrop_filter(),
            BackdropFilter::List(vec![FilterFunc::HueRotate(Angle::Deg(120.))].into(),)
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.backdrop_filter(),
            BackdropFilter::List(
                vec![
                    FilterFunc::Url("filters.svg#filter".to_string().into()),
                    FilterFunc::Blur(Length::Px(4.)),
                    FilterFunc::Saturate(Length::Ratio(1.5))
                ]
                .into(),
            )
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "blur()",
            BackdropFilter::List(vec![FilterFunc::Blur(Length::Px(0.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "hue-rotate()",
            BackdropFilter::List(vec![FilterFunc::HueRotate(Angle::Deg(0.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "invert()",
            BackdropFilter::List(vec![FilterFunc::Invert(Length::Ratio(0.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "opacity()",
            BackdropFilter::List(vec![FilterFunc::Opacity(Length::Ratio(1.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "brightness()",
            BackdropFilter::List(vec![FilterFunc::Brightness(Length::Ratio(1.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "contrast()",
            BackdropFilter::List(vec![FilterFunc::Contrast(Length::Ratio(1.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "grayscale()",
            BackdropFilter::List(vec![FilterFunc::Grayscale(Length::Ratio(0.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "sepia()",
            BackdropFilter::List(vec![FilterFunc::Sepia(Length::Ratio(0.))].into())
        );
        test_parse_property!(
            backdrop_filter,
            "backdrop-filter",
            "saturate()",
            BackdropFilter::List(vec![FilterFunc::Saturate(Length::Ratio(1.))].into())
        );
    }

    // 0xd4
    #[test]
    fn filter() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            filter: url(commonfilters.svg#filter);
        }
        .b {
            filter: contrast(40%);
        }
        .c {
            filter: hue-rotate(120deg);
        }
        .d {
            filter: url(filters.svg#filter) blur(4px) saturate(150%);
        }
        .e {
            filter: invert(1);
        }
        .f {
            filter: invert(75%);
        }
        .g {
            filter: blur(1%);
        }
        .h {
            filter: hue-rotate(120);
        }
    "#,
        );
        // println!("{:?}", ss);
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.filter(),
            Filter::List(
                vec![FilterFunc::Url(
                    "commonfilters.svg#filter".to_string().into()
                ),]
                .into(),
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.filter(),
            Filter::List(vec![FilterFunc::Contrast(Length::Ratio(0.4))].into(),)
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.filter(),
            Filter::List(vec![FilterFunc::HueRotate(Angle::Deg(120.))].into(),)
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.filter(),
            Filter::List(
                vec![
                    FilterFunc::Url("filters.svg#filter".to_string().into()),
                    FilterFunc::Blur(Length::Px(4.)),
                    FilterFunc::Saturate(Length::Ratio(1.5))
                ]
                .into(),
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.filter(),
            Filter::List(vec![FilterFunc::Invert(Length::Ratio(1.)),].into(),)
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.filter(),
            Filter::List(vec![FilterFunc::Invert(Length::Ratio(0.75)),].into(),)
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(np.filter(), Filter::None);
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(np.filter(), Filter::None);
    }

    // 0xd5
    #[test]
    fn transform_origin() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                transform-origin: top;
            }
            .b {
                transform-origin: right bottom;
            }
            .c {
                transform-origin: 20% 10px 10px;
            }
            .d {
                transform-origin: right right;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.transform_origin(),
            TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.), Length::Px(0.))
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.transform_origin(),
            TransformOrigin::LengthTuple(Length::Ratio(1.), Length::Ratio(1.), Length::Px(0.))
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.transform_origin(),
            TransformOrigin::LengthTuple(Length::Ratio(0.2), Length::Px(10.), Length::Px(10.))
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.transform_origin(),
            TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.5), Length::Px(0.))
        );
    }

    //0xd6
    #[test]
    fn mask_image() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a { mask-image: url("wechat.png"), url(wechat.png) }
                .b { mask-image: url("wechat.png"), none }
                .c { mask-image: linear-gradient(120deg, green 40%, red); }
                .d { mask-image: linear-gradient(green 20%, blue 75%, red); }
                .e { mask-image: radial-gradient(circle closest-corner at left bottom, green 20%, blue 75%, red);}
                .f { mask-image: radial-gradient(ellipse 20px 30% at 20% 30px, green 20%, blue 75%, red);}
                .g { mask-image: image(rtl url("wechat.png"), red); }
                .h { mask-image: image(url(wechat.png)) }
                .i { mask-image: element(#ele) }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                    BackgroundImageItem::Url("wechat.png".to_string().into())
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                    BackgroundImageItem::None
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(120.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.4)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.75)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.), Length::Ratio(1.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.75)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::Len(Length::Px(20.), Length::Ratio(0.3)),
                        GradientPosition::Pos(Length::Ratio(0.2), Length::Px(30.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.75)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Image(
                    ImageTags::RTL,
                    ImageSource::Url("wechat.png".to_string().into()),
                    Color::Specified(255, 0, 0, 255)
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Image(
                    ImageTags::LTR,
                    ImageSource::Url("wechat.png".to_string().into()),
                    Color::Undefined
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["i"], []);
        assert_eq!(
            np.mask_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Element("ele".to_string().into())].into()
            )
        );
    }

    //0xd7
    #[test]
    fn aspect_ratio() {
        test_parse_property!(aspect_ratio, "aspect-ratio", "auto", AspectRatio::Auto);
        test_parse_property!(
            aspect_ratio,
            "aspect-ratio",
            "1",
            AspectRatio::Ratio(Number::F32(1.), Number::F32(1.))
        );
        test_parse_property!(
            aspect_ratio,
            "aspect-ratio",
            "1/0.5",
            AspectRatio::Ratio(Number::F32(1.), Number::F32(0.5))
        );
        test_parse_property!(
            aspect_ratio,
            "aspect-ratio",
            "0.5/200",
            AspectRatio::Ratio(Number::F32(0.5), Number::F32(200.))
        );
        test_parse_property!(
            aspect_ratio,
            "aspect-ratio",
            "0.5    /    0.5",
            AspectRatio::Ratio(Number::F32(0.5), Number::F32(0.5))
        );
        test_parse_property!(aspect_ratio, "aspect-ratio", "0.5/", AspectRatio::Auto);
        test_parse_property!(aspect_ratio, "aspect-ratio", "/1", AspectRatio::Auto);
        test_parse_property!(aspect_ratio, "aspect-ratio", "/", AspectRatio::Auto);
    }

    //0xd8
    #[test]
    fn contain() {
        test_parse_property!(contain, "contain", "none", Contain::None);
        test_parse_property!(contain, "contain", "strict", Contain::Strict);
        test_parse_property!(contain, "contain", "content", Contain::Content);
        test_parse_property!(
            contain,
            "contain",
            "size",
            Contain::Multiple(vec![ContainKeyword::Size].into())
        );
        test_parse_property!(
            contain,
            "contain",
            "layout",
            Contain::Multiple(vec![ContainKeyword::Layout].into())
        );
        test_parse_property!(
            contain,
            "contain",
            "style",
            Contain::Multiple(vec![ContainKeyword::Style].into())
        );
        test_parse_property!(
            contain,
            "contain",
            "paint",
            Contain::Multiple(vec![ContainKeyword::Paint].into())
        );
        test_parse_property!(
            contain,
            "contain",
            "size layout style paint",
            Contain::Multiple(
                vec![
                    ContainKeyword::Size,
                    ContainKeyword::Layout,
                    ContainKeyword::Style,
                    ContainKeyword::Paint
                ]
                .into()
            )
        );
        test_parse_property!(
            contain,
            "contain",
            "paint size layout",
            Contain::Multiple(
                vec![
                    ContainKeyword::Size,
                    ContainKeyword::Layout,
                    ContainKeyword::Paint
                ]
                .into()
            )
        );
    }

    // 0xda
    #[test]
    fn touch_action() {
        test_parse_property!(touch_action, "touch-action", "auto", TouchAction::Auto);
        test_parse_property!(touch_action, "touch-action", "none", TouchAction::None);
        test_parse_property!(
            touch_action,
            "touch-action",
            "manipulation",
            TouchAction::Manipulation
        );
        test_parse_property!(
            touch_action,
            "touch-action",
            "pan-x",
            TouchAction::Gestures(TouchActionGestures {
                pan_left: true,
                pan_right: true,
                pan_up: false,
                pan_down: false
            })
        );
        test_parse_property!(
            touch_action,
            "touch-action",
            "pan-y",
            TouchAction::Gestures(TouchActionGestures {
                pan_left: false,
                pan_right: false,
                pan_up: true,
                pan_down: true
            })
        );
        test_parse_property!(
            touch_action,
            "touch-action",
            "pan-left",
            TouchAction::Gestures(TouchActionGestures {
                pan_left: true,
                pan_right: false,
                pan_up: false,
                pan_down: false
            })
        );
        test_parse_property!(
            touch_action,
            "touch-action",
            "pan-right",
            TouchAction::Gestures(TouchActionGestures {
                pan_left: false,
                pan_right: true,
                pan_up: false,
                pan_down: false
            })
        );
        test_parse_property!(
            touch_action,
            "touch-action",
            "pan-up",
            TouchAction::Gestures(TouchActionGestures {
                pan_left: false,
                pan_right: false,
                pan_up: true,
                pan_down: false
            })
        );
        test_parse_property!(
            touch_action,
            "touch-action",
            "pan-down",
            TouchAction::Gestures(TouchActionGestures {
                pan_left: false,
                pan_right: false,
                pan_up: false,
                pan_down: true
            })
        );
    }
