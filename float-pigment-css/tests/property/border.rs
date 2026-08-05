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
            .f {
                border: 2px 2px 2px 2px black solid;
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

        let np = query(&ssg, "", "", ["f"], []);
        assert_eq!(np.border_left_width(), Length::Px(3.));
        assert_eq!(np.border_right_width(), Length::Px(3.));
        assert_eq!(np.border_top_width(), Length::Px(3.));
        assert_eq!(np.border_bottom_width(), Length::Px(3.));
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
