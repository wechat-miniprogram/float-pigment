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
