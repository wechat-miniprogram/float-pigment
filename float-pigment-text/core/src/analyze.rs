//! Lazy line break (UAX#14) and grapheme boundary (UAX#29) analysis.
//!
//! Both are consumed sequentially by line breaking and hit testing,
//! so they are iterated on demand from the flattened text
//! instead of being materialized.

use unicode_linebreak::BreakOpportunity;
use unicode_segmentation::UnicodeSegmentation;

/// The kind of a line break opportunity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BreakKind {
    /// A line break is allowed.
    Allowed,
    /// A line break is required, e.g. after a newline character.
    Mandatory,
}

/// A line break opportunity at a UTF-8 byte offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BreakPoint {
    /// The UTF-8 byte offset of the boundary in the flattened text.
    pub(crate) utf8_index: u32,
    /// The break kind.
    pub(crate) kind: BreakKind,
}

/// Iterate line break opportunities of the flattened text, in order.
pub(crate) fn break_points(text: &str) -> impl Iterator<Item = BreakPoint> + '_ {
    unicode_linebreak::linebreaks(text).map(|(utf8_index, opportunity)| BreakPoint {
        utf8_index: utf8_index as u32,
        kind: match opportunity {
            BreakOpportunity::Allowed => BreakKind::Allowed,
            BreakOpportunity::Mandatory => BreakKind::Mandatory,
        },
    })
}

/// Iterate extended grapheme cluster boundaries (UTF-8 byte offsets) of
/// the flattened text, in order. Used by hit testing and selection.
pub(crate) fn grapheme_boundaries(text: &str) -> impl Iterator<Item = u32> + '_ {
    text.grapheme_indices(true).map(|(index, _)| index as u32)
}

/// Iterate word boundaries (UTF-8 byte offsets) of the flattened text,
/// in order. Used by justification for word spacing and stretching.
pub(crate) fn word_boundaries(text: &str) -> impl Iterator<Item = u32> + '_ {
    text.split_word_bound_indices()
        .map(|(index, _)| index as u32)
}
