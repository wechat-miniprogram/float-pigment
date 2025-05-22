//! CSS parser with a style sheet manager.
//!
//! Note: it is not a full web-compatible CSS parser because it supports a subset of CSS selectors and properties.
//!
//! ### Main Workflow
//!
//! The CSS parser is designed to be used in high-level UI frameworks. The main workflow:
//!
//! 1. Create a `StyleSheetGroup`.
//! 1. Parsing CSS text into a `StyleSheet` and added to the `StyleSheetGroup`.
//! 1. Create a `StyleQuery`.
//! 1. Run the `StyleQuery` with `StyleSheetGroup::query_matched_rules` and get a `MatchedRuleList`.
//! 1. Create a `NodeProperties` with `NodeProperties::new`.
//! 1. Merge the `MatchedRuleList` into `NodeProperties` with `MatchedRuleList::merge_node_properties`.
//!
//! The result `NodeProperties` contains all supported CSS properties.
//!
//! ### Supported CSS Features
//!
//! The supported selectors can be found in [StyleQuery] docs.
//!
//! The supported media features can be found in [MediaQueryStatus] docs.
//!
//! The supported style properties can be found in [SUPPORTED_CSS_PROPERTY_NAMES](crate::property::SUPPORTED_CSS_PROPERTY_NAMES) docs.
//!
//! ### The Binary Format
//!
//! The `StyleSheet` can be serialized into a specialized "bincode" format. (Note that it is not the same format as the `bincode` crate.)
//! This binary format can be deserialized with great performance, so it can be used as the cache of static style sheet text.
//! It also has compatibilities across different versions of this crate.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;

#[allow(unused_imports)]
use alloc::{boxed::Box, vec::Vec};

pub use fixed;
pub use num_traits;

#[cfg(debug_assertions)]
mod check_trait;
mod group;
mod path;
pub use group::{StyleSheetGroup, StyleSheetImportIndex, StyleSheetResource, TEMP_SHEET_INDEX};
pub mod sheet;
pub use sheet::{LinkedStyleSheet, StyleSheet};
pub mod property;
pub mod query;
mod resolve_font_size;
pub mod typing;
mod typing_stringify;
pub use query::{EnvValues, MediaQueryStatus, StyleQuery};
pub mod ffi;
pub mod length_num;
pub mod parser;

#[cfg(debug_assertions)]
use check_trait::CompatibilityCheck;

/// Serialize CSS to the JSON format.
#[cfg(all(feature = "serialize", feature = "serialize_json"))]
pub fn compile_style_sheet_to_json(filename: &str, style_text: &str) -> String {
    let (style_sheet, warnings) = parser::parse_style_sheet(filename, style_text);
    for w in warnings {
        warn!(
            "{} (at {:?}, from line {:?} column {:?} to line {:?} column {:?})",
            w.message.as_str(),
            filename,
            w.start_line,
            w.start_col,
            w.end_line,
            w.end_col,
        );
    }
    style_sheet.serialize_json()
}

/// Deserialize CSS from the JSON format.
#[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
pub fn style_sheet_from_json(json: &str) -> StyleSheetGroup {
    let mut ssg = StyleSheetGroup::new();
    let mut resource = StyleSheetResource::new();
    resource.add_json("", json);
    ssg.append_from_resource(&resource, "", None);
    ssg
}

/// Serialize CSS to the binary format.
#[cfg(feature = "serialize")]
pub fn compile_style_sheet_to_bincode(filename: &str, style_text: &str) -> Vec<u8> {
    let (style_sheet, warnings) = parser::parse_style_sheet(filename, style_text);
    for w in warnings {
        warn!(
            "{} (at {:?}, from line {:?} column {:?} to line {:?} column {:?})",
            w.message.as_str(),
            filename,
            w.start_line,
            w.start_col,
            w.end_line,
            w.end_col,
        );
    }
    style_sheet.serialize_bincode()
}

/// Deserialize CSS from the bincode format.
#[cfg(feature = "deserialize")]
pub fn style_sheet_from_bincode(bincode: Vec<u8>) -> StyleSheetGroup {
    let ptr = Box::into_raw(bincode.into_boxed_slice());
    let mut ssg = StyleSheetGroup::new();
    let mut resource = StyleSheetResource::new();
    unsafe {
        resource.add_bincode_zero_copy("", ptr, move || drop(Box::from_raw(ptr)));
    }
    ssg.append_from_resource(&resource, "", None);
    ssg
}
