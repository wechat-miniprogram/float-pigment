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
