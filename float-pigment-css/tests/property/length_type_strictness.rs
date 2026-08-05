use super::*;


    // padding: accepts lengths and percentages; rejects `auto`
    #[test]
    fn padding_rejects_auto() {
        // longhand: `auto` is rejected and falls back to 0
        test_parse_property!(padding_left, "padding-left", "auto", Length::Px(0.));
        // longhand: length is accepted
        test_parse_property!(padding_left, "padding-left", "10px", Length::Px(10.));
        // shorthand: `auto` is rejected and falls back to 0 on all sides
        test_parse_property!(padding_top, "padding", "auto", Length::Px(0.));
        test_parse_property!(padding_right, "padding", "auto", Length::Px(0.));
        // shorthand: percentage is accepted
        test_parse_property!(padding_top, "padding", "10%", Length::Ratio(0.1));
        test_parse_property!(padding_right, "padding", "10%", Length::Ratio(0.1));
    }

    // border-radius: accepts lengths and percentages; rejects `auto`
    #[test]
    fn border_radius_rejects_auto() {
        // `auto` is rejected and falls back to the initial value
        test_parse_property!(
            border_top_left_radius,
            "border-top-left-radius",
            "auto",
            BorderRadius::Pos(Length::Px(0.), Length::Px(0.))
        );
        // percentage is accepted
        test_parse_property!(
            border_top_left_radius,
            "border-top-left-radius",
            "50%",
            BorderRadius::Pos(Length::Ratio(0.5), Length::Ratio(0.5))
        );
        // length is accepted
        test_parse_property!(
            border_top_left_radius,
            "border-top-left-radius",
            "10px",
            BorderRadius::Pos(Length::Px(10.), Length::Px(10.))
        );
    }

    // text-indent: accepts lengths and percentages; rejects `auto`
    #[test]
    fn text_indent_rejects_auto() {
        // `auto` is rejected and falls back to the initial value
        test_parse_property!(text_indent, "text-indent", "auto", Length::Undefined);
        // percentage is accepted and resolved by the length resolver to px
        test_parse_property!(text_indent, "text-indent", "10%", Length::Px(1.6));
        // length is accepted
        test_parse_property!(text_indent, "text-indent", "20px", Length::Px(20.));
    }

    // letter-spacing: accepts lengths; rejects `auto` and percentage
    #[test]
    fn letter_spacing_rejects_auto_and_percentage() {
        // `auto` is rejected
        test_parse_property!(
            letter_spacing,
            "letter-spacing",
            "auto",
            LetterSpacing::Normal
        );
        // percentage is rejected
        test_parse_property!(
            letter_spacing,
            "letter-spacing",
            "50%",
            LetterSpacing::Normal
        );
        // length is accepted
        test_parse_property!(
            letter_spacing,
            "letter-spacing",
            "2px",
            LetterSpacing::Length(Length::Px(2.))
        );
        // keyword `normal` is accepted
        test_parse_property!(
            letter_spacing,
            "letter-spacing",
            "normal",
            LetterSpacing::Normal
        );
    }

    // word-spacing: accepts lengths; rejects `auto` and percentage
    #[test]
    fn word_spacing_rejects_auto_and_percentage() {
        // `auto` is rejected
        test_parse_property!(word_spacing, "word-spacing", "auto", WordSpacing::Normal);
        // percentage is rejected
        test_parse_property!(word_spacing, "word-spacing", "50%", WordSpacing::Normal);
        // length is accepted
        test_parse_property!(
            word_spacing,
            "word-spacing",
            "5px",
            WordSpacing::Length(Length::Px(5.))
        );
        // keyword `normal` is accepted
        test_parse_property!(word_spacing, "word-spacing", "normal", WordSpacing::Normal);
    }

    // box-shadow: accepts lengths; rejects `auto` and percentage
    #[test]
    fn box_shadow_rejects_auto_and_percentage() {
        // `auto` is rejected and parses as `none`
        test_parse_property!(box_shadow, "box-shadow", "auto 5px black", BoxShadow::None);
        // percentage is rejected and parses as `none`
        test_parse_property!(box_shadow, "box-shadow", "10% 5px black", BoxShadow::None);
        // length is accepted
        test_parse_property!(
            box_shadow,
            "box-shadow",
            "10px 5px black",
            BoxShadow::List(
                vec![BoxShadowItem::List(
                    vec![
                        ShadowItemType::OffsetX(Length::Px(10.)),
                        ShadowItemType::OffsetY(Length::Px(5.)),
                        ShadowItemType::BlurRadius(Length::Px(0.)),
                        ShadowItemType::SpreadRadius(Length::Px(0.)),
                        ShadowItemType::Color(Color::Specified(0, 0, 0, 255))
                    ]
                    .into()
                )]
                .into()
            )
        );
    }

    // text-shadow: accepts lengths; rejects `auto` and percentage
    #[test]
    fn text_shadow_rejects_auto_and_percentage() {
        // `auto` is rejected and parses as `none`
        test_parse_property!(
            text_shadow,
            "text-shadow",
            "auto 1px black",
            TextShadow::None
        );
        // percentage is rejected and parses as `none`
        test_parse_property!(
            text_shadow,
            "text-shadow",
            "10% 1px black",
            TextShadow::None
        );
        // length is accepted
        test_parse_property!(
            text_shadow,
            "text-shadow",
            "1px 2px black",
            TextShadow::List(
                vec![TextShadowItem::TextShadowValue(
                    Length::Px(1.),
                    Length::Px(2.),
                    Length::Undefined,
                    Color::Specified(0, 0, 0, 255)
                )]
                .into()
            )
        );
    }

    // font-size: accepts non-negative lengths and percentages; rejects `auto`
    #[test]
    fn font_size_rejects_auto() {
        // `auto` is rejected and falls back to the inherited value (`Undefined`)
        test_parse_property!(font_size, "font-size", "auto", Length::Undefined);
        // length is accepted
        test_parse_property!(font_size, "font-size", "20px", Length::Px(20.));
        // percentage is accepted (resolved via `resolve_set`: 50% of default 16px = 8px)
        test_parse_property!(font_size, "font-size", "50%", Length::Px(8.));
    }

    // gap: accepts non-negative lengths and percentages, and the `normal` keyword; rejects `auto`
    #[test]
    fn gap_rejects_auto() {
        // `auto` is rejected and falls back to `normal`
        test_parse_property!(row_gap, "row-gap", "auto", Gap::Normal);
        // length is accepted
        test_parse_property!(row_gap, "row-gap", "10px", Gap::Length(Length::Px(10.)));
        // percentage is accepted
        test_parse_property!(row_gap, "row-gap", "20%", Gap::Length(Length::Ratio(0.2)));
        // `normal` is accepted
        test_parse_property!(row_gap, "row-gap", "normal", Gap::Normal);
    }

    // transform-origin: x/y accept lengths and percentages; z accepts lengths only
    #[test]
    fn transform_origin_rejects_auto() {
        // `auto` is rejected and falls back to the default origin
        test_parse_property!(
            transform_origin,
            "transform-origin",
            "auto",
            TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.5), Length::Px(0.))
        );
        // percentages are accepted for x and y
        test_parse_property!(
            transform_origin,
            "transform-origin",
            "50% 50%",
            TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.5), Length::Px(0.))
        );
        // lengths are accepted for x, y, and z
        test_parse_property!(
            transform_origin,
            "transform-origin",
            "10px 20px 30px",
            TransformOrigin::LengthTuple(Length::Px(10.), Length::Px(20.), Length::Px(30.))
        );
    }

    // perspective(): z-distance accepts non-negative lengths only and rejects percentages
    #[test]
    fn transform_perspective_uses_non_negative_length_only() {
        // negative length is invalid → parsed as `none`
        test_parse_property!(
            transform,
            "transform",
            "perspective(-10px)",
            Transform::Series(vec![].into())
        );

        // positive length is accepted
        test_parse_property!(
            transform,
            "transform",
            "perspective(10px)",
            Transform::Series(vec![TransformItem::Perspective(Length::Px(10.))].into())
        );

        // percentage is invalid for perspective() → parsed as `none`
        test_parse_property!(
            transform,
            "transform",
            "perspective(10%)",
            Transform::Series(vec![].into())
        );
    }
