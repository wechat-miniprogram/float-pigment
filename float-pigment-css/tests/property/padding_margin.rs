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
