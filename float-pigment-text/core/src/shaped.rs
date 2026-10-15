//! Stage A result: the shaped block, reusable across container sizes.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use float_pigment_css::length_num::LengthNum;
use float_pigment_css::typing::{WhiteSpace, WordBreak, WritingMode};

use crate::model::{TextOrientation, TextOverflow};

/// Whether the character stays upright in vertical text with
/// `text-orientation: mixed`, i.e. its UAX#50 Vertical_Orientation
/// is Upright or TransformedUpright.
fn orientation_upright(c: char) -> bool {
    use icu_properties::props::VerticalOrientation;
    let vo = icu_properties::CodePointMapData::<VerticalOrientation>::new().get(c);
    matches!(
        vo,
        VerticalOrientation::Upright | VerticalOrientation::TransformedUpright
    )
}

/// Split a text range into sub-ranges by orientation class
/// (upright or sideways), for `text-orientation: mixed`.
/// Returns `(range, vertical_shaping)` pairs.
fn split_by_orientation_class(range: Range<usize>, text: &str) -> Vec<(Range<usize>, bool)> {
    let mut result = Vec::new();
    let mut start = range.start;
    let mut upright = None;
    for (offset, c) in text[range.clone()].char_indices() {
        let cur = orientation_upright(c);
        if let Some(prev) = upright {
            if cur != prev {
                result.push((start..range.start + offset, prev));
                start = range.start + offset;
            }
        }
        upright = Some(cur);
    }
    if let Some(prev) = upright {
        result.push((start..range.end, prev));
    }
    result
}

use crate::font::{FontFace, FontFaceId, FontMetrics, FontProvider, FontQuery, ResolvedFontList};
use crate::geometry::Size;
use crate::itemize::{itemize, TextItem};
use crate::layout::LayoutResult;
use crate::model::{Block, BlockProperty, Inline, Spacing, VerticalAlign};
use crate::shaper::{GlyphFlags, GlyphRun, ResolvedDirection, ShapeOptions, ShapedGlyph, Shaper};

/// A shaped text item (bidi- and style-uniform).
///
/// Items are stored contiguously per text and texts per block, so the
/// end of each range is the start of the successor; see
/// `ShapedBlock::item_glyph_range` and `ShapedBlock::item_flattened_end`.
/// The resolved direction is the parity of `bidi_level`.
#[derive(Debug, Clone)]
pub(crate) struct ShapedTextItem {
    /// The UTF-8 byte offset where the item starts in the flattened text.
    pub(crate) flattened_start: u32,
    /// The offset of the first glyph in `ShapedBlock::glyphs`.
    pub(crate) glyph_start: u32,
    /// The resolved font size.
    pub(crate) font_size: f32,
    /// The bidi level resolved by UAX#9.
    pub(crate) bidi_level: u8,
    /// Whether the shaper emitted glyphs in visual order (descending
    /// clusters), i.e. horizontal-shaped RTL text. Line breaking operates
    /// in logical order, so atoms of such items are iterated in reverse.
    pub(crate) visual_order: bool,
}

/// The subset of `InlineProperty` needed by line breaking, kept per
/// shaped text; font- and direction-related fields are resolved during
/// shaping and then dropped.
#[derive(Debug, Clone)]
pub(crate) struct ShapedTextProperty<L: LengthNum> {
    /// The white space handling mode.
    pub(crate) white_space: WhiteSpace,
    /// The word breaking mode.
    pub(crate) word_break: WordBreak,
    /// The extra spacing between glyphs.
    pub(crate) letter_spacing: L,
    /// The extra spacing between words.
    pub(crate) word_spacing: L,
}

/// A shaped `Inline::Text`.
#[derive(Debug, Clone)]
pub(crate) struct ShapedText<L: LengthNum> {
    /// The UTF-8 byte range of the whole inline text in the flattened text.
    pub(crate) flattened_range: Range<u32>,
    /// The items of this text.
    pub(crate) items: Box<[ShapedTextItem]>,
    /// The line breaking properties.
    pub(crate) property: ShapedTextProperty<L>,
    /// The spacing around the text.
    pub(crate) spacing: Spacing<L>,
}

/// The subset of `InlineItem` needed by layout; the font- and
/// direction-related inline properties are resolved during shaping and
/// then dropped.
#[derive(Debug, Clone)]
pub(crate) struct ShapedInlineItem<L: LengthNum> {
    /// The size of the item.
    pub(crate) size: Size<L>,
    /// The distance from the item top to its baseline; `None` means the item bottom.
    pub(crate) baseline_ascent: Option<L>,
    /// The vertical alignment relative to the line baseline.
    pub(crate) vertical_align: VerticalAlign<L>,
    /// The spacing around the item.
    pub(crate) spacing: Spacing<L>,
    /// Whether wrapping is disabled (`white-space: nowrap`).
    pub(crate) no_wrap: bool,
}

/// An `Inline::Item` with its bidi level resolved.
#[derive(Debug, Clone)]
pub(crate) struct PositionedInlineItem<L: LengthNum> {
    /// The bidi level resolved by UAX#9.
    pub(crate) bidi_level: u8,
    /// The UTF-8 offset of the U+FFFC placeholder character in the flattened text.
    pub(crate) flattened_offset: u32,
    /// The item content.
    pub(crate) item: ShapedInlineItem<L>,
}

/// The content- and style-dependent layout result of a block.
///
/// It can be reused when the container inline size changes.
/// The flattened text is kept (single buffer) so that
/// re-shaping around line break points remains possible.
///
/// Layout results hold an `Arc` of this struct,
/// sharing the shaped data instead of duplicating it.
#[derive(Debug, Clone)]
pub struct ShapedBlock<L: LengthNum, F: FontFace> {
    pub(crate) property: BlockProperty<L>,
    /// The whole block content in one buffer, inline items replaced by U+FFFC.
    pub(crate) flattened: Box<str>,
    /// The shaped `Inline::Text`s, in the order they appear in the block.
    pub(crate) texts: Box<[ShapedText<L>]>,
    /// The `Inline::Item`s with bidi levels, in the order they appear in the block.
    pub(crate) items: Box<[PositionedInlineItem<L>]>,
    /// The deduplicated faces used by all glyphs.
    pub(crate) faces: Box<[F]>,
    /// The flat glyph array of the whole block, indexed by
    /// `PositionedGlyph::glyph_ref`. Items are concatenated in logical
    /// order, but glyphs within an item keep the shaper output order
    /// (visual for horizontal RTL items, see `ShapedTextItem::visual_order`).
    pub(crate) glyphs: Box<[ShapedGlyph]>,
    /// The resolved base direction of the block.
    pub(crate) base_direction: ResolvedDirection,
    /// The shaped ellipsis glyphs (`…`), when `text_overflow` is `Ellipsis`.
    /// Clusters are `u32::MAX` since they have no source text.
    pub(crate) ellipsis_glyphs: Option<Box<[ShapedGlyph]>>,
    /// The metrics of the block default font at the block font size,
    /// used as the strut for lines without content (e.g. empty lines
    /// between newlines).
    pub(crate) strut_metrics: Option<FontMetrics>,
}

/// Shape `text[range]`, driving font fallback through the provider.
///
/// Missing spans are re-shaped with the next face in `faces`; when the
/// list is exhausted, the provider is asked for faces covering the
/// missing character, which are appended to `faces` (thus cached for
/// later items sharing the same query).
#[allow(clippy::too_many_arguments)]
fn shape_with_fallback<F, P, S>(
    text: &str,
    range: Range<usize>,
    faces: &mut ResolvedFontList<F>,
    face_at: usize,
    provider: &P,
    shaper: &S,
    options: &ShapeOptions,
    out: &mut Vec<GlyphRun<F>>,
) where
    F: FontFace,
    P: FontProvider<Face = F>,
    S: Shaper<Face = F>,
{
    let Some(face) = faces.faces().get(face_at).cloned() else {
        log::warn!("No font face available for shaping");
        return;
    };
    let runs = shaper.shape(text, range.clone(), &face, options);
    // harfbuzz outputs glyphs in visual order:
    // clusters ascend for LTR and descend for RTL
    let clusters_ascend = options.vertical || options.direction == ResolvedDirection::LTR;

    // Re-shape `span` with the next candidate face; if the list is
    // exhausted, ask the provider for more faces covering the span.
    // Keeps `.notdef` glyphs when no face can produce the span.
    fn fallback_span<F, P, S>(
        text: &str,
        span: Range<usize>,
        faces: &mut ResolvedFontList<F>,
        face_at: usize,
        provider: &P,
        shaper: &S,
        options: &ShapeOptions,
        out: &mut Vec<GlyphRun<F>>,
        primary: &F,
    ) where
        F: FontFace,
        P: FontProvider<Face = F>,
        S: Shaper<Face = F>,
    {
        if face_at + 1 == faces.faces().len() {
            let Some(sample) = text[span.clone()].chars().next() else {
                return;
            };
            let extra = provider.fallback_for(sample, options.language, faces);
            if extra.is_empty() {
                out.extend(shaper.shape(text, span, primary, options));
                return;
            }
            faces.append_fallbacks(extra);
        }
        shape_with_fallback(
            text,
            span,
            faces,
            face_at + 1,
            provider,
            shaper,
            options,
            out,
        );
    }

    let mut prev_found_cluster: Option<usize> = None;
    let mut missing_min: Option<usize> = None;
    let flush_missing = |out: &mut Vec<GlyphRun<F>>,
                         missing_min: &mut Option<usize>,
                         prev_found_cluster: Option<usize>,
                         next_found_cluster: Option<usize>,
                         faces: &mut ResolvedFontList<F>| {
        let Some(start) = missing_min.take() else {
            return;
        };
        let end = if clusters_ascend {
            next_found_cluster.unwrap_or(range.end)
        } else {
            prev_found_cluster.unwrap_or(range.end)
        };
        fallback_span(
            text,
            start..end,
            faces,
            face_at,
            provider,
            shaper,
            options,
            out,
            &face,
        );
    };

    for run in runs {
        let mut cur_glyphs: Vec<ShapedGlyph> = Vec::new();
        for glyph in run.glyphs {
            if glyph.flags.missing() {
                missing_min = Some(
                    missing_min
                        .map(|min: usize| min.min(glyph.cluster as usize))
                        .unwrap_or(glyph.cluster as usize),
                );
                continue;
            }
            if missing_min.is_some() {
                if !cur_glyphs.is_empty() {
                    out.push(GlyphRun {
                        face: run.face.clone(),
                        glyphs: std::mem::take(&mut cur_glyphs),
                    });
                }
                flush_missing(
                    out,
                    &mut missing_min,
                    prev_found_cluster,
                    Some(glyph.cluster as usize),
                    faces,
                );
            }
            prev_found_cluster = Some(glyph.cluster as usize);
            cur_glyphs.push(glyph);
        }
        if !cur_glyphs.is_empty() {
            out.push(GlyphRun {
                face: run.face.clone(),
                glyphs: cur_glyphs,
            });
        }
    }
    flush_missing(out, &mut missing_min, prev_found_cluster, None, faces);
}

impl<L: LengthNum, F: FontFace> ShapedBlock<L, F> {
    /// Itemize and shape the block content.
    pub fn new<P, S>(block: &Block<L>, provider: &P, shaper: &S) -> Self
    where
        P: FontProvider<Face = F>,
        S: Shaper<Face = F>,
    {
        let itemized = itemize(block);
        let flattened = itemized.flattened;
        let vertical = !matches!(block.property.writing_mode, WritingMode::HorizontalTb)
            && !matches!(block.property.text_orientation, TextOrientation::Sideways);
        let mixed = matches!(block.property.text_orientation, TextOrientation::Mixed);

        let mut face_indexes: HashMap<FontFaceId, u16> = HashMap::new();
        let mut faces: Vec<F> = Vec::new();
        let dedup_face =
            |face: &F, face_indexes: &mut HashMap<FontFaceId, u16>, faces: &mut Vec<F>| {
                *face_indexes.entry(face.id()).or_insert_with(|| {
                    faces.push(face.clone());
                    (faces.len() - 1) as u16
                })
            };

        let mut resolved_cache: Vec<(FontQuery, ResolvedFontList<F>)> = Vec::new();
        let mut glyphs: Vec<ShapedGlyph> = Vec::new();
        let mut texts: Vec<ShapedText<L>> = Vec::new();
        for (text_index, inline_text) in block
            .inlines
            .iter()
            .filter_map(|inline| match inline {
                Inline::Text(t) => Some(t),
                Inline::Item(_) => None,
            })
            .enumerate()
        {
            let flattened_range = flattened.text_ranges[text_index].clone();
            let query = inline_text
                .property
                .font_query
                .as_ref()
                .unwrap_or(&block.property.font_query);
            let cache_index = resolved_cache
                .iter()
                .position(|(q, _)| q == query)
                .unwrap_or_else(|| {
                    resolved_cache.push((query.clone(), provider.resolve(query)));
                    resolved_cache.len() - 1
                });

            let mut items: Vec<ShapedTextItem> = Vec::new();
            for item in itemized
                .text_items
                .iter()
                .filter(|item| item.text_index as usize == text_index)
            {
                let TextItem {
                    flattened_range,
                    bidi_level,
                    direction,
                    font_size,
                    ..
                } = item;
                // `text-orientation: mixed` in vertical writing shapes
                // upright scripts vertically and horizontal scripts sideways
                let sub_items: Vec<(Range<usize>, bool)> = if vertical && mixed {
                    split_by_orientation_class(
                        flattened_range.start as usize..flattened_range.end as usize,
                        &flattened.text,
                    )
                } else {
                    vec![(
                        flattened_range.start as usize..flattened_range.end as usize,
                        vertical,
                    )]
                };
                for (sub_range, sub_vertical) in sub_items {
                    let options = ShapeOptions {
                        size: font_size.to_f32(),
                        direction: *direction,
                        script: block.property.script,
                        language: block.property.language,
                        vertical: sub_vertical,
                    };
                    let mut item_faces = resolved_cache[cache_index].1.clone();
                    let mut runs = Vec::new();
                    shape_with_fallback(
                        &flattened.text,
                        sub_range.clone(),
                        &mut item_faces,
                        0,
                        provider,
                        shaper,
                        &options,
                        &mut runs,
                    );
                    resolved_cache[cache_index].1 = item_faces;
                    let sideways = !sub_vertical && vertical;
                    let glyph_start = glyphs.len() as u32;
                    for run in runs {
                        let face_index = dedup_face(&run.face, &mut face_indexes, &mut faces);
                        for mut glyph in run.glyphs {
                            glyph.face_index = face_index;
                            let whitespace = flattened.text[glyph.cluster as usize..]
                                .chars()
                                .next()
                                .map(|c| c.is_whitespace())
                                .unwrap_or(false);
                            if whitespace {
                                glyph.flags = glyph.flags.union(GlyphFlags::WHITESPACE);
                            }
                            if sideways {
                                glyph.flags = glyph.flags.union(GlyphFlags::SIDEWAYS);
                            }
                            glyphs.push(glyph);
                        }
                    }
                    items.push(ShapedTextItem {
                        flattened_start: sub_range.start as u32,
                        glyph_start,
                        bidi_level: *bidi_level,
                        visual_order: !sub_vertical && *direction == ResolvedDirection::RTL,
                        font_size: font_size.to_f32(),
                    });
                }
            }
            texts.push(ShapedText {
                flattened_range,
                items: items.into_boxed_slice(),
                property: ShapedTextProperty {
                    white_space: inline_text.property.white_space.clone(),
                    word_break: inline_text.property.word_break.clone(),
                    letter_spacing: inline_text.property.letter_spacing,
                    word_spacing: inline_text.property.word_spacing,
                },
                spacing: inline_text.spacing,
            });
        }

        let items = block
            .inlines
            .iter()
            .filter_map(|inline| match inline {
                Inline::Text(_) => None,
                Inline::Item(item) => Some(item),
            })
            .zip(itemized.item_levels.iter())
            .zip(flattened.item_positions.iter())
            .map(|((item, bidi_level), (flattened_offset, _))| {
                PositionedInlineItem {
                    bidi_level: *bidi_level,
                    flattened_offset: *flattened_offset,
                    item: ShapedInlineItem {
                        size: item.size,
                        baseline_ascent: item.baseline_ascent,
                        vertical_align: item.property.vertical_align,
                        spacing: item.spacing,
                        no_wrap: item.property.white_space == WhiteSpace::NoWrap,
                    },
                }
            })
            .collect();

        let ellipsis_text: Option<Box<str>> = match &block.property.text_overflow {
            TextOverflow::Clip => None,
            TextOverflow::Ellipsis => Some("…".into()),
            TextOverflow::Custom(text) => Some(text.clone()),
        };
        let ellipsis_glyphs = ellipsis_text.map(|ellipsis_text| {
            let options = ShapeOptions {
                size: block.property.font_size.to_f32(),
                direction: itemized.base_direction,
                script: block.property.script,
                language: block.property.language,
                vertical,
            };
            let mut runs = Vec::new();
            let mut ellipsis_faces = provider.resolve(&block.property.font_query);
            shape_with_fallback(
                &ellipsis_text,
                0..ellipsis_text.len(),
                &mut ellipsis_faces,
                0,
                provider,
                shaper,
                &options,
                &mut runs,
            );
            let mut ellipsis_glyphs: Vec<ShapedGlyph> = Vec::new();
            for run in runs {
                let face_index = dedup_face(&run.face, &mut face_indexes, &mut faces);
                for mut glyph in run.glyphs {
                    glyph.face_index = face_index;
                    glyph.cluster = u32::MAX;
                    ellipsis_glyphs.push(glyph);
                }
            }
            ellipsis_glyphs.into_boxed_slice()
        });

        let strut_metrics = provider
            .resolve(&block.property.font_query)
            .faces()
            .first()
            .map(|face| face.metrics(block.property.font_size.to_f32()));

        Self {
            property: block.property.clone(),
            flattened: flattened.text,
            texts: texts.into_boxed_slice(),
            items,
            faces: faces.into_boxed_slice(),
            glyphs: glyphs.into_boxed_slice(),
            base_direction: itemized.base_direction,
            ellipsis_glyphs,
            strut_metrics,
        }
    }

    /// Break lines and compute the final layout for the given inline size.
    pub fn layout(self: &Arc<Self>, inline_size: L) -> LayoutResult<L, F> {
        let lines = crate::line::break_lines(self, inline_size);
        crate::line::justify(self, lines, inline_size)
    }

    /// The glyph range of a shaped item: from its start to the start of
    /// the successor item (or the end of the whole glyph array).
    pub(crate) fn item_glyph_range(&self, text_index: usize, item_index: usize) -> Range<u32> {
        let text = &self.texts[text_index];
        let start = text.items[item_index].glyph_start;
        let end = if let Some(next) = text.items.get(item_index + 1) {
            next.glyph_start
        } else if let Some(first) = self
            .texts
            .get(text_index + 1)
            .and_then(|text| text.items.first())
        {
            first.glyph_start
        } else {
            self.glyphs.len() as u32
        };
        start..end
    }

    /// The flattened text offset where a shaped item ends: the start of
    /// the successor item, or the end of the owning text.
    pub(crate) fn item_flattened_end(&self, text_index: usize, item_index: usize) -> u32 {
        let text = &self.texts[text_index];
        text.items
            .get(item_index + 1)
            .map(|item| item.flattened_start)
            .unwrap_or(text.flattened_range.end)
    }

    /// The shaped item containing the given glyph, as
    /// `(text index, item index, item)`; `None` when the glyph index is
    /// out of any item range.
    pub(crate) fn item_at_glyph(&self, glyph_index: u32) -> Option<(usize, usize, &ShapedTextItem)> {
        let cluster = self.glyphs.get(glyph_index as usize)?.cluster;
        // texts cover the flattened text in order; items within a text
        // are ordered by glyph start
        let text_index = self
            .texts
            .partition_point(|text| text.flattened_range.end <= cluster);
        let text = self.texts.get(text_index)?;
        let item_index = text
            .items
            .partition_point(|item| item.glyph_start <= glyph_index)
            .checked_sub(1)?;
        let item = text.items.get(item_index)?;
        if self.item_glyph_range(text_index, item_index).contains(&glyph_index) {
            Some((text_index, item_index, item))
        } else {
            None
        }
    }

    /// The flat glyph array of the whole block.
    ///
    /// Items are concatenated in logical order, but glyphs within an item
    /// keep the shaper output order (visual for horizontal RTL items).
    pub fn glyphs(&self) -> &[ShapedGlyph] {
        &self.glyphs
    }

    /// The deduplicated faces used by all glyphs.
    pub fn faces(&self) -> &[F] {
        &self.faces
    }

    /// The flattened text, with inline items replaced by U+FFFC.
    pub fn flattened(&self) -> &str {
        &self.flattened
    }
}
