#![allow(
    clippy::unused_unit,
    clippy::needless_question_mark,
    clippy::type_complexity
)]

//! The list of supported CSS properties.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use cssparser::{ParseError, Parser, SourcePosition};

use super::parser::{property_value::*, CustomError, ParseState};
use super::resolve_font_size::ResolveFontSize;
use super::sheet::borrow::Array;
pub use super::sheet::PropertyMeta;
use super::typing::*;
use float_pigment_css_macro::*;

property_list! (PropertyValueWithGlobal, {
    // basic positioning
    0x01 Display: DisplayType as Initial default Display::Inline;
    0x02 Position: PositionType as Initial default Position::Static;
    0x03 OverflowX: OverflowType as Initial default Overflow::Visible;
    0x04 OverflowY: OverflowType as Initial default Overflow::Visible;
    0x05 PointerEvents: PointerEventsType as Inherit default PointerEvents::Auto;
    0x06 WxEngineTouchEvent: WxEngineTouchEventType as Inherit default WxEngineTouchEvent::Gesture;
    0x07 WxPartialZIndex: NumberType as Initial default Number::F32(0.);
    0x08 BoxSizing: BoxSizingType as Initial default BoxSizing::ContentBox;
    0x09 Transform: TransformType as Initial default Transform::Series(Array::empty());
    0x0a WxLineClamp: NumberType as Initial default Number::F32(0.);
    0x0b Float: FloatType as Initial default Float::None;
    0x0c OverflowWrap: OverflowWrapType as Inherit default OverflowWrap::Normal;
    0x0d Resize: ResizeType as Initial default Resize::None;
    0x0e ZIndex: ZIndexType as Initial default ZIndex::Auto;
    0x0f WxPointerEventRoot: PointerEventsType as Initial default PointerEvents::None;

    // color and visibility related
    0x10 Visibility: VisibilityType as Inherit default Visibility::Visible;
    0x11 Color: ColorType as Inherit default Color::Specified(0, 0, 0, 255);
    0x12 Opacity: NumberType as Initial default Number::F32(1.);
    0x13 CaretColor: ColorType as Inherit default Color::Undefined;

    // flex
    0x20 FlexDirection: FlexDirectionType as Initial default FlexDirection::Row;
    0x21 FlexWrap: FlexWrapType as Initial default FlexWrap::NoWrap;
    0x22 AlignItems: AlignItemsType as Initial default AlignItems::Stretch;
    0x23 AlignSelf: AlignSelfType as Initial default AlignSelf::Auto;
    0x24 AlignContent: AlignContentType as Initial default AlignContent::Stretch;
    0x25 JustifyContent: JustifyContentType as Initial default JustifyContent::FlexStart;
    0x26 FlexGrow: NumberType as Initial default Number::F32(0.);
    0x27 FlexShrink: NumberType as Initial default Number::F32(1.);
    0x28 FlexBasis: LengthType as Initial default Length::Undefined, resolver = Length::resolve_em;
    0x29 JustifyItems: JustifyItemsType as Initial default JustifyItems::Stretch;
    0x2a Order: NumberType as Initial default Number::I32(0);
    0x2b RowGap: GapType as Initial default Gap::Normal;
    0x2c ColumnGap: GapType as Initial default Gap::Normal;

    // background
    0x30 BackgroundColor: ColorType as Initial default Color::Specified(0, 0, 0, 0);
    0x31 BackgroundImage: BackgroundImageType as Initial default BackgroundImage::List(Array::empty());
    0x32 BackgroundSize: BackgroundSizeType as Initial default BackgroundSize::List(vec![BackgroundSizeItem::Auto].into());
    0x33 BackgroundPosition: BackgroundPositionType as Initial deprecated default BackgroundPosition::List(vec![BackgroundPositionItem::Pos(BackgroundPositionValue::Left(Length::Ratio(0.)), BackgroundPositionValue::Top(Length::Ratio(0.)))].into());
    0x34 BackgroundRepeat: BackgroundRepeatType as Initial default BackgroundRepeat::List(vec![BackgroundRepeatItem::Pos(BackgroundRepeatValue::Repeat, BackgroundRepeatValue::Repeat)].into());
    0x35 BackgroundAttachment: BackgroundAttachmentType as Initial default BackgroundAttachment::List(vec![BackgroundAttachmentItem::Scroll].into());
    0x36 BackgroundClip: BackgroundClipType as Initial default BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into());
    0x37 BackgroundOrigin: BackgroundOriginType as Initial default BackgroundOrigin::List(vec![BackgroundOriginItem::PaddingBox].into());
    0x38 BackgroundPositionX: BackgroundPositionType as Initial default BackgroundPosition::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.)))].into());
    0x39 BackgroundPositionY: BackgroundPositionType as Initial default BackgroundPosition::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.)))].into());

    // mask-*
    0x3a MaskSize: BackgroundSizeType as Initial default BackgroundSize::List(vec![BackgroundSizeItem::Auto].into());
    0x3b MaskRepeat: BackgroundRepeatType as Initial default BackgroundRepeat::List(vec![BackgroundRepeatItem::Pos(BackgroundRepeatValue::NoRepeat, BackgroundRepeatValue::NoRepeat)].into());
    0x3c MaskOrigin: BackgroundOriginType as Initial default BackgroundOrigin::List(vec![BackgroundOriginItem::BorderBox].into());
    0x3d MaskClip: BackgroundClipType as Initial default BackgroundClip::List(vec![BackgroundClipItem::BorderBox].into());
    0x3e MaskPosition: BackgroundPositionType as Initial deprecated default BackgroundPosition::List(vec![BackgroundPositionItem::Pos(BackgroundPositionValue::Left(Length::Ratio(0.5)), BackgroundPositionValue::Top(Length::Ratio(0.5)))].into());
    0x3f MaskMode: MaskModeType as Initial default MaskMode::List(vec![MaskModeItem::MatchSource].into());

    // basic sizing
    0x40 Width: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x41 Height: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x42 MinWidth: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x43 MinHeight: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x44 MaxWidth: LengthType as Initial default Length::Undefined, resolver = Length::resolve_em;
    0x45 MaxHeight: LengthType as Initial default Length::Undefined, resolver = Length::resolve_em;
    0x46 Left: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x47 Right: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x48 Top: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;
    0x49 Bottom: LengthType as Initial default Length::Auto, resolver = Length::resolve_em;

    // padding and margin
    0x50 PaddingLeft: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x51 PaddingRight: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x52 PaddingTop: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x53 PaddingBottom: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x54 MarginLeft: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x55 MarginRight: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x56 MarginTop: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;
    0x57 MarginBottom: LengthType as Initial default Length::Px(0.), resolver = Length::resolve_em;

    // other
    0x58 MaskPositionX: BackgroundPositionType as Initial default BackgroundPosition::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.)))].into());
    0x59 MaskPositionY: BackgroundPositionType as Initial default BackgroundPosition::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.)))].into());

    // border
    0x60 BorderLeftWidth: LengthType as Initial default Length::Px(3.), resolver = Length::resolve_em;
    0x61 BorderLeftStyle: BorderStyleType as Initial default BorderStyle::None;
    0x62 BorderLeftColor: ColorType as Initial default Color::CurrentColor;
    0x63 BorderRightWidth: LengthType as Initial default Length::Px(3.), resolver = Length::resolve_em;
    0x64 BorderRightStyle: BorderStyleType as Initial default BorderStyle::None;
    0x65 BorderRightColor: ColorType as Initial default Color::CurrentColor;
    0x66 BorderTopWidth: LengthType as Initial default Length::Px(3.), resolver = Length::resolve_em;
    0x67 BorderTopStyle: BorderStyleType as Initial default BorderStyle::None;
    0x68 BorderTopColor: ColorType as Initial default Color::CurrentColor;
    0x69 BorderBottomWidth: LengthType as Initial default Length::Px(3.), resolver = Length::resolve_em;
    0x6a BorderBottomStyle: BorderStyleType as Initial default BorderStyle::None;
    0x6b BorderBottomColor: ColorType as Initial default Color::CurrentColor;
    0x6c BoxShadow: BoxShadowType as Initial default BoxShadow::None;

    // border radius
    0x70 BorderTopLeftRadius: BorderRadiusType as Initial default BorderRadius::Pos(Length::Px(0.), Length::Px(0.));
    0x71 BorderTopRightRadius: BorderRadiusType as Initial default BorderRadius::Pos(Length::Px(0.), Length::Px(0.));
    0x72 BorderBottomRightRadius: BorderRadiusType as Initial default BorderRadius::Pos(Length::Px(0.), Length::Px(0.));
    0x73 BorderBottomLeftRadius: BorderRadiusType as Initial default BorderRadius::Pos(Length::Px(0.), Length::Px(0.));

    // transition
    0x80 TransitionProperty: TransitionPropertyType as Initial default TransitionProperty::List(Array::empty());
    0x81 TransitionDuration: TransitionTimeType as Initial default TransitionTime::List(Array::empty());
    0x82 TransitionTimingFunction: TransitionTimingFnType as Initial default TransitionTimingFn::List(Array::empty());
    0x83 TransitionDelay: TransitionTimeType as Initial default TransitionTime::ListI32(Array::empty());

    // animation
    0x84 AnimationDuration: TransitionTimeType as Initial default TransitionTime::List(Array::empty());
    0x85 AnimationTimingFunction: TransitionTimingFnType as Initial default TransitionTimingFn::List(Array::empty());
    0x86 AnimationDelay: TransitionTimeType as Initial default TransitionTime::ListI32(Array::empty());
    0x87 AnimationIterationCount: AnimationIterationCountType as Initial default AnimationIterationCount::List(Array::empty());
    0x88 AnimationDirection: AnimationDirectionType as Initial default AnimationDirection::List(Array::empty());
    0x89 AnimationFillMode: AnimationFillModeType as Initial default AnimationFillMode::List(Array::empty());
    0x8a AnimationPlayState: AnimationPlayStateType as Initial default AnimationPlayState::List(Array::empty());
    0x8b AnimationName: AnimationNameType as Initial default AnimationName::List(Array::empty());
    0x8c WillChange: WillChangeType as Initial default WillChange::Auto;

    // typography
    0x90 FontSize: LengthType as Inherit default Length::Undefined, resolver = Length::resolve_set;
    0x91 Direction: DirectionType as Inherit default Direction::Auto;
    0x92 WritingMode: WritingModeType as Inherit default WritingMode::HorizontalTb;
    0x93 LineHeight: LineHeightType as Inherit default LineHeight::Normal;
    0x94 TextAlign: TextAlignType as Inherit default TextAlign::Left;
    0x95 FontWeight: FontWeightType as Inherit default FontWeight::Normal;
    0x96 WordBreak: WordBreakType as Inherit default WordBreak::BreakWord;
    0x97 WhiteSpace: WhiteSpaceType as Inherit default WhiteSpace::Normal;
    0x98 TextOverflow: TextOverflowType as Inherit default TextOverflow::Clip;
    0x99 TextIndent: LengthType as Initial default Length::Undefined, resolver = Length::resolve_em_and_ratio;
    0x9a VerticalAlign: VerticalAlignType as Initial default VerticalAlign::Baseline;
    0x9b LetterSpacing: LetterSpacingType as Inherit default LetterSpacing::Normal;
    0x9c WordSpacing: WordSpacingType as Inherit default WordSpacing::Normal;
    0x9d FontFamily: FontFamilyType as Inherit default FontFamily::Names(Array::empty());
    0x9e FontStyle: FontStyleType as Inherit default FontStyle::Normal;
    0x9f TextShadow: TextShadowType as Inherit default TextShadow::None;
    0xa0 TextDecorationLine: TextDecorationLineType as Initial default TextDecorationLine::None;
    0xa1 TextDecorationStyle: TextDecorationStyleType as Initial default TextDecorationStyle::Solid;
    0xa2 TextDecorationColor: ColorType as Initial default Color::CurrentColor;
    0xa3 TextDecorationThickness: TextDecorationThicknessType as Initial default TextDecorationThickness::Auto;
    0xa4 FontFeatureSettings: FontFeatureSettingsType as Inherit default FontFeatureSettings::Normal;

    0xd0 ListStyleType: ListStyleTypeType as Inherit default ListStyleType::Disc;
    0xd1 ListStyleImage: ListStyleImageType as Inherit default ListStyleImage::None;
    0xd2 ListStylePosition: ListStylePositionType as Inherit default ListStylePosition::Outside;
    0xd3 BackdropFilter: BackdropFilterType as Initial default BackdropFilter::None;
    0xd4 Filter: FilterType as Initial default Filter::None;
    0xd5 TransformOrigin: TransformOriginType as Initial default TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.5), Length::Px(0.));
    0xd6 MaskImage: BackgroundImageType as Initial default BackgroundImage::List(Array::empty());
    0xd7 AspectRatio: AspectRatioType as Initial default AspectRatio::Auto;
    0xd8 Contain: ContainType as Initial default Contain::None;
    0xd9 Content: ContentType as Initial default Content::None;

    // wx-spec special properties
    0xe0 WxScrollbarX: ScrollbarType as Initial default Scrollbar::Auto;
    0xe1 WxScrollbarXColor: ColorType as Initial default Color::Undefined;
    0xe2 WxScrollbarY: ScrollbarType as Initial default Scrollbar::Auto;
    0xe3 WxScrollbarYColor: ColorType as Initial default Color::Undefined;
    0xe4 WxContain: ContainType as Initial default Contain::None;

    0xfa CustomProperty: CustomPropertyType as Initial default CustomProperty::None;
    // considering bincode performance, the max value should be 0xfa
});

property_value_format! (PropertyValueWithGlobal, {
    display: {{ Display
        = "none" => DisplayType::None
        | "block" => DisplayType::Block
        | "inline" => DisplayType::Inline
        | "inline-block" => DisplayType::InlineBlock
        | "flex" => DisplayType::Flex
        | "grid" => DisplayType::Grid
        | "flow-root" => DisplayType::FlowRoot
        | "inline-flex" => DisplayType::InlineFlex
    }};
    position: {{ Position
        = "static" => PositionType::Static
        | "relative" => PositionType::Relative
        | "absolute" => PositionType::Absolute
        | "fixed" => PositionType::Fixed
        | "sticky" => PositionType::Sticky
    }};
    float: {{ Float
        = "none" => FloatType::None
        | "left" => FloatType::Left
        | "right" => FloatType::Right
        | "inline-start" => FloatType::InlineStart
        | "inline-end" => FloatType::InlineEnd
    }};
    overflow_x: {{ OverflowX
        = "visible" => OverflowType::Visible
        | "hidden" => OverflowType::Hidden
        | "auto" => OverflowType::Auto
        | "scroll" => OverflowType::Scroll
    }};
    overflow_y: {{ OverflowY
        = "visible" => OverflowType::Visible
        | "hidden" => OverflowType::Hidden
        | "auto" => OverflowType::Auto
        | "scroll" => OverflowType::Scroll
    }};
    overflow: {{ (OverflowX, OverflowY)
        = [
            "visible" => OverflowType::Visible
            | "hidden" => OverflowType::Hidden
            | "auto" => OverflowType::Auto
            | "scroll" => OverflowType::Scroll
        ]{1, 2} -> split_hv
    }};
    overflow_wrap: {{ OverflowWrap
        = "normal" => OverflowWrapType::Normal
        | "break-word" => OverflowWrapType::BreakWord
    }};
    pointer_events: {{ PointerEvents
        = "auto" => PointerEventsType::Auto
        | "none" => PointerEventsType::None
    }};
    _wx_pointer_event_root: {{ WxPointerEventRoot
        = "auto" => PointerEventsType::Auto
        | "root" => PointerEventsType::WxRoot
    }};
    _wx_engine_touch_event: {{ WxEngineTouchEvent
        = "gesture" => WxEngineTouchEventType::Gesture
        | "click" => WxEngineTouchEventType::Click
        | "none" => WxEngineTouchEventType::None
    }};
    visibility: {{ Visibility
        = "visible" => VisibilityType::Visible
        | "hidden" => VisibilityType::Hidden
        | "collapse" => VisibilityType::Collapse
    }};
    flex_direction: {{ FlexDirection
        ="row" => FlexDirection::Row
        | "row-reverse" => FlexDirection::RowReverse
        | "column" => FlexDirection::Column
        | "column-reverse" => FlexDirection::ColumnReverse
    }};
    flex_wrap: {{ FlexWrap
        =  "nowrap" => FlexWrap::NoWrap
        | "wrap" => FlexWrap::Wrap
        | "wrap-reverse" => FlexWrap::WrapReverse
    }};
    align_items: {{ AlignItems
        = "stretch" => AlignItems::Stretch
        | "center" => AlignItems::Center
        | "flex-start" => AlignItems::FlexStart
        | "flex-end" => AlignItems::FlexEnd
        | "baseline" => AlignItems::Baseline
        | "normal" => AlignItems::Normal
        | "start" => AlignItems::Start
        | "end" => AlignItems::End
        | "self-start" => AlignItems::SelfStart
        | "self-end" => AlignItems::SelfEnd
    }};
    align_self: {{ AlignSelf
        = "auto" => AlignSelf::Auto
        | "stretch" => AlignSelf::Stretch
        | "center" => AlignSelf::Center
        | "flex-start" => AlignSelf::FlexStart
        | "flex-end" => AlignSelf::FlexEnd
        | "baseline" => AlignSelf::Baseline
        | "start" => AlignSelf::Start
        | "end" => AlignSelf::End
        | "self-start" => AlignSelf::SelfStart
        | "self-end" => AlignSelf::SelfEnd
        | "normal" => AlignSelf::Normal
    }};
    align_content: {{ AlignContent
        = "stretch" => AlignContent::Stretch
        | "center" => AlignContent::Center
        | "flex-start" => AlignContent::FlexStart
        | "flex-end" => AlignContent::FlexEnd
        | "space-between" => AlignContent::SpaceBetween
        | "space-around" => AlignContent::SpaceAround
        | "normal" => AlignContent::Normal
        | "start" => AlignContent::Start
        | "end" => AlignContent::End
        | "space-evenly" => AlignContent::SpaceEvenly
        | "baseline" => AlignContent::Baseline
    }};

    justify_content: {{ JustifyContent
        = "center" => JustifyContent::Center
        | "flex-start" => JustifyContent::FlexStart
        | "flex-end" => JustifyContent::FlexEnd
        | "space-between" => JustifyContent::SpaceBetween
        | "space-around" => JustifyContent::SpaceAround
        | "space-evenly" => JustifyContent::SpaceEvenly
        | "start" => JustifyContent::Start
        | "end" => JustifyContent::End
        | "left" => JustifyContent::Left
        | "right" => JustifyContent::Right
        | "baseline" => JustifyContent::Baseline
        | "stretch" => JustifyContent::Stretch
    }};
    justify_items: {{JustifyItems
        = "stretch" => JustifyItems::Stretch
        | "center" => JustifyItems::Center
        | "flex-start" => JustifyItems::FlexStart
        | "flex-end" => JustifyItems::FlexEnd
        | "start" => JustifyItems::Start
        | "end" => JustifyItems::End
        | "self-start" => JustifyItems::SelfStart
        | "self-end" => JustifyItems::SelfEnd
        | "left" => JustifyItems::Left
        | "right" => JustifyItems::Right
    }};
    order: {{ Order = <number> -> |x: Number| Number::I32(x.to_i32()); }};
    <gap_repr: Gap>:
        "normal" => Gap::Normal
        | <non_negative_length_percentage> -> |length| Gap::Length(length);
    ;
    column_gap: {{ ColumnGap = <gap_repr> }};
    row_gap: {{ RowGap = <gap_repr> }};
    gap: {{ (RowGap, ColumnGap)
        = [ <gap_repr> <gap_repr>? ] -> |(row_gap, column_gap): (Gap, Option<Gap>)| {
            if let Some(column_gap) = column_gap {
                return (row_gap, column_gap);
            }
            (row_gap.clone(), row_gap)
        };
    }};
    flex_grow: {{ FlexGrow = <number> }};
    flex_shrink: {{ FlexShrink = <number> }};
    flex_basis: {{ FlexBasis = <length> }};
    flex_flow: <flex_direction> || <flex_wrap>;
    flex: {{ (FlexGrow, FlexShrink, FlexBasis)
        = "auto" -> |_| (Number::F32(1.), Number::F32(1.), Length::Auto);
        | "none" -> |_| (Number::F32(0.), Number::F32(0.), Length::Auto);
        | [ <number> <number>? || <length> ] -> |(gs, b): (Option<(Number, Option<Number>)>, Option<Length>)| -> _ {
            let (g, s) = gs.unwrap_or((Number::F32(0.), None));
            let s = s.unwrap_or(Number::F32(1.));
            let b = b.unwrap_or(Length::Ratio(0.));
            (g, s, b)
        };
    }};

    direction: {{ Direction
        = "ltr" => Direction::LTR
        | "rtl" => Direction::RTL
    }};
    writing_mode: {{ WritingMode
        = "horizontal-tb" => WritingMode::HorizontalTb
        | "vertical-lr" => WritingMode::VerticalLr
        | "vertical-rl" => WritingMode::VerticalRl
    }};

    color: {{ Color = <color_repr> }};
    caret_color: {{ CaretColor = <color_repr> }};
    opacity: {{ Opacity = <number> }};
    z_index: {{ ZIndex
        =  "auto" => ZIndexType::Auto
        | <number> -> |x: Number| ZIndexType::Num(Number::I32(x.to_i32()));
    }};
    _wx_partial_z_index: {{ WxPartialZIndex = <number> }};
    font_size: {{ FontSize = <non_negative_length_percentage> }};
    <line_height_repr: LineHeightType>:
        "normal" => LineHeightType::Normal
        | <non_negative_number> -> |x: Number| LineHeightType::Num(x);
        | <non_negative_length_percentage> -> |x: Length| LineHeightType::Length(x);
    ;
    line_height: {{ LineHeight = <line_height_repr> }};
    text_align: {{ TextAlign
        = "left" => TextAlignType::Left
        | "center" => TextAlignType::Center
        | "right" => TextAlignType::Right
        | "justify" => TextAlignType::Justify
        | "justify-all" => TextAlignType::JustifyAll
        | "start" => TextAlignType::Start
        | "end" => TextAlignType::End
        | "match-parent" => TextAlignType::MatchParent
    }};
    <font_weight_repr: FontWeightType>:
        "normal" => FontWeightType::Normal
        | "bold" => FontWeightType::Bold
        | "bolder" => FontWeightType::Bolder
        | "lighter" => FontWeightType::Lighter
        | <number> -> |x: Number| FontWeightType::Num(x);
    ;
    font_weight: {{ FontWeight = <font_weight_repr> }};
    word_break: {{ WordBreak
        = "normal" => WordBreakType::BreakWord
        | "break-word" => WordBreakType::BreakWord
        | "break-all" => WordBreakType::BreakAll
        | "keep-all" => WordBreakType::KeepAll
    }};
    white_space: {{ WhiteSpace
        = "normal" => WhiteSpaceType::Normal
        | "nowrap" => WhiteSpaceType::NoWrap
        | "pre" => WhiteSpaceType::Pre
        | "pre-wrap" => WhiteSpaceType::PreWrap
        | "pre-line" => WhiteSpaceType::PreLine
        | "-wx-pre-edit" => WhiteSpaceType::WxPreEdit
    }};
    text_overflow: {{ TextOverflow
        = "clip" => TextOverflowType::Clip
        | "ellipsis" => TextOverflowType::Ellipsis
    }};
    text_indent: {{ TextIndent = <length_percentage> }};
    vertical_align: {{ VerticalAlign
        = "baseline" => VerticalAlignType::Baseline
        | "top" => VerticalAlignType::Top
        | "middle" => VerticalAlignType::Middle
        | "bottom" => VerticalAlignType::Bottom
        | "text-top" => VerticalAlignType::TextTop
        | "text-bottom" => VerticalAlignType::TextBottom
    }};
    letter_spacing: {{ LetterSpacing =
        "normal" => LetterSpacingType::Normal
        | <length_only> -> |x: Length| LetterSpacingType::Length(x);
    }};
    word_spacing: {{ WordSpacing =
        "normal" => WordSpacingType::Normal
        | <length_only> -> |x: Length| WordSpacingType::Length(x);
    }};
    font_family: {{ FontFamily = <font::font_family_repr> }};
    <font_style_repr: FontStyleType>:
        "normal" -> |_| { FontStyleType::Normal };
        | "italic" -> |_| { FontStyleType::Italic };
        | ["oblique" <angle>?] -> |x: ((), Option<Angle>)| {
            let mut angle = Angle::Deg(14.);
            if let Some(_angle) = x.1 {
                angle = _angle
            }
            FontStyleType::Oblique(angle)
        };
    ;
    font_style: {{ FontStyle = <font_style_repr>}};

    <background_repeat_single: BackgroundRepeatItem>:
        "repeat-x" -> |_| {
          BackgroundRepeatItem::Pos(BackgroundRepeatValue::Repeat, BackgroundRepeatValue::NoRepeat)
        };
        | "repeat-y" -> |_| {
          BackgroundRepeatItem::Pos(BackgroundRepeatValue::NoRepeat, BackgroundRepeatValue::Repeat)
        };
        | [
            "no-repeat" => BackgroundRepeatValue::NoRepeat
            | "repeat" => BackgroundRepeatValue::Repeat
            | "space" => BackgroundRepeatValue::Space
            | "round" => BackgroundRepeatValue::Round
        ]{1, 2} -> split_hv -> |(a, b): (_, _)| {
            BackgroundRepeatItem::Pos(a, b)
        };
    ;
    <background_position_single: BackgroundPositionItem>:
        <background::bg_pos_four_value>
        | <background::bg_pos_three_value>
        |<background::bg_pos_two_value>
        | <background::bg_pos_single_value>
    ;
    <background_position_single_without_extra_check: BackgroundPositionItem>:
        <background::bg_pos_four_value>
        | <background::bg_pos_three_value_without_extra_check>
        |<background::bg_pos_two_value_without_extra_check>
        | <background::bg_pos_single_value_without_extra_check>
    ;

    <background_size_single: BackgroundSizeItem>:
        "cover" => BackgroundSizeItem::Cover
        | "contain" => BackgroundSizeItem::Contain
        | [ <length> ]{1, 2} -> |v: Vec<_>| {
            let len = v.len();
            let mut v = v.into_iter();
            let a = v.next().unwrap_or(Length::Auto);
            let b = v.next().unwrap_or_else(|| a.clone());
            if len == 1 {
                BackgroundSizeItem::Length(a, Length::Auto)
            } else {
                BackgroundSizeItem::Length(a, b)
            }
        };
    ;
    <background_clip_single: BackgroundClipItem>:
        "border-box" => BackgroundClipItem::BorderBox
        | "padding-box" => BackgroundClipItem::PaddingBox
        | "content-box" => BackgroundClipItem::ContentBox
        | "text" => BackgroundClipItem::Text
    ;
    <background_attachment_single: BackgroundAttachmentItem>:
        "scroll" => BackgroundAttachmentItem::Scroll
        | "fixed" => BackgroundAttachmentItem::Fixed
        | "local" => BackgroundAttachmentItem::Local
    ;
    <background_origin_single: BackgroundOriginItem>:
        "border-box" => BackgroundOriginItem::BorderBox
        | "padding-box" => BackgroundOriginItem::PaddingBox
        | "content-box" => BackgroundOriginItem::ContentBox
    ;
    <image_single: BackgroundImageItem>:
        "none" => BackgroundImageItem::None
        | <image_func_repr>
        | <url_str> -> |x: String| BackgroundImageItem::Url(x.into());
        | <gradient::gradient_repr>
        | <element_func_repr>
    ;
    font: {{ (FontSize, FontFamily, FontStyle, FontWeight, LineHeight)
        = [
            [ <font_style_repr> || <font_weight_repr> ]? <non_negative_length_percentage> [ '/' <line_height_repr> ]? <font::font_family_repr>
        ] -> |x: (Option<(Option<FontStyleType>, Option<FontWeightType>)>, Length, Option<((), LineHeightType)>, FontFamilyType)| {
            let mut font_style = FontStyleType::Normal;
            let mut font_weight = FontWeightType::Normal;
            let mut line_height = LineHeightType::Normal;
            if let Some((style, weight)) = x.0 {
                if let Some(style) = style {
                    font_style = style;
                }
                if let Some(weight) = weight {
                    font_weight = weight;
                }
            }
            let font_size = x.1;
            if let Some(((), lh)) = x.2 {
                line_height = lh;
            }
            let font_family = x.3;
            (font_size, font_family, font_style, font_weight, line_height)
        };
    }};
    background: {{ (BackgroundColor, BackgroundImage, BackgroundRepeat, BackgroundPosition, BackgroundPositionX, BackgroundPositionY, BackgroundSize, BackgroundAttachment, BackgroundOrigin, BackgroundClip)
        ="none" -> |_| (
            Color::Specified(0, 0, 0, 0),
            BackgroundImageType::List(vec![BackgroundImageItem::None].into()),
            BackgroundRepeatType::List(vec![BackgroundRepeatItem::Pos(BackgroundRepeatValue::Repeat, BackgroundRepeatValue::Repeat)].into()),
            BackgroundPositionType::List(vec![BackgroundPositionItem::Pos(BackgroundPositionValue::Left(Length::Ratio(0.)), BackgroundPositionValue::Top(Length::Ratio(0.)))].into()),
            BackgroundPositionType::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.)))].into()),
            BackgroundPositionType::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.)))].into()),
            BackgroundSizeType::List(vec![BackgroundSizeItem::Auto].into()),
            BackgroundAttachmentType::List(vec![BackgroundAttachmentItem::Scroll].into()),
            BackgroundOriginType::List(vec![BackgroundOriginItem::PaddingBox].into()),
            BackgroundClipType::List(vec![BackgroundClipItem::BorderBox].into()),
        );
        | [
            <color_repr>
            || <image_single>
            || <background_repeat_single>
            || [<background_position_single_without_extra_check> [ '/' <background_size_single>]?]
            || <background_attachment_single>
            || <background_origin_single>
            || <background_clip_single>
        ]# -> ResultClosure |x: Vec<(
                Option<Color>,
                Option<_>,
                Option<_>,
                Option<(_, Option<(_, _)>)>,
                Option<BackgroundAttachmentItem>,
                Option<BackgroundOriginItem>,
                Option<BackgroundClipItem>
            )>, parser: &mut Parser<'i, 't> | -> Result<(_, BackgroundImageType, BackgroundRepeatType, BackgroundPositionType, BackgroundPositionType, BackgroundPositionType, BackgroundSizeType, BackgroundAttachmentType, BackgroundOriginType, BackgroundClipType), ParseError<'i, CustomError>> {
                let mut img = Vec::with_capacity(x.len());
                let mut rep = Vec::with_capacity(x.len());
                let mut pos = Vec::with_capacity(x.len());
                let mut pos_x = Vec::with_capacity(x.len());
                let mut pos_y = Vec::with_capacity(x.len());
                let mut size = Vec::with_capacity(x.len());
                let mut attach = Vec::with_capacity(x.len());
                let mut origin = Vec::with_capacity(x.len());
                let mut clip = Vec::with_capacity(x.len());
                let mut color = Color::Undefined;
                let len = x.len();
                for (index, v) in x.into_iter().enumerate() {
                    if let Some(_color) = v.0 {
                        if index < len - 1 { Err(parser.new_custom_error::<_, CustomError>(CustomError::Unmatched))?; }
                        color = _color;
                    }
                    match v.1 {
                        Some(x) => {
                            img.push(x);

                        }
                        None => {
                            img.push(BackgroundImageItem::None);
                        }
                    }
                    match v.2 {
                        Some(x) => {
                            rep.push(x);
                        }
                        None => {
                            rep.push(BackgroundRepeatItem::Pos(
                                BackgroundRepeatValue::Repeat,
                                BackgroundRepeatValue::Repeat
                            ));
                        }
                    }
                    match v.3 {
                        Some(pos_size) => {
                            let (__pos, __size) = pos_size;
                            {
                                if let BackgroundPositionItem::Pos(x, y) = &__pos {
                                    pos_x.push(BackgroundPositionItem::Value(x.clone()));
                                    pos_y.push(BackgroundPositionItem::Value(y.clone()));
                                }
                            }
                            pos.push(__pos);
                            match __size {
                            Some(s) => {
                                size.push(s.1);
                            },
                            None => {
                                size.push(BackgroundSizeItem::Auto);
                            }
                        }
                    }
                        None=> {
                            pos.push(
                                BackgroundPositionItem::Pos(
                                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                                    BackgroundPositionValue::Top(Length::Ratio(0.))
                                )
                            );
                            pos_x.push(BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.))));
                            pos_y.push(BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.))));
                            size.push(BackgroundSizeItem::Auto);
                        }
                    }
                    match v.4 {
                        Some(__attach) => {
                            attach.push(__attach);
                        },
                        None => {
                            attach.push(BackgroundAttachmentItem::Scroll);
                        }
                    }
                    if v.5.is_some() && v.6.is_some() {
                        if let Some(__origin) = v.5 {
                            origin.push(__origin);
                        }
                        if let Some(__clip) = v.6 {
                            clip.push(__clip);
                        }
                    } else if v.5.is_some() || v.6.is_some() {
                        if let Some(__origin) = v.5 {
                            origin.push(__origin.clone());
                            match __origin {
                                BackgroundOriginItem::PaddingBox => {
                                    clip.push(BackgroundClipItem::PaddingBox);
                                },
                                BackgroundOriginItem::BorderBox => {
                                    clip.push(BackgroundClipItem::BorderBox);
                                },
                                BackgroundOriginItem::ContentBox => {
                                    clip.push(BackgroundClipItem::ContentBox);
                                },
                            };
                        }
                        if let Some(__clip) = v.6 {
                            clip.push(__clip.clone());
                            match __clip {
                                BackgroundClipItem::PaddingBox => {
                                    origin.push(BackgroundOriginItem::PaddingBox);
                                },
                                BackgroundClipItem::BorderBox => {
                                    origin.push(BackgroundOriginItem::BorderBox);
                                },
                                BackgroundClipItem::ContentBox => {
                                    origin.push(BackgroundOriginItem::ContentBox);
                                },
                                _ => {},
                            };
                        }
                    } else {
                        origin.push(BackgroundOriginItem::PaddingBox);
                        clip.push(BackgroundClipItem::BorderBox);
                    }
                }
                Ok((
                    color,
                    BackgroundImageType::List(img.into()),
                    BackgroundRepeatType::List(rep.into()),
                    BackgroundPositionType::List(pos.into()),
                    BackgroundPositionType::List(pos_x.into()),
                    BackgroundPositionType::List(pos_y.into()),
                    BackgroundSizeType::List(size.into()),
                    BackgroundAttachmentType::List(attach.into()),
                    BackgroundOriginType::List(origin.into()),
                    BackgroundClipType::List(clip.into()),
                ))
        };
    }};
    background_color: {{ BackgroundColor = <color_repr> }};
    background_image: {{ BackgroundImage
        = [<image_single>]# -> |x: Vec<BackgroundImageItem>| BackgroundImageType::List(x.into());
    }};
    background_repeat: {{ BackgroundRepeat
        = [<background_repeat_single>]# -> |x: Vec<BackgroundRepeatItem>| BackgroundRepeatType::List(x.into());
    }};
    background_size: {{ BackgroundSize
        = [<background_size_single>]# -> |x: Vec<BackgroundSizeItem>| BackgroundSizeType::List(x.into());
    }};
    background_attachment: {{ BackgroundAttachment
        = [<background_attachment_single>]# -> |x: Vec<BackgroundAttachmentItem>| BackgroundAttachmentType::List(x.into());
    }};
    background_position: {{ (BackgroundPosition, BackgroundPositionX, BackgroundPositionY)
        = [<background_position_single>]# -> |arr: Vec<_>| {
            let mut x = vec![];
            let mut y = vec![];
            arr.iter().for_each(|item| {
                if let BackgroundPositionItem::Pos(_x, _y) = item {
                    x.push(BackgroundPositionItem::Value(_x.clone()));
                    y.push(BackgroundPositionItem::Value(_y.clone()));
                }
            });

            (BackgroundPositionType::List(arr.into()), BackgroundPositionType::List(x.into()), BackgroundPositionType::List(y.into()))
        };
    }};
    background_position_x: {{ BackgroundPositionX = <background::background_position_x_value> }};
    background_position_y: {{ BackgroundPositionY = <background::background_position_y_value> }};

    background_clip: {{ BackgroundClip
        = [<background_clip_single>]# -> |x: Vec<_>| {
            BackgroundClipType::List(x.into())
        };
    }};
    background_origin: {{ BackgroundOrigin
        = [<background_origin_single>]# -> |x: Vec<_>| {
            BackgroundOriginType::List(x.into())
        };
    }};

    box_sizing: {{ BoxSizing
        = "border-box" => BoxSizingType::BorderBox
        | "padding-box" => BoxSizingType::PaddingBox
        | "content-box" => BoxSizingType::ContentBox
    }};
    width: {{ Width = <length> }};
    height: {{ Height = <length> }};
    min_width: {{ MinWidth = <length> }};
    min_height: {{ MinHeight = <length> }};
    max_width: {{ MaxWidth = <length> }};
    max_height: {{ MaxHeight = <length> }};
    left: {{ Left = <length> }};
    right: {{ Right = <length> }};
    top: {{ Top = <length> }};
    bottom: {{ Bottom = <length> }};

    padding_left: {{ PaddingLeft = <length_percentage> }};
    padding_right: {{ PaddingRight = <length_percentage> }};
    padding_top: {{ PaddingTop = <length_percentage> }};
    padding_bottom: {{ PaddingBottom = <length_percentage> }};
    padding: {{ (PaddingTop, PaddingRight, PaddingBottom, PaddingLeft)
        = <length_percentage>{1, 4} -> split_edges
    }};

    margin_left: {{ MarginLeft = <length> }};
    margin_right: {{ MarginRight = <length> }};
    margin_top: {{ MarginTop = <length> }};
    margin_bottom: {{ MarginBottom = <length> }};
    margin:{{ (MarginTop, MarginRight, MarginBottom, MarginLeft)
        = <length>{1, 4} -> split_edges
    }};

    <border_style_repr: BorderStyle>:
        "none" => BorderStyle::None
        | "solid" => BorderStyle::Solid
        | "dotted" => BorderStyle::Dotted
        | "dashed" => BorderStyle::Dashed
        | "hidden" => BorderStyle::Hidden
        | "double" => BorderStyle::Double
        | "groove" => BorderStyle::Groove
        | "ridge" => BorderStyle::Ridge
        | "inset" => BorderStyle::Inset
        | "outset" => BorderStyle::Outset
    ;
    border_left_width: {{ BorderLeftWidth = <line_width> }};
    border_left_style: {{ BorderLeftStyle = <border_style_repr> }};
    border_left_color: {{ BorderLeftColor = <color_repr> }};
    border_left: <border_left_width> || <border_left_style> || <border_left_color>;
    border_right_width: {{ BorderRightWidth = <line_width> }};
    border_right_style: {{ BorderRightStyle = <border_style_repr> }};
    border_right_color: {{ BorderRightColor = <color_repr> }};
    border_right: <border_right_width> || <border_right_style> || <border_right_color>;
    border_top_width: {{ BorderTopWidth = <line_width> }};
    border_top_style: {{ BorderTopStyle = <border_style_repr> }};
    border_top_color: {{ BorderTopColor = <color_repr> }};
    border_top: <border_top_width> || <border_top_style> || <border_top_color>;
    border_bottom_width: {{ BorderBottomWidth = <line_width> }};
    border_bottom_style: {{ BorderBottomStyle = <border_style_repr> }};
    border_bottom_color: {{ BorderBottomColor = <color_repr> }};
    border_bottom: <border_bottom_width> || <border_bottom_style> || <border_bottom_color>;
    border_width: {{ (BorderTopWidth, BorderRightWidth, BorderBottomWidth, BorderLeftWidth)
        = <line_width>{1, 4} -> split_edges
    }};
    border_style: {{ (BorderTopStyle, BorderRightStyle, BorderBottomStyle, BorderLeftStyle)
        = <border_style_repr>{1, 4} -> split_edges
    }};
    border_color: {{ (BorderTopColor, BorderRightColor, BorderBottomColor, BorderLeftColor)
        = <color_repr>{1, 4} -> split_edges
    }};
    border: {{
      (
          BorderTopWidth,
          BorderRightWidth,
          BorderBottomWidth,
          BorderLeftWidth,
          BorderTopStyle,
          BorderRightStyle,
          BorderBottomStyle,
          BorderLeftStyle,
          BorderTopColor,
          BorderRightColor,
          BorderBottomColor,
          BorderLeftColor,
      ) = [
            <line_width> || <border_style_repr> || <color_repr>
          ] -> |x: (Option<Length>, Option<BorderStyle>, Option<Color>)| {
              let width = x.0;
              let style = x.1;
              let color = x.2;
              let mut w = LengthType::Initial;
              let mut s = BorderStyle::None;
              let mut c = ColorType::Initial;
              // for border: 'none'
              if let Some(style) = style {
                s = style;
              }
              if let Some(width) = width {
                  w = width.into();
              }
              if let Some(color) = color {
                  c = color.into();
              }
              (
                  w.clone(), w.clone(), w.clone(), w,
                  s.clone(), s.clone(), s.clone(), s,
                  c.clone(), c.clone(), c.clone(), c,
              )
          };
    }};
    border_top_left_radius: {{ BorderTopLeftRadius =
        <length_percentage>{1, 2} -> split_hv -> |(a, b)| {
            BorderRadius::Pos(a, b)
        };
    }};
    border_top_right_radius: {{ BorderTopRightRadius =
        <length_percentage>{1, 2} -> split_hv -> |(a, b)| {
            BorderRadius::Pos(a, b)
        };
    }};
    border_bottom_right_radius: {{ BorderBottomRightRadius =
        <length_percentage>{1, 2} -> split_hv -> |(a, b)| {
            BorderRadius::Pos(a, b)
        };
    }};
    border_bottom_left_radius: {{ BorderBottomLeftRadius =
        <length_percentage>{1, 2} -> split_hv -> |(a, b)| {
            BorderRadius::Pos(a, b)
        };
    }};
    border_radius:{{ (BorderTopLeftRadius, BorderTopRightRadius, BorderBottomRightRadius, BorderBottomLeftRadius)
        = [<length_percentage>{1, 4} ['/' <length_percentage>{1, 4}]?] -> |(a, b): (Vec<_>, Option<(_, Vec<_>)>)| {
            let horizontal = split_edges(a);
            let mut vertical =horizontal.clone();
            if let Some(v) = b {
                vertical = split_edges(v.1);
            }
            (
                BorderRadius::Pos(horizontal.0, vertical.0),
                BorderRadius::Pos(horizontal.1, vertical.1),
                BorderRadius::Pos(horizontal.2, vertical.2),
                BorderRadius::Pos(horizontal.3, vertical.3)
            )
        };
    }};
    box_shadow: {{ BoxShadow =
        "none" => BoxShadow::None
        | [
            "inset"? && <length_only>{2, 4} && <color_repr>?
        ]# -> ResultClosure |x: Vec<(Option<Option<_>>, Option<Vec<Length>>, Option<Option<Color>>)>, parser: &mut Parser<'i, 't> | -> Result<BoxShadow, ParseError<'i, CustomError>> {
            let mut ret = Vec::with_capacity(x.len());
            let mut error = false;
            x.into_iter().for_each(|item| {
                let mut r = vec![];
                let inset = item.0.unwrap();
                if inset.is_some() {
                    r.push(ShadowItemType::Inset)
                }
                if let Some(len) = item.1 {
                    let offset_x = len.get(0).unwrap();
                    let offset_y = len.get(1).unwrap();
                    r.push(ShadowItemType::OffsetX(offset_x.clone()));
                    r.push(ShadowItemType::OffsetY(offset_y.clone()));
                    let blur_radius = len.get(2);

                    let spread_radius = len.get(3);
                    if let Some(br) = blur_radius {
                        if !is_non_negative_length(br) {
                            error = true;
                        }
                        r.push(ShadowItemType::BlurRadius(br.clone()));
                    } else {
                        r.push(ShadowItemType::BlurRadius(Length::Px(0.)));
                    }
                    if let Some(sr) = spread_radius {
                        r.push(ShadowItemType::SpreadRadius(sr.clone()));
                    } else {
                        r.push(ShadowItemType::SpreadRadius(Length::Px(0.)));

                    }
                }
                let color = item.2.unwrap();
                if let Some(color) = color {
                    r.push(ShadowItemType::Color(color));
                } else {
                    r.push(ShadowItemType::Color(Color::CurrentColor));
                }
                ret.push(BoxShadowItem::List(r.into()));
            });
            if error {
                Err(parser.new_custom_error::<_, CustomError>(CustomError::Unsupported))?;
            }
            Ok(BoxShadow::List(ret.into()))
        };
    }};
    backdrop_filter: {{ BackdropFilter =
        "none" => BackdropFilter::None
        | <filter::filter_repr> -> |x: Vec<_>| { BackdropFilter::List(x.into()) };
    }};
    filter: {{ Filter =
        "none" => Filter::None
        | <filter::filter_repr> -> |x: Vec<_>| { Filter::List(x.into()) };
    }};
    transform: {{ Transform =
        "none" -> |_| { Transform::Series(Array::empty()) };
        | <transform_repr>
    }};
    transform_origin: {{ TransformOrigin =
        [
            [
                "center" => TransformOrigin::Center
                | "left" => TransformOrigin::Left
                | "right" => TransformOrigin::Right
                | <length_percentage> -> |x: Length| { TransformOrigin::Length(x) };
            ] && [
                "top" => TransformOrigin::Top
                | "center" => TransformOrigin::Center
                | "bottom" => TransformOrigin::Bottom
                | <length_percentage> -> |y: Length| { TransformOrigin::Length(y) };
            ] && [<length_only>?]
        ] -> |item: (Option<TransformOrigin>, Option<TransformOrigin>, Option<Option<_>>)| {
            let x = item.0;
            let y = item.1;
            let z = item.2;
            let x = match x.unwrap() {
                TransformOrigin::Center => Length::Ratio(0.5),
                TransformOrigin::Left => Length::Ratio(0.),
                TransformOrigin::Right => Length::Ratio(1.0),
                TransformOrigin::Length(x) => x,
                _ => Length::Ratio(0.5),
            };
            let y = match y.unwrap() {
                TransformOrigin::Center => Length::Ratio(0.5),
                TransformOrigin::Top => Length::Ratio(0.),
                TransformOrigin::Bottom => Length::Ratio(1.0),
                TransformOrigin::Length(y) => y,
                _ => Length::Ratio(0.5),
            };
            let z = match z.unwrap() {
                Some(z) => z,
                None => Length::Px(0.)
            };
            TransformOrigin::LengthTuple(x, y, z)
        };
        | [
            "left" -> |_| { TransformOrigin::LengthTuple(Length::Ratio(0.), Length::Ratio(0.5), Length::Px(0.)) };
            | "center" -> |_| { TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.5), Length::Px(0.)) };
            | "right" -> |_| { TransformOrigin::LengthTuple(Length::Ratio(1.), Length::Ratio(0.5), Length::Px(0.)) };
            | "top" -> |_| { TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(0.), Length::Px(0.)) };
            | "bottom" -> |_| { TransformOrigin::LengthTuple(Length::Ratio(0.5), Length::Ratio(1.), Length::Px(0.)) };
            | <length_percentage> -> |x: Length| { TransformOrigin::LengthTuple(x, Length::Ratio(0.5), Length::Px(0.)) };
        ]
    }};
    <transition_property_single: TransitionPropertyItem>:
        "none" => TransitionPropertyItem::None
        | "transform" => TransitionPropertyItem::Transform
        | "transform-origin" => TransitionPropertyItem::TransformOrigin
        | "line-height" => TransitionPropertyItem::LineHeight
        | "opacity" => TransitionPropertyItem::Opacity
        | "all" => TransitionPropertyItem::All
        | "height" => TransitionPropertyItem::Height
        | "width" => TransitionPropertyItem::Width
        | "min-height" => TransitionPropertyItem::MinHeight
        | "max-height" => TransitionPropertyItem::MaxHeight
        | "min-width" => TransitionPropertyItem::MinWidth
        | "max-width" => TransitionPropertyItem::MaxWidth
        | "margin-top" => TransitionPropertyItem::MarginTop
        | "margin-right" => TransitionPropertyItem::MarginRight
        | "margin-bottom" => TransitionPropertyItem::MarginBottom
        | "margin-left" => TransitionPropertyItem::MarginLeft
        | "margin" => TransitionPropertyItem::Margin
        | "padding-top" => TransitionPropertyItem::PaddingTop
        | "padding-right" => TransitionPropertyItem::PaddingRight
        | "padding-bottom" => TransitionPropertyItem::PaddingBottom
        | "padding-left" => TransitionPropertyItem::PaddingLeft
        | "padding" => TransitionPropertyItem::Padding
        | "top" => TransitionPropertyItem::Top
        | "right" => TransitionPropertyItem::Right
        | "bottom" => TransitionPropertyItem::Bottom
        | "left" => TransitionPropertyItem::Left
        | "flex-grow" => TransitionPropertyItem::FlexGrow
        | "flex-shrink" => TransitionPropertyItem::FlexShrink
        | "flex-basis" => TransitionPropertyItem::FlexBasis
        | "border-top-width" => TransitionPropertyItem::BorderTopWidth
        | "border-right-width" => TransitionPropertyItem::BorderRightWidth
        | "border-bottom-width" => TransitionPropertyItem::BorderBottomWidth
        | "border-left-width" => TransitionPropertyItem::BorderLeftWidth
        | "border-top-color" => TransitionPropertyItem::BorderTopColor
        | "border-right-color" => TransitionPropertyItem::BorderRightColor
        | "border-bottom-color" => TransitionPropertyItem::BorderBottomColor
        | "border-left-color" => TransitionPropertyItem::BorderLeftColor
        | "border-top-left-radius" => TransitionPropertyItem::BorderTopLeftRadius
        | "border-top-right-radius" => TransitionPropertyItem::BorderTopRightRadius
        | "border-bottom-left-radius" => TransitionPropertyItem::BorderBottomLeftRadius
        | "border-bottom-right-radius" => TransitionPropertyItem::BorderBottomRightRadius
        | "border" => TransitionPropertyItem::Border
        | "border-width" => TransitionPropertyItem::BorderWidth
        | "border-radius" => TransitionPropertyItem::BorderRadius
        | "border-color" => TransitionPropertyItem::BorderColor
        | "border-left" => TransitionPropertyItem::BorderLeft
        | "border-top" => TransitionPropertyItem::BorderTop
        | "border-right" => TransitionPropertyItem::BorderRight
        | "border-bottom" => TransitionPropertyItem::BorderBottom
        | "z-index" => TransitionPropertyItem::ZIndex
        | "filter" => TransitionPropertyItem::Filter
        | "backdrop-filter" => TransitionPropertyItem::BackdropFilter
        | "box-shadow" => TransitionPropertyItem::BoxShadow
        | "color" => TransitionPropertyItem::Color
        | "text-decoration-color" => TransitionPropertyItem::TextDecorationColor
        | "text-decoration-thickness" => TransitionPropertyItem::TextDecorationThickness
        | "font-size" => TransitionPropertyItem::FontSize
        | "font-weight" => TransitionPropertyItem::FontWeight
        | "letter-spacing" => TransitionPropertyItem::LetterSpacing
        | "word-spacing" => TransitionPropertyItem::WordSpacing
        | "background-color" => TransitionPropertyItem::BackgroundColor
        | "background-position" => TransitionPropertyItem::BackgroundPosition
        | "background-size" => TransitionPropertyItem::BackgroundSize
        | "background" => TransitionPropertyItem::Background
        | "flex" => TransitionPropertyItem::Flex
        | "background-position-x" => TransitionPropertyItem::BackgroundPositionX
        | "background-position-y" => TransitionPropertyItem::BackgroundPositionY
        | "mask-size" => TransitionPropertyItem::MaskSize
        | "mask-position-x" => TransitionPropertyItem::MaskPositionX
        | "mask-position-y" => TransitionPropertyItem::MaskPositionY
        | "mask-position" => TransitionPropertyItem::MaskPosition
        | "mask" => TransitionPropertyItem::Mask;
    transition_property: {{ TransitionProperty
        = <transition_property_single># -> |x: Vec<_>| TransitionPropertyType::List(x.into());
    }};
    transition_duration: {{ TransitionDuration
        = <time_u32_ms># -> |x: Vec<_>| TransitionTimeType::List(x.into());
    }};
    <step_position_repr: StepPosition>:
        "end" => StepPosition::End
        | "start" => StepPosition::Start
        | "jump-start" => StepPosition::JumpStart
        | "jump-end" => StepPosition::JumpEnd
        | "jump-none" => StepPosition::JumpNone;
    <timing_function_single: TransitionTimingFnItem>:
        "linear" => TransitionTimingFnItem::Linear
        | "ease" => TransitionTimingFnItem::Ease
        | "ease-in" => TransitionTimingFnItem::EaseIn
        | "ease-out" => TransitionTimingFnItem::EaseOut
        | "ease-in-out" => TransitionTimingFnItem::EaseInOut
        | "linear" => TransitionTimingFnItem::Linear
        | "step-start" => TransitionTimingFnItem::StepStart
        | "step-end" => TransitionTimingFnItem::StepEnd
        | steps(<float_repr> [',' <step_position_repr>]?) // TODO use number type
            -> |x: (f32, Option<(_, StepPosition)>)| {
                let a = x.0 as i32;
                let mut b = StepPosition::End;
                if let Some((_, step_position)) = x.1 {
                    b = step_position;
                }
                TransitionTimingFnItem::Steps(a, b)
            };
        | cubic_bezier(<float_repr> ',' <float_repr> ',' <float_repr> ',' <float_repr>) // TODO use number type
            -> |(a, _, b, _, c, _, d)| {
                TransitionTimingFnItem::CubicBezier(a, b, c, d)
            };
        ;
    transition_timing_function: {{ TransitionTimingFunction
        = <timing_function_single># -> |x: Vec<_>| TransitionTimingFnType::List(x.into());
    }};
    transition_delay: {{ TransitionDelay
        = <time_i32_ms># -> |x: Vec<_>| TransitionTimeType::ListI32(x.into());
    }};
    transition: {{ (TransitionProperty, TransitionDuration, TransitionTimingFunction, TransitionDelay)
        = [
            <transition_property_single>
            || <time_u32_ms>
            || <timing_function_single>
            || <time_i32_ms>
        ]# -> |x: Vec<(_, _, _, _)>| {
            let mut properties = Vec::with_capacity(x.len());
            let mut duration: Vec<u32> = Vec::with_capacity(x.len());
            let mut timing_fn: Vec<TransitionTimingFnItem> = Vec::with_capacity(x.len());
            let mut delay: Vec<i32> = Vec::with_capacity(x.len());
            for v in x {
                match v.0 {
                    Some(v) => properties.push(v),
                    None => properties.push(TransitionPropertyItem::All)
                }
                match v.1 {
                    Some(v) => duration.push(v),
                    None => duration.push(0)
                }
                match v.2 {
                    Some(v) => timing_fn.push(v),
                    None => timing_fn.push(TransitionTimingFnItem::Ease)
                }
                match v.3 {
                    Some(v) => delay.push(v),
                    None => delay.push(0)
                }
            }
            (
                TransitionPropertyType::List(properties.into()),
                TransitionTimeType::List(duration.into()),
                TransitionTimingFnType::List(timing_fn.into()),
                TransitionTimeType::ListI32(delay.into()),
            )
        };
    }};
    animation: {{ (AnimationDuration, AnimationTimingFunction, AnimationDelay, AnimationIterationCount, AnimationDirection, AnimationFillMode, AnimationPlayState, AnimationName)
        = [
            <time_u32_ms>
            || <timing_function_single>
            || <time_i32_ms>
            || <animation_iteration_count_single>
            || <animation_direction_single>
            || <animation_fill_mode_single>
            || <animation_play_state_single>
            || <animation_name_single>
        ]# -> |x: Vec<(_, _, _, _, _, _, _, _)>| {
            let len = x.len();
            let mut duration: Vec<u32> = Vec::with_capacity(len);
            let mut timing_fn: Vec<TransitionTimingFnItem> = Vec::with_capacity(len);
            let mut delay: Vec<i32> = Vec::with_capacity(len);
            let mut iteration_count: Vec<AnimationIterationCountItem> = Vec::with_capacity(len);
            let mut direction: Vec<AnimationDirectionItem> = Vec::with_capacity(len);
            let mut fill_mode: Vec<AnimationFillModeItem> = Vec::with_capacity(len);
            let mut play_state: Vec<AnimationPlayStateItem> = Vec::with_capacity(len);
            let mut name: Vec<AnimationNameItem> = Vec::with_capacity(len);
            for item in x {
                match item.0 {
                    Some(v) => duration.push(v),
                    None => duration.push(0)
                }
                match item.1 {
                    Some(v) => timing_fn.push(v),
                    None => timing_fn.push(TransitionTimingFnItem::Ease)
                }
                match item.2 {
                    Some(v) => delay.push(v),
                    None => delay.push(0)
                }
                match item.3 {
                    Some(v) => iteration_count.push(v),
                    None => iteration_count.push(AnimationIterationCountItem::Number(1.))
                }
                match item.4 {
                    Some(v) => direction.push(v),
                    None => direction.push(AnimationDirectionItem::Normal)
                }
                match item.5 {
                    Some(v) => fill_mode.push(v),
                    None => fill_mode.push(AnimationFillModeItem::None)
                }
                match item.6 {
                    Some(v) => play_state.push(v),
                    None => play_state.push(AnimationPlayStateItem::Running)
                }
                match item.7 {
                    Some(v) => name.push(v),
                    None => name.push(AnimationNameItem::None)
                }
            }
            (
                TransitionTimeType::List(duration.into()),
                TransitionTimingFnType::List(timing_fn.into()),
                TransitionTimeType::ListI32(delay.into()),
                AnimationIterationCountType::List(iteration_count.into()),
                AnimationDirectionType::List(direction.into()),
                AnimationFillModeType::List(fill_mode.into()),
                AnimationPlayStateType::List(play_state.into()),
                AnimationName::List(name.into())
            )
        };
    }};
    animation_duration: {{ AnimationDuration
        = <time_u32_ms># -> |x: Vec<_>| TransitionTimeType::List(x.into());
    }};
    animation_delay: {{ AnimationDelay
        = <time_i32_ms># -> |x: Vec<_>| TransitionTimeType::ListI32(x.into());
    }};
    animation_timing_function: {{ AnimationTimingFunction
        = <timing_function_single># -> |x: Vec<_>| TransitionTimingFnType::List(x.into());
    }};
    animation_iteration_count: {{ AnimationIterationCount
        = <animation_iteration_count_single># -> |x: Vec<_>| AnimationIterationCountType::List(x.into());
    }};
    <animation_iteration_count_single: AnimationIterationCountItem>:
        "infinite" => AnimationIterationCountItem::Infinite
        | <float_repr> -> |x: _| AnimationIterationCountItem::Number(x);
    ;
    animation_direction: {{ AnimationDirection
        = <animation_direction_single># -> |x: Vec<_>| AnimationDirectionType::List(x.into());
    }};
    <animation_direction_single: AnimationDirectionItem>:
        "normal" => AnimationDirectionItem::Normal
        | "reverse" => AnimationDirectionItem::Reverse
        | "alternate" => AnimationDirectionItem::Alternate
        | "alternate-reverse" => AnimationDirectionItem::AlternateReverse;
    animation_fill_mode: {{ AnimationFillMode
        = <animation_fill_mode_single># -> |x: Vec<_>| AnimationFillModeType::List(x.into());
    }};
    <animation_fill_mode_single: AnimationFillModeItem>:
        "none" => AnimationFillModeItem::None
        | "forwards" => AnimationFillModeItem::Forwards
        | "backwards" => AnimationFillModeItem::Backwards
        | "both" => AnimationFillModeItem::Both;
    animation_play_state: {{ AnimationPlayState
        = <animation_play_state_single># -> |x: Vec<_>| AnimationPlayStateType::List(x.into());
    }};
    <animation_play_state_single: AnimationPlayStateItem>:
        "running" => AnimationPlayStateItem::Running
        | "paused" => AnimationPlayStateItem::Paused;
    animation_name: {{ AnimationName
        = <animation_name_single># -> |x: Vec<_>| AnimationNameType::List(x.into());
    }};
    <animation_name_single: AnimationNameItem>:
        "none" => AnimationNameItem::None
        | <string> -> |custom_ident: String| AnimationNameItem::CustomIdent(custom_ident.into());
    ;
    will_change: {{ WillChange
        = "auto" => WillChange::Auto
       | <animateable_feature_single># -> |x: Vec<_>| WillChange::List(x.into());
    }};
    <animateable_feature_single: AnimateableFeature>:
        "contents" => AnimateableFeature::Contents
        | "scroll-position" => AnimateableFeature::ScrollPosition
        | <string> ->  |custom_ident: String| AnimateableFeature::CustomIdent(custom_ident.into());
    ;
    aspect_ratio: {{ AspectRatio
        = "auto" => AspectRatioType::Auto
        | [ <number> ['/' <number>]?] -> |r: (Number, Option<(_, Number)>)| {
            let width = r.0;
            let height = match r.1 {
                Some(h) => h.1,
                None => Number::F32(1.)
            };
            AspectRatioType::Ratio(width, height)
        };
    }};
    contain: {{ Contain
        = "none" => ContainType::None
        | "strict" => ContainType::Strict
        | "content" => ContainType::Content
        | ["size" || "layout" || "style" || "paint"] -> |x: (Option<()>, Option<()>, Option<()>, Option<()>)| {
            let mut v = vec![];
            if x.0.is_some() {
                v.push(ContainKeyword::Size);
            }
            if x.1.is_some() {
                v.push(ContainKeyword::Layout);
            }
            if x.2.is_some() {
                v.push(ContainKeyword::Style);
            }
            if x.3.is_some() {
                v.push(ContainKeyword::Paint);
            }
            ContainType::Multiple(v.into())
        };
    }};
    _wx_scrollbar_x: {{ WxScrollbarX
        = "hidden" => ScrollbarType::Hidden
        | "auto-hide" => ScrollbarType::AutoHide
        | "always-show" => ScrollbarType::AlwaysShow
    }};
    _wx_scrollbar_x_color: {{ WxScrollbarXColor = <color_repr> }};
    _wx_scrollbar_y: {{ WxScrollbarY
        = "hidden" => ScrollbarType::Hidden
        | "auto-hide" => ScrollbarType::AutoHide
        | "always-show" => ScrollbarType::AlwaysShow
    }};
    _wx_scrollbar_y_color: {{ WxScrollbarYColor = <color_repr> }};
    _wx_scrollbar_color: {{ (WxScrollbarXColor, WxScrollbarYColor)
        = <color_repr>{1, 2} -> split_hv
    }};
    _wx_line_clamp: {{ WxLineClamp = <number> }};
    _wx_contain: {{ WxContain
        = "none" => ContainType::None
        | "strict" => ContainType::Strict
        | "content" => ContainType::Content
        | ["size" || "layout" || "style" || "paint"] -> |x: (Option<()>, Option<()>, Option<()>, Option<()>)| {
            let mut v = vec![];
            if x.0.is_some() {
                v.push(ContainKeyword::Size);
            }
            if x.1.is_some() {
                v.push(ContainKeyword::Layout);
            }
            if x.2.is_some() {
                v.push(ContainKeyword::Style);
            }
            if x.3.is_some() {
                v.push(ContainKeyword::Paint);
            }
            ContainType::Multiple(v.into())
        };
    }};
    content: {{ Content
        = "none" => ContentType::None
        | "normal" => ContentType::Normal
        | <url_str> -> |x: String| ContentType::Url(x.into());
        | <string> -> |x: String| ContentType::Str(x.into());
    }};
    list_style_type: {{ ListStyleType
        = "none" => ListStyleType::None
        | "disc" => ListStyleType::Disc
        | "circle" => ListStyleType::Circle
        | "square" => ListStyleType::Square
        | "decimal" => ListStyleType::Decimal
        | "cjk-decimal" => ListStyleType::CjkDecimal
        | "decimal-leading-zero" => ListStyleType::DecimalLeadingZero
        | "lower-roman" => ListStyleType::LowerRoman
        | "upper-roman" => ListStyleType::UpperRoman
        | "lower-greek" => ListStyleType::LowerGreek
        | "lower-alpha" => ListStyleType::LowerAlpha
        | "lower-latin" => ListStyleType::LowerLatin
        | "upper-alpha" => ListStyleType::UpperAlpha
        | "upper-latin" => ListStyleType::UpperLatin
        | "armenian" => ListStyleType::Armenian
        | "georgian" => ListStyleType::Georgian
      // | <custom_ident_repr> -> |x: String| ListStyleType::CustomIdent(x.into());
    }};
    list_style_image: {{ ListStyleImage
        = "none" => ListStyleImage::None
        | <url_str> -> |x: String| ListStyleImage::Url(x.into());
    }};
    list_style_position: {{ ListStylePosition
        = "outside" => ListStylePosition::Outside
        | "inside" => ListStylePosition::Inside
    }};
    list_style: <list_style_type> || <list_style_position> || <list_style_image>;

    resize: {{ Resize
        = "none" => ResizeType::None
        | "both" => ResizeType::Both
        | "horizontal" => ResizeType::Horizontal
        | "vertical" => ResizeType::Vertical
        | "block" => ResizeType::Block
        | "inline" => ResizeType::Inline
    }};

    text_shadow: {{ TextShadow
        = "none" => TextShadowType::None
        | [
            [ <length_only>{2, 3} && <color_repr>? ]
        ]# -> |x: Vec<(Option<Vec<Length>>, Option<Option<Color>>)>| {
            let mut t = Vec::with_capacity(x.len());
            for item in x.into_iter() {
                let a = item.0.unwrap();
                let offset_x = a.get(0).unwrap().clone();
                let offset_y = a.get(1).unwrap().clone();
                let blur_radius = a.get(2);
                let blur_radius = match blur_radius {
                    Some(v) => v.clone(),
                    None => Length::Undefined
                };
                let b = item.1.unwrap();
                let color = match b {
                    Some(v) => v.clone(),
                    None => Color::Undefined
                };
                t.push(
                    TextShadowItem::TextShadowValue(
                        offset_x,
                        offset_y,
                        blur_radius,
                        color
                    )
                );
            }
            TextShadowType::List(t.into())
        };
    }};

    text_decoration_line: {{TextDecorationLine
        = "none" => TextDecorationLine::None
        | "spelling-error" => TextDecorationLine::SpellingError
        | "grammar-error" => TextDecorationLine::GrammarError
        | ["underline" || "overline" || "line-through" || "blink"] -> |x: (Option<()>, Option<()>, Option<()>, Option<()>)| {
            let mut v = vec![];
            if let Some(_underline) = x.0 {
                v.push(TextDecorationLineItem::Underline);
            }
            if let Some(_overline) = x.1 {
                v.push(TextDecorationLineItem::Overline);
            }
            if let Some(_line_through) = x.2 {
                v.push(TextDecorationLineItem::LineThrough);
            }
            if let Some(_blink) = x.3 {
                v.push(TextDecorationLineItem::Blink);
            }
            TextDecorationLine::List(v.into())
        };
    }};
    text_decoration_style: {{ TextDecorationStyle
        = "solid" => TextDecorationStyleType::Solid
        | "double" => TextDecorationStyleType::Double
        | "dotted" => TextDecorationStyleType::Dotted
        | "dashed" => TextDecorationStyleType::Dashed
        | "wavy" => TextDecorationStyleType::Wavy
    }};
    text_decoration_color: {{ TextDecorationColor = <color_repr> }};
    text_decoration_thickness: {{ TextDecorationThickness
        = "from-font" => TextDecorationThicknessType::FromFont
        | <length> -> |x: Length| { TextDecorationThicknessType::Length(x)};
    }};
    text_decoration: <text_decoration_style> || <text_decoration_color> || <text_decoration_thickness> || <text_decoration_line>;
    font_feature_settings: {{ FontFeatureSettings
        = "normal" => FontFeatureSettingsType::Normal
        | [<string> <font_feature_tag_value_repr>?]# -> |tags: Vec<(String, Option<Number>)>| {
            let ret: Vec<FeatureTag> = tags.into_iter().map(|item| {
                FeatureTag {
                    opentype_tag: item.0.into(),
                    value: item.1.unwrap_or_else(|| Number::F32(1.)),
                }
            }).collect();
            FontFeatureSettingsType::FeatureTags(ret.into())
        };
    }};
    <font_feature_tag_value_repr: Number>:
        "on" -> |_| {Number::F32(1.)};
        | "off" -> |_| {Number::F32(0.)};
        | <non_negative_number>
    ;
    mask_image: {{ MaskImage
        = [<image_single>]# -> |x: Vec<BackgroundImageItem>| BackgroundImageType::List(x.into());
    }};
    mask_size: {{ MaskSize
        = [<background_size_single>]# -> |x: Vec<BackgroundSizeItem>| BackgroundSizeType::List(x.into());
    }};
    mask_repeat: {{ MaskRepeat
      = [<background_repeat_single>]# -> |x: Vec<BackgroundRepeatItem>| BackgroundRepeatType::List(x.into());
    }};
    mask_origin: {{ MaskOrigin
        = [<background_origin_single>]# -> |x: Vec<_>| {
            BackgroundOriginType::List(x.into())
        };
    }};
    mask_clip: {{ MaskClip
        = [<background_clip_single>]# -> |x: Vec<_>| {
            BackgroundClipType::List(x.into())
        };
    }};
    mask_position: {{ (MaskPosition, MaskPositionX, MaskPositionY)
        = [<background_position_single>]# -> |arr: Vec<_>| {
            let mut x = vec![];
            let mut y = vec![];
            arr.iter().for_each(|item| {
                if let BackgroundPositionItem::Pos(_x, _y) = item {
                    x.push(BackgroundPositionItem::Value(_x.clone()));
                    y.push(BackgroundPositionItem::Value(_y.clone()));
                }
            });

            (BackgroundPositionType::List(arr.into()), BackgroundPositionType::List(x.into()), BackgroundPositionType::List(y.into()))
        };
    }};
    mask_position_x: {{ MaskPositionX = <background::background_position_x_value> }};
    mask_position_y: {{ MaskPositionY = <background::background_position_y_value> }};

    <mask_mode_single: MaskModeItem>:
        "match-source" => MaskModeItem::MatchSource
        | "luminance" => MaskModeItem::Luminance
        | "alpha" => MaskModeItem::Alpha
    ;
    mask_mode: {{ MaskMode
      = [<mask_mode_single>]# -> |x: Vec<_>| MaskModeType::List(x.into());
    }};
    mask: {{ (MaskImage, MaskRepeat, MaskPosition, MaskPositionX, MaskPositionY, MaskSize, BackgroundOrigin, BackgroundClip)
        = "none" -> |_| (
            BackgroundImageType::List(vec![BackgroundImageItem::None].into()),
            BackgroundRepeatType::List(vec![BackgroundRepeatItem::Pos(BackgroundRepeatValue::Repeat, BackgroundRepeatValue::Repeat)].into()),
            BackgroundPositionType::List(vec![BackgroundPositionItem::Pos(BackgroundPositionValue::Left(Length::Ratio(0.)), BackgroundPositionValue::Top(Length::Ratio(0.)))].into()),
            BackgroundPositionType::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.)))].into()),
            BackgroundPositionType::List(vec![BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.)))].into()),
            BackgroundSizeType::List(vec![BackgroundSizeItem::Auto].into()),
            BackgroundOriginType::List(vec![BackgroundOriginItem::BorderBox].into()),
            BackgroundClipType::List(vec![BackgroundClipItem::BorderBox].into()),
        );
        | [
            <image_single>
            || <background_repeat_single>
            || [<background_position_single_without_extra_check> [ '/' <background_size_single>]?]
            || <background_origin_single>
            || <background_clip_single>
        ]# -> |x: Vec<(
                Option<_>,
                Option<_>,
                Option<(_, Option<(_, _)>)>,
                Option<BackgroundOriginItem>,
                Option<BackgroundClipItem>
            )>| -> (BackgroundImageType, BackgroundRepeatType, BackgroundPositionType, BackgroundPositionType, BackgroundPositionType, BackgroundSizeType, BackgroundOriginType, BackgroundClipType) {
                let mut img = Vec::with_capacity(x.len());
                let mut rep = Vec::with_capacity(x.len());
                let mut pos = Vec::with_capacity(x.len());
                let mut pos_x = Vec::with_capacity(x.len());
                let mut pos_y = Vec::with_capacity(x.len());
                let mut size = Vec::with_capacity(x.len());
                let mut origin = Vec::with_capacity(x.len());
                let mut clip = Vec::with_capacity(x.len());
                for v in x.into_iter() {
                    match v.0 {
                        Some(x) => {
                            img.push(x);

                        }
                        None => {
                            img.push(BackgroundImageItem::None);
                        }
                    }
                    match v.1 {
                        Some(x) => {
                            rep.push(x);
                        }
                        None => {
                            rep.push(BackgroundRepeatItem::Pos(
                                BackgroundRepeatValue::Repeat,
                                BackgroundRepeatValue::Repeat
                            ));
                        }
                    }
                    match v.2 {
                        Some(pos_size) => {
                            let (__pos, __size) = pos_size;
                            {
                                if let BackgroundPositionItem::Pos(x, y) = &__pos {
                                    pos_x.push(BackgroundPositionItem::Value(x.clone()));
                                    pos_y.push(BackgroundPositionItem::Value(y.clone()));
                                }
                            }
                            pos.push(__pos);
                            match __size {
                            Some(s) => {
                                size.push(s.1);
                            },
                            None => {
                                size.push(BackgroundSizeItem::Auto);
                            }
                        }
                    }
                        None=> {
                            pos.push(
                                BackgroundPositionItem::Pos(
                                    BackgroundPositionValue::Left(Length::Ratio(0.)),
                                    BackgroundPositionValue::Top(Length::Ratio(0.))
                                )
                            );
                            pos_x.push(BackgroundPositionItem::Value(BackgroundPositionValue::Left(Length::Ratio(0.))));
                            pos_y.push(BackgroundPositionItem::Value(BackgroundPositionValue::Top(Length::Ratio(0.))));
                            size.push(BackgroundSizeItem::Auto);
                        }
                    }
                    if v.3.is_some() && v.4.is_some() {
                        if let Some(__origin) = v.3 {
                            origin.push(__origin);
                        }
                        if let Some(__clip) = v.4 {
                            clip.push(__clip);
                        }
                    } else if v.3.is_some() || v.4.is_some() {
                        if let Some(__origin) = v.3 {
                            origin.push(__origin.clone());
                            match __origin {
                                BackgroundOriginItem::PaddingBox => {
                                    clip.push(BackgroundClipItem::PaddingBox);
                                },
                                BackgroundOriginItem::BorderBox => {
                                    clip.push(BackgroundClipItem::BorderBox);
                                },
                                BackgroundOriginItem::ContentBox => {
                                    clip.push(BackgroundClipItem::ContentBox);
                                },
                            };
                        }
                        if let Some(__clip) = v.4 {
                            clip.push(__clip.clone());
                            match __clip {
                                BackgroundClipItem::PaddingBox => {
                                    origin.push(BackgroundOriginItem::PaddingBox);
                                },
                                BackgroundClipItem::BorderBox => {
                                    origin.push(BackgroundOriginItem::BorderBox);
                                },
                                BackgroundClipItem::ContentBox => {
                                    origin.push(BackgroundOriginItem::ContentBox);
                                },
                                _ => {},
                            };
                        }
                    } else {
                        origin.push(BackgroundOriginItem::PaddingBox);
                        clip.push(BackgroundClipItem::BorderBox);
                    }
                }
                (
                    BackgroundImageType::List(img.into()),
                    BackgroundRepeatType::List(rep.into()),
                    BackgroundPositionType::List(pos.into()),
                    BackgroundPositionType::List(pos_x.into()),
                    BackgroundPositionType::List(pos_y.into()),
                    BackgroundSizeType::List(size.into()),
                    BackgroundOriginType::List(origin.into()),
                    BackgroundClipType::List(clip.into()),
                )
        };
    }};
});

pub(crate) fn split_hv<T: Clone>(x: Vec<T>) -> (T, T) {
    let mut x = x.into_iter();
    let a = x.next().unwrap();
    let b = x.next().unwrap_or_else(|| a.clone());
    (a, b)
}

pub(crate) fn split_edges<T: Clone>(x: Vec<T>) -> (T, T, T, T) {
    let mut x = x.into_iter();
    let a = x.next().unwrap();
    let b = x.next().unwrap_or_else(|| a.clone());
    let c = x.next().unwrap_or_else(|| a.clone());
    let d = x.next().unwrap_or_else(|| b.clone());
    (a, b, c, d)
}
