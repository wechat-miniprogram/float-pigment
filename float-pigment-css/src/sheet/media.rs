use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};

use crate::{length_num::LengthNum, query::MediaQueryStatus};

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
        let allow_unknown = self.decorator != MediaTypeDecorator::Only;
        for cond in self.cond.iter() {
            let mut matched = match cond {
                MediaExpression::Unknown => allow_unknown,
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
                MediaExpression::Width(x) => mqs.width.to_f32() == *x,
                MediaExpression::MinWidth(x) => mqs.width.to_f32() >= *x,
                MediaExpression::MaxWidth(x) => mqs.width.to_f32() <= *x,
                MediaExpression::Height(x) => mqs.height.to_f32() == *x,
                MediaExpression::MinHeight(x) => mqs.height.to_f32() >= *x,
                MediaExpression::MaxHeight(x) => mqs.height.to_f32() <= *x,
                MediaExpression::Theme(t) => match t {
                    Theme::None => false,
                    Theme::Light => mqs.theme == Theme::Light,
                    Theme::Dark => mqs.theme == Theme::Dark,
                },
            };
            if self.decorator == MediaTypeDecorator::Not {
                matched = !matched;
            }
            if !matched {
                return false;
            }
        }
        true
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
                MediaExpression::Width(x) => format!("(width: {x}px)"),
                MediaExpression::MinWidth(x) => format!("(min-width: {x}px)"),
                MediaExpression::MaxWidth(x) => format!("(max-width: {x}px)"),
                MediaExpression::Height(x) => format!("(height: {x}px)"),
                MediaExpression::MinHeight(x) => format!("(min-height: {x}px)"),
                MediaExpression::MaxHeight(x) => format!("(max-height: {x}px)"),
                MediaExpression::Theme(t) => match t {
                    Theme::None => "(prefers-color-scheme: none)",
                    Theme::Light => "(prefers-color-scheme: light)",
                    Theme::Dark => "(prefers-color-scheme: dark)",
                }
                .into(),
            })
            .collect::<Box<[String]>>()
            .join(" and ");
        format!("{decorator}{cond}")
    }
}
