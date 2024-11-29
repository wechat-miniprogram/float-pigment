#![allow(clippy::useless_vec, unused_must_use)]

use float_pigment_css::{
    property::*, sheet::borrow::Array, sheet::str_store::StrRef, typing::*, MediaQueryStatus,
    StyleQuery, StyleSheet, StyleSheetGroup,
};

macro_rules! test_parse_stringify {
    ($prop: ident, $prop_name: expr, $str_value: expr, $value: expr) => {{
        let name = $prop_name;
        let value = $value;
        let str_value = $str_value;
        let style_str = format!(".a{{{}:{};}}", name, str_value);
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(&style_str);
        ssg.append(ss);
        let classes = vec![("a".into(), None)];
        let query = StyleQuery::single(None, None, None, "", "", &classes, &[]);
        let matched_rules =
            ssg.query_matched_rules(&vec![query], &MediaQueryStatus::<f32>::default_screen());
        let mut node_properties = NodeProperties::new(None);
        matched_rules.merge_node_properties(&mut node_properties, None, 16., &[]);
        assert_eq!(node_properties.$prop(), value);
        let p = matched_rules.rules[0].rule.properties().next().unwrap();
        let name = p.get_property_name();
        let value = p.get_property_value_string();
        assert_eq!(style_str, format!(".a{{{}:{};}}", name, value));
    }};
}

#[test]
fn display() {
    &[test_parse_stringify!(
        display,
        "display",
        "none",
        Display::None
    )];
    &[test_parse_stringify!(
        display,
        "display",
        "block",
        Display::Block
    )];
    &[test_parse_stringify!(
        display,
        "display",
        "flex",
        Display::Flex
    )];
    &[test_parse_stringify!(
        display,
        "display",
        "inline",
        Display::Inline
    )];
    &[test_parse_stringify!(
        display,
        "display",
        "inline-block",
        Display::InlineBlock
    )];
    &[test_parse_stringify!(
        display,
        "display",
        "grid",
        Display::Grid
    )];
}

#[test]
fn color() {
    &[test_parse_stringify!(
        color,
        "color",
        "currentcolor",
        Color::CurrentColor
    )];
    &[test_parse_stringify!(
        color,
        "color",
        "rgb(1, 1, 3)",
        Color::Specified(1, 1, 3, 255)
    )];
}

#[test]
fn overflow() {
    &[test_parse_stringify!(
        overflow_x,
        "overflow-x",
        "visible",
        Overflow::Visible
    )];
    &[test_parse_stringify!(
        overflow_x,
        "overflow-x",
        "hidden",
        Overflow::Hidden
    )];
    &[test_parse_stringify!(
        overflow_x,
        "overflow-x",
        "auto",
        Overflow::Auto
    )];
    &[test_parse_stringify!(
        overflow_x,
        "overflow-x",
        "scroll",
        Overflow::Scroll
    )];

    &[test_parse_stringify!(
        overflow_wrap,
        "overflow-wrap",
        "normal",
        OverflowWrap::Normal
    )];
    &[test_parse_stringify!(
        overflow_wrap,
        "overflow-wrap",
        "break-word",
        OverflowWrap::BreakWord
    )];
}

#[test]
fn pointer_events() {
    &[test_parse_stringify!(
        pointer_events,
        "pointer-events",
        "auto",
        PointerEvents::Auto
    )];
    &[test_parse_stringify!(
        pointer_events,
        "pointer-events",
        "none",
        PointerEvents::None
    )];
    // test_parse_stringify!(pointer_events, "pointer-events", "-wx-root", PointerEvents::WxRoot);
}

#[test]
fn wx_engine_touch_event() {
    &[test_parse_stringify!(
        wx_engine_touch_event,
        "-wx-engine-touch-event",
        "gesture",
        WxEngineTouchEvent::Gesture
    )];
    &[test_parse_stringify!(
        wx_engine_touch_event,
        "-wx-engine-touch-event",
        "click",
        WxEngineTouchEvent::Click
    )];
    &[test_parse_stringify!(
        wx_engine_touch_event,
        "-wx-engine-touch-event",
        "none",
        WxEngineTouchEvent::None
    )];
}

#[test]
fn visibility() {
    &[test_parse_stringify!(
        visibility,
        "visibility",
        "visible",
        Visibility::Visible
    )];
    &[test_parse_stringify!(
        visibility,
        "visibility",
        "hidden",
        Visibility::Hidden
    )];
    &[test_parse_stringify!(
        visibility,
        "visibility",
        "collapse",
        Visibility::Collapse
    )];
}

#[test]
fn flex() {
    &[test_parse_stringify!(
        flex_wrap,
        "flex-wrap",
        "nowrap",
        FlexWrap::NoWrap
    )];
    &[test_parse_stringify!(
        flex_wrap,
        "flex-wrap",
        "wrap",
        FlexWrap::Wrap
    )];
    &[test_parse_stringify!(
        flex_wrap,
        "flex-wrap",
        "wrap-reverse",
        FlexWrap::WrapReverse
    )];

    &[test_parse_stringify!(
        flex_direction,
        "flex-direction",
        "row",
        FlexDirection::Row
    )];
    &[test_parse_stringify!(
        flex_direction,
        "flex-direction",
        "row-reverse",
        FlexDirection::RowReverse
    )];
    &[test_parse_stringify!(
        flex_direction,
        "flex-direction",
        "column",
        FlexDirection::Column
    )];
    &[test_parse_stringify!(
        flex_direction,
        "flex-direction",
        "column-reverse",
        FlexDirection::ColumnReverse
    )];
}

#[test]
fn direction() {
    // test_parse_stringify!(direction, "direction", "auto", Direction::Auto);
    &[test_parse_stringify!(
        direction,
        "direction",
        "ltr",
        Direction::LTR
    )];
    &[test_parse_stringify!(
        direction,
        "direction",
        "rtl",
        Direction::RTL
    )];
}

#[test]
fn writing_mode() {
    &[test_parse_stringify!(
        writing_mode,
        "writing-mode",
        "horizontal-tb",
        WritingMode::HorizontalTb
    )];
    &[test_parse_stringify!(
        writing_mode,
        "writing-mode",
        "vertical-lr",
        WritingMode::VerticalLr
    )];
    &[test_parse_stringify!(
        writing_mode,
        "writing-mode",
        "vertical-rl",
        WritingMode::VerticalRl
    )];
}

#[test]
fn align_items() {
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "stretch",
        AlignItems::Stretch
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "normal",
        AlignItems::Normal
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "center",
        AlignItems::Center
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "start",
        AlignItems::Start
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "end",
        AlignItems::End
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "flex-start",
        AlignItems::FlexStart
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "flex-end",
        AlignItems::FlexEnd
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "self-start",
        AlignItems::SelfStart
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "self-end",
        AlignItems::SelfEnd
    )];
    &[test_parse_stringify!(
        align_items,
        "align-items",
        "baseline",
        AlignItems::Baseline
    )];
}

#[test]
fn align_self() {
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "auto",
        AlignSelf::Auto
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "normal",
        AlignSelf::Normal
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "stretch",
        AlignSelf::Stretch
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "center",
        AlignSelf::Center
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "start",
        AlignSelf::Start
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "end",
        AlignSelf::End
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "self-start",
        AlignSelf::SelfStart
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "self-end",
        AlignSelf::SelfEnd
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "flex-start",
        AlignSelf::FlexStart
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "flex-end",
        AlignSelf::FlexEnd
    )];
    &[test_parse_stringify!(
        align_self,
        "align-self",
        "baseline",
        AlignSelf::Baseline
    )];
}

#[test]
fn align_content() {
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "normal",
        AlignContent::Normal
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "start",
        AlignContent::Start
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "end",
        AlignContent::End
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "stretch",
        AlignContent::Stretch
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "center",
        AlignContent::Center
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "flex-start",
        AlignContent::FlexStart
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "flex-end",
        AlignContent::FlexEnd
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "space-between",
        AlignContent::SpaceBetween
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "space-around",
        AlignContent::SpaceAround
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "space-evenly",
        AlignContent::SpaceEvenly
    )];
    &[test_parse_stringify!(
        align_content,
        "align-content",
        "baseline",
        AlignContent::Baseline
    )];
}

#[test]
fn justify_content() {
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "center",
        JustifyContent::Center
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "flex-start",
        JustifyContent::FlexStart
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "flex-end",
        JustifyContent::FlexEnd
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "space-between",
        JustifyContent::SpaceBetween
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "space-around",
        JustifyContent::SpaceAround
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "space-evenly",
        JustifyContent::SpaceEvenly
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "start",
        JustifyContent::Start
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "end",
        JustifyContent::End
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "left",
        JustifyContent::Left
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "right",
        JustifyContent::Right
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "stretch",
        JustifyContent::Stretch
    )];
    &[test_parse_stringify!(
        justify_content,
        "justify-content",
        "baseline",
        JustifyContent::Baseline
    )];
}

#[test]
fn justify_items() {
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "stretch",
        JustifyItems::Stretch
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "center",
        JustifyItems::Center
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "start",
        JustifyItems::Start
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "end",
        JustifyItems::End
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "flex-start",
        JustifyItems::FlexStart
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "flex-end",
        JustifyItems::FlexEnd
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "self-start",
        JustifyItems::SelfStart
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "self-end",
        JustifyItems::SelfEnd
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "left",
        JustifyItems::Left
    )];
    &[test_parse_stringify!(
        justify_items,
        "justify-items",
        "right",
        JustifyItems::Right
    )];
}

#[test]
fn text_align() {
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "left",
        TextAlign::Left
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "center",
        TextAlign::Center
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "right",
        TextAlign::Right
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "justify",
        TextAlign::Justify
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "justify-all",
        TextAlign::JustifyAll
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "start",
        TextAlign::Start
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "end",
        TextAlign::End
    )];
    &[test_parse_stringify!(
        text_align,
        "text-align",
        "match-parent",
        TextAlign::MatchParent
    )];
}

#[test]
fn font_weight() {
    &[test_parse_stringify!(
        font_weight,
        "font-weight",
        "normal",
        FontWeight::Normal
    )];
    &[test_parse_stringify!(
        font_weight,
        "font-weight",
        "bold",
        FontWeight::Bold
    )];
    &[test_parse_stringify!(
        font_weight,
        "font-weight",
        "bolder",
        FontWeight::Bolder
    )];
    &[test_parse_stringify!(
        font_weight,
        "font-weight",
        "lighter",
        FontWeight::Lighter
    )];
    &[test_parse_stringify!(
        font_weight,
        "font-weight",
        "200",
        FontWeight::Num(Number::F32(200.0))
    )];
}

#[test]
fn word_break() {
    &[test_parse_stringify!(
        word_break,
        "word-break",
        "break-word",
        WordBreak::BreakWord
    )];
    &[test_parse_stringify!(
        word_break,
        "word-break",
        "break-all",
        WordBreak::BreakAll
    )];
    &[test_parse_stringify!(
        word_break,
        "word-break",
        "keep-all",
        WordBreak::KeepAll
    )];
}

#[test]
fn white_space() {
    &[test_parse_stringify!(
        white_space,
        "white-space",
        "normal",
        WhiteSpace::Normal
    )];
    &[test_parse_stringify!(
        white_space,
        "white-space",
        "nowrap",
        WhiteSpace::NoWrap
    )];
    &[test_parse_stringify!(
        white_space,
        "white-space",
        "pre",
        WhiteSpace::Pre
    )];
    &[test_parse_stringify!(
        white_space,
        "white-space",
        "pre-wrap",
        WhiteSpace::PreWrap
    )];
    &[test_parse_stringify!(
        white_space,
        "white-space",
        "pre-line",
        WhiteSpace::PreLine
    )];
}

#[test]
fn text_overflow() {
    &[test_parse_stringify!(
        text_overflow,
        "text-overflow",
        "clip",
        TextOverflow::Clip
    )];
    &[test_parse_stringify!(
        text_overflow,
        "text-overflow",
        "ellipsis",
        TextOverflow::Ellipsis
    )];
}

#[test]
fn vertical_align() {
    &[test_parse_stringify!(
        vertical_align,
        "vertical-align",
        "baseline",
        VerticalAlign::Baseline
    )];
    &[test_parse_stringify!(
        vertical_align,
        "vertical-align",
        "top",
        VerticalAlign::Top
    )];
    &[test_parse_stringify!(
        vertical_align,
        "vertical-align",
        "middle",
        VerticalAlign::Middle
    )];
    &[test_parse_stringify!(
        vertical_align,
        "vertical-align",
        "bottom",
        VerticalAlign::Bottom
    )];
    &[test_parse_stringify!(
        vertical_align,
        "vertical-align",
        "text-top",
        VerticalAlign::TextTop
    )];
    &[test_parse_stringify!(
        vertical_align,
        "vertical-align",
        "text-bottom",
        VerticalAlign::TextBottom
    )];
}

#[test]
fn line_height() {
    &[test_parse_stringify!(
        line_height,
        "line-height",
        "normal",
        LineHeight::Normal
    )];
    &[test_parse_stringify!(
        line_height,
        "line-height",
        "12px",
        LineHeight::Length(Length::Px(12.0))
    )];
    &[test_parse_stringify!(
        line_height,
        "line-height",
        "100",
        LineHeight::Num(Number::F32(100.0))
    )];
}

#[test]
fn font_family() {
    &[test_parse_stringify!(
        font_family,
        "font-family",
        "serif, sans-serif, monospace, cursive, fantasy, system-ui, \"Gill Sans Extrabold\"",
        FontFamily::Names(Array::from(vec![
            FontFamilyName::Serif,
            FontFamilyName::SansSerif,
            FontFamilyName::Monospace,
            FontFamilyName::Cursive,
            FontFamilyName::Fantasy,
            FontFamilyName::SystemUi,
            FontFamilyName::Title(StrRef::from(String::from("Gill Sans Extrabold"))),
        ]))
    )];
}

#[test]
fn box_sizing() {
    &[test_parse_stringify!(
        box_sizing,
        "box-sizing",
        "content-box",
        BoxSizing::ContentBox
    )];
    &[test_parse_stringify!(
        box_sizing,
        "box-sizing",
        "padding-box",
        BoxSizing::PaddingBox
    )];
    &[test_parse_stringify!(
        box_sizing,
        "box-sizing",
        "border-box",
        BoxSizing::BorderBox
    )];
}

#[test]
fn border_style() {
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "none",
        BorderStyle::None
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "solid",
        BorderStyle::Solid
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "dotted",
        BorderStyle::Dotted
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "dashed",
        BorderStyle::Dashed
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "hidden",
        BorderStyle::Hidden
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "double",
        BorderStyle::Double
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "groove",
        BorderStyle::Groove
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "ridge",
        BorderStyle::Ridge
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "inset",
        BorderStyle::Inset
    )];
    &[test_parse_stringify!(
        border_top_style,
        "border-top-style",
        "outset",
        BorderStyle::Outset
    )];
}

#[test]
fn transform() {
    &[test_parse_stringify!(transform, "transform", "translate(10px, 5px) rotate(10deg) matrix(1, 2, 3, 4, 5, 6) translate3d(12px, 50%, 3em) scale(2, 0.5) rotate(0.5turn) rotate3d(1, 2.1, 3.1, 10deg) skew(30deg, 20deg) perspective(17px)", 
        Transform::Series(Array::from(
            vec![
                TransformItem::Translate2D(Length::Px(10.0), Length::Px(5.0)),
                TransformItem::Rotate2D(Angle::Deg(10.0)),
                TransformItem::Matrix([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
                TransformItem::Translate3D(Length::Px(12.0), Length::Ratio(0.5), Length::Px(48.0)),
                TransformItem::Scale2D(2.0, 0.5), TransformItem::Rotate2D(Angle::Turn(0.5)),
                TransformItem::Rotate3D(1.0, 2.1, 3.1, Angle::Deg(10.0)),
                TransformItem::Skew(Angle::Deg(30.0), Angle::Deg(20.0)),
                TransformItem::Perspective(Length::Px(17.0))
            ]
        ))
    )];
}

#[test]
fn transition_property() {
    &[test_parse_stringify!(
        transition_property,
        "transition-property", 
        "line-height, opacity, height, width, min-height, max-height, min-width, max-width, margin-top, margin-right, margin-left, margin-bottom, margin, padding-top, padding-right, padding-bottom, padding-left, padding, top, right, bottom, left, flex-grow, flex-shrink, flex-basis, flex, border-top-width, border-right-width, border-bottom-width, border-left-width, border-top-color, border-right-color, border-bottom-color, border-left-color, border-top-left-radius, border-top-right-radius, border-bottom-left-radius, border-bottom-right-radius, border, border-width, border-color, border-radius, border-left, border-top, border-right, border-bottom, z-index, box-shadow, backdrop-filter, filter, color, text-decoration-color, text-decoration-thickness, font-size, font-weight, letter-spacing, word-spacing, background-color, background-position, background-size, background", 
        TransitionProperty::List(Array::from(vec![
            TransitionPropertyItem::LineHeight,
            TransitionPropertyItem::Opacity,
            TransitionPropertyItem::Height,
            TransitionPropertyItem::Width,
            TransitionPropertyItem::MinHeight,
            TransitionPropertyItem::MaxHeight,
            TransitionPropertyItem::MinWidth,
            TransitionPropertyItem::MaxWidth,
            TransitionPropertyItem::MarginTop,
            TransitionPropertyItem::MarginRight,
            TransitionPropertyItem::MarginLeft,
            TransitionPropertyItem::MarginBottom,
            TransitionPropertyItem::Margin,
            TransitionPropertyItem::PaddingTop,
            TransitionPropertyItem::PaddingRight,
            TransitionPropertyItem::PaddingBottom,
            TransitionPropertyItem::PaddingLeft,
            TransitionPropertyItem::Padding,
            TransitionPropertyItem::Top,
            TransitionPropertyItem::Right,
            TransitionPropertyItem::Bottom,
            TransitionPropertyItem::Left,
            TransitionPropertyItem::FlexGrow,
            TransitionPropertyItem::FlexShrink,
            TransitionPropertyItem::FlexBasis,
            TransitionPropertyItem::Flex,
            TransitionPropertyItem::BorderTopWidth,
            TransitionPropertyItem::BorderRightWidth,
            TransitionPropertyItem::BorderBottomWidth,
            TransitionPropertyItem::BorderLeftWidth,
            TransitionPropertyItem::BorderTopColor,
            TransitionPropertyItem::BorderRightColor,
            TransitionPropertyItem::BorderBottomColor,
            TransitionPropertyItem::BorderLeftColor,
            TransitionPropertyItem::BorderTopLeftRadius,
            TransitionPropertyItem::BorderTopRightRadius,
            TransitionPropertyItem::BorderBottomLeftRadius,
            TransitionPropertyItem::BorderBottomRightRadius,
            TransitionPropertyItem::Border,
            TransitionPropertyItem::BorderWidth,
            TransitionPropertyItem::BorderColor,
            TransitionPropertyItem::BorderRadius,
            TransitionPropertyItem::BorderLeft,
            TransitionPropertyItem::BorderTop,
            TransitionPropertyItem::BorderRight,
            TransitionPropertyItem::BorderBottom,
            TransitionPropertyItem::ZIndex,
            TransitionPropertyItem::BoxShadow,
            TransitionPropertyItem::BackdropFilter,
            TransitionPropertyItem::Filter,
            TransitionPropertyItem::Color,
            TransitionPropertyItem::TextDecorationColor,
            TransitionPropertyItem::TextDecorationThickness,
            TransitionPropertyItem::FontSize,
            TransitionPropertyItem::FontWeight,
            TransitionPropertyItem::LetterSpacing,
            TransitionPropertyItem::WordSpacing,
            TransitionPropertyItem::BackgroundColor,
            TransitionPropertyItem::BackgroundPosition,
            TransitionPropertyItem::BackgroundSize,
            TransitionPropertyItem::Background
        ]))
    )];
}

#[test]
fn transition_timing_function() {
    &[test_parse_stringify!(transition_timing_function , "transition-timing-function", "linear, ease, ease-in, ease-out, ease-in-out, step-start, step-end, steps(4, end), steps(4, jump-start), steps(4, jump-end), steps(4, jump-none), steps(4, start), cubic-bezier(0.1, 0.7, 1, 0.1)", TransitionTimingFn::List(Array::from(vec![
        TransitionTimingFnItem::Linear,
        TransitionTimingFnItem::Ease,
        TransitionTimingFnItem::EaseIn,
        TransitionTimingFnItem::EaseOut,
        TransitionTimingFnItem::EaseInOut,
        TransitionTimingFnItem::StepStart,
        TransitionTimingFnItem::StepEnd,
        TransitionTimingFnItem::Steps(4, StepPosition::End),
        TransitionTimingFnItem::Steps(4, StepPosition::JumpStart),
        TransitionTimingFnItem::Steps(4, StepPosition::JumpEnd),
        TransitionTimingFnItem::Steps(4, StepPosition::JumpNone),
        // TransitionTimingFnItem::Steps(4, StepPosition::JumpBoth),  property.ts未实现
        TransitionTimingFnItem::Steps(4, StepPosition::Start),
        TransitionTimingFnItem::CubicBezier(0.1, 0.7, 1.0, 0.1),
    ])))];
}

#[test]
fn transition_duration() {
    &[test_parse_stringify!(
        transition_duration,
        "transition-duration",
        "10s, 30ms, 230ms",
        TransitionTime::List(Array::from(vec![10000, 30, 230]))
    )];
}

#[test]
fn wx_scrollbar() {
    &[test_parse_stringify!(
        wx_scrollbar_x,
        "-wx-scrollbar-x",
        "hidden",
        Scrollbar::Hidden
    )];
    &[test_parse_stringify!(
        wx_scrollbar_x,
        "-wx-scrollbar-x",
        "auto-hide",
        Scrollbar::AutoHide
    )];
    &[test_parse_stringify!(
        wx_scrollbar_x,
        "-wx-scrollbar-x",
        "always-show",
        Scrollbar::AlwaysShow
    )];
}

#[test]
fn background_repeat() {
    &[test_parse_stringify!(
        background_repeat,
        "background-repeat",
        "repeat no-repeat, space round",
        BackgroundRepeat::List(Array::from(vec![
            BackgroundRepeatItem::Pos(
                BackgroundRepeatValue::Repeat,
                BackgroundRepeatValue::NoRepeat
            ),
            BackgroundRepeatItem::Pos(BackgroundRepeatValue::Space, BackgroundRepeatValue::Round),
        ]))
    )];
}

#[test]
fn background_size() {
    &[test_parse_stringify!(
        background_size,
        "background-size",
        "3em auto, cover, contain",
        BackgroundSize::List(Array::from(vec![
            BackgroundSizeItem::Length(Length::Px(48.0), Length::Auto),
            BackgroundSizeItem::Cover,
            BackgroundSizeItem::Contain,
        ]))
    )];
}

#[test]
fn background_image() {
    &[test_parse_stringify!(
        background_image,
        "background-image",
        r#"none, url("a/b.png"), linear-gradient(120deg, green 40%, red 100%), linear-gradient(green 20%, blue 75%, red 100%), radial-gradient(circle closest-corner at left bottom, green 20%, blue 75%, red 100%), radial-gradient(ellipse 20px 30% at 80% 30px, green 20%, blue 75%, red 100%), image(rtl url(wechat.png), red), image(url(wechat.png)), element(#ele)"#,
        BackgroundImage::List(Array::from(vec![
            BackgroundImageItem::None,
            BackgroundImageItem::Url("a/b.png".to_string().into()),
            BackgroundImageItem::Gradient(BackgroundImageGradientItem::LinearGradient(
                Angle::Deg(120.0),
                vec![
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 128, 0, 255),
                        Length::Ratio(0.4)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(255, 0, 0, 255),
                        Length::Ratio(1.0)
                    )
                ]
                .into()
            )),
            BackgroundImageItem::Gradient(BackgroundImageGradientItem::LinearGradient(
                Angle::Deg(180.),
                vec![
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 128, 0, 255),
                        Length::Ratio(0.2)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 0, 255, 255),
                        Length::Ratio(0.75)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(255, 0, 0, 255),
                        Length::Ratio(1.0)
                    )
                ]
                .into()
            )),
            BackgroundImageItem::Gradient(BackgroundImageGradientItem::RadialGradient(
                GradientShape::Circle,
                GradientSize::ClosestCorner,
                GradientPosition::Pos(Length::Ratio(0.), Length::Ratio(1.)),
                vec![
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 128, 0, 255),
                        Length::Ratio(0.2)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 0, 255, 255),
                        Length::Ratio(0.75)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(255, 0, 0, 255),
                        Length::Ratio(1.0)
                    )
                ]
                .into()
            )),
            BackgroundImageItem::Gradient(BackgroundImageGradientItem::RadialGradient(
                GradientShape::Ellipse,
                GradientSize::Len(Length::Px(20.), Length::Ratio(0.3)),
                GradientPosition::Pos(Length::Ratio(0.8), Length::Px(30.)),
                vec![
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 128, 0, 255),
                        Length::Ratio(0.2)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(0, 0, 255, 255),
                        Length::Ratio(0.75)
                    ),
                    GradientColorItem::ColorHint(
                        Color::Specified(255, 0, 0, 255),
                        Length::Ratio(1.0)
                    )
                ]
                .into()
            )),
            BackgroundImageItem::Image(
                ImageTags::RTL,
                ImageSource::Url("wechat.png".to_string().into()),
                Color::Specified(255, 0, 0, 255)
            ),
            BackgroundImageItem::Image(
                ImageTags::LTR,
                ImageSource::Url("wechat.png".to_string().into()),
                Color::Undefined
            ),
            BackgroundImageItem::Element("ele".to_string().into())
        ]))
    )];
}

#[test]
fn font_style() {
    &[test_parse_stringify!(
        font_style,
        "font-style",
        "normal",
        FontStyle::Normal
    )];
    &[test_parse_stringify!(
        font_style,
        "font-style",
        "italic",
        FontStyle::Italic
    )];
    &[test_parse_stringify!(
        font_style,
        "font-style",
        "oblique",
        FontStyle::Oblique(Angle::Deg(14.))
    )];
    &[test_parse_stringify!(
        font_style,
        "font-style",
        "oblique 10deg",
        FontStyle::Oblique(Angle::Deg(10.))
    )];
}

#[test]
fn background_position() {
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "center",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(0.5)),
                BackgroundPositionValue::Top(Length::Ratio(0.5))
            )]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "left, right",
        BackgroundPosition::List(
            vec![
                BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                ),
                BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(1.)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )
            ]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "20% bottom",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(0.2)),
                BackgroundPositionValue::Top(Length::Ratio(1.))
            ),]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "30% 70%, center",
        BackgroundPosition::List(
            vec![
                BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.3)),
                    BackgroundPositionValue::Top(Length::Ratio(0.7))
                ),
                BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    BackgroundPositionValue::Top(Length::Ratio(0.5))
                )
            ]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "right",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(1.)),
                BackgroundPositionValue::Top(Length::Ratio(0.5))
            ),]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "left 100px bottom 20%",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Px(100.)),
                BackgroundPositionValue::Bottom(Length::Ratio(0.2))
            ),]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "right bottom 70%",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(1.)),
                BackgroundPositionValue::Bottom(Length::Ratio(0.7))
            ),]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "right top",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(1.)),
                BackgroundPositionValue::Top(Length::Ratio(0.))
            ),]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "right 20px top",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Right(Length::Px(20.)),
                BackgroundPositionValue::Top(Length::Ratio(0.))
            ),]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "left 30%",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(0.)),
                BackgroundPositionValue::Top(Length::Ratio(0.3))
            )]
            .into()
        )
    )];
    &[test_parse_stringify!(
        background_position,
        "background-position",
        "left 30%",
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(Length::Ratio(0.)),
                BackgroundPositionValue::Top(Length::Ratio(0.3))
            ),]
            .into()
        )
    )];
}

#[test]
fn float() {
    &[test_parse_stringify!(float, "float", "none", Float::None)];
    &[test_parse_stringify!(float, "float", "left", Float::Left)];
    &[test_parse_stringify!(float, "float", "right", Float::Right)];
    &[test_parse_stringify!(
        float,
        "float",
        "inline-start",
        Float::InlineStart
    )];
    &[test_parse_stringify!(
        float,
        "float",
        "inline-end",
        Float::InlineEnd
    )];
}

#[test]
fn background_clip() {
    &[test_parse_stringify!(
        background_clip,
        "background-clip",
        "border-box",
        BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into())
    )];
    &[test_parse_stringify!(
        background_clip,
        "background-clip",
        "padding-box, content-box",
        BackgroundClip::List(
            vec![
                BackgroundClipItem::PaddingBox,
                BackgroundClipItem::ContentBox
            ]
            .into()
        )
    )];
}

#[test]
fn background_origin() {
    &[test_parse_stringify!(
        background_origin,
        "background-origin",
        "border-box",
        BackgroundOrigin::List(vec![BackgroundOriginItem::BorderBox].into())
    )];
    &[test_parse_stringify!(
        background_origin,
        "background-origin",
        "padding-box, content-box",
        BackgroundOrigin::List(
            vec![
                BackgroundOriginItem::PaddingBox,
                BackgroundOriginItem::ContentBox
            ]
            .into()
        )
    )];
}

#[test]
fn background_attachment() {
    &[test_parse_stringify!(
        background_attachment,
        "background-attachment",
        "local",
        BackgroundAttachment::List(vec![BackgroundAttachmentItem::Local].into())
    )];
    &[test_parse_stringify!(
        background_attachment,
        "background-attachment",
        "fixed, scroll",
        BackgroundAttachment::List(
            vec![
                BackgroundAttachmentItem::Fixed,
                BackgroundAttachmentItem::Scroll
            ]
            .into()
        )
    )];
}

#[test]
fn list_style_type() {
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "disc",
        ListStyleType::Disc
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "none",
        ListStyleType::None
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "circle",
        ListStyleType::Circle
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "square",
        ListStyleType::Square
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "decimal",
        ListStyleType::Decimal
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "cjk-decimal",
        ListStyleType::CjkDecimal
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "decimal-leading-zero",
        ListStyleType::DecimalLeadingZero
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "lower-roman",
        ListStyleType::LowerRoman
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "upper-roman",
        ListStyleType::UpperRoman
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "lower-greek",
        ListStyleType::LowerGreek
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "lower-alpha",
        ListStyleType::LowerAlpha
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "lower-latin",
        ListStyleType::LowerLatin
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "upper-alpha",
        ListStyleType::UpperAlpha
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "upper-latin",
        ListStyleType::UpperLatin
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "armenian",
        ListStyleType::Armenian
    )];
    &[test_parse_stringify!(
        list_style_type,
        "list-style-type",
        "georgian",
        ListStyleType::Georgian
    )];
}

#[test]
fn list_style_position() {
    &[test_parse_stringify!(
        list_style_position,
        "list-style-position",
        "outside",
        ListStylePosition::Outside
    )];
    &[test_parse_stringify!(
        list_style_position,
        "list-style-position",
        "inside",
        ListStylePosition::Inside
    )];
}

#[test]
fn resize() {
    &[test_parse_stringify!(
        resize,
        "resize",
        "none",
        Resize::None
    )];
    &[test_parse_stringify!(
        resize,
        "resize",
        "both",
        Resize::Both
    )];
    &[test_parse_stringify!(
        resize,
        "resize",
        "horizontal",
        Resize::Horizontal
    )];
    &[test_parse_stringify!(
        resize,
        "resize",
        "vertical",
        Resize::Vertical
    )];
    &[test_parse_stringify!(
        resize,
        "resize",
        "block",
        Resize::Block
    )];
    &[test_parse_stringify!(
        resize,
        "resize",
        "inline",
        Resize::Inline
    )];
}

#[test]
fn list_style_image() {
    &[test_parse_stringify!(
        list_style_image,
        "list-style-image",
        "url(wechat.gif)",
        ListStyleImage::Url("wechat.gif".to_string().into())
    )];
}

#[test]
fn z_index() {
    &[test_parse_stringify!(
        z_index,
        "z-index",
        "auto",
        ZIndex::Auto
    )];
    &[test_parse_stringify!(
        z_index,
        "z-index",
        "999",
        ZIndex::Num(Number::I32(999))
    )];
}

#[test]
fn text_shadow() {
    &[test_parse_stringify!(
        text_shadow,
        "text-shadow",
        "none",
        TextShadow::None
    )];
    &[test_parse_stringify!(
        text_shadow,
        "text-shadow",
        "1px 1px 2px black",
        TextShadow::List(
            vec![TextShadowItem::TextShadowValue(
                Length::Px(1.),
                Length::Px(1.),
                Length::Px(2.),
                Color::Specified(0, 0, 0, 255)
            )]
            .into()
        )
    )];
    &[test_parse_stringify!(
        text_shadow,
        "text-shadow",
        "2px 5px white",
        TextShadow::List(
            vec![TextShadowItem::TextShadowValue(
                Length::Px(2.),
                Length::Px(5.),
                Length::Undefined,
                Color::Specified(255, 255, 255, 255)
            )]
            .into()
        )
    )];
    &[test_parse_stringify!(
        text_shadow,
        "text-shadow",
        "1px 1px 2px rgb(1, 1, 1), 3px 4px 5px rgb(2, 1, 1)",
        TextShadow::List(Array::from(vec![
            TextShadowItem::TextShadowValue(
                Length::Px(1.0),
                Length::Px(1.0),
                Length::Px(2.0),
                Color::Specified(1, 1, 1, 255)
            ),
            TextShadowItem::TextShadowValue(
                Length::Px(3.0),
                Length::Px(4.0),
                Length::Px(5.0),
                Color::Specified(2, 1, 1, 255)
            )
        ]))
    )];
    &[test_parse_stringify!(
        text_shadow,
        "text-shadow",
        "4px 5px",
        TextShadow::List(
            vec![TextShadowItem::TextShadowValue(
                Length::Px(4.),
                Length::Px(5.),
                Length::Undefined,
                Color::Undefined
            )]
            .into()
        )
    )];
    &[test_parse_stringify!(
        text_shadow,
        "text-shadow",
        "1px 1px 2px rgb(1, 1, 1), 3px 4px 5px rgb(2, 1, 1)",
        TextShadow::List(Array::from(vec![
            TextShadowItem::TextShadowValue(
                Length::Px(1.0),
                Length::Px(1.0),
                Length::Px(2.0),
                Color::Specified(1, 1, 1, 255)
            ),
            TextShadowItem::TextShadowValue(
                Length::Px(3.0),
                Length::Px(4.0),
                Length::Px(5.0),
                Color::Specified(2, 1, 1, 255)
            )
        ]))
    )];
}

#[test]
fn text_decoration_line() {
    &[test_parse_stringify!(
        text_decoration_line,
        "text-decoration-line",
        "none",
        TextDecorationLine::None
    )];
    &[test_parse_stringify!(
        text_decoration_line,
        "text-decoration-line",
        "spelling-error",
        TextDecorationLine::SpellingError
    )];
    &[test_parse_stringify!(
        text_decoration_line,
        "text-decoration-line",
        "grammar-error",
        TextDecorationLine::GrammarError
    )];
    &[test_parse_stringify!(
        text_decoration_line,
        "text-decoration-line",
        "underline overline line-through",
        TextDecorationLine::List(
            vec![
                TextDecorationLineItem::Underline,
                TextDecorationLineItem::Overline,
                TextDecorationLineItem::LineThrough,
            ]
            .into()
        )
    )];
}

#[test]
fn text_decoration_style() {
    &[test_parse_stringify!(
        text_decoration_style,
        "text-decoration-style",
        "solid",
        TextDecorationStyle::Solid
    )];
    &[test_parse_stringify!(
        text_decoration_style,
        "text-decoration-style",
        "double",
        TextDecorationStyle::Double
    )];
    &[test_parse_stringify!(
        text_decoration_style,
        "text-decoration-style",
        "dotted",
        TextDecorationStyle::Dotted
    )];
    &[test_parse_stringify!(
        text_decoration_style,
        "text-decoration-style",
        "dashed",
        TextDecorationStyle::Dashed
    )];
    &[test_parse_stringify!(
        text_decoration_style,
        "text-decoration-style",
        "wavy",
        TextDecorationStyle::Wavy
    )];
}

#[test]
fn text_decoration_thickness() {
    // test_parse_stringify!(text_decoration_thickness, "text-decoration-thickness", "auto", TextDecorationThickness::Auto);
    &[test_parse_stringify!(
        text_decoration_thickness,
        "text-decoration-thickness",
        "from-font",
        TextDecorationThickness::FromFont
    )];
    &[test_parse_stringify!(
        text_decoration_thickness,
        "text-decoration-thickness",
        "10%",
        TextDecorationThickness::Length(Length::Px(1.6))
    )];
}

#[test]
fn letter_spacing() {
    &[test_parse_stringify!(
        letter_spacing,
        "letter-spacing",
        "normal",
        LetterSpacing::Normal
    )];
    &[test_parse_stringify!(
        letter_spacing,
        "letter-spacing",
        "3em",
        LetterSpacing::Length(Length::Px(48.))
    )];
}

#[test]
fn word_spacing() {
    &[test_parse_stringify!(
        word_spacing,
        "word-spacing",
        "normal",
        WordSpacing::Normal
    )];
    &[test_parse_stringify!(
        word_spacing,
        "word-spacing",
        "3em",
        WordSpacing::Length(Length::Px(48.))
    )];
}

#[test]
fn border_top_left_radius() {
    &[test_parse_stringify!(
        border_top_left_radius,
        "border-top-left-radius",
        "40px 40px",
        BorderRadius::Pos(Length::Px(40.), Length::Px(40.))
    )];
}

#[test]
fn box_shadow() {
    &[test_parse_stringify!(
        box_shadow,
        "box-shadow",
        "10px 5px 5px 0px black",
        BoxShadow::List(
            vec![BoxShadowItem::List(
                vec![
                    ShadowItemType::OffsetX(Length::Px(10.)),
                    ShadowItemType::OffsetY(Length::Px(5.)),
                    ShadowItemType::BlurRadius(Length::Px(5.)),
                    ShadowItemType::SpreadRadius(Length::Px(0.)),
                    ShadowItemType::Color(Color::Specified(0, 0, 0, 255))
                ]
                .into()
            ),]
            .into()
        )
    )];
}

#[test]
fn filter() {
    &[test_parse_stringify!(
        filter,
        "filter",
        "url(filters.svg#filter) blur(4px) saturate(150%)",
        Filter::List(
            vec![
                FilterFunc::Url("filters.svg#filter".to_string().into()),
                FilterFunc::Blur(Length::Px(4.)),
                FilterFunc::Saturate(Length::Ratio(1.5))
            ]
            .into(),
        )
    )];
}

#[test]
fn transform_origin() {
    &[test_parse_stringify!(
        transform_origin,
        "transform-origin",
        "20% 10px 10px",
        TransformOrigin::LengthTuple(Length::Ratio(0.2), Length::Px(10.), Length::Px(10.))
    )];
    &[test_parse_stringify!(
        transform_origin,
        "transform-origin",
        "right bottom",
        TransformOrigin::LengthTuple(Length::Ratio(1.), Length::Ratio(1.), Length::Px(0.))
    )];
}

#[test]
fn mask_mode() {
    &[test_parse_stringify!(
        mask_mode,
        "mask-mode",
        "luminance, match-source, alpha",
        MaskMode::List(
            vec![
                MaskModeItem::Luminance,
                MaskModeItem::MatchSource,
                MaskModeItem::Alpha
            ]
            .into()
        )
    )];
}

#[test]
fn aspect_ratio() {
    &[test_parse_stringify!(
        aspect_ratio,
        "aspect-ratio",
        "auto",
        AspectRatio::Auto
    )];
    &[test_parse_stringify!(
        aspect_ratio,
        "aspect-ratio",
        "2 / 3",
        AspectRatio::Ratio(Number::F32(2.0), Number::F32(3.0))
    )];
}
