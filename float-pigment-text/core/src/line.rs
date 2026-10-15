//! Steps 4 and 5: line breaking and justification, in abstract axis coordinates.

use float_pigment_css::length_num::{length_sum, LengthNum};
use float_pigment_css::typing::{Overflow, TextAlign, WhiteSpace, WordBreak, WritingMode};

use crate::model::TextOverflow;

use crate::analyze::{break_points, grapheme_boundaries, word_boundaries, BreakKind};
use crate::font::FontFace;
use crate::geometry::{Point, Rect, Size};
use crate::layout::{
    LayoutResult, LineLayout, PositionedGlyph, PositionedItemRect, EXTRA_GLYPH_TAG,
};
use crate::model::VerticalAlign;
use crate::shaped::ShapedBlock;
use crate::shaper::{ResolvedDirection, ShapedGlyph};

/// The advance of a glyph along the inline axis.
pub(crate) fn glyph_inline_advance<L: LengthNum>(glyph: &ShapedGlyph) -> L {
    L::from_f32(glyph.advance)
}

/// The content of a line atom, packed into a single u32: the top two
/// bits are the kind tag, the rest is the payload (an index or a
/// flattened text offset).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AtomContent(u32);

impl AtomContent {
    const TAG_MASK: u32 = 3 << 30;
    const TAG_GLYPH: u32 = 0;
    const TAG_ITEM: u32 = 1 << 30;
    const TAG_SPACING: u32 = 2 << 30;
    const TAG_FORCED_BREAK: u32 = 3 << 30;

    /// A glyph, indexing `ShapedBlock::glyphs`.
    fn glyph(index: u32) -> Self {
        debug_assert!(index & Self::TAG_MASK == 0);
        Self(index)
    }
    /// An inline item, indexing `ShapedBlock::items`.
    fn item(index: u32) -> Self {
        debug_assert!(index & Self::TAG_MASK == 0);
        Self(Self::TAG_ITEM | index)
    }
    /// A fixed spacing.
    const SPACING: Self = Self(Self::TAG_SPACING);
    /// A forced line break marker, holding the flattened text offset of
    /// the newline character.
    fn forced_break(offset: u32) -> Self {
        debug_assert!(offset & Self::TAG_MASK == 0);
        Self(Self::TAG_FORCED_BREAK | offset)
    }

    fn glyph_index(self) -> Option<u32> {
        (self.0 & Self::TAG_MASK == Self::TAG_GLYPH).then_some(self.0)
    }
    fn item_index(self) -> Option<u32> {
        (self.0 & Self::TAG_MASK == Self::TAG_ITEM).then_some(self.0 & !Self::TAG_MASK)
    }
    fn forced_break_offset(self) -> Option<u32> {
        (self.0 & Self::TAG_MASK == Self::TAG_FORCED_BREAK).then_some(self.0 & !Self::TAG_MASK)
    }
}

const ATOM_BREAK_BEFORE: u8 = 1 << 0;
const ATOM_EMERGENCY_BREAKABLE: u8 = 1 << 1;
const ATOM_WORD_START: u8 = 1 << 2;
const ATOM_TRIMMABLE: u8 = 1 << 3;

fn atom_flags(break_before: bool, emergency_breakable: bool, word_start: bool, trimmable: bool) -> u8 {
    (break_before as u8 * ATOM_BREAK_BEFORE)
        | (emergency_breakable as u8 * ATOM_EMERGENCY_BREAKABLE)
        | (word_start as u8 * ATOM_WORD_START)
        | (trimmable as u8 * ATOM_TRIMMABLE)
}

/// A line-breakable unit, in abstract axis coordinates.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Atom<L: LengthNum> {
    pub(crate) content: AtomContent,
    /// The inline advance, including letter/word spacing for glyphs.
    pub(crate) advance: L,
    /// The bidi level of the owning item.
    pub(crate) bidi_level: u8,
    /// The `ATOM_*` markers.
    pub(crate) flags: u8,
}

impl<L: LengthNum> Atom<L> {
    /// Whether a line break is allowed before this atom.
    fn break_before(&self) -> bool {
        self.flags & ATOM_BREAK_BEFORE != 0
    }
    /// Whether an emergency break is allowed before this atom when no
    /// legal break point exists (word-break / overflow-wrap semantics,
    /// suppressed when wrapping is disabled).
    fn emergency_breakable(&self) -> bool {
        self.flags & ATOM_EMERGENCY_BREAKABLE != 0
    }
    /// Whether this atom is a word start, i.e. a justification stretch point.
    fn word_start(&self) -> bool {
        self.flags & ATOM_WORD_START != 0
    }
    /// Whether this atom is collapsible whitespace, trimmed at line edges.
    fn trimmable(&self) -> bool {
        self.flags & ATOM_TRIMMABLE != 0
    }
}

/// A broken line, in abstract axis coordinates.
#[derive(Debug, Clone)]
pub(crate) struct Line<L: LengthNum> {
    /// The atoms of the line, in logical order.
    pub(crate) atoms: Vec<Atom<L>>,
    /// The inline size occupied by the line content, after trimming.
    pub(crate) width: L,
    /// Whether the line ends with a mandatory break.
    pub(crate) mandatory_break: bool,
    /// Whether the line is truncated with an ellipsis.
    pub(crate) ellipsized: bool,
    /// The flattened text offset where the line content starts,
    /// excluding trimmed whitespace. For an empty line between newlines,
    /// this is the position of the newline that created the line.
    pub(crate) text_start: u32,
}

fn is_forced_break_char(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// Compute the flattened text offset where the line content starts.
/// `fallback` is used when the line has no content atoms, i.e. an empty
/// line created by a forced break.
fn line_text_start<L: LengthNum, F: FontFace>(
    shaped: &ShapedBlock<L, F>,
    atoms: &[Atom<L>],
    fallback: Option<u32>,
) -> u32 {
    for atom in atoms {
        let cluster = atom
            .content
            .glyph_index()
            .map(|g| shaped.glyphs[g as usize].cluster)
            .or_else(|| {
                atom.content
                    .item_index()
                    .map(|i| shaped.items[i as usize].flattened_offset)
            });
        if let Some(c) = cluster {
            return c;
        }
    }
    fallback.unwrap_or(0)
}

struct AtomBuilder<'a, L: LengthNum, F: FontFace> {
    shaped: &'a ShapedBlock<L, F>,
    vertical_layout: bool,
    atoms: Vec<Atom<L>>,
}

impl<'a, L: LengthNum, F: FontFace> AtomBuilder<'a, L, F> {
    fn spacing(&mut self, width: L, bidi_level: u8) {
        if width > L::zero() {
            self.atoms.push(Atom {
                content: AtomContent::SPACING,
                advance: width,
                bidi_level,
                flags: 0,
            });
        }
    }

    fn glyph_inline_advance(&self, glyph_index: u32) -> L {
        glyph_inline_advance(&self.shaped.glyphs[glyph_index as usize])
    }

    fn text(&mut self, text_index: u32) {
        let text = &self.shaped.texts[text_index as usize];
        let property = &text.property;
        let collapse = matches!(
            property.white_space,
            WhiteSpace::Normal | WhiteSpace::PreLine
        );
        let newline_breaks = !matches!(
            property.white_space,
            WhiteSpace::Normal | WhiteSpace::NoWrap
        );
        let no_wrap = property.white_space == WhiteSpace::NoWrap;
        let flattened = &self.shaped.flattened;
        let range = text.flattened_range.clone();

        // merge iterators over break opportunities and word boundaries
        let mut breaks = break_points(flattened)
            .skip_while(|p| p.utf8_index < range.start)
            .peekable();
        let mut graphemes = (property.word_break == WordBreak::BreakAll).then(|| {
            grapheme_boundaries(flattened)
                .skip_while(|p| *p < range.start)
                .peekable()
        });
        let mut words = word_boundaries(flattened)
            .skip_while(|p| *p < range.start)
            .peekable();

        self.spacing(
            text.spacing.inline_start,
            text.items.first().map(|i| i.bidi_level).unwrap_or(0),
        );
        let mut prev_whitespace = false;
        let mut prev_cluster: Option<u32> = None;
        let mut prev_was_cr = false;
        let mut first_glyph = true;
        for (item_index, item) in text.items.iter().enumerate() {
            let level = item.bidi_level;
            // line breaking operates in logical order: glyphs of items
            // shaped in visual order (horizontal RTL) are iterated reversed
            let glyph_range = self.shaped.item_glyph_range(text_index as usize, item_index);
            let glyph_count = glyph_range.end - glyph_range.start;
            for k in 0..glyph_count {
                let glyph_index = if item.visual_order {
                    glyph_range.end - 1 - k
                } else {
                    glyph_range.start + k
                };
                let glyph = &self.shaped.glyphs[glyph_index as usize];
                let ch = flattened[glyph.cluster as usize..].chars().next();
                if newline_breaks && ch.map(is_forced_break_char).unwrap_or(false) {
                    // each newline produces its own break marker (empty lines
                    // between consecutive newlines are preserved), except
                    // that "\r\n" is a single line break (CSS newline
                    // normalization)
                    let c = ch.unwrap_or('\n');
                    if !(c == '\n' && prev_was_cr) {
                        self.atoms.push(Atom {
                            content: AtomContent::forced_break(glyph.cluster),
                            advance: L::zero(),
                            bidi_level: level,
                            flags: 0,
                        });
                    }
                    prev_was_cr = c == '\r';
                    prev_whitespace = true;
                    continue;
                }
                prev_was_cr = false;
                let whitespace = glyph.flags.whitespace();
                if collapse && whitespace && prev_whitespace {
                    continue;
                }
                prev_whitespace = whitespace;

                // line break opportunity at the cluster start
                let cluster = glyph.cluster;
                let mut allowed_before = false;
                while let Some(point) = breaks.peek() {
                    if point.utf8_index >= cluster {
                        allowed_before =
                            point.utf8_index == cluster && point.kind == BreakKind::Allowed;
                        break;
                    }
                    breaks.next();
                }
                // never break inside a shaped cluster (e.g. a ligature)
                let same_cluster = prev_cluster == Some(cluster);
                prev_cluster = Some(cluster);
                let mut break_before = allowed_before && !same_cluster;
                if let Some(graphemes) = graphemes.as_mut() {
                    while let Some(boundary) = graphemes.peek() {
                        if *boundary >= cluster {
                            if *boundary == cluster {
                                break_before = !same_cluster;
                            }
                            break;
                        }
                        graphemes.next();
                    }
                }
                match property.word_break {
                    WordBreak::KeepAll => {
                        let prev_char = flattened[..cluster as usize].chars().next_back();
                        break_before = break_before
                            && prev_char
                                .map(|c| c.is_whitespace() || c == '-')
                                .unwrap_or(false);
                    }
                    _ => {}
                }
                if no_wrap {
                    break_before = false;
                }

                let word_start = {
                    let mut is_start = false;
                    while let Some(boundary) = words.peek() {
                        if *boundary >= cluster {
                            is_start = *boundary == cluster;
                            break;
                        }
                        words.next();
                    }
                    is_start && !first_glyph
                };

                let mut advance = self.glyph_inline_advance(glyph_index);
                if !first_glyph {
                    advance += property.letter_spacing;
                    if word_start {
                        advance += property.word_spacing;
                    }
                }
                first_glyph = false;
                self.atoms.push(Atom {
                    content: AtomContent::glyph(glyph_index),
                    advance,
                    bidi_level: level,
                    flags: atom_flags(
                        break_before,
                        property.word_break == WordBreak::BreakWord && !no_wrap,
                        word_start,
                        collapse && whitespace,
                    ),
                });
            }
        }
        self.spacing(
            text.spacing.inline_end,
            text.items.last().map(|i| i.bidi_level).unwrap_or(0),
        );
    }

    fn item(&mut self, item_index: u32) {
        let item = &self.shaped.items[item_index as usize];
        let content = &item.item;
        let mut break_before = {
            let mut allowed = false;
            for point in break_points(&self.shaped.flattened) {
                if point.utf8_index >= item.flattened_offset {
                    allowed = point.utf8_index == item.flattened_offset
                        && point.kind == BreakKind::Allowed;
                    break;
                }
            }
            allowed
        };
        if content.no_wrap {
            break_before = false;
        }
        self.spacing(content.spacing.inline_start, item.bidi_level);
        let advance = if self.vertical_layout {
            content.size.height
        } else {
            content.size.width
        };
        self.atoms.push(Atom {
            content: AtomContent::item(item_index),
            advance,
            bidi_level: item.bidi_level,
            flags: atom_flags(break_before, false, false, false),
        });
        self.spacing(content.spacing.inline_end, item.bidi_level);
    }
}

/// Build the atom list of the block, in logical (text) order.
fn build_atoms<L: LengthNum, F: FontFace>(shaped: &ShapedBlock<L, F>) -> Vec<Atom<L>> {
    let vertical_layout = !matches!(shaped.property.writing_mode, WritingMode::HorizontalTb);
    let mut builder = AtomBuilder {
        shaped,
        vertical_layout,
        atoms: Vec::with_capacity(shaped.glyphs.len() + shaped.items.len()),
    };
    builder.spacing(shaped.property.text_indent, 0);

    // merge texts and items in flattened text order
    let mut text_index = 0u32;
    let mut item_index = 0u32;
    loop {
        let text_pos = shaped
            .texts
            .get(text_index as usize)
            .map(|t| t.flattened_range.start);
        let item_pos = shaped
            .items
            .get(item_index as usize)
            .map(|i| i.flattened_offset);
        match (text_pos, item_pos) {
            (Some(t), Some(i)) => {
                if t <= i {
                    builder.text(text_index);
                    text_index += 1;
                } else {
                    builder.item(item_index);
                    item_index += 1;
                }
            }
            (Some(_), None) => {
                builder.text(text_index);
                text_index += 1;
            }
            (None, Some(_)) => {
                builder.item(item_index);
                item_index += 1;
            }
            (None, None) => break,
        }
    }
    builder.atoms
}

/// Breaks the shaped block into lines for the given inline size.
#[allow(unused_assignments)] // flush! macro resets state after the final flush
pub(crate) fn break_lines<L: LengthNum, F: FontFace>(
    shaped: &ShapedBlock<L, F>,
    inline_size: L,
) -> Vec<Line<L>> {
    let atoms = build_atoms(shaped);
    let ellipsis = {
        let clamped = shaped.property.line_clamp.is_some();
        let overflow_ellipsis = shaped.property.overflow_inline == Overflow::Hidden
            && !matches!(shaped.property.text_overflow, TextOverflow::Clip);
        if clamped || overflow_ellipsis {
            let line_index = shaped
                .property
                .line_clamp
                .map(|c| c.get() as usize - 1)
                .unwrap_or(0);
            let ellipsis_advance = shaped
                .ellipsis_glyphs
                .as_ref()
                .map(|glyphs| length_sum(glyphs.iter().map(glyph_inline_advance)))
                .unwrap_or(L::zero());
            Some((line_index, ellipsis_advance))
        } else {
            None
        }
    };

    let mut lines: Vec<Line<L>> = Vec::new();
    let mut cur: Vec<Atom<L>> = Vec::new();
    // total width of cur, and the trailing trimmable width
    let mut cur_width = L::zero();
    let mut trailing_trim = L::zero();
    // the last legal break point: atom index in cur, plus width snapshots
    let mut last_break: Option<(usize, L, L)> = None;
    // the start position of the empty line created by a forced break
    let mut forced_break_pos: Option<u32> = None;
    // set when the last processed atom is a forced break: a trailing
    // newline creates an empty last line
    let mut trailing_break: Option<u32> = None;

    fn content_width<L: LengthNum>(total: L, trim: L) -> L {
        total - trim
    }

    macro_rules! flush {
        ($mandatory:expr, $ellipsized:expr) => {{
            // trim trailing whitespace
            while let Some(atom) = cur.last() {
                if atom.trimmable() {
                    cur_width -= atom.advance;
                    trailing_trim -= atom.advance;
                    cur.pop();
                } else {
                    break;
                }
            }
            if !cur.is_empty() || $mandatory {
                let text_start = line_text_start(shaped, &cur, forced_break_pos.take());
                lines.push(Line {
                    atoms: std::mem::take(&mut cur),
                    width: cur_width,
                    mandatory_break: $mandatory,
                    ellipsized: $ellipsized,
                    text_start,
                });
            }
            cur_width = L::zero();
            trailing_trim = L::zero();
            last_break = None;
        }};
    }

    for atom in atoms {
        if let Some(cluster) = atom.content.forced_break_offset() {
            // the line created by this break starts right after the newline
            let newline_len = shaped.flattened[cluster as usize..]
                .chars()
                .next()
                .map(|c| c.len_utf8() as u32)
                .unwrap_or(0);
            forced_break_pos = Some(cluster);
            trailing_break = Some(cluster + newline_len);
            flush!(true, false);
            continue;
        }
        trailing_break = None;

        // ellipsis line: hard-truncate when exceeding the limit
        if let Some((ellipsis_line, ellipsis_advance)) = ellipsis {
            if lines.len() == ellipsis_line {
                let new_width = content_width(cur_width, trailing_trim) + atom.advance;
                if new_width > inline_size {
                    // drop trailing trimmables first: their width is already
                    // excluded from the content width
                    while let Some(atom) = cur.last() {
                        if atom.trimmable() {
                            trailing_trim -= atom.advance;
                            cur_width -= atom.advance;
                            cur.pop();
                        } else {
                            break;
                        }
                    }
                    let cluster_of = |atom: &Atom<L>| {
                        atom.content
                            .glyph_index()
                            .map(|glyph_ref| shaped.glyphs[glyph_ref as usize].cluster)
                    };
                    let mut width = cur_width;
                    while width + ellipsis_advance > inline_size {
                        let Some(popped) = cur.pop() else {
                            break;
                        };
                        width -= popped.advance;
                        // never cut inside a grapheme cluster:
                        // pop the remaining glyphs sharing the cluster
                        if let Some(cluster) = cluster_of(&popped) {
                            while let Some(last) = cur.last() {
                                if cluster_of(last) == Some(cluster) {
                                    width -= last.advance;
                                    cur.pop();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    cur_width = width;
                    trailing_trim = L::zero();
                    flush!(false, true);
                    break;
                }
            }
        }

        // the atom may still overflow after a break: re-check in a loop
        //
        // collapsible whitespace never overflows by itself: at a line edge
        // it is trimmed away, and otherwise its width is accounted when the
        // following atom is checked
        if !atom.trimmable() {
            loop {
                let new_content_width = content_width(cur_width, trailing_trim) + atom.advance;
                if new_content_width <= inline_size || cur.is_empty() {
                    break;
                }
                // the freshest break point is right before this atom
                if atom.break_before() {
                    flush!(false, false);
                    break;
                }
                if let Some((break_at, width_snapshot, trim_snapshot)) = last_break {
                    let mut tail = cur.split_off(break_at);
                    // restore the width state at the break point, then flush
                    cur_width = width_snapshot;
                    trailing_trim = trim_snapshot;
                    // drop leading whitespace of the next line
                    while let Some(first) = tail.first() {
                        if first.trimmable() {
                            tail.remove(0);
                        } else {
                            break;
                        }
                    }
                    flush!(false, false);
                    // rebuild the width state from the tail
                    cur_width = length_sum(tail.iter().map(|a| a.advance));
                    trailing_trim = L::zero();
                    for a in tail.iter().rev() {
                        if a.trimmable() {
                            trailing_trim += a.advance;
                        } else {
                            break;
                        }
                    }
                    cur = tail;
                } else if atom.emergency_breakable() {
                    // no legal break point: emergency break before this atom,
                    // otherwise the content overflows
                    flush!(false, false);
                    break;
                } else {
                    break;
                }
            }
        }

        // leading collapsible whitespace of a line is dropped
        if atom.trimmable() && cur.is_empty() {
            continue;
        }

        if atom.break_before() && !cur.is_empty() {
            last_break = Some((cur.len(), cur_width, trailing_trim));
        }
        cur_width += atom.advance;
        if atom.trimmable() {
            trailing_trim += atom.advance;
        } else {
            trailing_trim = L::zero();
        }
        cur.push(atom);
    }
    flush!(false, false);
    // a trailing newline creates an empty last line
    if let Some(start) = trailing_break {
        lines.push(Line {
            atoms: Vec::new(),
            width: L::zero(),
            mandatory_break: true,
            ellipsized: false,
            text_start: start,
        });
    }
    lines
}

/// Reorder atoms into visual order (UAX#9 L2).
///
/// Atoms are in logical order; each contiguous sequence of atoms with
/// level >= L is reversed, from the lowest odd level up to the max level.
fn reorder_visual<L: LengthNum>(atoms: &mut [Atom<L>]) {
    if atoms.is_empty() {
        return;
    }
    // spacing atoms adopt the level of the following atom (or previous at end)
    let mut next_level = atoms.last().map(|a| a.bidi_level).unwrap_or(0);
    for i in (0..atoms.len()).rev() {
        if atoms[i].content == AtomContent::SPACING {
            atoms[i].bidi_level = next_level;
        } else {
            next_level = atoms[i].bidi_level;
        }
    }
    let max_level = atoms.iter().map(|a| a.bidi_level).max().unwrap_or(0);
    let Some(lowest_odd) = atoms
        .iter()
        .map(|a| a.bidi_level)
        .filter(|l| l % 2 == 1)
        .min()
    else {
        return;
    };
    for level in (lowest_odd..=max_level).rev() {
        let mut i = 0;
        while i < atoms.len() {
            if atoms[i].bidi_level >= level {
                let start = i;
                while i < atoms.len() && atoms[i].bidi_level >= level {
                    i += 1;
                }
                atoms[start..i].reverse();
            } else {
                i += 1;
            }
        }
    }
}

/// Computes baselines, aligns segments, reorders bidi runs visually,
/// and produces the final layout result in physical coordinates.
pub(crate) fn justify<L: LengthNum, F: FontFace>(
    shaped: &std::sync::Arc<ShapedBlock<L, F>>,
    lines: Vec<Line<L>>,
    inline_size: L,
) -> LayoutResult<L, F> {
    let writing_mode = &shaped.property.writing_mode;
    let horizontal = matches!(writing_mode, WritingMode::HorizontalTb);
    let to_physical = |inline: L, block: L| -> Point<L> {
        match writing_mode {
            WritingMode::HorizontalTb => Point::new(inline, block),
            // vertical-rl is remapped once the total block size is known
            _ => Point::new(block, inline),
        }
    };

    let mut glyphs_out: Vec<PositionedGlyph<L>> = Vec::new();
    let mut items_out: Vec<PositionedItemRect<L>> = Vec::new();
    let mut lines_out: Vec<LineLayout<L>> = Vec::new();
    let mut block_pen = L::zero();
    let mut max_inline = L::zero();
    let need_ellipsis_glyphs = lines.iter().any(|line| line.ellipsized);
    let extra_glyphs = if need_ellipsis_glyphs {
        shaped
            .ellipsis_glyphs
            .as_ref()
            .map(|g| g.to_vec())
            .unwrap_or_default()
    } else {
        vec![]
    };
    let ellipsis_base = EXTRA_GLYPH_TAG;

    for line in lines {
        let glyph_start = glyphs_out.len() as u32;
        let item_start = items_out.len() as u32;
        let text_start = line.text_start;

        // line metrics
        let mut text_ascent = L::zero();
        let mut text_descent = L::zero();
        let mut line_gap = L::zero();
        let mut inline_block_ascent = L::zero();
        let mut inline_block_descent = L::zero();
        let mut has_text = false;
        for atom in &line.atoms {
            if let Some(glyph_ref) = atom.content.glyph_index() {
                has_text = true;
                let glyph = &shaped.glyphs[glyph_ref as usize];
                let font_size = shaped
                    .item_at_glyph(glyph_ref)
                    .map(|(_, _, item)| item.font_size)
                    .unwrap_or(shaped.property.font_size.to_f32());
                let metrics = shaped.faces[glyph.face_index as usize].metrics(font_size);
                // descent is negative below the baseline internally
                text_ascent = text_ascent.lower_bound(L::from_f32(metrics.ascent));
                text_descent = text_descent.upper_bound(L::from_f32(-metrics.descent));
                line_gap = line_gap.lower_bound(L::from_f32(metrics.line_gap));
            } else if let Some(item_index) = atom.content.item_index() {
                let item = &shaped.items[item_index as usize].item;
                let block_size = if horizontal {
                    item.size.height
                } else {
                    item.size.width
                };
                let ascent = item.baseline_ascent.unwrap_or(block_size);
                inline_block_ascent = inline_block_ascent.lower_bound(ascent);
                inline_block_descent = inline_block_descent.upper_bound(ascent - block_size);
            }
        }
        // lines without text content (e.g. empty lines between newlines)
        // take the block default font metrics as the strut
        if !has_text {
            if let Some(strut) = &shaped.strut_metrics {
                text_ascent = L::from_f32(strut.ascent);
                text_descent = L::from_f32(-strut.descent);
                line_gap = L::from_f32(strut.line_gap);
            }
        }
        let text_line_height = text_ascent - text_descent;
        let (text_ascent, text_descent) = if let Some(line_height) = shaped.property.line_height {
            let half_diff = (line_height - text_line_height).div_i32(2);
            (text_ascent + half_diff, text_descent - half_diff)
        } else {
            (text_ascent, text_descent)
        };
        let total_ascent = text_ascent.lower_bound(inline_block_ascent);
        let total_descent = text_descent.upper_bound(inline_block_descent);
        let gap = if shaped.property.line_height.is_some() || lines_out.is_empty() {
            L::zero()
        } else {
            line_gap
        };
        let baseline = block_pen + gap + total_ascent;

        // alignment
        let line_width_limit = if inline_size == L::max_value() {
            line.width
        } else {
            inline_size
        };
        let free = line_width_limit - line.width;
        let justify = match shaped.property.text_align {
            TextAlign::Justify => !line.mandatory_break && !line.ellipsized,
            TextAlign::JustifyAll => true,
            _ => false,
        } && free > L::zero();
        let stretch_points = if justify {
            line.atoms
                .iter()
                .enumerate()
                .filter(|(i, atom)| {
                    *i > 0 && atom.word_start() && atom.content.glyph_index().is_some()
                })
                .count() as i32
        } else {
            0
        };
        let justify_space = if stretch_points > 0 {
            free.div_i32(stretch_points)
        } else {
            L::zero()
        };
        let align_offset = match shaped.property.text_align {
            TextAlign::Left | TextAlign::Justify | TextAlign::JustifyAll => L::zero(),
            TextAlign::Center => free.div_i32(2),
            TextAlign::Right => free,
            TextAlign::Start | TextAlign::MatchParent => {
                if shaped.base_direction == ResolvedDirection::RTL {
                    free
                } else {
                    L::zero()
                }
            }
            TextAlign::End => {
                if shaped.base_direction == ResolvedDirection::RTL {
                    L::zero()
                } else {
                    free
                }
            }
        };
        let align_offset = if free > L::zero() {
            align_offset
        } else {
            L::zero()
        };

        // reorder atoms from logical order into visual order
        let mut atoms = line.atoms;
        reorder_visual(&mut atoms);

        // position atoms
        let mut pen = align_offset;
        let mut first_glyph_seen = false;
        for atom in &atoms {
            if let Some(glyph_ref) = atom.content.glyph_index() {
                if first_glyph_seen && atom.word_start() {
                    pen += justify_space;
                }
                first_glyph_seen = true;
                let glyph = &shaped.glyphs[glyph_ref as usize];
                let origin = if horizontal {
                    to_physical(
                        pen + L::from_f32(glyph.offset.x),
                        baseline - L::from_f32(glyph.offset.y),
                    )
                } else if glyph.flags.sideways() {
                    // shaped horizontally: offsets map to the inline axis
                    to_physical(
                        pen + L::from_f32(glyph.offset.x),
                        baseline - L::from_f32(glyph.offset.y),
                    )
                } else {
                    to_physical(
                        pen + L::from_f32(glyph.offset.y),
                        baseline + L::from_f32(glyph.offset.x),
                    )
                };
                glyphs_out.push(PositionedGlyph { origin, glyph_ref });
                pen += atom.advance;
            } else if let Some(item_index) = atom.content.item_index() {
                let item = &shaped.items[item_index as usize].item;
                let block_size = if horizontal {
                    item.size.height
                } else {
                    item.size.width
                };
                let baseline_ascent = item.baseline_ascent.unwrap_or(block_size);
                let item_ascent = match item.vertical_align {
                    VerticalAlign::Baseline { offset } => baseline_ascent + offset,
                    VerticalAlign::Top => total_ascent,
                    VerticalAlign::TextTop => text_ascent,
                    VerticalAlign::Middle => (text_ascent + text_descent + block_size).div_i32(2),
                    VerticalAlign::TextBottom => text_descent + block_size,
                    VerticalAlign::Bottom => total_descent + block_size,
                };
                let block_start = baseline - item_ascent;
                let origin = to_physical(pen, block_start);
                let rect = if horizontal {
                    Rect::new(origin, item.size)
                } else {
                    Rect::new(origin, Size::new(item.size.height, item.size.width))
                };
                items_out.push(PositionedItemRect { rect, item_index });
                pen += atom.advance;
            } else if atom.content == AtomContent::SPACING {
                pen += atom.advance;
            }
        }

        // append ellipsis glyphs at the line end
        if line.ellipsized {
            if let Some(ellipsis_glyphs) = shaped.ellipsis_glyphs.as_ref() {
                for (i, glyph) in ellipsis_glyphs.iter().enumerate() {
                    let origin = if horizontal {
                        to_physical(
                            pen + L::from_f32(glyph.offset.x),
                            baseline - L::from_f32(glyph.offset.y),
                        )
                    } else {
                        to_physical(
                            pen + L::from_f32(glyph.offset.y),
                            baseline + L::from_f32(glyph.offset.x),
                        )
                    };
                    glyphs_out.push(PositionedGlyph {
                        origin,
                        glyph_ref: ellipsis_base | i as u32,
                    });
                    pen += glyph_inline_advance(glyph);
                }
            }
        }

        max_inline = max_inline.lower_bound(line.width);
        lines_out.push(LineLayout {
            baseline_offset: baseline,
            text_ascent,
            text_descent,
            inline_start: align_offset,
            inline_end: pen,
            glyph_start,
            item_start,
            text_start,
        });
        block_pen = baseline - total_descent;
    }

    let block_size = block_pen;
    // vertical-rl stacks lines right-to-left: remap block coordinates
    // from the right edge of the content box
    if matches!(writing_mode, WritingMode::VerticalRl) {
        for glyph in &mut glyphs_out {
            glyph.origin.x = block_size - glyph.origin.x;
        }
        for item in &mut items_out {
            item.rect.origin.x = block_size - item.rect.origin.x - item.rect.size.width;
        }
        for line in &mut lines_out {
            line.baseline_offset = block_size - line.baseline_offset;
        }
    }
    let size = if horizontal {
        Size::new(max_inline, block_size)
    } else {
        Size::new(block_size, max_inline)
    };

    LayoutResult {
        size,
        shaped: shaped.clone(),
        lines: lines_out.into_boxed_slice(),
        glyphs: glyphs_out.into_boxed_slice(),
        items: items_out.into_boxed_slice(),
        extra_glyphs: extra_glyphs.into_boxed_slice(),
    }
}
