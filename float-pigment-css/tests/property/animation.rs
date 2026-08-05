use super::*;

    // 0x84
    #[test]
    fn animation_duration() {
        test_parse_property!(
            animation_duration,
            "animation-duration",
            "750ms, 3s, 0s, 0ms",
            TransitionTime::List(vec![750, 3000, 0, 0].into())
        );
    }

    // 0x85
    #[test]
    fn animation_timing_function() {
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "ease, ease-in, ease-out, ease-in-out, linear, step-start, step-end",
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
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "cubic-bezier(0.1, 0.7, 1.0, 0.1)",
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::CubicBezier(0.1, 0.7, 1.0, 0.1)].into()
            )
        );
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "steps(4, start)",
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::Steps(4, StepPosition::Start)].into()
            )
        );
        test_parse_property!(
            animation_timing_function,
            "animation-timing-function",
            "steps(2)",
            TransitionTimingFn::List(
                vec![TransitionTimingFnItem::Steps(2, StepPosition::End)].into()
            )
        );
    }

    // 0x86
    #[test]
    fn animation_delay() {
        test_parse_property!(
            animation_delay,
            "animation-delay",
            "-120ms, 6s",
            TransitionTime::ListI32(vec![-120, 6000].into())
        );
    }

    // 0x87
    #[test]
    fn animation_iteration_count() {
        test_parse_property!(
            animation_iteration_count,
            "animation-iteration-count",
            "2.5, 0, infinite",
            AnimationIterationCount::List(
                vec![
                    AnimationIterationCountItem::Number(2.5),
                    AnimationIterationCountItem::Number(0.),
                    AnimationIterationCountItem::Infinite,
                ]
                .into()
            )
        );
    }

    // 0x88
    #[test]
    fn animation_direction() {
        test_parse_property!(
            animation_direction,
            "animation-direction",
            "alternate, reverse, normal",
            AnimationDirection::List(
                vec![
                    AnimationDirectionItem::Alternate,
                    AnimationDirectionItem::Reverse,
                    AnimationDirectionItem::Normal,
                ]
                .into()
            )
        );
    }

    // 0x89
    #[test]
    fn animation_fill_mode() {
        test_parse_property!(
            animation_fill_mode,
            "animation-fill-mode",
            "both, forwards, none",
            AnimationFillMode::List(
                vec![
                    AnimationFillModeItem::Both,
                    AnimationFillModeItem::Forwards,
                    AnimationFillModeItem::None,
                ]
                .into()
            )
        );
    }

    // 0x8a
    #[test]
    fn animation_play_state() {
        test_parse_property!(
            animation_play_state,
            "animation-play-state",
            "paused, running, running",
            AnimationPlayState::List(
                vec![
                    AnimationPlayStateItem::Paused,
                    AnimationPlayStateItem::Running,
                    AnimationPlayStateItem::Running,
                ]
                .into()
            )
        );
    }

    // 0x8b
    #[test]
    fn animation_name() {
        test_parse_property!(
            animation_name,
            "animation-name",
            "none, -moz-specific, sliding",
            AnimationName::List(
                vec![
                    AnimationNameItem::None,
                    AnimationNameItem::CustomIdent("-moz-specific".into()),
                    AnimationNameItem::CustomIdent("sliding".into()),
                ]
                .into()
            )
        );
    }

    #[test]
    fn animation() {
        // animation: 3s ease-in 1s 2 reverse both paused slidein;
        test_parse_property!(
            animation_duration,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            TransitionTime::List(vec![3000].into())
        );
        test_parse_property!(
            animation_timing_function,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            TransitionTimingFn::List(vec![TransitionTimingFnItem::EaseIn].into())
        );
        test_parse_property!(
            animation_delay,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            TransitionTime::ListI32(vec![1000].into())
        );
        test_parse_property!(
            animation_iteration_count,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationIterationCount::List(vec![AnimationIterationCountItem::Number(2.)].into())
        );
        test_parse_property!(
            animation_direction,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationDirection::List(vec![AnimationDirectionItem::Reverse].into())
        );
        test_parse_property!(
            animation_fill_mode,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationFillMode::List(vec![AnimationFillModeItem::Both].into())
        );
        test_parse_property!(
            animation_play_state,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein;
            ",
            AnimationPlayState::List(vec![AnimationPlayStateItem::Paused].into())
        );
        test_parse_property!(
            animation_name,
            "animation",
            "3s ease-in 1s 2 reverse both paused slidein",
            AnimationName::List(vec![AnimationNameItem::CustomIdent("slidein".into())].into())
        );

        // animation: 3s linear 1s slidein;
        test_parse_property!(
            animation_duration,
            "animation",
            "3s linear 1s slidein",
            TransitionTime::List(vec![3000].into())
        );
        test_parse_property!(
            animation_timing_function,
            "animation",
            "3s linear 1s slidein",
            TransitionTimingFn::List(vec![TransitionTimingFnItem::Linear].into())
        );
        test_parse_property!(
            animation_delay,
            "animation",
            "3s linear 1s slidein",
            TransitionTime::ListI32(vec![1000].into())
        );
        test_parse_property!(
            animation_name,
            "animation",
            "3s linear 1s slidein",
            AnimationName::List(vec![AnimationNameItem::CustomIdent("slidein".into())].into())
        );

        // animation: 3s linear slidein, 3s ease-out 5s slideout;
        test_parse_property!(
            animation_duration,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            TransitionTime::List(vec![3000, 3000].into())
        );
        test_parse_property!(
            animation_timing_function,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            TransitionTimingFn::List(
                vec![
                    TransitionTimingFnItem::Linear,
                    TransitionTimingFnItem::EaseOut
                ]
                .into()
            )
        );
        test_parse_property!(
            animation_delay,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            TransitionTime::ListI32(vec![0, 5000].into())
        );
        test_parse_property!(
            animation_name,
            "animation",
            "3s linear slidein, 3s ease-out 5s slideout",
            AnimationName::List(
                vec![
                    AnimationNameItem::CustomIdent("slidein".into()),
                    AnimationNameItem::CustomIdent("slideout".into())
                ]
                .into()
            )
        );
    }

    // 0x8c
    #[test]
    fn will_change() {
        test_parse_property!(will_change, "will-change", "auto", WillChange::Auto);
        test_parse_property!(
            will_change,
            "will-change",
            "contents",
            WillChange::List(vec![AnimateableFeature::Contents].into())
        );
        test_parse_property!(
            will_change,
            "will-change",
            "scroll-position",
            WillChange::List(vec![AnimateableFeature::ScrollPosition].into())
        );
        test_parse_property!(
            will_change,
            "will-change",
            "transform",
            WillChange::List(vec![AnimateableFeature::CustomIdent("transform".into())].into())
        );
        test_parse_property!(
            will_change,
            "will-change",
            "transform, opacity",
            WillChange::List(
                vec![
                    AnimateableFeature::CustomIdent("transform".into()),
                    AnimateableFeature::CustomIdent("opacity".into())
                ]
                .into()
            )
        );
        test_parse_property!(
            will_change,
            "will-change",
            "auto, transform, opacity",
            WillChange::Auto
        );
    }
