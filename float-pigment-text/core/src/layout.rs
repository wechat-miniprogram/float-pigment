//! Stage B result: the final layout with physical coordinates.

use std::ops::Range;
use std::sync::Arc;

use float_pigment_css::length_num::LengthNum;

use crate::font::FontFace;
use crate::geometry::{Point, Rect, Size};
use crate::shaped::ShapedBlock;
use crate::shaper::ShapedGlyph;

pub(crate) const EXTRA_GLYPH_TAG: u32 = 1 << 31;

/// A glyph with its final position.
///
/// Compact storage: 12 bytes when `L` is `f32`.
/// All other glyph properties (advance, cluster, face, markers) are shared
/// from the shaped block, resolved via `LayoutResult::shaped_glyph`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionedGlyph<L: LengthNum> {
    /// The glyph origin (baseline start point, shaper offset applied),
    /// relative to the block origin.
    pub origin: Point<L>,
    /// The index into `ShapedBlock::glyphs`, or into the extra glyphs of
    /// `LayoutResult` when the highest bit is set (for glyphs created
    /// during layout, e.g. ellipsis).
    pub glyph_ref: u32,
}

/// A laid out line.
#[derive(Debug, Clone, PartialEq)]
pub struct LineLayout<L: LengthNum> {
    /// The baseline offset from the block origin, in the block direction.
    pub baseline_offset: L,
    /// The distance above the baseline occupied by the text.
    pub text_ascent: L,
    /// The distance below the baseline occupied by the text.
    pub text_descent: L,
    /// The start offset of the line content along the baseline.
    pub inline_start: L,
    /// The end offset of the line content along the baseline.
    pub inline_end: L,
    /// The range of glyphs in `LayoutResult::glyphs`.
    pub glyph_range: Range<u32>,
    /// The range of items in `LayoutResult::items`.
    pub item_range: Range<u32>,
}

/// A positioned inline item.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionedItemRect<L: LengthNum> {
    /// The item rect relative to the block origin.
    pub rect: Rect<L>,
    /// The index of the source `Inline::Item` in the block.
    pub item_index: u32,
}

/// The affinity of a caret position, used when a UTF-8 offset sits on a
/// bidi boundary or a line break.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaretAffinity {
    /// The caret belongs to the leading (upstream) side.
    Upstream,
    /// The caret belongs to the trailing (downstream) side.
    Downstream,
}

/// A caret position in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextPosition {
    /// The index of the source `Inline::Text` in the block.
    pub inline_index: u32,
    /// The UTF-8 byte offset in the source inline text.
    pub utf8_index: u32,
    /// The caret affinity.
    pub affinity: CaretAffinity,
}

/// The final layout result of a block, with physical coordinates.
///
/// Holds a shared reference of the shaped block:
/// glyph properties other than the origin are resolved from it.
#[derive(Debug, Clone)]
pub struct LayoutResult<L: LengthNum, F: FontFace> {
    /// The total size occupied by the content.
    pub size: Size<L>,
    /// The shaped block this layout is computed from.
    pub shaped: Arc<ShapedBlock<L, F>>,
    /// The laid out lines.
    pub lines: Box<[LineLayout<L>]>,
    /// The positioned glyphs, in visual order within each line.
    pub glyphs: Box<[PositionedGlyph<L>]>,
    /// The positioned inline items.
    pub items: Box<[PositionedItemRect<L>]>,
    /// Glyphs created during layout (e.g. ellipsis),
    /// indexed by `PositionedGlyph::glyph_ref` with `EXTRA_GLYPH_TAG`.
    pub(crate) extra_glyphs: Box<[ShapedGlyph]>,
}

impl<L: LengthNum, F: FontFace> LayoutResult<L, F> {
    /// Resolve a `PositionedGlyph::glyph_ref` to the shared glyph data.
    pub fn shaped_glyph(&self, _glyph_ref: u32) -> &ShapedGlyph {
        todo!()
    }

    /// Map a flattened UTF-8 offset back to the source position,
    /// i.e. the `Inline::Text` index and the UTF-8 offset within it.
    pub fn source_position(&self, _flattened_offset: u32) -> Option<(u32, u32)> {
        todo!()
    }

    /// Find the caret position nearest to the given physical point.
    pub fn hit_test(&self, _point: Point<L>) -> Option<TextPosition> {
        todo!()
    }

    /// Compute the bounding rect of a UTF-8 byte range in one inline text,
    /// e.g. for selection.
    pub fn rect_for_range(&self, _inline_index: u32, _utf8_range: Range<u32>) -> Vec<Rect<L>> {
        todo!()
    }
}
