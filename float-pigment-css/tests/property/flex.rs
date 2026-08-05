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
            assert_eq!(np.justify_content(), JustifyContent::Normal);
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

        test_parse_property!(flex_grow, "flex", "100px", Number::F32(1.));
    }
