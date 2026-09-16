use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};

use cssparser::match_ignore_ascii_case;

use crate::{length_num::LengthNum, query::MediaQueryStatus, typing::Length};

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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    And(Vec<MediaExpression>),
    Or(Vec<MediaExpression>),
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

#[cfg(test)]
mod tests {
    use super::*;

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

}
