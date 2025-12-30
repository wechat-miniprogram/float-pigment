use float_pigment_css::{typing::*, StyleSheet, StyleSheetGroup};

mod utils;
use utils::*;

mod base {
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
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
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
        assert_eq!(np.background_image(), BackgroundImage::List(vec![].into()));
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
}
mod position {
    use super::*;
    // 0x01 Display
    #[test]
    fn display() {
        test_parse_property!(display, "display", "none", Display::None);
        test_parse_property!(display, "display", "block", Display::Block);
        test_parse_property!(display, "display", "flex", Display::Flex);
        test_parse_property!(display, "display", "inline", Display::Inline);
        test_parse_property!(display, "display", "inline-block", Display::InlineBlock);
        test_parse_property!(display, "display", "grid", Display::Grid);
        test_parse_property!(display, "display", "flow-root", Display::FlowRoot);
    }

    // 0x02 Position
    #[test]
    fn position() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { position: absolute }
            .b { position: fixed }
            .c { position: relative }
            .d { position: sticky }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.position(), Position::Static);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.position(), Position::Absolute);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.position(), Position::Fixed);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.position(), Position::Relative);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.position(), Position::Sticky);
    }

    // 0x03 OverflowX
    #[test]
    fn overflow_x() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { overflow-x: hidden }
            .b { overflow-x: auto }
            .c { overflow-x: scroll }
            .d { overflow-x: visible }
        "#,
        );
        // println!("{:?}", ss);
        ssg.append(ss);
        let np = query(&ssg, "", "", [], []);
        assert_eq!(np.overflow_x(), Overflow::Visible);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.overflow_x(), Overflow::Hidden);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.overflow_x(), Overflow::Auto);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.overflow_x(), Overflow::Scroll);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.overflow_x(), Overflow::Visible);
    }

    // 0x04 OverflowY
    #[test]
    fn overflow_y() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { overflow-y: hidden }
            .b { overflow-y: auto }
            .c { overflow-y: scroll }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [], []);
        assert_eq!(np.overflow_y(), Overflow::Visible);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.overflow_y(), Overflow::Hidden);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.overflow_y(), Overflow::Auto);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.overflow_y(), Overflow::Scroll);
    }

    // Overflow
    #[test]
    fn overflow() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { overflow: auto }
            .b { overflow: scroll hidden }
            .c { overflow: visible auto }
        "#,
        );
        // println!("{:?}", ss);
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.overflow_x(), Overflow::Visible);
        assert_eq!(np.overflow_y(), Overflow::Visible);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.overflow_x(), Overflow::Auto);
        assert_eq!(np.overflow_y(), Overflow::Auto);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.overflow_x(), Overflow::Scroll);
        assert_eq!(np.overflow_y(), Overflow::Hidden);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.overflow_x(), Overflow::Visible);
        assert_eq!(np.overflow_y(), Overflow::Auto);
    }

    // 0x05 PointerEvents
    #[test]
    fn pointer_events() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { pointer-events: auto}
        .b { pointer-events: none}
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [], []);
        assert_eq!(np.pointer_events(), PointerEvents::Auto);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.pointer_events(), PointerEvents::Auto);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.pointer_events(), PointerEvents::None);
    }

    // 0x06 WxEngineTouchEvent
    #[test]
    fn wx_engine_touch_event() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { -wx-engine-touch-event: gesture}
            .b { -wx-engine-touch-event: click}
            .c { -wx-engine-touch-event: none}
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_engine_touch_event(), WxEngineTouchEvent::Gesture);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_engine_touch_event(), WxEngineTouchEvent::Click);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.wx_engine_touch_event(), WxEngineTouchEvent::None);
    }

    // 0x07 WxPartialZIndex
    #[test]
    fn wx_partial_z_index() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { -wx-partial-z-index: 0; }
            .b { -wx-partial-z-index: 999; }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_partial_z_index(), Number::F32(0.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_partial_z_index(), Number::F32(999.));
    }

    // 0x08 BoxSizing
    #[test]
    fn box_sizing() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { box-sizing: border-box }
            .b { box-sizing: padding-box }
            .c { box-sizing: content-box }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.box_sizing(), BoxSizing::BorderBox);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.box_sizing(), BoxSizing::PaddingBox);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.box_sizing(), BoxSizing::ContentBox);
    }

    // 0x09 Transform
    #[test]
    fn transform() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
              transform: matrix(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);   
            }
            .b {
              transform: translate(12px, 50%);
            }
            .c {
              transform: translateX(2em) translateY(3em);
            }
            .d {
              transform: scale(2, 0.5) rotate(0.5turn) skew(30deg, 20deg); 
            }
            .e {
              transform: matrix3d(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
            }
            .f {
                transform: translate(12px);
            }
            .g {
                transform: skew(10deg);
            }
            .h {
                transform: scale(2);
            }
            .i {
                transform: rotate3d(1, 2, 3, 10deg);
            }
            .j {
                transform: none;
            }
            .k {
                transform: scale(50%);
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(vec![TransformItem::Matrix([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(
                vec![TransformItem::Translate2D(
                    Length::Px(12.),
                    Length::Ratio(0.5)
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(
                vec![
                    TransformItem::Translate2D(Length::Px(32.), Length::Px(0.)),
                    TransformItem::Translate2D(Length::Px(0.), Length::Px(48.))
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(
                vec![
                    TransformItem::Scale2D(2.0, 0.5),
                    TransformItem::Rotate2D(Angle::Turn(0.5)),
                    TransformItem::Skew(Angle::Deg(30.), Angle::Deg(20.))
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(
                vec![TransformItem::Matrix3D([
                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11., 12., 13., 14., 15.,
                    16.,
                ])]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(
                vec![TransformItem::Translate2D(Length::Px(12.), Length::Px(0.)),].into()
            )
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(vec![TransformItem::Skew(Angle::Deg(10.), Angle::Deg(10.))].into())
        );
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(vec![TransformItem::Scale2D(2.0, 2.0)].into())
        );
        let np = query(&ssg, "", "", ["i"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(vec![TransformItem::Rotate3D(1., 2., 3., Angle::Deg(10.))].into())
        );
        let np = query(&ssg, "", "", ["j"], []);
        assert_eq!(np.transform(), Transform::Series(vec![].into()));
        let np = query(&ssg, "", "", ["k"], []);
        assert_eq!(
            np.transform(),
            Transform::Series(vec![TransformItem::Scale2D(0.5, 0.5)].into())
        );
    }

    // 0x0a WxLineClamp
    #[test]
    fn wx_line_clamp() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a {
                -wx-line-clamp: 11;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_line_clamp(), Number::F32(11.));
    }

    // 0x0b Float
    #[test]
    fn float() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a {
                float: left;
            }
            .b {
                float: inline-start;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.float(), Float::None);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.float(), Float::Left);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.float(), Float::InlineStart);
    }

    // 0x0c OverflowWrap
    #[test]
    fn overflow_wrap() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { overflow-wrap: normal }
            .b { overflow-wrap: break-word }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.overflow_wrap(), OverflowWrap::Normal);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.overflow_wrap(), OverflowWrap::BreakWord);
    }

    // 0x0d Resize
    #[test]
    fn resize() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            resize: both;
        }
        .b {
            resize: block;
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.resize(), Resize::Both);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.resize(), Resize::Block);
    }

    // 0x0e ZIndex
    #[test]
    fn z_index() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { z-index: auto; }
            .b { z-index: 999; }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.z_index(), ZIndex::Auto);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.z_index(), ZIndex::Num(Number::I32(999)));
    }
}

mod visibility_color {
    use super::*;
    // 0x10 Visibility
    #[test]
    fn visibility() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { visibility: visible }
        .b { visibility: hidden }
        .c { visibility: collapse }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.visibility(), Visibility::Visible);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.visibility(), Visibility::Visible);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.visibility(), Visibility::Hidden);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.visibility(), Visibility::Collapse);
    }
    // 0x11 Color
    #[test]
    fn color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            "
            .a { color: currentColor }
            .b { color: rgba(4, 3, 2, 1) }
            .c { color: red; }
            .d { color: #ff0000 }
        ",
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.color(), Color::Specified(0, 0, 0, 255));
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.color(), Color::CurrentColor);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.color(), Color::Specified(4, 3, 2, 255));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.color(), Color::Specified(255, 0, 0, 255));
        {
            let np = query(&ssg, "", "", ["d"], []);
            assert_eq!(np.color(), Color::Specified(255, 0, 0, 255));
        }
    }
    // 0x12 Opacity
    #[test]
    fn opacity() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { opacity: 0.5 }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.opacity(), Number::F32(1.));
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.opacity(), Number::F32(0.5));
    }
}

mod flex {
    use super::*;
    // 0x20
    #[test]
    fn flex_direction() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { flex-direction: row }
            .b { flex-direction: column }
            .c { flex-direction: row-reverse }
            .d { flex-direction: column-reverse }
        "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.flex_direction(), FlexDirection::Row);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.flex_direction(), FlexDirection::Row);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.flex_direction(), FlexDirection::Column);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.flex_direction(), FlexDirection::RowReverse);
        }
        {
            let np = query(&ssg, "", "", ["d"], []);
            assert_eq!(np.flex_direction(), FlexDirection::ColumnReverse);
        }
    }

    // 0x21
    #[test]
    fn flex_wrap() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
          .a { flex-wrap: wrap }
          .b { flex-wrap: wrap-reverse }
          .c { flex-wrap: nowrap }
        "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.flex_wrap(), FlexWrap::NoWrap);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.flex_wrap(), FlexWrap::Wrap);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.flex_wrap(), FlexWrap::WrapReverse);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.flex_wrap(), FlexWrap::NoWrap);
        }
    }

    // 0x22
    #[test]
    fn align_items() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { align-items: center }
      .b { align-items: flex-start }
      .c { align-items: flex-end }
      .d { align-items: baseline }
      .e { align-items: normal }
      .f { align-items: start }
      .g { align-items: end }
      .h { align-items: self-start }
      .i { align-items: self-end }
      .j { align-items: space-around }
      .k { align-items: space-between }
      .l { align-items: auto }
    "#,
        );
        // println!("{:?}", ss);
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.align_items(), AlignItems::Stretch);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.align_items(), AlignItems::Center);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.align_items(), AlignItems::FlexStart);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.align_items(), AlignItems::FlexEnd);
        }
        {
            let np = query(&ssg, "", "", ["d"], []);
            assert_eq!(np.align_items(), AlignItems::Baseline);
        }
        {
            let np = query(&ssg, "", "", ["e"], []);
            assert_eq!(np.align_items(), AlignItems::Normal);
        }
        {
            let np = query(&ssg, "", "", ["f"], []);
            assert_eq!(np.align_items(), AlignItems::Start);
        }
        {
            let np = query(&ssg, "", "", ["g"], []);
            assert_eq!(np.align_items(), AlignItems::End);
        }
        {
            let np = query(&ssg, "", "", ["h"], []);
            assert_eq!(np.align_items(), AlignItems::SelfStart);
        }
        {
            let np = query(&ssg, "", "", ["i"], []);
            assert_eq!(np.align_items(), AlignItems::SelfEnd);
        }
    }

    // 0x23
    #[test]
    fn align_self() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { align-self: stretch }
        .b { align-self: center}
        .c { align-self: flex-start}
        .d { align-self: flex-end}
        .f { align-self: baseline}
        .g { align-self: space-between }
        .h { align-self: space-around }
        .i { align-self: start }
        .j { align-self: end }
        .k { align-self: self-start }
        .l { align-self: self-end }
        .m { align-self: normal }
        .n { align-self: auto }
    "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.align_self(), AlignSelf::Auto);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.align_self(), AlignSelf::Stretch);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.align_self(), AlignSelf::Center);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.align_self(), AlignSelf::FlexStart);
        }
        {
            let np = query(&ssg, "", "", ["d"], []);
            assert_eq!(np.align_self(), AlignSelf::FlexEnd);
        }
        // {
        //     let query = StyleQuery::single("", "", Box::new(["e"]));
        //     let mut np = NodeProperties::new(None);
        //     ssg.query_single(&query, None, &MediaQueryStatus::default_screen(), &mut np);
        //     assert_eq!(np.align_self(), AlignSelf::Unset);
        // }
        {
            let np = query(&ssg, "", "", ["f"], []);
            assert_eq!(np.align_self(), AlignSelf::Baseline);
        }
        {
            let np = query(&ssg, "", "", ["i"], []);
            assert_eq!(np.align_self(), AlignSelf::Start);
        }
        {
            let np = query(&ssg, "", "", ["j"], []);
            assert_eq!(np.align_self(), AlignSelf::End);
        }
        {
            let np = query(&ssg, "", "", ["k"], []);
            assert_eq!(np.align_self(), AlignSelf::SelfStart);
        }
        {
            let np = query(&ssg, "", "", ["l"], []);
            assert_eq!(np.align_self(), AlignSelf::SelfEnd);
        }
        {
            let np = query(&ssg, "", "", ["m"], []);
            assert_eq!(np.align_self(), AlignSelf::Normal);
        }
        {
            let np = query(&ssg, "", "", ["n"], []);
            assert_eq!(np.align_self(), AlignSelf::Auto);
        }
    }

    // 0x24
    #[test]
    fn align_content() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { align-content: start }
            .b { align-content: end}
            .c { align-content: stretch }
            .d { align-content: auto }
            .e { align-content: flex-start }
            .f { align-content: flex-end }
            .g { align-content: center }
            .h { align-content: baseline }
            .i { align-content: space-between }
            .j { align-content: space-around }
            .k { align-content: space-evenly }
        "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.align_content(), AlignContent::Stretch);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.align_content(), AlignContent::Start);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.align_content(), AlignContent::End);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.align_content(), AlignContent::Stretch);
        }
        {
            let np = query(&ssg, "", "", ["e"], []);
            assert_eq!(np.align_content(), AlignContent::FlexStart);
        }
        {
            let np = query(&ssg, "", "", ["f"], []);
            assert_eq!(np.align_content(), AlignContent::FlexEnd);
        }
        {
            let np = query(&ssg, "", "", ["g"], []);
            assert_eq!(np.align_content(), AlignContent::Center);
        }
        {
            let np = query(&ssg, "", "", ["h"], []);
            assert_eq!(np.align_content(), AlignContent::Baseline);
        }
        {
            let np = query(&ssg, "", "", ["i"], []);
            assert_eq!(np.align_content(), AlignContent::SpaceBetween);
        }
        {
            let np = query(&ssg, "", "", ["j"], []);
            assert_eq!(np.align_content(), AlignContent::SpaceAround);
        }
        {
            let np = query(&ssg, "", "", ["k"], []);
            assert_eq!(np.align_content(), AlignContent::SpaceEvenly);
        }
    }

    // 0x25
    #[test]
    fn justify_content() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { justify-content: flex-end }
      .b { justify-content: center }
      .c { justify-content: space-around }
      .d { justify-content: space-between }
      .f { justify-content: space-evenly }
      .g { justify-content: flex-start}
      .h { justify-content: start }
      .i { justify-content: end }
      .j { justify-content: left }
      .k { justify-content: right }
      .l { justify-content: baseline }
      .m { justify-content: stretch }
    "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.justify_content(), JustifyContent::FlexStart);
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.justify_content(), JustifyContent::FlexEnd);
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.justify_content(), JustifyContent::Center);
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(np.justify_content(), JustifyContent::SpaceAround);
        }
        {
            let np = query(&ssg, "", "", ["d"], []);
            assert_eq!(np.justify_content(), JustifyContent::SpaceBetween);
        }
        // {
        //     let query = StyleQuery::single("", "", Box::new(["e"]));
        //     let mut np = NodeProperties::new(None);
        //     ssg.query_single(&query, None, &MediaQueryStatus::default_screen(), &mut np);
        //     assert_eq!(np.justify_content(), JustifyContent::Unset);
        // }
        {
            let np = query(&ssg, "", "", ["f"], []);
            assert_eq!(np.justify_content(), JustifyContent::SpaceEvenly);
        }
        {
            let np = query(&ssg, "", "", ["g"], []);
            assert_eq!(np.justify_content(), JustifyContent::FlexStart);
        }
        {
            let np = query(&ssg, "", "", ["h"], []);
            assert_eq!(np.justify_content(), JustifyContent::Start);
        }
        {
            let np = query(&ssg, "", "", ["i"], []);
            assert_eq!(np.justify_content(), JustifyContent::End);
        }
        {
            let np = query(&ssg, "", "", ["j"], []);
            assert_eq!(np.justify_content(), JustifyContent::Left);
        }
        {
            let np = query(&ssg, "", "", ["k"], []);
            assert_eq!(np.justify_content(), JustifyContent::Right);
        }
        {
            let np = query(&ssg, "", "", ["l"], []);
            assert_eq!(np.justify_content(), JustifyContent::Baseline);
        }
        {
            let np = query(&ssg, "", "", ["m"], []);
            assert_eq!(np.justify_content(), JustifyContent::Stretch);
        }
    }

    // 0x26
    #[test]
    fn flex_grow() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { flex-grow: 2 }
        .b { flex-grow: 0.6}
    "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.flex_grow(), Number::F32(0.));
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.flex_grow(), Number::F32(2.));
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.flex_grow(), Number::F32(0.6));
        }
    }

    // 0x27
    #[test]
    fn flex_shrink() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { flex-shrink: 2 }
        .b { flex-shrink: 0.6 }
    "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", [], []);
            assert_eq!(np.flex_shrink(), Number::F32(1.));
        }
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(np.flex_shrink(), Number::F32(2.));
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(np.flex_shrink(), Number::F32(0.6));
        }
    }

    // 0x28
    #[test]
    fn flex_basis() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { flex-basis: auto }
            .b { flex-basis: 200px }
            .c { flex-basis: 30em }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [], []);
        assert_eq!(np.flex_basis(), Length::Undefined);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.flex_basis(), Length::Auto);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.flex_basis(), Length::Px(200.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.flex_basis(), Length::Px(480.));
    }

    // 0x29
    #[test]
    fn justify_items() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { justify-items: stretch  }
            .b { justify-items: center }
            .c { justify-items: start }
            .d { justify-items: end }
            .e { justify-items: flex-start  }
            .f { justify-items: flex-end }
            .g { justify-items: self-start }
            .h { justify-items: self-end }
            .i { justify-items: left }
            .j { justify-items: right }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.justify_items(), JustifyItems::Stretch);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.justify_items(), JustifyItems::Center);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.justify_items(), JustifyItems::Start);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.justify_items(), JustifyItems::End);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.justify_items(), JustifyItems::FlexStart);
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(np.justify_items(), JustifyItems::FlexEnd);
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(np.justify_items(), JustifyItems::SelfStart);
        let np = query(&ssg, "", "", ["h"], []);
        assert_eq!(np.justify_items(), JustifyItems::SelfEnd);
        let np = query(&ssg, "", "", ["i"], []);
        assert_eq!(np.justify_items(), JustifyItems::Left);
        let np = query(&ssg, "", "", ["j"], []);
        assert_eq!(np.justify_items(), JustifyItems::Right);
    }
    // 0x2a
    #[test]
    fn order() {
        test_parse_property!(order, "order", "1", Number::I32(1));
        test_parse_property!(order, "order", "-100", Number::I32(-100));
    }
    // 0x2b
    #[test]
    fn row_gap() {
        test_parse_property!(row_gap, "row-gap", "normal", Gap::Normal);
        test_parse_property!(row_gap, "row-gap", "10px", Gap::Length(Length::Px(10.)));
    }

    // 0x2c
    #[test]
    fn column_gap() {
        test_parse_property!(column_gap, "column-gap", "normal", Gap::Normal);
        test_parse_property!(column_gap, "column-gap", "-10%", Gap::Normal);
    }

    #[test]
    fn gap() {
        test_parse_property!(row_gap, "gap", "normal", Gap::Normal);
        test_parse_property!(column_gap, "gap", "normal", Gap::Normal);

        test_parse_property!(row_gap, "gap", "30px", Gap::Length(Length::Px(30.)));
        test_parse_property!(column_gap, "gap", "20px", Gap::Length(Length::Px(20.)));

        test_parse_property!(row_gap, "gap", "normal 10px", Gap::Normal);
        test_parse_property!(row_gap, "gap", "10px normal", Gap::Length(Length::Px(10.)));
        test_parse_property!(
            column_gap,
            "gap",
            "normal 10px",
            Gap::Length(Length::Px(10.))
        );
        test_parse_property!(column_gap, "gap", "10px normal", Gap::Normal);

        test_parse_property!(row_gap, "gap", "30px 40px", Gap::Length(Length::Px(30.)));
        test_parse_property!(column_gap, "gap", "30px 40px", Gap::Length(Length::Px(40.)));
    }

    #[test]
    fn flex_flow() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
                .a { flex-flow: column wrap; }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.flex_direction(), FlexDirection::Column);
        assert_eq!(np.flex_wrap(), FlexWrap::Wrap);
    }

    #[test]
    fn flex() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { flex: 2 0.5 200px }
            .b { flex: 2 0.5 }
            .c { flex: 2 200px }
            .d { flex: 200px }
            .e { flex: 1 }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.flex_grow(), Number::F32(2.));
        assert_eq!(np.flex_shrink(), Number::F32(0.5));
        assert_eq!(np.flex_basis(), Length::Px(200.0));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.flex_grow(), Number::F32(2.));
        assert_eq!(np.flex_shrink(), Number::F32(0.5));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.flex_grow(), Number::F32(2.));
        assert_eq!(np.flex_basis(), Length::Px(200.0));
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.flex_basis(), Length::Px(200.0));
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.flex_grow(), Number::F32(1.));
        test_parse_property!(
            flex_shrink,
            "flex",
            "0 100000000000000000000000000000000000000 100px",
            Number::F32(100000000000000000000000000000000000000.)
        );
    }
}

mod background {
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
}

mod sizing {
    use super::*;
    // 0x40
    #[test]
    fn width() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { width: 200px }
            .b { width: auto }
            .c { width: 30rem }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.width(), Length::Px(200.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.width(), Length::Auto);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.width(), Length::Rem(30.));
    }

    // 0x41
    #[test]
    fn height() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
    .a { height: 200px }
    .b { height: auto }
    .c { height: 30rem }
  "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.height(), Length::Px(200.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.height(), Length::Auto);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.height(), Length::Rem(30.));
    }

    #[test]
    fn width_height() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            "
            .a { width: 10vw; height: 10vh; }
        ",
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.width(), Length::Auto);
        assert_eq!(np.height(), Length::Auto);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.width(), Length::Vw(10.));
        assert_eq!(np.height(), Length::Vh(10.));
    }

    // 0x42
    #[test]
    fn min_width() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { min-width: 200px }
      .b { min-width: 10em }
      .c { min-width: 30rem }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.min_width(), Length::Px(200.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.min_width(), Length::Px(160.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.min_width(), Length::Rem(30.));
    }

    // 0x43
    #[test]
    fn min_height() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { min-height: 200px }
      .b { min-height: 10em }
      .c { min-height: 30rem }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.min_height(), Length::Px(200.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.min_height(), Length::Px(160.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.min_height(), Length::Rem(30.));
    }

    // 0x44
    #[test]
    fn max_width() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { max-width: 200px }
      .b { max-width: 10em }
      .c { max-width: 30rem }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.max_width(), Length::Px(200.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.max_width(), Length::Px(160.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.max_width(), Length::Rem(30.));
    }

    // 0x45
    #[test]
    fn max_height() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { max-height: 200px }
      .b { max-height: 10em }
      .c { max-height: 30rem }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.max_height(), Length::Px(200.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.max_height(), Length::Px(160.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.max_height(), Length::Rem(30.));
    }

    // 0x46 & 0x47 & 0x48 & 0x49
    #[test]
    fn left_right_top_bottom() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { left: 200px; right: 300px; top: 20em; bottom: auto }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.left(), Length::Px(200.));
        assert_eq!(np.right(), Length::Px(300.));
        assert_eq!(np.top(), Length::Px(320.));
        assert_eq!(np.bottom(), Length::Auto);
    }
}

mod padding_margin {
    use super::*;
    // 0x50 & 0x51 & 0x52 & 0x53
    #[test]
    fn padding_left_right_top_bottom() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
          padding-left: 200px; 
          padding-right: 50%; 
          padding-top: 30em; 
          padding-bottom: 300px 
        }
      "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.padding_left(), Length::Px(200.));
        assert_eq!(np.padding_right(), Length::Ratio(0.5));
        assert_eq!(np.padding_top(), Length::Px(480.));
        assert_eq!(np.padding_bottom(), Length::Px(300.));
    }

    #[test]
    fn padding() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            "
        .a {
            padding-left: 1em;
            padding-right: 2rem;
            padding-top: 3rpx;
            padding-bottom: 4%;
        }
        .b {
            padding: 5px;
        }
        .c {
            padding: 6px 7px;
        }
        .d {
            padding: 8px 9px 10px;
        }
        .e {
            padding: -11px 1.2px -1.3px 0;
        }
    ",
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(np.padding_left(), Length::Px(0.));
        assert_eq!(np.padding_right(), Length::Px(0.));
        assert_eq!(np.padding_top(), Length::Px(0.));
        assert_eq!(np.padding_bottom(), Length::Px(0.));
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.padding_left(), Length::Px(16.));
        assert_eq!(np.padding_right(), Length::Rem(2.));
        assert_eq!(np.padding_top(), Length::Rpx(3.));
        assert_eq!(np.padding_bottom(), Length::Ratio(0.04));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.padding_left(), Length::Px(5.));
        assert_eq!(np.padding_right(), Length::Px(5.));
        assert_eq!(np.padding_top(), Length::Px(5.));
        assert_eq!(np.padding_bottom(), Length::Px(5.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.padding_left(), Length::Px(7.));
        assert_eq!(np.padding_right(), Length::Px(7.));
        assert_eq!(np.padding_top(), Length::Px(6.));
        assert_eq!(np.padding_bottom(), Length::Px(6.));
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.padding_left(), Length::Px(9.));
        assert_eq!(np.padding_right(), Length::Px(9.));
        assert_eq!(np.padding_top(), Length::Px(8.));
        assert_eq!(np.padding_bottom(), Length::Px(10.));
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.padding_left(), Length::Px(0.));
        assert_eq!(np.padding_right(), Length::Px(1.2));
        assert_eq!(np.padding_top(), Length::Px(-11.));
        assert_eq!(np.padding_bottom(), Length::Px(-1.3));
    }

    // 0x54 & 0x55 & 0x56 & 0x57
    #[test]
    fn margin_left_right_top_bottom() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { 
        margin-left: 200px; 
        margin-right: 50%; 
        margin-top: 30em; 
        margin-bottom: auto;
      }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.margin_left(), Length::Px(200.));
        assert_eq!(np.margin_right(), Length::Ratio(0.5));
        assert_eq!(np.margin_top(), Length::Px(480.));
        assert_eq!(np.margin_bottom(), Length::Auto);
    }

    #[test]
    fn margin() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
      .a { 
        margin: 200px 30em auto 60%;
      }
      .b {
        margin: 200vmin auto 30vmax;
      }
      .c {
        margin: 100px auto;
      }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.margin_top(), Length::Px(200.));
        assert_eq!(np.margin_right(), Length::Px(480.));
        assert_eq!(np.margin_bottom(), Length::Auto);
        assert_eq!(np.margin_left(), Length::Ratio(0.6));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.margin_top(), Length::Vmin(200.));
        assert_eq!(np.margin_right(), Length::Auto);
        assert_eq!(np.margin_bottom(), Length::Vmax(30.));
        assert_eq!(np.margin_left(), Length::Auto);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.margin_top(), Length::Px(100.));
        assert_eq!(np.margin_right(), Length::Auto);
        assert_eq!(np.margin_bottom(), Length::Px(100.));
        assert_eq!(np.margin_left(), Length::Auto);
    }
}

mod border {
    use super::*;

    // 0x60 & 0x61 & 0x62 & 0x63 & 0x64 & 0x65 & 0x66 & 0x67 & 0x68 & 0x69 & 0x6a & 0x6b
    #[test]
    fn border() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                border: 20px solid red;
            }
            .b {
                border: dashed 12px red;
            }
            .c {
                border: red dotted 3px;
            }
            .d {
                border: none;
            }
            .e {
                border: 1px none;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.border_left_width(), Length::Px(20.));
        assert_eq!(np.border_right_width(), Length::Px(20.));
        assert_eq!(np.border_top_width(), Length::Px(20.));
        assert_eq!(np.border_bottom_width(), Length::Px(20.));

        assert_eq!(np.border_left_style(), BorderStyle::Solid);
        assert_eq!(np.border_right_style(), BorderStyle::Solid);
        assert_eq!(np.border_top_style(), BorderStyle::Solid);
        assert_eq!(np.border_bottom_style(), BorderStyle::Solid);

        assert_eq!(np.border_left_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_right_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(255, 0, 0, 255));

        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.border_left_width(), Length::Px(12.));
        assert_eq!(np.border_right_width(), Length::Px(12.));
        assert_eq!(np.border_top_width(), Length::Px(12.));
        assert_eq!(np.border_bottom_width(), Length::Px(12.));

        assert_eq!(np.border_left_style(), BorderStyle::Dashed);
        assert_eq!(np.border_right_style(), BorderStyle::Dashed);
        assert_eq!(np.border_top_style(), BorderStyle::Dashed);
        assert_eq!(np.border_bottom_style(), BorderStyle::Dashed);

        assert_eq!(np.border_left_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_right_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(255, 0, 0, 255));

        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.border_left_width(), Length::Px(3.));
        assert_eq!(np.border_right_width(), Length::Px(3.));
        assert_eq!(np.border_top_width(), Length::Px(3.));
        assert_eq!(np.border_bottom_width(), Length::Px(3.));

        assert_eq!(np.border_left_style(), BorderStyle::Dotted);
        assert_eq!(np.border_right_style(), BorderStyle::Dotted);
        assert_eq!(np.border_top_style(), BorderStyle::Dotted);
        assert_eq!(np.border_bottom_style(), BorderStyle::Dotted);

        assert_eq!(np.border_left_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_right_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(255, 0, 0, 255));

        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.border_left_width_type(), LengthType::Initial);
        assert_eq!(np.border_right_width_type(), LengthType::Initial);
        assert_eq!(np.border_top_width_type(), LengthType::Initial);
        assert_eq!(np.border_bottom_width_type(), LengthType::Initial);

        assert_eq!(np.border_left_style(), BorderStyle::None);
        assert_eq!(np.border_right_style(), BorderStyle::None);
        assert_eq!(np.border_top_style(), BorderStyle::None);
        assert_eq!(np.border_bottom_style(), BorderStyle::None);

        assert_eq!(np.border_left_color_type(), ColorType::Initial);
        assert_eq!(np.border_right_color_type(), ColorType::Initial);
        assert_eq!(np.border_top_color_type(), ColorType::Initial);
        assert_eq!(np.border_bottom_color_type(), ColorType::Initial);

        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.border_left_width(), Length::Px(1.));
        assert_eq!(np.border_right_width(), Length::Px(1.));
        assert_eq!(np.border_top_width(), Length::Px(1.));
        assert_eq!(np.border_bottom_width(), Length::Px(1.));

        assert_eq!(np.border_left_style(), BorderStyle::None);
        assert_eq!(np.border_right_style(), BorderStyle::None);
        assert_eq!(np.border_top_style(), BorderStyle::None);
        assert_eq!(np.border_bottom_style(), BorderStyle::None);

        assert_eq!(np.border_left_color_type(), ColorType::Initial);
        assert_eq!(np.border_right_color_type(), ColorType::Initial);
        assert_eq!(np.border_top_color_type(), ColorType::Initial);
        assert_eq!(np.border_bottom_color_type(), ColorType::Initial);
    }

    #[test]
    fn border_width() {
        test_parse_property!(
            border_left_width,
            "border-left-width",
            "thin",
            Length::Px(1.)
        );
        test_parse_property!(
            border_top_width,
            "border-top-width",
            "medium",
            Length::Px(3.)
        );
        test_parse_property!(
            border_bottom_width,
            "border-bottom-width",
            "thick",
            Length::Px(5.)
        );
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
            border-left-width: 200px; 
            border-right-width: 100px;
            border-top-width: 20px;
            border-bottom-width: 8px;
        }
        .b {
            border-width: 100px 20px 30px 8px;
        }
        .c {
            border-width: 100px 20px 8px;
        }
        .d {
            border-width: 100px 8px;
        }
        .e {
            border-width: 100px;
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.border_left_width(), Length::Px(200.));
        assert_eq!(np.border_right_width(), Length::Px(100.));
        assert_eq!(np.border_top_width(), Length::Px(20.));
        assert_eq!(np.border_bottom_width(), Length::Px(8.));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.border_left_width(), Length::Px(8.));
        assert_eq!(np.border_right_width(), Length::Px(20.));
        assert_eq!(np.border_top_width(), Length::Px(100.));
        assert_eq!(np.border_bottom_width(), Length::Px(30.));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.border_left_width(), Length::Px(20.));
        assert_eq!(np.border_right_width(), Length::Px(20.));
        assert_eq!(np.border_top_width(), Length::Px(100.));
        assert_eq!(np.border_bottom_width(), Length::Px(8.));
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.border_left_width(), Length::Px(8.));
        assert_eq!(np.border_right_width(), Length::Px(8.));
        assert_eq!(np.border_top_width(), Length::Px(100.));
        assert_eq!(np.border_bottom_width(), Length::Px(100.));
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.border_left_width(), Length::Px(100.));
        assert_eq!(np.border_right_width(), Length::Px(100.));
        assert_eq!(np.border_top_width(), Length::Px(100.));
        assert_eq!(np.border_bottom_width(), Length::Px(100.));
    }

    #[test]
    fn border_style() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
          border-left-style: hidden; 
          border-right-style: dotted;
          border-top-style: dashed;
          border-bottom-style: solid;
        }
        .b {
          border-style: hidden dotted dashed solid;
        }
        .c {
          border-style: solid dotted dashed;
        }
        .d {
          border-style: solid dotted;
        }
        .e {
          border-style: solid;
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.border_left_style(), BorderStyle::Hidden);
        assert_eq!(np.border_right_style(), BorderStyle::Dotted);
        assert_eq!(np.border_top_style(), BorderStyle::Dashed);
        assert_eq!(np.border_bottom_style(), BorderStyle::Solid);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.border_left_style(), BorderStyle::Solid);
        assert_eq!(np.border_right_style(), BorderStyle::Dotted);
        assert_eq!(np.border_top_style(), BorderStyle::Hidden);
        assert_eq!(np.border_bottom_style(), BorderStyle::Dashed);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.border_left_style(), BorderStyle::Dotted);
        assert_eq!(np.border_right_style(), BorderStyle::Dotted);
        assert_eq!(np.border_top_style(), BorderStyle::Solid);
        assert_eq!(np.border_bottom_style(), BorderStyle::Dashed);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.border_left_style(), BorderStyle::Dotted);
        assert_eq!(np.border_right_style(), BorderStyle::Dotted);
        assert_eq!(np.border_top_style(), BorderStyle::Solid);
        assert_eq!(np.border_bottom_style(), BorderStyle::Solid);
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.border_left_style(), BorderStyle::Solid);
        assert_eq!(np.border_right_style(), BorderStyle::Solid);
        assert_eq!(np.border_top_style(), BorderStyle::Solid);
        assert_eq!(np.border_bottom_style(), BorderStyle::Solid);
    }

    #[test]
    fn border_color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
          border-left-color: rgba(255, 0, 0, 255); 
          border-right-color: rgba(255, 255, 0, 255);
          border-top-color: rgba(255, 255, 255, 255);
          border-bottom-color: rgba(0, 0, 0, 255);
        }
        .b {
          border-color: red blue lime white;
        }
        .c {
          border-color: red blue lime;
        }
        .d {
          border-color: red white;
        }
        .e {
          border-color: red;
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.border_top_color(), Color::Specified(255, 255, 255, 255));
        assert_eq!(np.border_right_color(), Color::Specified(255, 255, 0, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(0, 0, 0, 255));
        assert_eq!(np.border_left_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_right_color(), Color::Specified(0, 0, 255, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(0, 255, 0, 255));
        assert_eq!(np.border_left_color(), Color::Specified(255, 255, 255, 255));
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_right_color(), Color::Specified(0, 0, 255, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(0, 255, 0, 255));
        assert_eq!(np.border_left_color(), Color::Specified(0, 0, 255, 255));
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(
            np.border_right_color(),
            Color::Specified(255, 255, 255, 255)
        );
        assert_eq!(np.border_bottom_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_left_color(), Color::Specified(255, 255, 255, 255));
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(np.border_top_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_right_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_bottom_color(), Color::Specified(255, 0, 0, 255));
        assert_eq!(np.border_left_color(), Color::Specified(255, 0, 0, 255));
    }

    // 0x6c
    #[test]
    fn box_shadow() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a {
            box-shadow: 10px 5px 5px black;
        }
        .b {
            box-shadow: inset 3px 3px red, -1em 0 0.4em 5px green;
        }
        .c {
            box-shadow: none;
        }
        .d {
            box-shadow: 10px 4px blue;
        }
        .e {
            box-shadow: 10px 4px;
        }
        .f {
            box-shadow: 10px 40px -10px;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.box_shadow(),
            BoxShadow::List(
                vec![BoxShadowItem::List(
                    vec![
                        ShadowItemType::OffsetX(Length::Px(10.)),
                        ShadowItemType::OffsetY(Length::Px(5.)),
                        ShadowItemType::BlurRadius(Length::Px(5.)),
                        ShadowItemType::SpreadRadius(Length::Px(0.0)),
                        ShadowItemType::Color(Color::Specified(0, 0, 0, 255))
                    ]
                    .into()
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.box_shadow(),
            BoxShadow::List(
                vec![
                    BoxShadowItem::List(
                        vec![
                            ShadowItemType::Inset,
                            ShadowItemType::OffsetX(Length::Px(3.)),
                            ShadowItemType::OffsetY(Length::Px(3.)),
                            ShadowItemType::BlurRadius(Length::Px(0.)),
                            ShadowItemType::SpreadRadius(Length::Px(0.0)),
                            ShadowItemType::Color(Color::Specified(255, 0, 0, 255))
                        ]
                        .into(),
                    ),
                    BoxShadowItem::List(
                        vec![
                            ShadowItemType::OffsetX(Length::Px(-16.)),
                            ShadowItemType::OffsetY(Length::Px(0.)),
                            ShadowItemType::BlurRadius(Length::Px(6.4)),
                            ShadowItemType::SpreadRadius(Length::Px(5.)),
                            ShadowItemType::Color(Color::Specified(0, 128, 0, 255))
                        ]
                        .into(),
                    )
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.box_shadow(), BoxShadow::None,);
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.box_shadow(),
            BoxShadow::List(
                vec![BoxShadowItem::List(
                    vec![
                        ShadowItemType::OffsetX(Length::Px(10.)),
                        ShadowItemType::OffsetY(Length::Px(4.)),
                        ShadowItemType::BlurRadius(Length::Px(0.)),
                        ShadowItemType::SpreadRadius(Length::Px(0.0)),
                        ShadowItemType::Color(Color::Specified(0, 0, 255, 255))
                    ]
                    .into()
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.box_shadow(),
            BoxShadow::List(
                vec![BoxShadowItem::List(
                    vec![
                        ShadowItemType::OffsetX(Length::Px(10.)),
                        ShadowItemType::OffsetY(Length::Px(4.)),
                        ShadowItemType::BlurRadius(Length::Px(0.)),
                        ShadowItemType::SpreadRadius(Length::Px(0.0)),
                        ShadowItemType::Color(Color::CurrentColor)
                    ]
                    .into()
                ),]
                .into()
            )
        );
    }
}

mod border_radius {
    use super::*;
    // 0x70 & 0x71 & 0x72 & 0x73
    #[test]
    fn border_radius() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
            border-top-left-radius: 10px;
            border-top-right-radius: 20px 30px;
            border-bottom-left-radius: 30px 40px;
            border-bottom-right-radius: 40px 50px;
        }
        .b {
            border-radius: 40px 30px 20px 10px;
        }
        .c {
            border-radius: 40px 30px 10px;
        }
        .d {
            border-radius: 40px 30px;
        }
        .e {
            border-radius: 40px;
        }
        .f {
            border-radius: 4px 3px 6px / 2px 4px;
        }
        .g {
            border-radius: 3px 9px / 4px 2px 1px;
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(10.), Length::Px(10.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(20.), Length::Px(30.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(50.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(30.), Length::Px(40.))
        );

        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(30.), Length::Px(30.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(20.), Length::Px(20.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(10.), Length::Px(10.))
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(30.), Length::Px(30.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(10.), Length::Px(10.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(30.), Length::Px(30.))
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(30.), Length::Px(30.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(30.), Length::Px(30.))
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
        );
        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(4.), Length::Px(2.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(3.), Length::Px(4.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(6.), Length::Px(2.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(3.), Length::Px(4.))
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(
            np.border_top_left_radius(),
            BorderRadius::Pos(Length::Px(3.), Length::Px(4.))
        );
        assert_eq!(
            np.border_top_right_radius(),
            BorderRadius::Pos(Length::Px(9.), Length::Px(2.))
        );
        assert_eq!(
            np.border_bottom_right_radius(),
            BorderRadius::Pos(Length::Px(3.), Length::Px(1.))
        );
        assert_eq!(
            np.border_bottom_left_radius(),
            BorderRadius::Pos(Length::Px(9.), Length::Px(2.))
        );
    }
}
mod transition {
    use super::*;
    // 0x80
    #[test]
    fn transition_property() {
        test_parse_property!(
            transition_property,
            "transition-property",
            "mask, mask-position, mask-size, mask-position-x, mask-position-y",
            TransitionProperty::List(
                vec![
                    TransitionPropertyItem::Mask,
                    TransitionPropertyItem::MaskPosition,
                    TransitionPropertyItem::MaskSize,
                    TransitionPropertyItem::MaskPositionX,
                    TransitionPropertyItem::MaskPositionY,
                ]
                .into()
            )
        );
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
          .a { 
            transition-property: all, opacity;
          }
          .b {
            transition-property: opacity, all;
          }
          .c {
            transition-property: padding-left, margin-right;
          }
      "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.transition_property(),
            TransitionProperty::List(
                vec![TransitionPropertyItem::All, TransitionPropertyItem::Opacity].into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.transition_property(),
            TransitionProperty::List(
                vec![
                    TransitionPropertyItem::PaddingLeft,
                    TransitionPropertyItem::MarginRight
                ]
                .into()
            )
        );
    }

    // 0x81
    #[test]
    fn transition_duration() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                transition-duration: 6s;
            }
            .b {
                transition-duration: 120ms, 14ms;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.transition_duration(),
            TransitionTime::List(vec![6000u32].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.transition_duration(),
            TransitionTime::List(vec![120u32, 14u32].into())
        );
    }

    // 0x82
    #[test]
    fn transition_timing_fn() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
            transition-timing-function: ease, ease-in, ease-out, ease-in-out, linear, step-start, step-end;
        }
        .b {
            transition-timing-function: cubic-bezier(0.1, 0.7, 1.0, 0.1);
        }
        .c {
            transition-timing-function: steps(4, start);
        }
        .d {
            transition-timing-function: steps(2);
        }
    "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.transition_timing_function(),
            TransitionTimingFn::List(
                vec![
                    TransitionTimingFnItem::Ease,
                    TransitionTimingFnItem::EaseIn,
                    TransitionTimingFnItem::EaseOut,
                    TransitionTimingFnItem::EaseInOut,
                    TransitionTimingFnItem::Linear,
                    TransitionTimingFnItem::StepStart,
                    TransitionTimingFnItem::StepEnd,
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.transition_timing_function(),
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::CubicBezier(0.1, 0.7, 1.0, 0.1)].into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.transition_timing_function(),
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::Steps(4, StepPosition::Start)].into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.transition_timing_function(),
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::Steps(2, StepPosition::End)].into()
            )
        );
    }

    // 0x83
    #[test]
    fn transition_delay() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                transition-delay: 6s;
            }
            .b {
                transition-delay: 120ms, 14ms;
            }
            .c {
                transition-delay: -120ms, 14ms;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.transition_delay(),
            TransitionTime::ListI32(vec![6000i32].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.transition_delay(),
            TransitionTime::ListI32(vec![120i32, 14i32].into())
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.transition_delay(),
            TransitionTime::ListI32(vec![-120i32, 14i32].into())
        );
    }

    #[test]
    fn transition() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { 
            transition: opacity 4s ease-in-out 1s, transform 3s ease-out 2s;
        }
        .b { 
            transition: opacity 4s ease-in-out, transform;
        }
        .c {
            transition: 3s, opacity, linear;
        }
    "#,
        );
        ssg.append(ss);
        {
            let np = query(&ssg, "", "", ["a"], []);
            assert_eq!(
                np.transition_property(),
                TransitionProperty::List(
                    vec![
                        TransitionPropertyItem::Opacity,
                        TransitionPropertyItem::Transform
                    ]
                    .into()
                )
            );
            assert_eq!(
                np.transition_duration(),
                TransitionTime::List(vec![4000u32, 3000u32].into())
            );

            assert_eq!(
                np.transition_timing_function(),
                TransitionTimingFn::List(
                    vec![
                        TransitionTimingFnItem::EaseInOut,
                        TransitionTimingFnItem::EaseOut
                    ]
                    .into()
                )
            );
            assert_eq!(
                np.transition_delay(),
                TransitionTime::ListI32(vec![1000i32, 2000i32].into())
            )
        }
        {
            let np = query(&ssg, "", "", ["b"], []);
            assert_eq!(
                np.transition_property(),
                TransitionProperty::List(
                    vec![
                        TransitionPropertyItem::Opacity,
                        TransitionPropertyItem::Transform,
                    ]
                    .into()
                )
            );
            assert_eq!(
                np.transition_duration(),
                TransitionTime::List(vec![4000u32, 0u32].into())
            );

            assert_eq!(
                np.transition_timing_function(),
                TransitionTimingFn::List(
                    vec![
                        TransitionTimingFnItem::EaseInOut,
                        TransitionTimingFnItem::Ease,
                    ]
                    .into()
                )
            );
            assert_eq!(
                np.transition_delay(),
                TransitionTime::ListI32(vec![0i32, 0i32].into())
            )
        }
        {
            let np = query(&ssg, "", "", ["c"], []);
            assert_eq!(
                np.transition_property(),
                TransitionProperty::List(
                    vec![
                        TransitionPropertyItem::All,
                        TransitionPropertyItem::Opacity,
                        TransitionPropertyItem::All,
                    ]
                    .into()
                )
            );
            assert_eq!(
                np.transition_duration(),
                TransitionTime::List(vec![3000u32, 0u32, 0u32].into())
            );

            assert_eq!(
                np.transition_timing_function(),
                TransitionTimingFn::List(
                    vec![
                        TransitionTimingFnItem::Ease,
                        TransitionTimingFnItem::Ease,
                        TransitionTimingFnItem::Linear,
                    ]
                    .into()
                )
            );
            assert_eq!(
                np.transition_delay(),
                TransitionTime::ListI32(vec![0i32, 0i32, 0i32].into())
            )
        }
    }
}
mod animation {
    use super::*;
    // 0x84
    #[test]
    fn animation_duration() {
        test_parse_property!(
            animation_duration,
            "animation-duration",
            "750ms, 3s, 0s, 0ms",
            TransitionTime::List(vec![750, 3000, 0, 0].into())
        );
    }

    // 0x85
    #[test]
    fn animation_timing_function() {
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "ease, ease-in, ease-out, ease-in-out, linear, step-start, step-end",
            TransitionTimingFn::List(
                vec![
                    TransitionTimingFnItem::Ease,
                    TransitionTimingFnItem::EaseIn,
                    TransitionTimingFnItem::EaseOut,
                    TransitionTimingFnItem::EaseInOut,
                    TransitionTimingFnItem::Linear,
                    TransitionTimingFnItem::StepStart,
                    TransitionTimingFnItem::StepEnd,
                ]
                .into()
            )
        );
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "cubic-bezier(0.1, 0.7, 1.0, 0.1)",
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::CubicBezier(0.1, 0.7, 1.0, 0.1)].into()
            )
        );
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "steps(4, start)",
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::Steps(4, StepPosition::Start)].into()
            )
        );
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "steps(2)",
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::Steps(2, StepPosition::End)].into()
            )
        );
    }

    // 0x86
    #[test]
    fn animation_delay() {
        test_parse_property!(
            animation_delay,
            "animation-delay",
            "-120ms, 6s",
            TransitionTime::ListI32(vec![-120, 6000].into())
        );
    }

    // 0x87
    #[test]
    fn animation_iteration_count() {
        test_parse_property!(
            animation_iteration_count,
            "animation-iteration-count",
            "2.5, 0, infinite",
            AnimationIterationCount::List(
                vec![
                    AnimationIterationCountItem::Number(2.5),
                    AnimationIterationCountItem::Number(0.),
                    AnimationIterationCountItem::Infinite,
                ]
                .into()
            )
        );
    }

    // 0x88
    #[test]
    fn animation_direction() {
        test_parse_property!(
            animation_direction,
            "animation-direction",
            "alternate, reverse, normal",
            AnimationDirection::List(
                vec![
                    AnimationDirectionItem::Alternate,
                    AnimationDirectionItem::Reverse,
                    AnimationDirectionItem::Normal,
                ]
                .into()
            )
        );
    }

    // 0x89
    #[test]
    fn animation_fill_mode() {
        test_parse_property!(
            animation_fill_mode,
            "animation-fill-mode",
            "both, forwards, none",
            AnimationFillMode::List(
                vec![
                    AnimationFillModeItem::Both,
                    AnimationFillModeItem::Forwards,
                    AnimationFillModeItem::None,
                ]
                .into()
            )
        );
    }

    // 0x8a
    #[test]
    fn animation_play_state() {
        test_parse_property!(
            animation_play_state,
            "animation-play-state",
            "paused, running, running",
            AnimationPlayState::List(
                vec![
                    AnimationPlayStateItem::Paused,
                    AnimationPlayStateItem::Running,
                    AnimationPlayStateItem::Running,
                ]
                .into()
            )
        );
    }

    // 0x8b
    #[test]
    fn animation_name() {
        test_parse_property!(
            animation_name,
            "animation-name",
            "none, -moz-specific, sliding",
            AnimationName::List(
                vec![
                    AnimationNameItem::None,
                    AnimationNameItem::CustomIdent("-moz-specific".into()),
                    AnimationNameItem::CustomIdent("sliding".into()),
                ]
                .into()
            )
        );
    }

    #[test]
    fn animation() {
        // animation: 3s ease-in 1s 2 reverse both paused slidein;
        test_parse_property!(
            animation_duration,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            TransitionTime::List(vec![3000].into())
        );
        test_parse_property!(
            animation_timing_function,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            TransitionTimingFn::List(vec![TransitionTimingFnItem::EaseIn].into())
        );
        test_parse_property!(
            animation_delay,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            TransitionTime::ListI32(vec![1000].into())
        );
        test_parse_property!(
            animation_iteration_count,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationIterationCount::List(vec![AnimationIterationCountItem::Number(2.)].into())
        );
        test_parse_property!(
            animation_direction,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationDirection::List(vec![AnimationDirectionItem::Reverse].into())
        );
        test_parse_property!(
            animation_fill_mode,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationFillMode::List(vec![AnimationFillModeItem::Both].into())
        );
        test_parse_property!(
            animation_play_state,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein;
            ",
            AnimationPlayState::List(vec![AnimationPlayStateItem::Paused].into())
        );
        test_parse_property!(
            animation_name,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationName::List(vec![AnimationNameItem::CustomIdent("slidein".into())].into())
        );

        // animation: 3s linear 1s slidein;
        test_parse_property!(
            animation_duration,
            "animation",
            "3s linear 1s slidein",
            TransitionTime::List(vec![3000].into())
        );
        test_parse_property!(
            animation_timing_function,
            "animation",
            "3s linear 1s slidein",
            TransitionTimingFn::List(vec![TransitionTimingFnItem::Linear].into())
        );
        test_parse_property!(
            animation_delay,
            "animation",
            "3s linear 1s slidein",
            TransitionTime::ListI32(vec![1000].into())
        );
        test_parse_property!(
            animation_name,
            "animation",
            "3s linear 1s slidein",
            AnimationName::List(vec![AnimationNameItem::CustomIdent("slidein".into())].into())
        );

        // animation: 3s linear slidein, 3s ease-out 5s slideout;
        test_parse_property!(
            animation_duration,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            TransitionTime::List(vec![3000, 3000].into())
        );
        test_parse_property!(
            animation_timing_function,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            TransitionTimingFn::List(
                vec![
                    TransitionTimingFnItem::Linear,
                    TransitionTimingFnItem::EaseOut
                ]
                .into()
            )
        );
        test_parse_property!(
            animation_delay,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            TransitionTime::ListI32(vec![0, 5000].into())
        );
        test_parse_property!(
            animation_name,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            AnimationName::List(
                vec![
                    AnimationNameItem::CustomIdent("slidein".into()),
                    AnimationNameItem::CustomIdent("slideout".into())
                ]
                .into()
            )
        );
    }

    // 0x8c
    #[test]
    fn will_change() {
        test_parse_property!(will_change, "will-change", "auto", WillChange::Auto);
        test_parse_property!(
            will_change,
            "will-change",
            "contents",
            WillChange::List(vec![AnimateableFeature::Contents].into())
        );
        test_parse_property!(
            will_change,
            "will-change",
            "scroll-position",
            WillChange::List(vec![AnimateableFeature::ScrollPosition].into())
        );
        test_parse_property!(
            will_change,
            "will-change",
            "transform",
            WillChange::List(vec![AnimateableFeature::CustomIdent("transform".into())].into())
        );
        test_parse_property!(
            will_change,
            "will-change",
            "transform, opacity",
            WillChange::List(
                vec![
                    AnimateableFeature::CustomIdent("transform".into()),
                    AnimateableFeature::CustomIdent("opacity".into())
                ]
                .into()
            )
        );
        test_parse_property!(
            will_change,
            "will-change",
            "auto, transform, opacity",
            WillChange::Auto
        );
    }
}

mod typography {
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
        assert_eq!(np.text_align(), TextAlign::Left);
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
}

mod other {
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
}

mod wx_special {
    use super::*;
    // 0xe0
    #[test]
    fn wx_scrollbar_x() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                -wx-scrollbar-x: hidden;
            }
            .b {
                -wx-scrollbar-x: auto-hide;
            }
            .c {
                -wx-scrollbar-x: always-show;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_scrollbar_x(), Scrollbar::Hidden,);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_scrollbar_x(), Scrollbar::AutoHide,);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.wx_scrollbar_x(), Scrollbar::AlwaysShow,);
    }
    // 0xe1
    #[test]
    fn wx_scrollbar_x_color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                -wx-scrollbar-x-color: red;
            }
            .b {
                -wx-scrollbar-x-color: rgb(10, 20, 30);
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_scrollbar_x_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_scrollbar_x_color(), Color::Specified(10, 20, 30, 255));
    }

    // 0xe2
    #[test]
    fn wx_scrollbar_y() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                -wx-scrollbar-y: hidden;
            }
            .b {
                -wx-scrollbar-y: auto-hide;
            }
            .c {
                -wx-scrollbar-y: always-show;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_scrollbar_y(), Scrollbar::Hidden,);
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_scrollbar_y(), Scrollbar::AutoHide,);
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(np.wx_scrollbar_y(), Scrollbar::AlwaysShow,);
    }

    // 0xe3
    #[test]
    fn wx_scrollbar_y_color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                -wx-scrollbar-y-color: red;
            }
            .b {
                -wx-scrollbar-y-color: rgb(10, 20, 30);
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_scrollbar_y_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_scrollbar_y_color(), Color::Specified(10, 20, 30, 255));
    }

    #[test]
    fn wx_scrollbar_color() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { 
                -wx-scrollbar-color: red;
            }
            .b {
                -wx-scrollbar-color: rgb(10, 20, 30) red;
            }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_scrollbar_x_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.wx_scrollbar_y_color(), Color::Specified(255, 0, 0, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_scrollbar_x_color(), Color::Specified(10, 20, 30, 255));
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(np.wx_scrollbar_y_color(), Color::Specified(255, 0, 0, 255));
    }

    // 0xe4
    #[test]
    fn wx_contain() {
        test_parse_property!(wx_contain, "-wx-contain", "none", Contain::None);
        test_parse_property!(wx_contain, "-wx-contain", "strict", Contain::Strict);
        test_parse_property!(wx_contain, "-wx-contain", "content", Contain::Content);
        test_parse_property!(
            wx_contain,
            "-wx-contain",
            "size",
            Contain::Multiple(vec![ContainKeyword::Size].into())
        );
        test_parse_property!(
            wx_contain,
            "-wx-contain",
            "layout",
            Contain::Multiple(vec![ContainKeyword::Layout].into())
        );
        test_parse_property!(
            wx_contain,
            "-wx-contain",
            "style",
            Contain::Multiple(vec![ContainKeyword::Style].into())
        );
        test_parse_property!(
            wx_contain,
            "-wx-contain",
            "paint",
            Contain::Multiple(vec![ContainKeyword::Paint].into())
        );
        test_parse_property!(
            wx_contain,
            "-wx-contain",
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
            wx_contain,
            "-wx-contain",
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
}

mod mask {
    use super::*;
    // 0xf0
    #[test]
    fn mask_size() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
              .a { mask-size: 50% auto, cover;}
              .b { mask-size: contain, cover;}
              .c { mask-size: auto, auto auto, auto 30%}
              .d { mask-size: 45%; }
              .e { mask-size: 25% 50%; }
              .f { mask-size: auto 100px; }
              .g { }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_size(),
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
            np.mask_size(),
            BackgroundSize::List(
                vec![BackgroundSizeItem::Contain, BackgroundSizeItem::Cover].into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.mask_size(),
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
            np.mask_size(),
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
            np.mask_size(),
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
            np.mask_size(),
            BackgroundSize::List(
                vec![BackgroundSizeItem::Length(Length::Auto, Length::Px(100.)),].into()
            )
        );
        let np = query(&ssg, "", "", ["g"], []);
        assert_eq!(
            np.mask_size(),
            BackgroundSize::List(vec![BackgroundSizeItem::Auto].into())
        );
    }

    // 0xf1
    #[test]
    fn mask_repeat() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { mask-repeat: no-repeat;}
            .b { mask-repeat: repeat;}
            .c { mask-repeat: repeat-x; }
            .d { mask-repeat: repeat-y; }
            .e { mask-repeat: space no-repeat; }
            .f { mask-repeat: round; }
            .g {}
      "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_repeat(),
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
            np.mask_repeat(),
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
            np.mask_repeat(),
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
            np.mask_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::NoRepeat,
                    BackgroundRepeatValue::Repeat,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.mask_repeat(),
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
            np.mask_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::Round,
                    BackgroundRepeatValue::Round,
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.mask_repeat(),
            BackgroundRepeat::List(
                vec![BackgroundRepeatItem::Pos(
                    BackgroundRepeatValue::NoRepeat,
                    BackgroundRepeatValue::NoRepeat,
                ),]
                .into()
            )
        );
    }

    // 0xf2
    #[test]
    fn mask_origin() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
          .a { mask-origin: border-box; }
          .b { mask-origin: padding-box, content-box; }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::BorderBox].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.mask_origin(),
            BackgroundOrigin::List(
                vec![
                    BackgroundOriginItem::PaddingBox,
                    BackgroundOriginItem::ContentBox
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.mask_origin(),
            BackgroundOrigin::List(vec![BackgroundOriginItem::BorderBox,].into())
        );
    }

    // 0xf3
    #[test]
    fn mask_clip() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
        .a { mask-clip: border-box; }
        .b { mask-clip: padding-box, content-box; }

      "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.mask_clip(),
            BackgroundClip::List(
                vec![
                    BackgroundClipItem::PaddingBox,
                    BackgroundClipItem::ContentBox
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.mask_clip(),
            BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
        );
    }

    // 0xf4
    #[test]
    fn mask_position() {
        test_parse_property!(
            mask_position,
            "mask-position",
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
            mask_position_x,
            "mask-position-x",
            "right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(1.0)),
                )]
                .into()
            )
        );
        test_parse_property!(
            mask_position_x,
            "mask-position",
            "right",
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Value(
                    BackgroundPositionValue::Left(Length::Ratio(1.0)),
                )]
                .into()
            )
        );
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
            .a { mask-position: center; }
            .b { mask-position: left, right; }
            .c { mask-position: 20% bottom; }
            .d { mask-position: 30% 70%, center; }
            .e { mask-position: right }
        "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5)),
                )]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.mask_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(1.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    )
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.mask_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.2)),
                    BackgroundPositionValue::Top(Length::Ratio(1.)),
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["d"], []);
        assert_eq!(
            np.mask_position(),
            BackgroundPosition::List(
                vec![
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.3)),
                        BackgroundPositionValue::Top(Length::Ratio(0.7)),
                    ),
                    BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    )
                ]
                .into()
            )
        );
        let np = query(&ssg, "", "", ["e"], []);
        assert_eq!(
            np.mask_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5)),
                ),]
                .into()
            )
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.mask_position(),
            BackgroundPosition::List(
                vec![BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5)),
                ),]
                .into()
            )
        );
    }

    // 0xf5
    #[test]
    fn mask_mode() {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(
            r#"
              .a { mask-mode: alpha; }
              .b { mask-mode: luminance, match-source; }
              .c { mask-mode: match-source; }
            "#,
        );
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(
            np.mask_mode(),
            MaskMode::List(vec![MaskModeItem::Alpha].into())
        );
        let np = query(&ssg, "", "", ["b"], []);
        assert_eq!(
            np.mask_mode(),
            MaskMode::List(vec![MaskModeItem::Luminance, MaskModeItem::MatchSource].into())
        );
        let np = query(&ssg, "", "", ["c"], []);
        assert_eq!(
            np.mask_mode(),
            MaskMode::List(vec![MaskModeItem::MatchSource].into())
        );
        let np = query(&ssg, "", "", [""], []);
        assert_eq!(
            np.mask_mode(),
            MaskMode::List(vec![MaskModeItem::MatchSource].into())
        );
    }
}
