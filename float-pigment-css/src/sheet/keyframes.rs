use alloc::{borrow::ToOwned, string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use super::PropertyMeta;

#[cfg(debug_assertions)]
use float_pigment_css_macro::CompatibilityEnumCheck;

/// A `@keyframes`` definition.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct KeyFrames {
    pub ident: String,
    pub keyframes: Vec<KeyFrameRule>,
}

impl KeyFrames {
    pub(crate) fn new(ident: String, keyframes: Vec<KeyFrameRule>) -> Self {
        Self { ident, keyframes }
    }
}

impl core::fmt::Display for KeyFrames {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "@keyframes {} {{ ", self.ident)?;
        for keyframe in &self.keyframes {
            write!(f, "{} ", keyframe)?;
        }
        write!(f, "}}")
    }
}

/// The percentage field in a keyframe item.
#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum KeyFrame {
    /// `from` in a keyframe.
    From,
    /// `to` in a keyframe.
    To,
    /// Percentage value in a keyframe.
    Ratio(f32),
}

impl KeyFrame {
    /// Get the normalized ratio (between `0.` and `1.`).
    pub fn ratio(&self) -> f32 {
        match self {
            Self::From => 0.,
            Self::To => 1.,
            Self::Ratio(x) => *x,
        }
    }
}

/// A keyframe item in `@keyframes`, e.g. `50% { ... }`.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct KeyFrameRule {
    pub keyframe: Vec<KeyFrame>,
    pub properties: Vec<PropertyMeta>,
}

impl KeyFrameRule {
    pub(crate) fn new(keyframe: Vec<KeyFrame>, properties: Vec<PropertyMeta>) -> Self {
        Self {
            keyframe,
            properties,
        }
    }
}

impl core::fmt::Display for KeyFrameRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {{ ",
            self.keyframe
                .iter()
                .map(|x| {
                    match x {
                        KeyFrame::From => "from".to_owned(),
                        KeyFrame::To => "to".to_owned(),
                        KeyFrame::Ratio(ratio) => format!("{:.2}%", ratio * 100.),
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        for prop in &self.properties {
            write!(
                f,
                "{}: {}; ",
                prop.get_property_name(),
                prop.get_property_value_string()
            )?;
        }
        write!(f, "}}")
    }
}
