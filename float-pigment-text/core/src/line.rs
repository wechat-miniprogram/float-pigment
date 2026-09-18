//! Steps 4 and 5: line breaking and justification, in abstract axis coordinates.

use std::ops::Range;

use float_pigment_css::length_num::LengthNum;

use crate::axis::AxialSize;
use crate::font::FontFace;
use crate::layout::LayoutResult;
use crate::shaped::ShapedBlock;

/// A segment of content within a line.
#[derive(Debug, Clone)]
pub(crate) enum LineSegment<L: LengthNum> {
    /// A range of glyphs from a shaped text item.
    Glyphs {
        /// The index in `ShapedBlock::texts`.
        text_index: u32,
        /// The index in `ShapedText::items`.
        item_index: u32,
        /// The glyph range in `ShapedBlock::glyphs`.
        glyph_range: Range<u32>,
    },
    /// An inline item.
    Item {
        /// The index in `ShapedBlock::items`.
        item_index: u32,
    },
    /// A fixed spacing width.
    Spacing(L),
}

/// A broken line, in abstract axis coordinates.
#[derive(Debug, Clone)]
pub(crate) struct Line<L: LengthNum> {
    /// The segments of the line, in logical order.
    pub(crate) segments: Vec<LineSegment<L>>,
    /// The size occupied by the line content.
    pub(crate) size: AxialSize<L>,
    /// Whether the line ends with a mandatory break.
    pub(crate) mandatory_break: bool,
    /// Whether the line is the last line of the block.
    pub(crate) last: bool,
    /// Whether the line ends with an ellipsis.
    pub(crate) ellipsized: bool,
}

/// Breaks the shaped block into lines for the given inline size.
#[allow(unused)]
pub(crate) fn break_lines<L: LengthNum, F: FontFace>(_shaped: &ShapedBlock<L, F>, _inline_size: L) -> Vec<Line<L>> {
    todo!()
}

/// Computes baselines, aligns segments, reorders bidi runs visually,
/// and produces the final layout result in physical coordinates.
#[allow(unused)]
pub(crate) fn justify<L: LengthNum, F: FontFace>(
    _shaped: &ShapedBlock<L, F>,
    _lines: Vec<Line<L>>,
    _inline_size: L,
) -> LayoutResult<L, F> {
    todo!()
}
