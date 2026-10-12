//! Stage A result: the shaped block, reusable across container sizes.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use float_pigment_css::length_num::LengthNum;
use float_pigment_css::typing::WritingMode;

use crate::model::TextOverflow;

use crate::font::{FontFace, FontFaceId, FontProvider, FontQuery, ResolvedFontList};
use crate::itemize::{itemize, TextItem};
use crate::layout::LayoutResult;
use crate::model::{Block, BlockProperty, Inline, InlineItem, InlineProperty, Spacing};
use crate::shaper::{GlyphFlags, GlyphRun, ResolvedDirection, ShapeOptions, ShapedGlyph, Shaper};

/// A shaped text item (bidi- and style-uniform).
///
/// `flattened_range` and `direction` are used by hit testing and selection.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct ShapedTextItem {
    /// The UTF-8 byte range in the flattened text.
    pub(crate) flattened_range: Range<u32>,
    /// The glyph range in `ShapedBlock::glyphs`.
    pub(crate) glyph_range: Range<u32>,
    /// The bidi level resolved by UAX#9.
    pub(crate) bidi_level: u8,
    /// The resolved base direction.
    pub(crate) direction: ResolvedDirection,
    /// The resolved font size.
    pub(crate) font_size: f32,
}

/// A shaped `Inline::Text`.
#[derive(Debug, Clone)]
pub(crate) struct ShapedText<L: LengthNum> {
    /// The UTF-8 byte range of the whole inline text in the flattened text.
    pub(crate) flattened_range: Range<u32>,
    /// The items of this text.
    pub(crate) items: Box<[ShapedTextItem]>,
    /// The resolved inline property.
    pub(crate) property: InlineProperty<L>,
    /// The spacing around the text.
    pub(crate) spacing: Spacing<L>,
}

/// An `Inline::Item` with its bidi level resolved.
#[derive(Debug, Clone)]
pub(crate) struct PositionedInlineItem<L: LengthNum> {
    /// The bidi level resolved by UAX#9.
    pub(crate) bidi_level: u8,
    /// The UTF-8 offset of the U+FFFC placeholder character in the flattened text.
    pub(crate) flattened_offset: u32,
    /// The item content.
    pub(crate) item: InlineItem<L>,
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
    /// The flat glyph array of the whole block, in logical order,
    /// indexed by `PositionedGlyph::glyph_ref`.
    pub(crate) glyphs: Box<[ShapedGlyph]>,
    /// The resolved base direction of the block.
    pub(crate) base_direction: ResolvedDirection,
    /// The shaped ellipsis glyphs (`…`), when `text_overflow` is `Ellipsis`.
    /// Clusters are `u32::MAX` since they have no source text.
    pub(crate) ellipsis_glyphs: Option<Box<[ShapedGlyph]>>,
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
        shape_with_fallback(text, span, faces, face_at + 1, provider, shaper, options, out);
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
        fallback_span(text, start..end, faces, face_at, provider, shaper, options, out, &face);
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
            && !matches!(block.property.text_orientation, crate::model::TextOrientation::Sideways);

        let mut face_indexes: HashMap<FontFaceId, u16> = HashMap::new();
        let mut faces: Vec<F> = Vec::new();
        let dedup_face = |face: &F,
                              face_indexes: &mut HashMap<FontFaceId, u16>,
                              faces: &mut Vec<F>| {
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
                let options = ShapeOptions {
                    size: font_size.to_f32(),
                    direction: *direction,
                    script: block.property.script,
                    language: block.property.language,
                    vertical,
                };
                let mut item_faces = resolved_cache[cache_index].1.clone();
                let mut runs = Vec::new();
                shape_with_fallback(
                    &flattened.text,
                    flattened_range.start as usize..flattened_range.end as usize,
                    &mut item_faces,
                    0,
                    provider,
                    shaper,
                    &options,
                    &mut runs,
                );
                resolved_cache[cache_index].1 = item_faces;
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
                        glyphs.push(glyph);
                    }
                }
                items.push(ShapedTextItem {
                    flattened_range: flattened_range.clone(),
                    glyph_range: glyph_start..glyphs.len() as u32,
                    bidi_level: *bidi_level,
                    direction: *direction,
                    font_size: font_size.to_f32(),
                });
            }
            texts.push(ShapedText {
                flattened_range,
                items: items.into_boxed_slice(),
                property: inline_text.property.clone(),
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
            .map(|((item, bidi_level), (flattened_offset, _))| PositionedInlineItem {
                bidi_level: *bidi_level,
                flattened_offset: *flattened_offset,
                item: item.clone(),
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

        Self {
            property: block.property.clone(),
            flattened: flattened.text,
            texts: texts.into_boxed_slice(),
            items,
            faces: faces.into_boxed_slice(),
            glyphs: glyphs.into_boxed_slice(),
            base_direction: itemized.base_direction,
            ellipsis_glyphs,
        }
    }

    /// Break lines and compute the final layout for the given inline size.
    pub fn layout(self: &Arc<Self>, inline_size: L) -> LayoutResult<L, F> {
        let lines = crate::line::break_lines(self, inline_size);
        crate::line::justify(self, lines, inline_size)
    }

    /// The flat glyph array of the whole block, in logical order.
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
