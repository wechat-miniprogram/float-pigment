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
        test_parse_property!(
            color,
            "color",
            "hsl(100deg 80% 40%)",
            Color::Specified(75, 184, 20, 255)
        );

        test_parse_property!(
            color,
            "color",
            "hwb(100 20% 20%)",
            Color::Specified(102, 204, 51, 255)
        );
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
