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
    /// Per MQ4 the whole query is treated as `not all` (`not` cannot rescue it).
    UnknownFeature,
    /// A known sizing feature in boolean context, e.g. `(width)`.
    Boolean(SizedFeature),
    /// A known discrete feature in boolean context, e.g. `(orientation)`.
    AlwaysTrue,
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
            Self::MinWidth => width >= px,
            Self::MaxWidth => width <= px,
            Self::Height => media_value_eq(height, px),
            Self::MinHeight => height >= px,
            Self::MaxHeight => height <= px,
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

/// Equality with a small epsilon, so that computed values (e.g. `201.6dpi / 96`,
/// `0.3em * 14`) compare equal to mathematically-equal literals.
fn media_value_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= 1e-3
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
        self.cond.push(mq)
    }

    fn is_valid<L: LengthNum>(&self, mqs: &MediaQueryStatus<L>) -> bool {
        let width = mqs.width.to_f32();
        let height = mqs.height.to_f32();
        let mut matched = true;
        for cond in self.cond.iter() {
            matched = match cond {
                MediaExpression::UnknownFeature => return false,
                MediaExpression::Unknown => false,
                MediaExpression::MediaType(mt) => match mt {
                    MediaType::None => false,
                    MediaType::All => true,
                    MediaType::Screen => mqs.is_screen,
                },
                MediaExpression::Orientation(o) => match o {
                    Orientation::None => false,
                    Orientation::Portrait => mqs.width <= mqs.height,
                    Orientation::Landscape => mqs.width > mqs.height,
                },
                MediaExpression::Width(x) => SizedFeature::Width.matches(width, height, *x),
                MediaExpression::MinWidth(x) => SizedFeature::MinWidth.matches(width, height, *x),
                MediaExpression::MaxWidth(x) => SizedFeature::MaxWidth.matches(width, height, *x),
                MediaExpression::Height(x) => SizedFeature::Height.matches(width, height, *x),
                MediaExpression::MinHeight(x) => SizedFeature::MinHeight.matches(width, height, *x),
                MediaExpression::MaxHeight(x) => SizedFeature::MaxHeight.matches(width, height, *x),
                MediaExpression::Theme(t) => match t {
                    Theme::None => false,
                    Theme::Light => mqs.theme == Theme::Light,
                    Theme::Dark => mqs.theme == Theme::Dark,
                },
                MediaExpression::Resolution(x) => media_value_eq(mqs.pixel_ratio, *x),
                MediaExpression::MinResolution(x) => mqs.pixel_ratio >= *x,
                MediaExpression::MaxResolution(x) => mqs.pixel_ratio <= *x,
                MediaExpression::Sized(feature, len) => {
                    let base_font_size = mqs.base_font_size.to_f32();
                    match len.resolve_to_f32(mqs, base_font_size, false) {
                        Some(px) => feature.matches(width, height, px),
                        None => false,
                    }
                }
                MediaExpression::Boolean(feature) => match feature {
                    SizedFeature::Width => width != 0.,
                    SizedFeature::Height => height != 0.,
                    // min-/max- prefixed features are invalid in boolean context
                    _ => false,
                },
                MediaExpression::AlwaysTrue => true,
            };
            if !matched {
                break;
            }
        }
        if self.decorator == MediaTypeDecorator::Not {
            !matched
        } else {
            matched
        }
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
            .map(|cond| match cond {
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
            })
            .collect::<Box<[String]>>()
            .join(" and ");
        format!("{decorator}{cond}")
    }
}
