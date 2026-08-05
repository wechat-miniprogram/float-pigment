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
