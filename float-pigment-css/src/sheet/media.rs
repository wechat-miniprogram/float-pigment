use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};

use core::fmt;
use cssparser::match_ignore_ascii_case;
use serde::de::{DeserializeSeed, EnumAccess, Error, SeqAccess, VariantAccess, Visitor};

use super::borrow::Array;
use crate::{length_num::LengthNum, query::MediaQueryStatus, typing::Length};

pub(crate) const MAX_MEDIA_EXPRESSION_DEPTH: usize = 64;

#[cfg(debug_assertions)]
use float_pigment_css_macro::CompatibilityEnumCheck;

#[derive(Debug, Clone)]
pub(crate) struct Media {
    pub(crate) parent: Option<Rc<Media>>,
    pub(crate) media_queries: Vec<MediaQuery>,
}

#[derive(Debug, Clone)]
pub(crate) struct MediaQuery {
    pub(crate) decorator: MediaTypeDecorator,
    pub(crate) cond: Vec<MediaExpression>,
}

#[repr(C)]
#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum MediaExpression {
    Unknown,
    MediaType(MediaType),
    Orientation(Orientation),
    Width(f32),
    MinWidth(f32),
    MaxWidth(f32),
    Height(f32),
    MinHeight(f32),
    MaxHeight(f32),
    Theme(Theme),
    Resolution(f32),
    MinResolution(f32),
    MaxResolution(f32),
    Sized(SizedFeature, Length),
    /// A media feature that is not recognized, or whose value is invalid.
    UnknownFeature,
    /// A known sizing feature in boolean context, e.g. `(width)`.
    Boolean(SizedFeature),
    /// A known discrete feature in boolean context, e.g. `(orientation)`.
    AlwaysTrue,
    Not(Box<MediaExpression>),
    And(#[serde(serialize_with = "serialize_expressions")] Array<MediaExpression>),
    Or(#[serde(serialize_with = "serialize_expressions")] Array<MediaExpression>),
    Range(SizedFeature, MediaComparison, Length),
    ResolutionRange(MediaComparison, f32),
    InfiniteResolution(MediaComparison),
}

/// The sizing media features, i.e. `width` / `height` and their `min-` / `max-` variants.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum SizedFeature {
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
}

impl SizedFeature {
    pub(crate) fn from_name(name: &str) -> Option<Self> {
        match_ignore_ascii_case! { name,
            "width" => Some(Self::Width),
            "min-width" => Some(Self::MinWidth),
            "max-width" => Some(Self::MaxWidth),
            "height" => Some(Self::Height),
            "min-height" => Some(Self::MinHeight),
            "max-height" => Some(Self::MaxHeight),
            _ => None,
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Width => "width",
            Self::MinWidth => "min-width",
            Self::MaxWidth => "max-width",
            Self::Height => "height",
            Self::MinHeight => "min-height",
            Self::MaxHeight => "max-height",
        }
    }

    fn matches(&self, width: f32, height: f32, px: f32) -> bool {
        match self {
            Self::Width => media_value_eq(width, px),
            Self::MinWidth => MediaComparison::GreaterEqual.matches(width, px),
            Self::MaxWidth => MediaComparison::LessEqual.matches(width, px),
            Self::Height => media_value_eq(height, px),
            Self::MinHeight => MediaComparison::GreaterEqual.matches(height, px),
            Self::MaxHeight => MediaComparison::LessEqual.matches(height, px),
        }
    }

    /// Keep `px` values in the legacy variants for binary format compatibility.
    pub(crate) fn into_expression(self, len: Length) -> MediaExpression {
        match (self, len) {
            (Self::Width, Length::Px(x)) => MediaExpression::Width(x),
            (Self::MinWidth, Length::Px(x)) => MediaExpression::MinWidth(x),
            (Self::MaxWidth, Length::Px(x)) => MediaExpression::MaxWidth(x),
            (Self::Height, Length::Px(x)) => MediaExpression::Height(x),
            (Self::MinHeight, Length::Px(x)) => MediaExpression::MinHeight(x),
            (Self::MaxHeight, Length::Px(x)) => MediaExpression::MaxHeight(x),
            (feature, len) => MediaExpression::Sized(feature, len),
        }
    }
}

// Allow one f32 rounding step from unit conversion, not a fixed CSS-unit tolerance.
fn media_value_eq(a: f32, b: f32) -> bool {
    a == b
        || (a.is_finite() && b.is_finite() && (a - b).abs() <= f32::EPSILON * a.abs().max(b.abs()))
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum MediaComparison {
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl MediaComparison {
    pub(crate) fn reverse(self) -> Self {
        match self {
            Self::Equal => Self::Equal,
            Self::Less => Self::Greater,
            Self::LessEqual => Self::GreaterEqual,
            Self::Greater => Self::Less,
            Self::GreaterEqual => Self::LessEqual,
        }
    }
    fn matches(self, a: f32, b: f32) -> bool {
        let equal = media_value_eq(a, b);
        match self {
            Self::Equal => equal,
            Self::Less => a < b && !equal,
            Self::LessEqual => a < b || equal,
            Self::Greater => a > b && !equal,
            Self::GreaterEqual => a > b || equal,
        }
    }
    fn symbol(self) -> &'static str {
        match self {
            Self::Equal => "=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Truth {
    False,
    True,
    Unknown,
}
impl Truth {
    fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            _ => Self::True,
        }
    }
    fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            _ => Self::False,
        }
    }
}
impl From<bool> for Truth {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

fn resolution_to_string(name: &str, x: f32) -> String {
    if x.is_infinite() {
        format!("({name}: infinite)")
    } else {
        format!("({name}: {x}dppx)")
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum MediaTypeDecorator {
    None,
    Not,
    Only,
}

#[repr(C)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum MediaType {
    None,
    All,
    Screen,
}

#[repr(C)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub(crate) enum Orientation {
    None,
    Portrait,
    Landscape,
}

/// The current theme of the system environment, e.g. dark mode.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum Theme {
    /// Unspecified.
    None,
    /// Light mode.
    Light,
    /// Dark mode.
    Dark,
}

impl Media {
    pub(crate) fn new(parent: Option<Rc<Media>>) -> Self {
        Self {
            parent,
            media_queries: vec![],
        }
    }

    pub(crate) fn add_media_query(&mut self, mq: MediaQuery) {
        self.media_queries.push(mq)
    }

    pub(crate) fn is_valid<L: LengthNum>(&self, mqs: &MediaQueryStatus<L>) -> bool {
        if let Some(parent) = &self.parent {
            if !parent.is_valid(mqs) {
                return false;
            }
        }
        for mq in self.media_queries.iter() {
            if mq.is_valid(mqs) {
                return true;
            }
        }
        false
    }

    pub(crate) fn to_media_query_string_list(&self, list: &mut Vec<String>) {
        if let Some(p) = &self.parent {
            p.to_media_query_string_list(list);
        }
        list.push(
            self.media_queries
                .iter()
                .map(|x| x.to_media_query_string())
                .collect::<Box<[String]>>()
                .join(", "),
        )
    }
}

impl MediaQuery {
    pub(crate) fn new() -> Self {
        Self {
            decorator: MediaTypeDecorator::None,
            cond: vec![],
        }
    }

    pub(crate) fn set_decorator(&mut self, d: MediaTypeDecorator) {
        self.decorator = d;
    }

    pub(crate) fn add_media_expression(&mut self, mq: MediaExpression) {
        if let MediaExpression::And(expressions) = mq {
            self.cond.extend(expressions);
        } else {
            self.cond.push(mq);
        }
    }

    fn is_valid<L: LengthNum>(&self, mqs: &MediaQueryStatus<L>) -> bool {
        let value = self
            .cond
            .iter()
            .fold(Truth::True, |v, x| v.and(x.evaluate(mqs)));
        let value = if self.decorator == MediaTypeDecorator::Not {
            value.not()
        } else {
            value
        };
        value == Truth::True
    }

    pub(crate) fn to_media_query_string(&self) -> String {
        let decorator = match self.decorator {
            MediaTypeDecorator::None => "",
            MediaTypeDecorator::Not => "not ",
            MediaTypeDecorator::Only => "only ",
        };
        let cond = self
            .cond
            .iter()
            .map(MediaExpression::to_query_string)
            .collect::<Box<[String]>>()
            .join(" and ");
        format!("{decorator}{cond}")
    }
}

impl MediaExpression {
    pub(crate) fn is_within_depth_limit(&self) -> bool {
        fn check(expression: &MediaExpression, depth: usize) -> bool {
            if depth > MAX_MEDIA_EXPRESSION_DEPTH {
                return false;
            }
            match expression {
                MediaExpression::Not(child) => check(child, depth + 1),
                MediaExpression::And(children) | MediaExpression::Or(children) => {
                    children.iter().all(|child| check(child, depth + 1))
                }
                _ => true,
            }
        }
        check(self, 1)
    }

    fn evaluate<L: LengthNum>(&self, mqs: &MediaQueryStatus<L>) -> Truth {
        let width = mqs.width.to_f32();
        let height = mqs.height.to_f32();
        match self {
            Self::UnknownFeature => Truth::Unknown,
            Self::Not(expr) => expr.evaluate(mqs).not(),
            Self::And(exprs) => exprs
                .iter()
                .fold(Truth::True, |v, x| v.and(x.evaluate(mqs))),
            Self::Or(exprs) => exprs
                .iter()
                .fold(Truth::False, |v, x| v.or(x.evaluate(mqs))),
            Self::Range(feature, op, len) => {
                let actual = match feature {
                    SizedFeature::Width => width,
                    SizedFeature::Height => height,
                    // Keep the serialized feature type, but reject invalid range variants.
                    _ => return Truth::Unknown,
                };
                len.resolve_to_f32(mqs, mqs.base_font_size.to_f32(), false)
                    .map(|v| Truth::from(op.matches(actual, v)))
                    .unwrap_or(Truth::Unknown)
            }
            Self::ResolutionRange(op, value) => op.matches(mqs.pixel_ratio, *value).into(),
            Self::InfiniteResolution(op) => op.matches(mqs.pixel_ratio, f32::INFINITY).into(),
            Self::Unknown => Truth::False,
            Self::MediaType(mt) => match mt {
                MediaType::None => Truth::False,
                MediaType::All => Truth::True,
                MediaType::Screen => mqs.is_screen.into(),
            },
            Self::Orientation(o) => match o {
                Orientation::None => Truth::False,
                Orientation::Portrait => (mqs.width <= mqs.height).into(),
                Orientation::Landscape => (mqs.width > mqs.height).into(),
            },
            Self::Width(x) => SizedFeature::Width.matches(width, height, *x).into(),
            Self::MinWidth(x) => SizedFeature::MinWidth.matches(width, height, *x).into(),
            Self::MaxWidth(x) => SizedFeature::MaxWidth.matches(width, height, *x).into(),
            Self::Height(x) => SizedFeature::Height.matches(width, height, *x).into(),
            Self::MinHeight(x) => SizedFeature::MinHeight.matches(width, height, *x).into(),
            Self::MaxHeight(x) => SizedFeature::MaxHeight.matches(width, height, *x).into(),
            Self::Theme(t) => match t {
                Theme::None => Truth::False,
                Theme::Light => (mqs.theme == Theme::Light).into(),
                Theme::Dark => (mqs.theme == Theme::Dark).into(),
            },
            Self::Resolution(x) => media_value_eq(mqs.pixel_ratio, *x).into(),
            Self::MinResolution(x) => MediaComparison::GreaterEqual
                .matches(mqs.pixel_ratio, *x)
                .into(),
            Self::MaxResolution(x) => MediaComparison::LessEqual
                .matches(mqs.pixel_ratio, *x)
                .into(),
            Self::Sized(feature, len) => len
                .resolve_to_f32(mqs, mqs.base_font_size.to_f32(), false)
                .map(|px| Truth::from(feature.matches(width, height, px)))
                .unwrap_or(Truth::False),
            Self::Boolean(feature) => match feature {
                SizedFeature::Width => (width != 0.).into(),
                SizedFeature::Height => (height != 0.).into(),
                _ => Truth::False,
            },
            Self::AlwaysTrue => Truth::True,
        }
    }
    fn to_query_string(&self) -> String {
        match self {
            MediaExpression::Unknown => "unknown".into(),
            MediaExpression::MediaType(mt) => match mt {
                MediaType::None => "none".into(),
                MediaType::All => "all".into(),
                MediaType::Screen => "screen".into(),
            },
            MediaExpression::Orientation(o) => match o {
                Orientation::None => "(orientation: none)",
                Orientation::Portrait => "(orientation: portrait)",
                Orientation::Landscape => "(orientation: landscape)",
            }
            .into(),
            MediaExpression::Width(x) => format!("({}: {x}px)", SizedFeature::Width.name()),
            MediaExpression::MinWidth(x) => {
                format!("({}: {x}px)", SizedFeature::MinWidth.name())
            }
            MediaExpression::MaxWidth(x) => {
                format!("({}: {x}px)", SizedFeature::MaxWidth.name())
            }
            MediaExpression::Height(x) => format!("({}: {x}px)", SizedFeature::Height.name()),
            MediaExpression::MinHeight(x) => {
                format!("({}: {x}px)", SizedFeature::MinHeight.name())
            }
            MediaExpression::MaxHeight(x) => {
                format!("({}: {x}px)", SizedFeature::MaxHeight.name())
            }
            MediaExpression::Theme(t) => match t {
                Theme::None => "(prefers-color-scheme: none)",
                Theme::Light => "(prefers-color-scheme: light)",
                Theme::Dark => "(prefers-color-scheme: dark)",
            }
            .into(),
            MediaExpression::Resolution(x) => resolution_to_string("resolution", *x),
            MediaExpression::MinResolution(x) => resolution_to_string("min-resolution", *x),
            MediaExpression::MaxResolution(x) => resolution_to_string("max-resolution", *x),
            MediaExpression::Sized(feature, len) => format!("({}: {len})", feature.name()),
            MediaExpression::UnknownFeature => "(unknown: unknown)".into(),
            MediaExpression::Boolean(feature) => format!("({})", feature.name()),
            MediaExpression::AlwaysTrue => "(orientation)".into(),

            Self::Not(expr) => format!("(not {})", expr.to_query_string()),
            Self::And(exprs) | Self::Or(exprs) => {
                let joiner = if matches!(self, Self::And(_)) {
                    " and "
                } else {
                    " or "
                };
                format!(
                    "({})",
                    exprs
                        .iter()
                        .map(Self::to_query_string)
                        .collect::<Vec<_>>()
                        .join(joiner)
                )
            }
            Self::Range(feature, op, value) => {
                format!("({} {} {value})", feature.name(), op.symbol())
            }
            Self::ResolutionRange(op, value) => format!("(resolution {} {value}dppx)", op.symbol()),
            Self::InfiniteResolution(op) => format!("(resolution {} infinite)", op.symbol()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for MediaExpression {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ExpressionSeed(1).deserialize(deserializer)
    }
}

fn serialize_expressions<S: serde::Serializer>(
    expressions: &Array<MediaExpression>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(expressions.iter())
}

struct ExpressionSeed(usize);

impl<'de> DeserializeSeed<'de> for ExpressionSeed {
    type Value = MediaExpression;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        if self.0 > MAX_MEDIA_EXPRESSION_DEPTH {
            return Err(D::Error::custom(
                "media expression nesting exceeds 64 levels",
            ));
        }
        deserializer.deserialize_enum("MediaExpression", VARIANTS, self)
    }
}

// Keep variant order and payload shapes identical to the derived wire format.
const VARIANTS: &[&str] = &[
    "Unknown",
    "MediaType",
    "Orientation",
    "Width",
    "MinWidth",
    "MaxWidth",
    "Height",
    "MinHeight",
    "MaxHeight",
    "Theme",
    "Resolution",
    "MinResolution",
    "MaxResolution",
    "Sized",
    "UnknownFeature",
    "Boolean",
    "AlwaysTrue",
    "Not",
    "And",
    "Or",
    "Range",
    "ResolutionRange",
    "InfiniteResolution",
];

#[derive(serde::Deserialize)]
#[serde(field_identifier)]
enum Kind {
    Unknown,
    MediaType,
    Orientation,
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
    Theme,
    Resolution,
    MinResolution,
    MaxResolution,
    Sized,
    UnknownFeature,
    Boolean,
    AlwaysTrue,
    Not,
    And,
    Or,
    Range,
    ResolutionRange,
    InfiniteResolution,
}

impl<'de> Visitor<'de> for ExpressionSeed {
    type Value = MediaExpression;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a media expression")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (kind, variant) = data.variant::<Kind>()?;
        match kind {
            Kind::Unknown => {
                variant.unit_variant()?;
                Ok(MediaExpression::Unknown)
            }
            Kind::MediaType => variant.newtype_variant().map(MediaExpression::MediaType),
            Kind::Orientation => variant.newtype_variant().map(MediaExpression::Orientation),
            Kind::Width => variant.newtype_variant().map(MediaExpression::Width),
            Kind::MinWidth => variant.newtype_variant().map(MediaExpression::MinWidth),
            Kind::MaxWidth => variant.newtype_variant().map(MediaExpression::MaxWidth),
            Kind::Height => variant.newtype_variant().map(MediaExpression::Height),
            Kind::MinHeight => variant.newtype_variant().map(MediaExpression::MinHeight),
            Kind::MaxHeight => variant.newtype_variant().map(MediaExpression::MaxHeight),
            Kind::Theme => variant.newtype_variant().map(MediaExpression::Theme),
            Kind::Resolution => variant.newtype_variant().map(MediaExpression::Resolution),
            Kind::MinResolution => variant
                .newtype_variant()
                .map(MediaExpression::MinResolution),
            Kind::MaxResolution => variant
                .newtype_variant()
                .map(MediaExpression::MaxResolution),
            Kind::Sized => {
                let (value0, value1) = variant.tuple_variant(2, SizedVisitor)?;
                Ok(MediaExpression::Sized(value0, value1))
            }
            Kind::UnknownFeature => {
                variant.unit_variant()?;
                Ok(MediaExpression::UnknownFeature)
            }
            Kind::Boolean => variant.newtype_variant().map(MediaExpression::Boolean),
            Kind::AlwaysTrue => {
                variant.unit_variant()?;
                Ok(MediaExpression::AlwaysTrue)
            }
            Kind::Not => variant
                .newtype_variant_seed(ExpressionSeed(self.0 + 1))
                .map(|value| MediaExpression::Not(Box::new(value))),
            Kind::And => variant
                .newtype_variant_seed(ExpressionsSeed(self.0 + 1))
                .map(MediaExpression::And),
            Kind::Or => variant
                .newtype_variant_seed(ExpressionsSeed(self.0 + 1))
                .map(MediaExpression::Or),
            Kind::Range => {
                let (value0, value1, value2) = variant.tuple_variant(3, RangeVisitor)?;
                Ok(MediaExpression::Range(value0, value1, value2))
            }
            Kind::ResolutionRange => {
                let (value0, value1) = variant.tuple_variant(2, ResolutionRangeVisitor)?;
                Ok(MediaExpression::ResolutionRange(value0, value1))
            }
            Kind::InfiniteResolution => variant
                .newtype_variant()
                .map(MediaExpression::InfiniteResolution),
        }
    }
}

struct ExpressionsSeed(usize);

impl<'de> DeserializeSeed<'de> for ExpressionsSeed {
    type Value = Array<MediaExpression>;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_seq(self)
    }
}

impl<'de> Visitor<'de> for ExpressionsSeed {
    type Value = Array<MediaExpression>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("media expressions")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut expressions = Vec::new();
        while let Some(expression) = seq.next_element_seed(ExpressionSeed(self.0))? {
            expressions.push(expression);
        }
        Ok(expressions.into())
    }
}

struct SizedVisitor;

impl<'de> Visitor<'de> for SizedVisitor {
    type Value = (SizedFeature, Length);

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Sized fields")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok((
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(0, &self))?,
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(1, &self))?,
        ))
    }
}

struct RangeVisitor;

impl<'de> Visitor<'de> for RangeVisitor {
    type Value = (SizedFeature, MediaComparison, Length);

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Range fields")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok((
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(0, &self))?,
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(1, &self))?,
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(2, &self))?,
        ))
    }
}

struct ResolutionRangeVisitor;

impl<'de> Visitor<'de> for ResolutionRangeVisitor {
    type Value = (MediaComparison, f32);

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ResolutionRange fields")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        Ok((
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(0, &self))?,
            seq.next_element()?
                .ok_or_else(|| A::Error::invalid_length(1, &self))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_pigment_consistent_bincode::Options;

    #[test]
    fn media_expression_decode_depth_limit() {
        use float_pigment_consistent_bincode::Options;
        let mut expression = MediaExpression::Unknown;
        for _ in 1..65 {
            expression = MediaExpression::Not(Box::new(expression));
        }
        let bytes = float_pigment_consistent_bincode::DefaultOptions::new()
            .serialize(&expression)
            .unwrap();
        let result = float_pigment_consistent_bincode::DefaultOptions::new()
            .deserialize::<MediaExpression>(&bytes);
        assert!(result.is_err(), "65 expression levels must be rejected");
    }

    #[test]
    fn invalid_range_features_remain_unknown() {
        let status = MediaQueryStatus::<f32>::default_screen();
        for feature in [
            SizedFeature::MinWidth,
            SizedFeature::MaxWidth,
            SizedFeature::MinHeight,
            SizedFeature::MaxHeight,
        ] {
            let expression =
                MediaExpression::Range(feature, MediaComparison::Equal, Length::Px(600.));
            assert!(expression.evaluate(&status) == Truth::Unknown);
            assert!(MediaExpression::Not(Box::new(expression)).evaluate(&status) == Truth::Unknown);
        }
    }

    fn chain(depth: usize, tags: &[u32]) -> MediaExpression {
        let mut expression = MediaExpression::Unknown;
        for level in 1..depth {
            expression = match tags[(level - 1) % tags.len()] {
                17 => MediaExpression::Not(Box::new(expression)),
                18 => MediaExpression::And(alloc::vec![expression].into()),
                19 => MediaExpression::Or(alloc::vec![expression].into()),
                _ => unreachable!(),
            };
        }
        expression
    }

    // Build hostile input without recursively serializing or dropping a hostile tree.
    fn chain_bytes(depth: usize, tags: &[u32]) -> Vec<u8> {
        let options = float_pigment_consistent_bincode::DefaultOptions::new();
        let leaf = options.serialize(&MediaExpression::Unknown).unwrap();
        let mut size = leaf.len();
        let mut headers = Vec::new();
        for level in 1..depth {
            let tag = tags[(level - 1) % tags.len()];
            let sequence = if tag == 17 {
                Vec::new()
            } else {
                options.serialize(&1u64).unwrap()
            };
            let mut header = options.serialize(&tag).unwrap();
            header.extend(
                options
                    .serialize(&((size + sequence.len()) as u32))
                    .unwrap(),
            );
            header.extend(sequence);
            size += header.len();
            headers.push(header);
        }
        let mut bytes = Vec::with_capacity(size);
        for header in headers.into_iter().rev() {
            bytes.extend(header);
        }
        bytes.extend(leaf);
        bytes
    }

    #[test]
    fn wire_format_roundtrip() {
        let expressions = alloc::vec![
            MediaExpression::Unknown,
            MediaExpression::MediaType(MediaType::Screen),
            MediaExpression::Orientation(Orientation::Portrait),
            MediaExpression::Width(1.),
            MediaExpression::MinWidth(2.),
            MediaExpression::MaxWidth(3.),
            MediaExpression::Height(4.),
            MediaExpression::MinHeight(5.),
            MediaExpression::MaxHeight(6.),
            MediaExpression::Theme(Theme::Dark),
            MediaExpression::Resolution(1.),
            MediaExpression::MinResolution(2.),
            MediaExpression::MaxResolution(3.),
            MediaExpression::Sized(SizedFeature::Width, Length::Em(2.)),
            MediaExpression::UnknownFeature,
            MediaExpression::Boolean(SizedFeature::Height),
            MediaExpression::AlwaysTrue,
            MediaExpression::Not(Box::new(MediaExpression::Unknown)),
            MediaExpression::And(alloc::vec![MediaExpression::AlwaysTrue].into()),
            MediaExpression::Or(alloc::vec![MediaExpression::UnknownFeature].into()),
            MediaExpression::Range(
                SizedFeature::Width,
                MediaComparison::Greater,
                Length::Px(20.)
            ),
            MediaExpression::ResolutionRange(MediaComparison::Less, 2.),
            MediaExpression::InfiniteResolution(MediaComparison::Equal),
        ];
        let options = float_pigment_consistent_bincode::DefaultOptions::new();
        for (tag, expression) in expressions.iter().enumerate() {
            let bytes = options.serialize(expression).unwrap();
            assert_eq!(bytes[0], tag as u8);
            let decoded: MediaExpression = options.deserialize(&bytes).unwrap();
            assert_eq!(options.serialize(&decoded).unwrap(), bytes);
            let json = serde_json::to_string(expression).unwrap();
            let decoded: MediaExpression = serde_json::from_str(&json).unwrap();
            assert_eq!(serde_json::to_string(&decoded).unwrap(), json);
        }
        assert_eq!(
            options.serialize(&expressions[17]).unwrap(),
            alloc::vec![17, 2, 0, 0]
        );
        assert_eq!(
            options.serialize(&expressions[18]).unwrap(),
            alloc::vec![18, 3, 1, 16, 0]
        );
        assert_eq!(
            options.serialize(&expressions[19]).unwrap(),
            alloc::vec![19, 3, 1, 14, 0]
        );
        assert_eq!(
            serde_json::to_string(&expressions[18]).unwrap(),
            r#"{"And":["AlwaysTrue"]}"#
        );
        assert_eq!(
            serde_json::to_string(&expressions[19]).unwrap(),
            r#"{"Or":["UnknownFeature"]}"#
        );
    }

    #[test]
    fn depth_counts_paths_not_siblings() {
        let options = float_pigment_consistent_bincode::DefaultOptions::new();
        let expression =
            MediaExpression::And(alloc::vec![MediaExpression::AlwaysTrue; 1000].into());
        let bytes = options.serialize(&expression).unwrap();
        let decoded: MediaExpression = options.deserialize(&bytes).unwrap();
        assert!(decoded.evaluate(&MediaQueryStatus::<f32>::default_screen()) == Truth::True);

        // A complete sibling must be safely dropped when decoding the next one fails.
        let expression =
            MediaExpression::Or(alloc::vec![chain(63, &[17]), chain(64, &[17])].into());
        let bytes = options.serialize(&expression).unwrap();
        assert!(options.deserialize::<MediaExpression>(&bytes).is_err());
        let bytes = options.serialize(&MediaExpression::AlwaysTrue).unwrap();
        assert!(options.deserialize::<MediaExpression>(&bytes).is_ok());
    }

    #[cfg(feature = "std")]
    #[test]
    fn small_stack_depth_limit() {
        const WORKER: &str = "FP_MEDIA_DEPTH_TEST_WORKER";
        if std::env::var_os(WORKER).is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "sheet::media::tests::small_stack_depth_limit",
                    "--nocapture",
                ])
                .env(WORKER, "1")
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "child failed: {}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let options = float_pigment_consistent_bincode::DefaultOptions::new();
                for tags in [&[17][..], &[18][..], &[19][..], &[17, 18, 19][..]] {
                    let boundary = chain(64, tags);
                    let bytes = chain_bytes(64, tags);
                    assert_eq!(options.serialize(&boundary).unwrap(), bytes);
                    let decoded: MediaExpression = options.deserialize(&bytes).unwrap();
                    assert!(decoded.is_within_depth_limit());
                    decoded.evaluate(&MediaQueryStatus::<f32>::default_screen());
                    drop(decoded);
                    for depth in [65, 50_000] {
                        let bytes = chain_bytes(depth, tags);
                        let error = options.deserialize::<MediaExpression>(&bytes).unwrap_err();
                        assert!(
                            error
                                .to_string()
                                .contains("media expression nesting exceeds 64 levels"),
                            "{error}"
                        );
                    }
                }
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
