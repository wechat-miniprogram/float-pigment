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
