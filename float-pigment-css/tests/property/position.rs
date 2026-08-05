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
            Transform::Series(vec![TransformItem::Skew(Angle::Deg(10.), Angle::Deg(0.))].into())
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
