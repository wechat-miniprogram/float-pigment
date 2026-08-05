use super::*;

    #[test]
    fn linear_gradient() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a { background-image: linear-gradient(red, green, blue); }
                .b { background-image: linear-gradient(red, green 20%, blue 10%, yellow 30%, pink 20%, black)}
                .c { background-image: linear-gradient(red 10%, green, blue, black) }
                .d { background-image: linear-gradient(red 10% 20%, green, blue, black)}
                .e { background-image: linear-gradient(red, yellow 10%, blue, green, pink 60%, orange, black) }
                .f { background-image: linear-gradient(red, yellow 10%, blue 10px, green, pink 60%, orange 80%, black) }
                .g { background-image: linear-gradient(red) }
                .h { background-image: linear-gradient(to 20px, red, blue) }
                .i { background-image: linear-gradient(red 10%, green, blue 200px, black 80%) }
                .j { background-image: linear-gradient(to top, red, green, blue) }
                .k { background-image: linear-gradient(to top right, red, green, blue) }
                .l { background-image: linear-gradient(to left bottom, red, green, blue) }
                .m { background-image: linear-gradient(120deg to top, red, green, blue) }
                .n { background-image: linear-gradient(to top 120deg, red, green, blue) }
                .o { background-image: linear-gradient(to 120deg, red, green, blue) }
                .p { background-image: linear-gradient(red 0%, 20%, green, blue) }
                .q { background-image: linear-gradient(0%, red 20% 20%, green, blue) }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 255, 0, 255),
                                Length::Ratio(0.3)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 192, 203, 255),
                                Length::Ratio(0.3)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.1)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.39999998)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.7)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.1)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.4666667)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.73333335)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.0)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 255, 0, 255),
                                Length::Ratio(0.1)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.26666668)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.43333334)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 192, 203, 255),
                                Length::Ratio(0.6)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 165, 0, 255),
                                Length::Ratio(0.8)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
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
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.0)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 255, 0, 255),
                                Length::Ratio(0.1)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Px(10.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Auto
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 192, 203, 255),
                                Length::Ratio(0.6)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 165, 0, 255),
                                Length::Ratio(0.8)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
                                Length::Ratio(1.)
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
            np.background_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Gradient(BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![GradientColorItem::ColorHint(
                            Color::Specified(255, 0, 0, 255),
                            Length::Ratio(1.)
                        ),]
                        .into()
                    ))
                    .into()
                ]
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
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.1)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Auto
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Px(200.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
                                Length::Ratio(0.8)
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
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(0.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.)
                            ),
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
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(45.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.)
                            ),
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
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(225.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.)
                            ),
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["m"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["n"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["o"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["p"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::LinearGradient(
                        Angle::Deg(180.),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.20)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.6)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.)
                            ),
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["q"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
    }
    #[test]
    fn radial_gradient() {
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle at top, red, green)",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(1.)
                            ),
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(red, green, blue)",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(red, green 20%, blue 10%, yellow 30%, pink 20%, black)",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(0.2)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 255, 0, 255),
                                Length::Ratio(0.3)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 192, 203, 255),
                                Length::Ratio(0.3)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 0, 255),
                                Length::Ratio(1.)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-side at right bottom, red, green, blue)",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestSide,
                        GradientPosition::Pos(Length::Ratio(1.), Length::Ratio(1.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(50px circle at 10px 10px, green, lightgreen)",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::Len(Length::Px(50.), Length::Px(50.)),
                        GradientPosition::Pos(Length::Px(10.), Length::Px(10.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(144, 238, 144, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at right 20px, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(1.), Length::Px(20.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        // <position-one>
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at bottom, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(1.0)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at center, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at left, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.0), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at 30px, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Px(30.0), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        // <position-two>
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at left top, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.0), Length::Ratio(0.0)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at 30% bottom, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.3), Length::Ratio(1.0)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at bottom 30%, red, green, blue);",
            BackgroundImage::List(vec![].into())
        );

        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at center center, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );

        // <position-four>
        test_parse_property!(
            background_image,
            "background-image",
            "radial-gradient(circle closest-corner at left 20px bottom 10%, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::SpecifiedPos(
                            GradientSpecifiedPos::Left(Length::Px(20.0)),
                            GradientSpecifiedPos::Bottom(Length::Ratio(0.1))
                        ),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .d { background-image: radial-gradient(ellipse farthest-corner at bottom, red, green, blue);}
                .e { background-image: radial-gradient(ellipse farthest-side at 20px top, red, green, blue);}
                .f { background-image: radial-gradient(circle closest-corner at right 20px, red, green, blue);}
                .g { background-image: radial-gradient(circle 20px at bottom, red, green, blue);}
                .h { background-image: radial-gradient(ellipse 20px 30px at bottom, red, green, blue);}
                .i { background-image: radial-gradient(20px 30% at 20% 30px, red, green, blue);}
                .j { background-image: radial-gradient(20% 30px at bottom circle, red, black) }
                .k { background-image: radial-gradient(red) }
                .l { background-image: radial-gradient(closest-corner circle at bottom, red, green, blue) }
                .m { background-image: radial-gradient(20px 20px circle, red, green, blue) }
                .n { background-image: radial-gradient(closest-corner 20px at bottom, red, green, blue) }
                .o { background-image: radial-gradient(closest-corner 20px circle at bottom, red, green, blue) }
                .p { background-image: radial-gradient(20px 30px, closest-corner, red, green, blue) }
                .q { background-image: radial-gradient(20px 30px ellipse, red, green, blue) }
                .r { background-image: radial-gradient(20px circle at bottom, red, green, blue);}
                .s { background-image: radial-gradient(20px ellipse at bottom, red, green, blue);}
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["d"], []);
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
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
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
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestSide,
                        GradientPosition::Pos(Length::Px(20.), Length::Ratio(0.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
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
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(1.), Length::Px(20.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
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
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::Len(Length::Px(20.), Length::Px(20.)),
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(1.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::Len(Length::Px(20.), Length::Px(30.)),
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(1.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["i"], []);
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
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["j"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["k"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![
                    BackgroundImageItem::Gradient(BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::FarthestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        vec![GradientColorItem::ColorHint(
                            Color::Specified(255, 0, 0, 255),
                            Length::Ratio(1.)
                        )]
                        .into()
                    ))
                    .into()
                ]
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
                        GradientSize::ClosestCorner,
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(1.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["m"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["n"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["o"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["p"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
        let np = query(&ssg, "", "", ["q"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Ellipse,
                        GradientSize::Len(Length::Px(20.), Length::Px(30.)),
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["r"], []);
        assert_eq!(
            np.background_image(),
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        GradientShape::Circle,
                        GradientSize::Len(Length::Px(20.), Length::Px(20.)),
                        GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(1.)),
                        vec![
                            GradientColorItem::ColorHint(
                                Color::Specified(255, 0, 0, 255),
                                Length::Ratio(0.)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 128, 0, 255),
                                Length::Ratio(0.5)
                            ),
                            GradientColorItem::ColorHint(
                                Color::Specified(0, 0, 255, 255),
                                Length::Ratio(1.0)
                            )
                        ]
                        .into()
                    )
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["s"], []);
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
    }

    #[test]
    fn conic_gradient_repr() {
        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(0.),
                        position: GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(1.)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(red 50%, green 30%, blue 20%);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(0.),
                        position: GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(0.5)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(red 50%, green, blue 20%);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(0.),
                        position: GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(0.5)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(red, green calc(25% + 25%), blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(0.),
                        position: GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(1.)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(red, green calc(90deg + 25%), blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(0.),
                        position: GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(1.)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(from 190deg at 20% 30%, red, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(190.),
                        position: GradientPosition::Pos(Length::Ratio(0.2), Length::Ratio(0.3)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.5)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(1.)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(from 190deg at 20% 30%, red 20% 40%, green, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(190.),
                        position: GradientPosition::Pos(Length::Ratio(0.2), Length::Ratio(0.3)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.2)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.4)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.70000005)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(1.)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );

        test_parse_property!(
            background_image,
            "background-image",
            "conic-gradient(from 190deg at 20% 30%, red 20% 30%, green 30deg, blue);",
            BackgroundImage::List(
                vec![BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle: Angle::Deg(190.),
                        position: GradientPosition::Pos(Length::Ratio(0.2), Length::Ratio(0.3)),
                        items: vec![
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.2)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(255, 0, 0, 255),
                                AngleOrPercentage::Percentage(0.3)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 128, 0, 255),
                                AngleOrPercentage::Percentage(0.3)
                            ),
                            GradientColorItem::AngleOrPercentageColorHint(
                                Color::Specified(0, 0, 255, 255),
                                AngleOrPercentage::Percentage(1.)
                            )
                        ]
                        .into()
                    })
                ),]
                .into()
            )
        );
    }
