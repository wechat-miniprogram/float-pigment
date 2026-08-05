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
