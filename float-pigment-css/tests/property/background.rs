use super::*;


    // 0x30
    #[test]
    fn background_color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { background-color: red }
        .b { background-color: rgba(255, 20, 10, 255) }
        .c { background-color: currentColor }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.background_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.background_color(), Color::Specified(255, 20, 10, 255));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.background_color(), Color::CurrentColor);
    }

    // 0x31
    #[test]
    fn background_image() {
        test_parse_property!(
            background_image,
            "background-image",
            r#"url("https://t7.baidu.com/it/u=963301259,1982396977&fm=193&f=GIF")"#,
            BackgroundImage::List(
                vec![BackgroundImageItem::Url(
                    "https://t7.baidu.com/it/u=963301259,1982396977&fm=193&f=GIF"
                        .to_string()
                        .into()
                ),]
                .into()
            )
        );
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { background-image: url("wechat.png") }
        .b { background-image: url("wechat.png"), url(wechat.png)}
        .c { background-image: url("wechat.png"), none}
        .d { background-image: linear-gradient(to right, green,red);}
        .e { background-image: linear-gradient(to bottom left, green, red);}
        .f { background-image: linear-gradient(120deg, green 40%, red);}
        .g { background-image: linear-gradient(green 20%, blue 75%, red);}
        .h { background-image: linear-gradient(to asd, green, red);}
        .i { background-image: radial-gradient(circle closest-corner at left bottom, green 20%, blue 75%, red);}
        .j { background-image: radial-gradient(farthest-corner at right, green 20%, blue 75%, red);}
        .k { background-image: radial-gradient(at bottom, green 20%, blue 75%, red);}
        .l { background-image: radial-gradient(circle at 20% 30px, green 20%, blue 75%, red);}
        .m { background-image: radial-gradient(ellipse 20px 30% at 20% 30px, green 20%, blue 75%, red);}
        .n { background-image: radial-gradient(circle 45px, green 20%, blue 75%, red);}
        .o { background-image: radial-gradient(green 20%, blue 75%, red);}
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Url("wechat.png".to_string().into()),].into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                    BackgroundImageItem::Url("wechat.png".to_string().into())
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                    BackgroundImageItem::None
                ]
                .into()
            )
        );

        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(90.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.)
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
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(225.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.background_image(),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(
            np.background_image(),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["i"], []);
        assert_eq!(
            np.background_image(),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["j"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(1.), Length::Ratio(0.5)),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["k"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(1.)),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );

        let np = query(&ssg, "", "", ["l"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::FarthestCorner,
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );

        let np = query(&ssg, "", "", ["m"], []);
        assert_eq!(
            np.background_image(),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );

        let np = query(&ssg, "", "", ["n"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::Len(Length::Px(45.), Length::Px(45.)),
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );

        let np = query(&ssg, "", "", ["o"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
    }

    #[test]
    fn background_image_gradient_position() {
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at left bottom, green 20%, blue 75%, red)",
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
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
    }
    // 0x32
    #[test]
    fn background_size() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { background-size: 50% auto, cover;}
      .b { background-size: contain, cover;}
      .c { background-size: auto, auto auto, auto 30%}
      .d { background-size: 45%; }
      .e { background-size: 25% 50%; }
      .f { background-size: auto 100px; }
      .g { background-size: asdsad }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(
                vec![
                    BackgroundSizeItem::Length(Length::Ratio(0.5), Length::Auto),
                    BackgroundSizeItem::Cover
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(
                vec![BackgroundSizeItem::Contain, BackgroundSizeItem::Cover].into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(
                vec![
                    BackgroundSizeItem::Length(Length::Auto, Length::Auto),
                    BackgroundSizeItem::Length(Length::Auto, Length::Auto),
                    BackgroundSizeItem::Length(Length::Auto, Length::Ratio(0.3))
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(
                vec![BackgroundSizeItem::Length(
                    Length::Ratio(0.45),
                    Length::Auto
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(
                vec![BackgroundSizeItem::Length(
                    Length::Ratio(0.25),
                    Length::Ratio(0.5)
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(
                vec![BackgroundSizeItem::Length(Length::Auto, Length::Px(100.)),].into()
            )
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Auto].into())
        );
    }

    // 0x33
    #[test]
    fn background_position() {
        // 1-value only keyword
        test_parse_property!(
            background_position,
            "background-position",
            "center",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "left",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.0)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "top",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(0.0))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        // 1-value only length
        test_parse_property!(
            background_position,
            "background-position",
            "20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.2)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )]
                .into()
            )
        );
        // 2-value only keyword
        test_parse_property!(
            background_position,
            "background-position",
            "left top",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "top top",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "right right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "center right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom center",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        // 2-value only length
        test_parse_property!(
            background_position,
            "background-position",
            "20% 75%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.2)),
                    BackgroundPositionValue::Top(Length::Ratio(0.75))
                )]
                .into()
            )
        );
        // 2-value with length & keyword
        test_parse_property!(
            background_position,
            "background-position",
            "20% right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "20% bottom",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.2)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "right 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.2))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "20% 70%, center",
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.2)),
                        BackgroundPositionValue::Top(Length::Ratio(0.7))
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5))
                    )
                ]
                .into()
            )
        );
        // 3-value
        test_parse_property!(
            background_position,
            "background-position",
            "right 20% 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom 20% 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "right 20% bottom",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Right(Length::Ratio(0.2)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "right bottom 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Bottom(Length::Ratio(0.2))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom right 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Right(Length::Ratio(0.2)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom 20% right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Bottom(Length::Ratio(0.2))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "center 20% right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom center 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        // 4-value
        test_parse_property!(
            background_position,
            "background-position",
            "left 20% bottom 60%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.2)),
                    BackgroundPositionValue::Bottom(Length::Ratio(0.6))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom 20% right 70%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Right(Length::Ratio(0.7)),
                    BackgroundPositionValue::Bottom(Length::Ratio(0.2))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "left left left left",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        test_parse_property!(
            background_position,
            "background-position",
            "bottom 20% center 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a {
                background-position: 10px, 10px;
                background-position-x: 20px;
                background-position-y: 30px;
            }
            .b {
                background-position: 10px, 10px !important;
                background-position-x: 20px;
                background-position-y: 30px;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Px(10.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5))
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Px(10.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5))
                    ),
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_position_x(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Px(20.)),
                ),]
                .into()
            )
        );
        assert_eq!(
            np.background_position_y(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Px(30.)
                ),),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Px(10.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5))
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Px(10.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5))
                    ),
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_position_x(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Px(10.)),),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Px(10.)),),
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_position_y(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ),
                    BackgroundPositionItem::Value(
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ),
                ]
                .into()
            )
        );
    }

    // 0x34
    #[test]
    fn background_repeat() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { background-repeat: no-repeat;}
        .b { background-repeat: repeat;}
        .c { background-repeat: repeat-x; }
        .d { background-repeat: repeat-y repeat-x; }
        .e { background-repeat: space no-repeat; }
        .f { background-repeat: round; }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::NoRepeat,
                    BackgroundRepeatValue::NoRepeat,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::Repeat,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::NoRepeat,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::Repeat,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Space,
                    BackgroundRepeatValue::NoRepeat,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Round,
                    BackgroundRepeatValue::Round,
                ),]
                .into()
            )
        );
    }

    // 0x35
    #[test]
    fn background_attachment() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { background-attachment: local ; }
      .b { background-attachment: fixed, scroll ; }

    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(vec![BackgroundAttachmentItem::Local].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(
                vec![
                    BackgroundAttachmentItem::Fixed,
                    BackgroundAttachmentItem::Scroll
                ]
                .into()
            )
        );
    }

    // 0x36
    #[test]
    fn background_clip() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
          .a { background-clip: border-box; }
          .b { background-clip: padding-box, content-box; }

        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(
                vec![
                    BackgroundClipItem::PaddingBox,
                    BackgroundClipItem::ContentBox
                ]
                .into()
            )
        );
    }

    // 0x37
    #[test]
    fn background_origin() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
              .a { background-origin: border-box; }
              .b { background-origin: padding-box, content-box; }

            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::BorderBox].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(
                vec![
                    BackgroundOriginItem::PaddingBox,
                    BackgroundOriginItem::ContentBox
                ]
                .into()
            )
        );
    }

    // 0x38
    #[test]
    fn background_position_x() {
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "10px, left, 20%",
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Px(10.))),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.))),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(
                        0.2
                    )))
                ]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "center, right 10%, left 20%",
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(
                        0.5
                    ))),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Right(Length::Ratio(
                        0.1
                    ))),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(
                        0.2
                    )))
                ]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "center 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(0.))
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "center, top 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(0.))
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "top",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(0.))
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "left",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(0.))
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "center",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(0.5))
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(1.))
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_position_x,
            "background-position-x",
            "right 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Right(Length::Ratio(0.2))
                ),]
                .into()
            )
        );
    }

    // 0x39
    #[test]
    fn background_position_y() {
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "10px, top, 20%",
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Px(10.))),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.))),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.2)))
                ]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "10px, left, 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "center",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.5)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "center 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "center, left 20%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "top",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "center",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.5)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "bottom",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(1.)
                )),]
                .into()
            )
        );
        test_parse_property!(
            background_position_y,
            "background-position-y",
            "bottom 10%",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Bottom(Length::Ratio(0.1))
                ),]
                .into()
            )
        );
    }

    #[test]
    fn background() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a { background: repeat url("wechat.png") center / cover, no-repeat url("wechat.png") red; }
                .b { background: repeat url("wechat.png"), no-repeat url("wechat.png") left 30% / cover red; }
                .c { background: repeat url("wechat.png") border-box padding-box fixed right 40% / cover red;}
                .d { background: red 50%, center }
                .e { background: green repeat-x bottom center; }
                .f { background: red left right; }
                .g { background: url("hello"), red; }
                .h { background: none }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![
                    BackgroundRepeatItem::Pos(
                        BackgroundRepeatValue::Repeat,
                        BackgroundRepeatValue::Repeat,
                    ),
                    BackgroundRepeatItem::Pos(
                        BackgroundRepeatValue::NoRepeat,
                        BackgroundRepeatValue::NoRepeat,
                    ),
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                ]
                .into()
            )
        );
        assert_eq!(np.background_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5))
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.))
                    )
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Cover, BackgroundSizeItem::Auto].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![
                    BackgroundRepeatItem::Pos(
                        BackgroundRepeatValue::Repeat,
                        BackgroundRepeatValue::Repeat,
                    ),
                    BackgroundRepeatItem::Pos(
                        BackgroundRepeatValue::NoRepeat,
                        BackgroundRepeatValue::NoRepeat,
                    ),
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                    BackgroundImageItem::Url("wechat.png".to_string().into()),
                ]
                .into()
            )
        );
        assert_eq!(np.background_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.))
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.3))
                    )
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Auto, BackgroundSizeItem::Cover].into())
        );

        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::Repeat,
                ),]
                .into()
            )
        );
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Url("wechat.png".to_string().into()),].into()
            )
        );
        assert_eq!(np.background_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.4))
                ),]
                .into()
            )
        );
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Cover,].into())
        );
        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::BorderBox].into())
        );
        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::PaddingBox].into())
        );
        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(vec![BackgroundAttachmentItem::Fixed].into())
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::Repeat
                )]
                .into()
            )
        );
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        assert_eq!(np.background_color(), Color::Specified(0, 0, 0, 0));
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Auto].into())
        );
        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::PaddingBox].into())
        );
        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );
        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(vec![BackgroundAttachmentItem::Scroll].into())
        );

        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::NoRepeat
                )]
                .into()
            )
        );
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(vec![BackgroundImageItem::None].into())
        );
        assert_eq!(np.background_color(), Color::Specified(0, 128, 0, 255));
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(1.))
                )]
                .into()
            )
        );
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Auto].into())
        );
        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::PaddingBox].into())
        );
        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );
        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(vec![BackgroundAttachmentItem::Scroll].into())
        );

        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.background_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Repeat,
                    BackgroundRepeatValue::Repeat
                )]
                .into()
            )
        );
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        assert_eq!(np.background_color(), Color::Specified(0, 0, 0, 0));
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        assert_eq!(
            np.background_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Auto].into())
        );
        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::PaddingBox].into())
        );
        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );
        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(vec![BackgroundAttachmentItem::Scroll].into())
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.))
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.))
                    )
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_position_x(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                    ),
                    BackgroundPositionItem::Value(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                    )
                ]
                .into()
            )
        );
        assert_eq!(
            np.background_position_y(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.)),),
                    BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.)),)
                ]
                .into()
            )
        );

        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(np.background_color(), Color::Specified(0, 0, 0, 0),);
        assert_eq!(
            np.background_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.))
                )]
                .into()
            )
        );
        assert_eq!(
            np.background_position_x(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                )]
                .into()
            )
        );
        assert_eq!(
            np.background_position_y(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.)
                ))]
                .into()
            )
        );
        assert_eq!(
            np.background_position_y(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                    Length::Ratio(0.)
                ))]
                .into()
            )
        );

        assert_eq!(
            np.background_attachment(),
            BackgroundAttachment::List(vec![BackgroundAttachmentItem::Scroll].into())
        );

        assert_eq!(
            np.background_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );

        assert_eq!(
            np.background_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::PaddingBox].into())
        );
    }
