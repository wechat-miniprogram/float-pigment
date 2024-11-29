use alloc::{string::String, vec::Vec};

use crate::typing::{FontFamilyName, FontStyleType, FontWeightType};
#[cfg(debug_assertions)]
use float_pigment_css_macro::CompatibilityEnumCheck;

/// A `@font-face`` definition.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub struct FontFace {
    pub font_family: FontFamilyName, // required
    pub src: Vec<FontSrc>,           // required
    pub font_style: Option<FontStyleType>,
    pub font_weight: Option<FontWeightType>,
    pub font_display: Option<FontDisplay>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum FontSrc {
    Local(FontFamilyName),
    Url(FontUrl),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct FontUrl {
    pub url: String,
    pub format: Option<Vec<String>>,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(CompatibilityEnumCheck))]
pub enum FontDisplay {
    Auto,
    Block,
    Swap,
    Fallback,
    Optional,
}

impl Default for FontFace {
    fn default() -> Self {
        Self {
            font_family: FontFamilyName::Serif,
            src: vec![],
            font_style: None,
            font_weight: None,
            font_display: None,
        }
    }
}

impl FontFace {
    /// Create an empty font-face definition.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the family name.
    pub fn with_font_family(&mut self, ff: FontFamilyName) -> &mut Self {
        self.font_family = ff;
        self
    }

    /// Set the `src` URL.
    pub fn with_src(&mut self, src: Vec<FontSrc>) -> &mut Self {
        self.src = src;
        self
    }

    /// Set the `font-style`.
    pub fn with_font_style(&mut self, fs: Option<FontStyleType>) -> &mut Self {
        self.font_style = fs;
        self
    }

    /// Set the `font-weight`.
    pub fn with_font_weight(&mut self, fw: Option<FontWeightType>) -> &mut Self {
        self.font_weight = fw;
        self
    }

    /// Set the `font-display`.
    pub fn with_font_display(&mut self, fd: Option<FontDisplay>) -> &mut Self {
        self.font_display = fd;
        self
    }
}
