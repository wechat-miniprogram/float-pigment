//! CSS value types for each CSS property.
//!
//! The possible CSS values are different for each CSS property. This module lists all of them.
//!
//! For example, enum `BoxSizing` list out all possible values of the `box-sizing` property.

use alloc::{
    boxed::Box,
    string::{String, ToString},
};

use float_pigment_css_macro::{property_value_type, ResolveFontSize};

#[cfg(debug_assertions)]
use float_pigment_css_macro::{CompatibilityEnumCheck, CompatibilityStructCheck};

use serde::{Deserialize, Serialize};

use crate::length_num::LengthNum;
use crate::property::PropertyValueWithGlobal;
use crate::query::MediaQueryStatus;
use crate::resolve_font_size::ResolveFontSize;
use crate::sheet::{borrow::Array, str_store::StrRef};

/// A bitset for representing `!important`.
///
/// It is used in the binary format for better size.
/// Not suitable for common cases.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ImportantBitSet {
    None,
    Array(Array<u8>),
}

/// An expression inside `calc(...)`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum CalcExpr {
    /// A length, e.g. `2px`.
    Length(Box<Length>),
    /// A number, e.g. `0.1`.
    Number(Box<Number>),
    /// An angle, e.g. `45deg`.
    Angle(Box<Angle>),
    /// `+` expression.
    Plus(Box<CalcExpr>, Box<CalcExpr>),
    /// `-` expression.
    Sub(Box<CalcExpr>, Box<CalcExpr>),
    /// `*` expression.
    Mul(Box<CalcExpr>, Box<CalcExpr>),
    /// `/` expression.
    Div(Box<CalcExpr>, Box<CalcExpr>),
}

impl Default for CalcExpr {
    fn default() -> Self {
        Self::Length(Box::new(Length::Undefined))
    }
}

/// A number or an expression that evaluates to a number.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for NumberType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Number {
    F32(f32),
    I32(i32),
    Calc(Box<CalcExpr>),
}

impl Default for Number {
    fn default() -> Self {
        Self::I32(0)
    }
}

impl From<f32> for NumberType {
    fn from(x: f32) -> Self {
        NumberType::F32(x)
    }
}

impl From<i32> for NumberType {
    fn from(x: i32) -> Self {
        NumberType::I32(x)
    }
}

impl Number {
    /// Convert the number to `f32`.
    ///
    /// Panics if it is an expression.
    pub fn to_f32(&self) -> f32 {
        match self {
            Number::F32(x) => *x,
            Number::I32(x) => *x as f32,
            _ => panic!("cannot convert an expression to a number"),
        }
    }

    /// Convert the number to `i32`.
    ///
    /// Panics if it is an expression.
    pub fn to_i32(&self) -> i32 {
        match self {
            Number::I32(x) => *x,
            Number::F32(x) => *x as i32,
            _ => panic!("cannot convert an expression to a number"),
        }
    }
}

impl ResolveFontSize for Number {
    fn resolve_font_size(&mut self, _: f32) {
        // empty
    }
}

/// A color value or `current-color`.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ColorType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Color {
    Undefined,
    CurrentColor,
    Specified(u8, u8, u8, u8),
}

/// A length value or an expression that evaluates to a length value.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for LengthType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Length {
    Undefined,
    Auto,
    Px(f32),
    Vw(f32),
    Vh(f32),
    Rem(f32),
    Rpx(f32),
    Em(f32),
    Ratio(f32),
    Expr(LengthExpr),
    Vmin(f32),
    Vmax(f32),
}

#[allow(clippy::derivable_impls)]
impl Default for Length {
    fn default() -> Self {
        Length::Undefined
    }
}

impl Length {
    pub(crate) fn ratio_to_f32(&self) -> Option<f32> {
        match self {
            Length::Ratio(x) => Some(*x),
            _ => None,
        }
    }

    pub(crate) fn resolve_set(&mut self, font_size: f32) {
        *self = Self::Px(font_size);
    }

    pub(crate) fn resolve_em(&mut self, font_size: f32) {
        if let Self::Em(x) = *self {
            *self = Self::Px(x * font_size);
        }
    }

    pub(crate) fn resolve_em_and_ratio(&mut self, font_size: f32) {
        if let Self::Em(x) = *self {
            *self = Self::Px(x * font_size);
        } else if let Self::Ratio(x) = *self {
            *self = Self::Px(x * font_size);
        }
    }

    /// Resolve the length value to `f32`.
    ///
    /// The `relative_length` is used to calculate `...%` length.
    /// If `length_as_parent_font_size` is set, the `relative_length` is used for `em` length;
    /// otherwise the base font size in `media_query_status` is used.
    pub fn resolve_to_f32<L: LengthNum>(
        &self,
        media_query_status: &MediaQueryStatus<L>,
        relative_length: f32,
        length_as_parent_font_size: bool,
    ) -> Option<f32> {
        let r = match self {
            Length::Undefined | Length::Auto => None?,
            Length::Px(x) => *x,
            Length::Vw(x) => media_query_status.width.to_f32() / 100. * *x,
            Length::Vh(x) => media_query_status.height.to_f32() / 100. * *x,
            Length::Rem(x) => media_query_status.base_font_size.to_f32() * *x,
            Length::Rpx(x) => media_query_status.width.to_f32() / 750. * *x,
            Length::Em(x) => {
                if length_as_parent_font_size {
                    relative_length * *x
                } else {
                    media_query_status.base_font_size.to_f32() * *x
                }
            }
            Length::Ratio(x) => relative_length * *x,
            Length::Expr(x) => match x {
                LengthExpr::Invalid => None?,
                LengthExpr::Env(name, default_value) => match name.as_str() {
                    "safe-area-inset-left" => media_query_status.env.safe_area_inset_left.to_f32(),
                    "safe-area-inset-top" => media_query_status.env.safe_area_inset_top.to_f32(),
                    "safe-area-inset-right" => {
                        media_query_status.env.safe_area_inset_right.to_f32()
                    }
                    "safe-area-inset-bottom" => {
                        media_query_status.env.safe_area_inset_bottom.to_f32()
                    }
                    _ => default_value.resolve_to_f32(
                        media_query_status,
                        relative_length,
                        length_as_parent_font_size,
                    )?,
                },
                LengthExpr::Calc(x) => x.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?,
            },
            Length::Vmin(x) => {
                media_query_status
                    .width
                    .upper_bound(media_query_status.height)
                    .to_f32()
                    / 100.
                    * *x
            }
            Length::Vmax(x) => {
                media_query_status
                    .width
                    .lower_bound(media_query_status.height)
                    .to_f32()
                    / 100.
                    * *x
            }
        };
        Some(r)
    }

    /// Resolve the length value to `L`.
    ///
    /// The `relative_length` is used to calculate `...%` length.
    /// The base font size in `media_query_status` is used for `em` length.
    pub fn resolve_length<L: LengthNum>(
        &self,
        media_query_status: &MediaQueryStatus<L>,
        relative_length: L,
    ) -> Option<L> {
        let r = match self {
            Length::Undefined | Length::Auto => None?,
            Length::Expr(x) => match x {
                LengthExpr::Invalid => None?,
                LengthExpr::Env(name, default_value) => match name.as_str() {
                    "safe-area-inset-left" => media_query_status.env.safe_area_inset_left,
                    "safe-area-inset-top" => media_query_status.env.safe_area_inset_top,
                    "safe-area-inset-right" => media_query_status.env.safe_area_inset_right,
                    "safe-area-inset-bottom" => media_query_status.env.safe_area_inset_bottom,
                    _ => default_value.resolve_length(media_query_status, relative_length)?,
                },
                LengthExpr::Calc(x) => L::from_f32(x.resolve_to_f32(
                    media_query_status,
                    relative_length.to_f32(),
                    false,
                )?),
            },
            _ => L::from_f32(self.resolve_to_f32(
                media_query_status,
                relative_length.to_f32(),
                false,
            )?),
        };
        Some(r)
    }
}

/// An expression for a length value.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum LengthExpr {
    Invalid,
    Env(StrRef, Box<Length>),
    Calc(Box<CalcExpr>),
}

impl Default for LengthExpr {
    fn default() -> Self {
        Self::Invalid
    }
}

impl CalcExpr {
    fn resolve_to_f32<L: LengthNum>(
        &self,
        media_query_status: &MediaQueryStatus<L>,
        relative_length: f32,
        length_as_parent_font_size: bool,
    ) -> Option<f32> {
        let ret = match self {
            CalcExpr::Length(x) => x.resolve_to_f32(
                media_query_status,
                relative_length,
                length_as_parent_font_size,
            )?,
            CalcExpr::Number(_) => None?,
            CalcExpr::Angle(_) => None?,
            CalcExpr::Plus(x, y) => {
                let x = x.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                let y = y.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                x + y
            }
            CalcExpr::Sub(x, y) => {
                let x = x.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                let y = y.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                x - y
            }
            CalcExpr::Mul(x, y) => {
                let x = x.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                let y = y.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                x * y
            }
            CalcExpr::Div(x, y) => {
                let x = x.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                let y = y.resolve_to_f32(
                    media_query_status,
                    relative_length,
                    length_as_parent_font_size,
                )?;
                x / y
            }
        };
        Some(ret)
    }

    /// Resolve the length value to `L`.
    ///
    /// The `relative_length` is used to calculate `...%` length.
    /// The base font size in `media_query_status` is used for `em` length.
    pub fn resolve_length<L: LengthNum>(
        &self,
        media_query_status: &MediaQueryStatus<L>,
        relative_length: L,
    ) -> Option<L> {
        self.resolve_to_f32(media_query_status, relative_length.to_f32(), false)
            .map(|x| L::from_f32(x))
    }
}

/// An angle value or an expression that evaluates to an angle.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AngleType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
    Calc(Box<CalcExpr>),
}

impl Default for Angle {
    fn default() -> Self {
        Self::Deg(0.)
    }
}

impl ResolveFontSize for Angle {
    fn resolve_font_size(&mut self, _: f32) {
        // empty
    }
}

/// An angle value or a percentage value.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AngleOrPercentage {
    Angle(Angle),
    Percentage(f32),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for DisplayType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Display {
    None,
    Block,
    Flex,
    Inline,
    InlineBlock,
    Grid,
    FlowRoot,
    InlineFlex,
    InlineGrid,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for PositionType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for OverflowType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Overflow {
    Visible,
    Hidden,
    Auto,
    Scroll,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for OverflowWrapType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum OverflowWrap {
    Normal,
    BreakWord,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for PointerEventsType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum PointerEvents {
    Auto,
    None,
    /// A `wx` specific pointer-events type.
    WxRoot,
}

/// `wx` specific touch handling strategy.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for WxEngineTouchEventType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum WxEngineTouchEvent {
    Gesture,
    Click,
    None,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for VisibilityType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FlexWrapType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FlexDirectionType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for DirectionType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Direction {
    Auto,
    LTR,
    RTL,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for WritingModeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum WritingMode {
    HorizontalTb,
    VerticalLr,
    VerticalRl,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AlignItemsType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AlignItems {
    Stretch,
    Normal,
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
    SelfStart,
    SelfEnd,
    Baseline,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AlignSelfType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AlignSelf {
    Auto,
    Normal,
    Stretch,
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
    Baseline,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AlignContentType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AlignContent {
    Normal,
    Start,
    End,
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Baseline,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for JustifyContentType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum JustifyContent {
    Center,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Start,
    End,
    Left,
    Right,
    Stretch,
    Baseline,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for JustifyItemsType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum JustifyItems {
    Stretch,
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
    SelfStart,
    SelfEnd,
    Left,
    Right,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TextAlignType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    JustifyAll,
    Start,
    End,
    MatchParent,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FontWeightType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Num(Number),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for WordBreakType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum WordBreak {
    BreakWord,
    BreakAll,
    KeepAll,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for WhiteSpaceType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum WhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreWrap,
    PreLine,
    WxPreEdit,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TextOverflowType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for VerticalAlignType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum VerticalAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
    TextTop,
    TextBottom,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for LineHeightType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum LineHeight {
    Normal,
    #[resolve_font_size(Length::resolve_em_and_ratio)]
    Length(Length),
    Num(Number),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FontFamilyType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FontFamily {
    Names(Array<FontFamilyName>),
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FontFamilyName {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    Title(StrRef),
    SystemUi,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BoxSizingType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BoxSizing {
    ContentBox,
    PaddingBox,
    BorderBox,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BorderStyleType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BorderStyle {
    None,
    Solid,
    Dotted,
    Dashed,
    Hidden,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

/// The CSS `transform` item series.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TransformType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Transform {
    Series(Array<TransformItem>),
}

/// A `transform` item.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransformItem {
    None,
    Matrix([f32; 6]),
    Matrix3D([f32; 16]),
    #[resolve_font_size(Length::resolve_em)]
    Translate2D(Length, Length),
    #[resolve_font_size(Length::resolve_em)]
    Translate3D(Length, Length, Length),
    Scale2D(f32, f32),
    Scale3D(f32, f32, f32),
    Rotate2D(Angle),
    Rotate3D(f32, f32, f32, Angle),
    Skew(Angle, Angle),
    #[resolve_font_size(Length::resolve_em)]
    Perspective(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TransitionPropertyType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransitionProperty {
    List(Array<TransitionPropertyItem>),
}

/// The property name allowed in CSS `transition-property`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransitionPropertyItem {
    None,
    Transform,
    TransformOrigin,
    LineHeight,
    Opacity,
    All,
    Height,
    Width,
    MinHeight,
    MaxHeight,
    MinWidth,
    MaxWidth,
    MarginTop,
    MarginRight,
    MarginLeft,
    MarginBottom,
    Margin,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    Padding,
    Top,
    Right,
    Bottom,
    Left,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    Flex,
    BorderTopWidth,
    BorderRightWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderTopColor,
    BorderRightColor,
    BorderBottomColor,
    BorderLeftColor,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    Border,
    BorderWidth,
    BorderColor,
    BorderRadius,
    BorderLeft,
    BorderTop,
    BorderRight,
    BorderBottom,
    Font,
    ZIndex,
    BoxShadow,
    BackdropFilter,
    Filter,
    Color,
    TextDecorationColor,
    TextDecorationThickness,
    FontSize,
    FontWeight,
    LetterSpacing,
    WordSpacing,
    BackgroundColor,
    BackgroundPosition,
    BackgroundSize,
    Background,
    BackgroundPositionX,
    BackgroundPositionY,
    Mask,
    MaskSize,
    MaskPositionX,
    MaskPositionY,
    MaskPosition,
}

/// A helper type for `transition-timing-function`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum StepPosition {
    End,
    JumpStart,
    JumpEnd,
    JumpNone,
    JumpBoth,
    Start,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TransitionTimeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransitionTime {
    List(Array<u32>),
    ListI32(Array<i32>),
}

/// The CSS `transition-timing-funcction`.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TransitionTimingFnType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransitionTimingFn {
    List(Array<TransitionTimingFnItem>),
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransitionTimingFnItem {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    Steps(i32, StepPosition),
    CubicBezier(f32, f32, f32, f32),
}

/// The scroll bar options.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ScrollbarType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Scrollbar {
    /// Show the scroll bar when needed.
    Auto,
    /// Always hide the scroll bar.
    Hidden,
    /// Hide the scroll bar when not scrolling.
    AutoHide,
    /// Always show the scroll bar.
    AlwaysShow,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundRepeatType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundRepeat {
    List(Array<BackgroundRepeatItem>),
}

/// An item in `background-repeat`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundRepeatItem {
    /// The two-value form of `background-repeat`.
    Pos(BackgroundRepeatValue, BackgroundRepeatValue),
}

/// A `background-repeat` value.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundRepeatValue {
    Repeat,
    NoRepeat,
    Space,
    Round,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundSizeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundSize {
    List(Array<BackgroundSizeItem>),
}

/// An item in `background-size`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundSizeItem {
    Auto,
    #[resolve_font_size(Length::resolve_em)]
    Length(Length, Length),
    Cover,
    Contain,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundImageType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundImage {
    List(Array<BackgroundImageItem>),
}

/// An image tag in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ImageTags {
    LTR,
    RTL,
}

/// An image source in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ImageSource {
    None,
    Url(StrRef),
}

/// An item in `background-image`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundImageItem {
    None,
    Url(StrRef),
    Gradient(BackgroundImageGradientItem),
    Image(ImageTags, ImageSource, Color),
    Element(StrRef),
}

/// Gradient types in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundImageGradientItem {
    LinearGradient(Angle, Array<GradientColorItem>),
    RadialGradient(
        GradientShape,
        GradientSize,
        GradientPosition,
        Array<GradientColorItem>,
    ),
    ConicGradient(ConicGradientItem),
}

/// Gradient size types in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GradientSize {
    FarthestCorner,
    ClosestSide,
    ClosestCorner,
    FarthestSide,
    #[resolve_font_size(Length::resolve_em)]
    Len(Length, Length),
}

/// Gradient position types in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GradientPosition {
    #[resolve_font_size(Length::resolve_em)]
    Pos(Length, Length),
    SpecifiedPos(GradientSpecifiedPos, GradientSpecifiedPos),
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GradientSpecifiedPos {
    #[resolve_font_size(Length::resolve_em)]
    Left(Length),
    #[resolve_font_size(Length::resolve_em)]
    Right(Length),
    #[resolve_font_size(Length::resolve_em)]
    Top(Length),
    #[resolve_font_size(Length::resolve_em)]
    Bottom(Length),
    // TODO: Support x/y-start/end, block-start/end, inline-start/end, start, end
}

/// Gradient shape types in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GradientShape {
    Ellipse,
    Circle,
}

/// Gradient color types in image description.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GradientColorItem {
    ColorHint(Color, #[resolve_font_size(Length::resolve_em)] Length),
    SimpleColorHint(Color),
    AngleOrPercentageColorHint(Color, AngleOrPercentage),
}

/// A conic-gradient item.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct ConicGradientItem {
    pub angle: Angle,
    pub position: GradientPosition,
    pub items: Array<GradientColorItem>,
}

impl<T: ResolveFontSize> ResolveFontSize for Option<T> {
    fn resolve_font_size(&mut self, font_size: f32) {
        if let Some(value) = self {
            value.resolve_font_size(font_size)
        }
    }
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundPositionType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundPosition {
    List(Array<BackgroundPositionItem>),
}

/// An item in `background-position`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundPositionItem {
    Pos(BackgroundPositionValue, BackgroundPositionValue),
    Value(BackgroundPositionValue),
}

/// A `background-position` value.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundPositionValue {
    #[resolve_font_size(Length::resolve_em)]
    Top(Length),
    #[resolve_font_size(Length::resolve_em)]
    Bottom(Length),
    #[resolve_font_size(Length::resolve_em)]
    Left(Length),
    #[resolve_font_size(Length::resolve_em)]
    Right(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FontStyleType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundClipType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundClip {
    List(Array<BackgroundClipItem>),
}

/// An item in `background-clip`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundClipItem {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundOriginType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundOrigin {
    List(Array<BackgroundOriginItem>),
}

/// An item in `background-origin`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundOriginItem {
    BorderBox,
    PaddingBox,
    ContentBox,
}

/// An item in `background-attachment`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundAttachmentItem {
    Scroll,
    Fixed,
    Local,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackgroundAttachmentType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackgroundAttachment {
    List(Array<BackgroundAttachmentItem>),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FloatType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Float {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ListStyleTypeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ListStyleType {
    Disc,
    None,
    Circle,
    Square,
    Decimal,
    CjkDecimal,
    DecimalLeadingZero,
    LowerRoman,
    UpperRoman,
    LowerGreek,
    LowerAlpha,
    LowerLatin,
    UpperAlpha,
    UpperLatin,
    Armenian,
    Georgian,
    CustomIdent(StrRef),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ListStyleImageType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ListStyleImage {
    None,
    Url(StrRef),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ListStylePositionType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ListStylePosition {
    Outside,
    Inside,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ResizeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ZIndexType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ZIndex {
    Auto,
    Num(Number),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TextShadowType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextShadow {
    None,
    List(Array<TextShadowItem>),
}

/// An item in `text-shadow`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextShadowItem {
    /// A value of offset-y, offset-x, blur-radius, and color.
    TextShadowValue(
        #[resolve_font_size(Length::resolve_em_and_ratio)] Length,
        #[resolve_font_size(Length::resolve_em_and_ratio)] Length,
        #[resolve_font_size(Length::resolve_em_and_ratio)] Length,
        Color,
    ),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TextDecorationLineType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextDecorationLine {
    None,
    SpellingError,
    GrammarError,
    List(Array<TextDecorationLineItem>),
}

/// An item in `text-decoration-line`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextDecorationLineItem {
    Overline,
    LineThrough,
    Underline,
    Blink,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TextDecorationStyleType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TextDecorationThicknessType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TextDecorationThickness {
    Auto,
    FromFont,
    #[resolve_font_size(Length::resolve_em_and_ratio)]
    Length(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for LetterSpacingType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum LetterSpacing {
    Normal,
    #[resolve_font_size(Length::resolve_em_and_ratio)]
    Length(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for WordSpacingType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum WordSpacing {
    Normal,
    #[resolve_font_size(Length::resolve_em_and_ratio)]
    Length(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BorderRadiusType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BorderRadius {
    #[resolve_font_size(Length::resolve_em)]
    Pos(Length, Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BoxShadowType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BoxShadow {
    None,
    List(Array<BoxShadowItem>),
}

/// An item in `box-shadow`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BoxShadowItem {
    List(Array<ShadowItemType>),
}

/// A shadow type in `box-shadow`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ShadowItemType {
    Inset,
    #[resolve_font_size(Length::resolve_em)]
    OffsetX(Length),
    #[resolve_font_size(Length::resolve_em)]
    OffsetY(Length),
    #[resolve_font_size(Length::resolve_em)]
    BlurRadius(Length),
    #[resolve_font_size(Length::resolve_em)]
    SpreadRadius(Length),
    Color(Color),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for BackdropFilterType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum BackdropFilter {
    None,
    List(Array<FilterFunc>),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FilterType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Filter {
    None,
    List(Array<FilterFunc>),
}

/// A filter function for `filter` and `backdrop-filter`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FilterFunc {
    Url(StrRef),
    #[resolve_font_size(Length::resolve_em)]
    Blur(Length),
    #[resolve_font_size(Length::resolve_em)]
    Brightness(Length),
    #[resolve_font_size(Length::resolve_em)]
    Contrast(Length),
    DropShadow(DropShadow),
    #[resolve_font_size(Length::resolve_em)]
    Grayscale(Length),
    HueRotate(Angle),
    #[resolve_font_size(Length::resolve_em)]
    Invert(Length),
    #[resolve_font_size(Length::resolve_em)]
    Opacity(Length),
    #[resolve_font_size(Length::resolve_em)]
    Saturate(Length),
    #[resolve_font_size(Length::resolve_em)]
    Sepia(Length),
}

/// A drop shadow in filter function for `filter` and `backdrop-filter`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum DropShadow {
    List(Array<ShadowItemType>),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for TransformOriginType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TransformOrigin {
    #[resolve_font_size(Length::resolve_em)]
    LengthTuple(Length, Length, Length),
    Left,
    Right,
    Center,
    Bottom,
    Top,
    #[resolve_font_size(Length::resolve_em)]
    Length(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for MaskModeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum MaskMode {
    List(Array<MaskModeItem>),
}

/// An item in mask-mode.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum MaskModeItem {
    MatchSource,
    Alpha,
    Luminance,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AspectRatioType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AspectRatio {
    Auto,
    Ratio(Number, Number),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ContainType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Contain {
    None,
    Strict,
    Content,
    Multiple(Array<ContainKeyword>),
}

/// An item in multi-value form of `contain`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum ContainKeyword {
    Size,
    Layout,
    Style,
    Paint,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for ContentType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Content {
    None,
    Normal,
    Str(StrRef),
    Url(StrRef),
}

/// A unknown property.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for CustomPropertyType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum CustomProperty {
    None,
    Expr(StrRef, StrRef),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AnimationIterationCountType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationIterationCount {
    List(Array<AnimationIterationCountItem>),
}

/// An item in `animation-iteration-count`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationIterationCountItem {
    Number(f32),
    Infinite,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AnimationDirectionType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationDirection {
    List(Array<AnimationDirectionItem>),
}

/// An item in `animation-direction`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationDirectionItem {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AnimationFillModeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationFillMode {
    List(Array<AnimationFillModeItem>),
}

/// An item in `animation-fill-mode`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationFillModeItem {
    None,
    Forwards,
    Backwards,
    Both,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AnimationPlayStateType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationPlayState {
    List(Array<AnimationPlayStateItem>),
}

/// An item in `animation-play-state`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationPlayStateItem {
    Running,
    Paused,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for AnimationNameType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationName {
    List(Array<AnimationNameItem>),
}

/// An item in `animation-name`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimationNameItem {
    None,
    CustomIdent(StrRef),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for WillChangeType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum WillChange {
    Auto,
    List(Array<AnimateableFeature>),
}

/// An animation kind for `will-change`.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum AnimateableFeature {
    /// The content of the element is likely to be animated.
    Contents,
    /// The content of the element is scrollable.
    ScrollPosition,
    /// An unknown kind.
    CustomIdent(StrRef),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for FontFeatureSettingsType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FontFeatureSettings {
    Normal,
    FeatureTags(Array<FeatureTag>),
}

/// A font feature tag for `font-feature-settings`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityStructCheck))]
pub struct FeatureTag {
    /// The four-letter OpenType tag, e.g. `liga`.
    pub opentype_tag: StrRef,
    /// The optional number value in `font-feature-settings`.
    pub value: Number,
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for GapType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Gap {
    Normal,
    #[resolve_font_size(Length::resolve_em)]
    Length(Length),
}

/// The `grid-template-rows` property defines the line names and track sizing functions of the grid rows.
#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for GridTemplateType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GridTemplate {
    /// A keyword meaning that there is no explicit grid
    None,
    TrackList(Array<TrackListItem>),
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TrackListItem {
    LineNames(Array<StrRef>),
    TrackSize(TrackSize),
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum TrackSize {
    MinContent,
    MaxContent,
    Fr(f32),
    #[resolve_font_size(Length::resolve_em)]
    Length(Length),
}

#[allow(missing_docs)]
#[repr(C)]
#[property_value_type(PropertyValueWithGlobal for GridAutoFlowType)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ResolveFontSize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum GridAutoFlow {
    Row,
    Column,
    RowDense,
    ColumnDense,
}
