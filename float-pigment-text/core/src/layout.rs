//! Stage B result: the final layout with physical coordinates.

use std::ops::Range;
use std::sync::Arc;

use float_pigment_css::length_num::{length_sum, LengthNum};
use float_pigment_css::typing::WritingMode;

use crate::font::FontFace;
use crate::line::glyph_inline_advance;
use crate::shaped::ShapedBlock;
use crate::shaper::{ResolvedDirection, ShapedGlyph};

pub use crate::geometry::{Point, Rect, Size};

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
    /// The offset of the first glyph in `LayoutResult::glyphs`; the end
    /// is the start of the next line (see `LayoutResult::line_glyph_range`).
    pub glyph_start: u32,
    /// The offset of the first item in `LayoutResult::items`; the end is
    /// the start of the next line (see `LayoutResult::line_item_range`).
    pub item_start: u32,
    /// The flattened text offset where the line content starts (see
    /// `ShapedBlock::flattened`), excluding trimmed whitespace.
    /// For an empty line between newlines, this is the position of the
    /// newline that created the line.
    pub text_start: u32,
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
    /// The positioned inline items, in visual order within each line.
    pub items: Box<[PositionedItemRect<L>]>,
    /// Glyphs created during layout (e.g. ellipsis),
    /// indexed by `PositionedGlyph::glyph_ref` with `EXTRA_GLYPH_TAG`.
    pub(crate) extra_glyphs: Box<[ShapedGlyph]>,
}

impl<L: LengthNum, F: FontFace> LayoutResult<L, F> {
    /// The range of a line's glyphs in `LayoutResult::glyphs`: from the
    /// line start to the start of the next line (or the array end).
    pub fn line_glyph_range(&self, line_index: usize) -> Range<u32> {
        let start = self.lines[line_index].glyph_start;
        let end = self
            .lines
            .get(line_index + 1)
            .map(|line| line.glyph_start)
            .unwrap_or(self.glyphs.len() as u32);
        start..end
    }

    /// The range of a line's items in `LayoutResult::items`: from the
    /// line start to the start of the next line (or the array end).
    pub fn line_item_range(&self, line_index: usize) -> Range<u32> {
        let start = self.lines[line_index].item_start;
        let end = self
            .lines
            .get(line_index + 1)
            .map(|line| line.item_start)
            .unwrap_or(self.items.len() as u32);
        start..end
    }

    /// Resolve a `PositionedGlyph::glyph_ref` to the shared glyph data.
    pub fn shaped_glyph(&self, glyph_ref: u32) -> &ShapedGlyph {
        if glyph_ref & EXTRA_GLYPH_TAG != 0 {
            &self.extra_glyphs[(glyph_ref & !EXTRA_GLYPH_TAG) as usize]
        } else {
            &self.shaped.glyphs()[glyph_ref as usize]
        }
    }

    /// Map a flattened UTF-8 offset back to the source position,
    /// i.e. the `Inline::Text` index and the UTF-8 offset within it.
    ///
    /// An offset equal to the end of the whole text maps to the trailing
    /// caret of the last inline text; an offset inside a U+FFFC inline item
    /// placeholder returns `None`.
    pub fn source_position(&self, flattened_offset: u32) -> Option<(u32, u32)> {
        let texts = &self.shaped.texts;
        let count = texts.partition_point(|text| text.flattened_range.start <= flattened_offset);
        let index = count.checked_sub(1)?;
        let text = texts.get(index)?;
        (flattened_offset <= text.flattened_range.end)
            .then(|| (index as u32, flattened_offset - text.flattened_range.start))
    }

    /// Convert a physical point to abstract axis coordinates `(inline, block)`.
    fn axis_point(&self, point: Point<L>) -> (L, L) {
        let writing_mode = &self.shaped.property.writing_mode;
        if matches!(writing_mode, WritingMode::HorizontalTb) {
            (point.x, point.y)
        } else {
            // vertical-rl lines stack right-to-left: the block axis was
            // remapped from the right edge of the content box
            let block = if matches!(writing_mode, WritingMode::VerticalRl) {
                self.size.width - point.x
            } else {
                point.x
            };
            (point.y, block)
        }
    }

    /// Convert abstract axis ranges to a physical rect.
    fn axis_rect(&self, inline_start: L, inline_end: L, block_start: L, block_end: L) -> Rect<L> {
        let writing_mode = &self.shaped.property.writing_mode;
        if matches!(writing_mode, WritingMode::HorizontalTb) {
            Rect::new(
                Point::new(inline_start, block_start),
                Size::new(inline_end - inline_start, block_end - block_start),
            )
        } else {
            let x = if matches!(writing_mode, WritingMode::VerticalRl) {
                self.size.width - block_end
            } else {
                block_start
            };
            Rect::new(
                Point::new(x, inline_start),
                Size::new(block_end - block_start, inline_end - inline_start),
            )
        }
    }

    /// The inline coordinate of the pen start of a positioned glyph,
    /// i.e. its origin with the shaper offset removed.
    fn pen_inline(&self, glyph: &PositionedGlyph<L>) -> L {
        let sg = self.shaped_glyph(glyph.glyph_ref);
        let horizontal = matches!(self.shaped.property.writing_mode, WritingMode::HorizontalTb);
        if horizontal {
            glyph.origin.x - L::from_f32(sg.offset.x)
        } else if sg.flags.sideways() {
            glyph.origin.y - L::from_f32(sg.offset.x)
        } else {
            glyph.origin.y - L::from_f32(sg.offset.y)
        }
    }

    /// The flattened text range and direction of the cluster containing the
    /// given glyph (an index into `ShapedBlock::glyphs`).
    fn cluster_info(&self, glyph_index: u32) -> Option<(Range<u32>, ResolvedDirection)> {
        let cluster = self.shaped.glyphs[glyph_index as usize].cluster;
        let (text_index, item_index, item) = self.shaped.item_at_glyph(glyph_index)?;
        let end = self
            .shaped
            .item_glyph_range(text_index, item_index)
            .filter_map(|i| {
                let c = self.shaped.glyphs[i as usize].cluster;
                (c > cluster).then_some(c)
            })
            .min()
            .unwrap_or_else(|| self.shaped.item_flattened_end(text_index, item_index));
        let direction = if item.bidi_level % 2 == 1 {
            ResolvedDirection::RTL
        } else {
            ResolvedDirection::LTR
        };
        Some((cluster..end, direction))
    }

    /// The line extent in axis block coordinates, where line extents always
    /// ascend in line order (the vertical-rl physical remap is undone).
    fn line_block_extent(&self, line: &LineLayout<L>) -> (L, L) {
        let baseline = line.baseline_offset;
        if matches!(self.shaped.property.writing_mode, WritingMode::VerticalRl) {
            // undo the reflection: the axis baseline is `w - baseline`
            let w = self.size.width;
            (
                w - baseline - line.text_ascent,
                w - baseline - line.text_descent,
            )
        } else {
            (baseline - line.text_ascent, baseline - line.text_descent)
        }
    }

    /// The caret position for a line without caret units (e.g. an empty
    /// line between newlines): the first text position at or after the
    /// line start, skipping inline item placeholders.
    fn caret_position_for_line(&self, line_index: usize) -> Option<TextPosition> {
        let mut offset = self.lines[line_index].text_start;
        let limit = self.shaped.flattened.len() as u32;
        loop {
            if let Some((inline_index, utf8_index)) = self.source_position(offset) {
                return Some(TextPosition {
                    inline_index,
                    utf8_index,
                    affinity: CaretAffinity::Upstream,
                });
            }
            if offset >= limit {
                return None;
            }
            offset += 1;
        }
    }

    /// Whether the line can hold a caret: it has caret units, or it is an
    /// empty line with a resolvable text position.
    fn line_is_caret_addressable(&self, line_index: usize) -> bool {
        self.line_glyph_range(line_index)
            .any(|g| self.glyphs[g as usize].glyph_ref & EXTRA_GLYPH_TAG == 0)
            || self.caret_position_for_line(line_index).is_some()
    }

    /// Select the nearest caret-addressable line by the axis block
    /// coordinate.
    fn select_line(&self, block_pos: L) -> Option<usize> {
        let distance = |line: &LineLayout<L>| {
            let (start, end) = self.line_block_extent(line);
            if block_pos < start {
                start - block_pos
            } else if block_pos > end {
                block_pos - end
            } else {
                L::zero()
            }
        };
        let idx = self
            .lines
            .partition_point(|line| self.line_block_extent(line).1 < block_pos);
        let after = (idx..self.lines.len()).find(|&i| self.line_is_caret_addressable(i));
        let before = (0..idx).rev().find(|&i| self.line_is_caret_addressable(i));
        match (before, after) {
            (None, None) => None,
            (Some(b), None) => Some(b),
            (None, Some(a)) => Some(a),
            (Some(b), Some(a)) => {
                if distance(&self.lines[a]) < distance(&self.lines[b]) {
                    Some(a)
                } else {
                    Some(b)
                }
            }
        }
    }

    /// The caret units of a line: consecutive glyphs sharing a cluster are
    /// one unit. Yields `(pen start, pen end, first glyph index)` per unit,
    /// in visual order. The pen end of a unit is the pen start of the next
    /// unit, so spacing (letter/word/justify) belongs to the preceding unit.
    fn line_groups(&self, line_index: usize) -> impl Iterator<Item = (L, L, u32)> + '_ {
        let mut iter = self.line_glyph_range(line_index);
        // (pen, advance sum, cluster, first glyph index) of the pending unit
        let mut pending: Option<(L, L, u32, u32)> = None;
        let mut done = false;
        std::iter::from_fn(move || {
            loop {
                if done {
                    return None;
                }
                let Some(i) = iter.next() else {
                    done = true;
                    return pending.take().map(|(pen, adv, _, g)| (pen, pen + adv, g));
                };
                let pg = &self.glyphs[i as usize];
                if pg.glyph_ref & EXTRA_GLYPH_TAG != 0 {
                    // extra glyphs (e.g. ellipsis) are appended at the line
                    // end and are not caret units
                    done = true;
                    return pending.take().map(|(pen, adv, _, g)| (pen, pen + adv, g));
                }
                let sg = &self.shaped.glyphs[pg.glyph_ref as usize];
                let advance = glyph_inline_advance(sg);
                if matches!(&pending, Some((_, _, c, _)) if *c == sg.cluster) {
                    if let Some(p) = &mut pending {
                        p.1 += advance;
                    }
                    continue;
                }
                let pen = self.pen_inline(pg);
                let new = (pen, advance, sg.cluster, pg.glyph_ref);
                if let Some((prev_pen, _, _, prev_glyph)) = pending.replace(new) {
                    return Some((prev_pen, pen, prev_glyph));
                }
            }
        })
    }

    /// Find the caret position nearest to the given physical point.
    ///
    /// The caret granularity is the shaped cluster (e.g. a ligature is hit
    /// as a whole). Returns the text start when the block contains no
    /// glyphs, and `None` when the block contains no inline text at all.
    pub fn hit_test(&self, point: Point<L>) -> Option<TextPosition> {
        let (inline_pos, block_pos) = self.axis_point(point);
        let Some(line_index) = self.select_line(block_pos) else {
            return self.shaped.texts.first().map(|_| TextPosition {
                inline_index: 0,
                utf8_index: 0,
                affinity: CaretAffinity::Upstream,
            });
        };
        let glyph_range = self.line_glyph_range(line_index);
        if !glyph_range
            .clone()
            .any(|g| self.glyphs[g as usize].glyph_ref & EXTRA_GLYPH_TAG == 0)
        {
            // a line without caret units, e.g. an empty line between
            // newlines: the caret goes to its text position
            return self.caret_position_for_line(line_index).or_else(|| {
                self.shaped.texts.first().map(|_| TextPosition {
                    inline_index: 0,
                    utf8_index: 0,
                    affinity: CaretAffinity::Upstream,
                })
            });
        }
        let glyphs = &self.glyphs[glyph_range.start as usize..glyph_range.end as usize];
        // pens are non-decreasing in visual order: binary search the last
        // glyph whose pen starts at or before the point, skipping extra
        // glyphs (e.g. ellipsis) appended at the line end
        let mut i = glyphs.partition_point(|pg| self.pen_inline(pg) <= inline_pos);
        let hit = loop {
            if i == 0 {
                break glyphs
                    .iter()
                    .position(|pg| pg.glyph_ref & EXTRA_GLYPH_TAG == 0)?;
            }
            i -= 1;
            if glyphs[i].glyph_ref & EXTRA_GLYPH_TAG == 0 {
                break i;
            }
        };
        // expand to the caret unit: glyphs sharing the cluster
        let cluster = self.shaped.glyphs[glyphs[hit].glyph_ref as usize].cluster;
        let same_cluster = |pg: &PositionedGlyph<L>| {
            pg.glyph_ref & EXTRA_GLYPH_TAG == 0
                && self.shaped.glyphs[pg.glyph_ref as usize].cluster == cluster
        };
        let mut first = hit;
        while first > 0 && same_cluster(&glyphs[first - 1]) {
            first -= 1;
        }
        let mut last = hit;
        while last + 1 < glyphs.len() && same_cluster(&glyphs[last + 1]) {
            last += 1;
        }
        let pen_start = self.pen_inline(&glyphs[first]);
        let next = (last + 1..glyphs.len())
            .find(|&i| glyphs[i].glyph_ref & EXTRA_GLYPH_TAG == 0)
            .map(|i| self.pen_inline(&glyphs[i]));
        let pen_end =
            next.unwrap_or_else(|| {
                pen_start
                    + length_sum(
                        glyphs[first..=last]
                            .iter()
                            .map(|pg| glyph_inline_advance(self.shaped_glyph(pg.glyph_ref))),
                    )
            });
        let leading = inline_pos < (pen_start + pen_end).div_i32(2);
        let (range, direction) = self.cluster_info(glyphs[hit].glyph_ref)?;
        // the visual leading half of an RTL cluster is its logical end
        let at_start = match direction {
            ResolvedDirection::LTR => leading,
            ResolvedDirection::RTL => !leading,
        };
        let offset = if at_start { range.start } else { range.end };
        let (inline_index, utf8_index) = self.source_position(offset)?;
        Some(TextPosition {
            inline_index,
            utf8_index,
            affinity: if at_start {
                CaretAffinity::Upstream
            } else {
                CaretAffinity::Downstream
            },
        })
    }

    /// Compute the bounding rects of a UTF-8 byte range in one inline text,
    /// e.g. for selection. Visually discontinuous parts (e.g. across a bidi
    /// boundary) produce multiple rects. An empty or out-of-range range
    /// produces no rects.
    pub fn rect_for_range(&self, inline_index: u32, utf8_range: Range<u32>) -> Vec<Rect<L>> {
        let mut rects = Vec::new();
        let Some(text) = self.shaped.texts.get(inline_index as usize) else {
            return rects;
        };
        let len = text.flattened_range.end - text.flattened_range.start;
        let start = utf8_range.start.min(len);
        let end = utf8_range.end.min(len);
        if start >= end {
            return rects;
        }
        let flattened_start = text.flattened_range.start + start;
        let flattened_end = text.flattened_range.start + end;
        for line_index in 0..self.lines.len() {
            let (block_start, block_end) = self.line_block_extent(&self.lines[line_index]);
            let mut cur: Option<(L, L)> = None;
            for (pen, end, glyph_index) in self.line_groups(line_index) {
                let cluster = self.shaped.glyphs[glyph_index as usize].cluster;
                let selected = cluster >= flattened_start && cluster < flattened_end;
                match (cur, selected) {
                    (Some((s, _)), true) => cur = Some((s, end)),
                    (Some((s, e)), false) => {
                        rects.push(self.axis_rect(s, e, block_start, block_end));
                        cur = None;
                    }
                    (None, true) => cur = Some((pen, end)),
                    (None, false) => {}
                }
            }
            if let Some((s, e)) = cur {
                rects.push(self.axis_rect(s, e, block_start, block_end));
            }
        }
        rects
    }
}
