//! The built-in pure-Rust font stack (feature `rust-font`):
//! font parsing with `skrifa` and shaping with `harfrust`.
//!
//! It contains no OS interface access: font data is registered as bytes,
//! so it works on wasm as well as native targets.

use std::str::FromStr;
use std::sync::{Arc, OnceLock};

use euclid::Vector2D;
use read_fonts::TableProvider;
use skrifa::instance::{LocationRef, Size};
use skrifa::MetadataProvider;

use crate::font::{
    FamilyName, FontFace, FontFaceId, FontMetrics, FontProperties, FontProvider, FontQuery,
    FontStyle, ResolvedFontList,
};
use crate::shaper::{
    GlyphFlags, GlyphRun, Language, ResolvedDirection, ShapeOptions, ShapedGlyph, Shaper,
};

/// A font face parsed from in-memory font data (TTF/OTF/TTC/OTC).
#[derive(Clone)]
pub struct ParsedFont {
    inner: Arc<ParsedFontInner>,
}

struct ParsedFontInner {
    id: FontFaceId,
    data: Arc<Vec<u8>>,
    face_index: u32,
    units_per_em: f32,
    ascent: f32,
    descent: f32,
    line_gap: f32,
    has_colr: bool,
    has_svg: bool,
    has_bitmap: bool,
    /// The design languages from the `meta` table `dlng` entry.
    design_languages: Box<[Language]>,
    shaper_data: OnceLock<harfrust::ShaperData>,
}

impl std::fmt::Debug for ParsedFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParsedFont")
            .field("id", &self.inner.id)
            .field("face_index", &self.inner.face_index)
            .finish()
    }
}

impl ParsedFont {
    /// Parse a font face from font data bytes.
    pub fn from_bytes(data: Arc<Vec<u8>>, face_index: u32, id: FontFaceId) -> Option<Self> {
        let font = skrifa::FontRef::from_index(data.as_slice(), face_index).ok()?;
        let metrics = font.metrics(Size::unscaled(), LocationRef::default());
        let units_per_em = metrics.units_per_em as f32;
        let has_table = |tag: &[u8; 4]| {
            font.table_directory()
                .table_records()
                .iter()
                .any(|record| record.tag() == skrifa::Tag::new(tag))
        };
        let has_colr = has_table(b"COLR");
        let has_svg = has_table(b"SVG ");
        // NOTE: GX-era `bdat`/`bloc` tags (predecessors of EBDT/EBLC) are not
        // recognized by skrifa's bitmap strikes, thus treated as unsupported.
        let has_bitmap = !font.bitmap_strikes().is_empty();
        let design_languages = read_design_languages(&font);
        Some(Self {
            inner: Arc::new(ParsedFontInner {
                id,
                data,
                face_index,
                units_per_em,
                ascent: metrics.ascent,
                descent: metrics.descent,
                line_gap: metrics.leading,
                has_colr,
                has_svg,
                has_bitmap,
                design_languages,
                shaper_data: OnceLock::new(),
            }),
        })
    }

    /// The font data bytes.
    pub fn data(&self) -> &[u8] {
        &self.inner.data
    }

    /// The face index within the font data (for font collections).
    pub fn face_index(&self) -> u32 {
        self.inner.face_index
    }

    /// The design languages from the `meta` table `dlng` entry.
    pub fn design_languages(&self) -> &[Language] {
        &self.inner.design_languages
    }

    fn skrifa_font(&self) -> Option<skrifa::FontRef<'_>> {
        skrifa::FontRef::from_index(&self.inner.data, self.inner.face_index).ok()
    }

    fn harfrust_font(&self) -> Option<harfrust::FontRef<'_>> {
        harfrust::FontRef::from_index(&self.inner.data, self.inner.face_index).ok()
    }

    fn shaper(&self) -> Option<harfrust::Shaper<'_>> {
        let font = self.harfrust_font()?;
        let data = self
            .inner
            .shaper_data
            .get_or_init(|| harfrust::ShaperData::new(&font));
        Some(data.shaper(&font).build())
    }

    /// Whether the font has a cmap mapping for the character.
    pub fn covers(&self, c: char) -> bool {
        self.skrifa_font()
            .map(|f| f.charmap().map(c).is_some())
            .unwrap_or(false)
    }

    fn is_colored_glyph(&self, glyph_id: u16, size: f32) -> bool {
        let inner = &self.inner;
        if !inner.has_colr && !inner.has_bitmap && !inner.has_svg {
            return false;
        }
        let Some(font) = self.skrifa_font() else {
            return false;
        };
        let glyph_id = skrifa::GlyphId::from(glyph_id);
        if inner.has_colr && font.color_glyphs().get(glyph_id).is_some() {
            return true;
        }
        if inner.has_bitmap {
            let strikes = font.bitmap_strikes();
            if strikes.glyph_for_size(Size::new(size), glyph_id).is_some() {
                return true;
            }
        }
        if inner.has_svg {
            if let Ok(svg) = font.svg() {
                if svg.glyph_data(glyph_id).is_some() {
                    return true;
                }
            }
        }
        false
    }
}

impl FontFace for ParsedFont {
    fn id(&self) -> FontFaceId {
        self.inner.id
    }

    fn metrics(&self, size: f32) -> FontMetrics {
        let scale = size / self.inner.units_per_em;
        FontMetrics {
            ascent: self.inner.ascent * scale,
            descent: -self.inner.descent * scale,
            line_gap: self.inner.line_gap * scale,
        }
    }
}

struct RegisteredFont {
    families: Box<[FamilyName]>,
    properties: FontProperties,
    font: ParsedFont,
}

/// Read the design languages from the `meta` table `dlng` entry.
fn read_design_languages(font: &skrifa::FontRef) -> Box<[Language]> {
    let Ok(meta) = font.meta() else {
        return Box::default();
    };
    let Some(record) = meta
        .data_maps()
        .iter()
        .find(|record| record.tag() == read_fonts::tables::meta::DLNG)
    else {
        return Box::default();
    };
    let Ok(read_fonts::tables::meta::Metadata::ScriptLangTags(tags)) =
        record.data(meta.offset_data())
    else {
        return Box::default();
    };
    tags.iter()
        .filter_map(|tag| tag.ok())
        .filter_map(|tag| tag.as_str().parse::<Language>().ok())
        .collect()
}

/// Whether the character is a Han ideograph, whose glyph form depends on
/// the content language due to Han unification.
fn is_han_char(c: char) -> bool {
    let c = c as u32;
    matches!(
        c,
        0x3400..=0x4DBF  // Extension A
        | 0x4E00..=0x9FFF   // Unified Ideographs
        | 0xF900..=0xFAFF   // Compatibility Ideographs
        | 0x20000..=0x2A6DF // Extension B
        | 0x2A700..=0x2EBEF // Extension C..F
    )
}

/// The Han glyph variety of a language tag, derived from the explicit
/// script subtag (`zh-Hans`/`zh-Hant`) or region conventions
/// (CN/SG use simplified, TW/HK/MO use traditional).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HanVariety {
    Hans,
    Hant,
    Ja,
    Ko,
}

fn han_variety(language: &Language) -> Option<HanVariety> {
    let tag = language.as_str();
    let mut subtags = tag.split(['-', ',']);
    let primary = subtags.next().unwrap_or_default();
    if primary.eq_ignore_ascii_case("ja") {
        return Some(HanVariety::Ja);
    }
    if primary.eq_ignore_ascii_case("ko") {
        return Some(HanVariety::Ko);
    }
    if !primary.eq_ignore_ascii_case("zh") {
        return None;
    }
    for subtag in subtags {
        if subtag.eq_ignore_ascii_case("hans") {
            return Some(HanVariety::Hans);
        }
        if subtag.eq_ignore_ascii_case("hant") {
            return Some(HanVariety::Hant);
        }
        if ["tw", "hk", "mo"].contains(&subtag.to_ascii_lowercase().as_str()) {
            return Some(HanVariety::Hant);
        }
        if ["cn", "sg"].contains(&subtag.to_ascii_lowercase().as_str()) {
            return Some(HanVariety::Hans);
        }
    }
    // bare `zh` defaults to simplified
    Some(HanVariety::Hans)
}

/// The fallback priority group of a font for a Han character
/// (Han unification): exact tag match first, then the same Han variety,
/// then the other zh variety, then ja/ko fonts, then fonts without
/// language metadata, and finally fonts declaring non-Han languages.
fn han_fallback_group(design_languages: &[Language], content: &Language) -> u8 {
    let content_tag = content.as_str();
    let content_variety = han_variety(content);
    let mut best = if design_languages.is_empty() { 4 } else { 5 };
    for language in design_languages {
        let group = if language.as_str().eq_ignore_ascii_case(content_tag) {
            0
        } else {
            match (content_variety, han_variety(language)) {
                (Some(c), Some(f)) if c == f => 1,
                (
                    Some(HanVariety::Hans | HanVariety::Hant),
                    Some(HanVariety::Hans | HanVariety::Hant),
                ) => 2,
                (Some(_), Some(_)) => 3,
                _ => continue,
            }
        };
        best = best.min(group);
    }
    best
}

// CSS Fonts Level 4: narrower/below is preferred first for desired values
// at or below normal, and wider/above first for values above normal.
fn stretch_rank(actual: f32, desired: f32) -> f32 {
    if actual == desired {
        return 0.0;
    }
    let diff = (actual - desired).abs();
    let preferred_side = if desired <= 1.0 {
        actual < desired
    } else {
        actual > desired
    };
    if preferred_side {
        diff
    } else {
        100.0 + diff
    }
}

// CSS Fonts Level 4: italic prefers oblique over normal,
// oblique prefers italic over normal, normal prefers oblique over italic.
fn style_rank(actual: FontStyle, desired: FontStyle) -> u8 {
    use FontStyle::*;
    match (desired, actual) {
        (Normal, Normal) | (Italic, Italic) | (Oblique, Oblique) => 0,
        (Normal, Oblique) | (Italic, Oblique) | (Oblique, Italic) => 1,
        _ => 2,
    }
}

// CSS Fonts Level 4: weights 400..=500 prefer each other first,
// then below in descending order, then above in ascending order;
// desired below 400 checks below first, above 500 checks above first.
fn weight_rank(actual: f32, desired: f32) -> f32 {
    if actual == desired {
        return 0.0;
    }
    let diff = (actual - desired).abs();
    if (400.0..=500.0).contains(&desired) {
        if (400.0..=500.0).contains(&actual) {
            diff
        } else if actual < desired {
            1000.0 + diff
        } else {
            2000.0 + diff
        }
    } else if desired < 400.0 {
        if actual < desired {
            diff
        } else {
            1000.0 + diff
        }
    } else {
        if actual > desired {
            diff
        } else {
            1000.0 + diff
        }
    }
}

/// A `FontProvider` backed by in-memory registered fonts.
///
/// Font lookup matches registered fonts by family name equality,
/// then picks the best properties match.
#[derive(Default)]
pub struct MemoryFontProvider {
    fonts: Vec<RegisteredFont>,
    next_id: u64,
}

impl std::fmt::Debug for MemoryFontProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryFontProvider")
            .field("font_count", &self.fonts.len())
            .finish()
    }
}

impl MemoryFontProvider {
    /// Create an empty provider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a font face from font data bytes.
    pub fn register(
        &mut self,
        data: Arc<Vec<u8>>,
        face_index: u32,
        families: Box<[FamilyName]>,
        properties: FontProperties,
    ) -> Option<FontFaceId> {
        let id = FontFaceId(self.next_id);
        self.next_id += 1;
        let font = ParsedFont::from_bytes(data, face_index, id)?;
        self.fonts.push(RegisteredFont {
            families,
            properties,
            font,
        });
        Some(id)
    }

    fn best_match(&self, family: &FamilyName, properties: &FontProperties) -> Option<ParsedFont> {
        // follows the CSS Fonts Level 4 font matching algorithm:
        // lexicographic filtering by stretch, then style, then weight
        self.fonts
            .iter()
            .filter(|f| f.families.contains(family))
            .min_by(|a, b| {
                let key = |f: &RegisteredFont| {
                    (
                        stretch_rank(f.properties.stretch.0, properties.stretch.0),
                        style_rank(f.properties.style, properties.style),
                        weight_rank(f.properties.weight.0, properties.weight.0),
                    )
                };
                let (a, b) = (key(a), key(b));
                a.0.total_cmp(&b.0)
                    .then(a.1.cmp(&b.1))
                    .then(a.2.total_cmp(&b.2))
            })
            .map(|f| f.font.clone())
    }
}

impl FontProvider for MemoryFontProvider {
    type Face = ParsedFont;

    fn resolve(&self, query: &FontQuery) -> ResolvedFontList<ParsedFont> {
        let mut faces = Vec::new();
        for family in query.families.iter() {
            if let Some(font) = self.best_match(family, &query.properties) {
                if !faces.iter().any(|f: &ParsedFont| f.id() == font.id()) {
                    faces.push(font);
                }
            }
        }
        ResolvedFontList::from_faces(faces)
    }

    fn fallback_for(
        &self,
        sample: char,
        language: Option<Language>,
        resolved: &ResolvedFontList<ParsedFont>,
    ) -> Vec<ParsedFont> {
        let mut covering: Vec<&RegisteredFont> = self
            .fonts
            .iter()
            .filter(|f| f.font.covers(sample))
            .filter(|f| !resolved.faces().iter().any(|r| r.id() == f.font.id()))
            .collect();
        if let Some(language) = language {
            if is_han_char(sample) && han_variety(&language).is_some() {
                covering
                    .sort_by_key(|f| han_fallback_group(&f.font.inner.design_languages, &language));
            }
        }
        covering.into_iter().map(|f| f.font.clone()).collect()
    }
}

/// A `Shaper` implemented with `harfrust`, shaping with `ParsedFont`s.
#[derive(Debug, Default)]
pub struct RustShaper;

impl RustShaper {
    /// Create a new shaper.
    pub fn new() -> Self {
        Self
    }

    fn shape_range(
        &self,
        full_text: &str,
        range: std::ops::Range<usize>,
        font: &ParsedFont,
        options: &ShapeOptions,
    ) -> Vec<ShapedGlyph> {
        let Some(shaper) = font.shaper() else {
            return vec![];
        };
        let mut buffer = harfrust::UnicodeBuffer::new();
        buffer.push_str(&full_text[range.clone()]);
        if range.start > 0 {
            buffer.set_pre_context(&full_text[..range.start]);
        }
        if range.end < full_text.len() {
            buffer.set_post_context(&full_text[range.end..]);
        }
        buffer.set_direction(if options.vertical {
            harfrust::Direction::TopToBottom
        } else {
            match options.direction {
                ResolvedDirection::LTR => harfrust::Direction::LeftToRight,
                ResolvedDirection::RTL => harfrust::Direction::RightToLeft,
            }
        });
        if let Some(script) = options.script {
            if let Ok(script) =
                harfrust::Script::from_str(std::str::from_utf8(&script.0).unwrap_or_default())
            {
                buffer.set_script(script);
            }
        }
        if let Some(language) = &options.language {
            if let Ok(language) = harfrust::Language::from_str(language.as_str()) {
                buffer.set_language(language);
            }
        }
        buffer.guess_segment_properties();

        let scale = options.size / font.inner.units_per_em;
        let glyph_buffer = shaper.shape(buffer, harfrust::ShapeOptions::default());
        glyph_buffer
            .glyph_infos()
            .iter()
            .zip(glyph_buffer.glyph_positions().iter())
            .map(|(info, pos)| {
                let missing = info.glyph_id == 0;
                let colored = !missing && font.is_colored_glyph(info.glyph_id as u16, options.size);
                ShapedGlyph {
                    advance: Vector2D::new(
                        pos.x_advance as f32 * scale,
                        pos.y_advance as f32 * scale,
                    ),
                    offset: Vector2D::new(pos.x_offset as f32 * scale, pos.y_offset as f32 * scale),
                    cluster: info.cluster + range.start as u32,
                    glyph_id: info.glyph_id as u16,
                    face_index: 0,
                    flags: if missing {
                        GlyphFlags::MISSING
                    } else if colored {
                        GlyphFlags::COLORED
                    } else {
                        GlyphFlags::NONE
                    },
                }
            })
            .collect()
    }
}

impl Shaper for RustShaper {
    type Face = ParsedFont;

    fn shape(
        &self,
        text: &str,
        range: std::ops::Range<usize>,
        face: &ParsedFont,
        options: &ShapeOptions,
    ) -> Vec<GlyphRun<ParsedFont>> {
        let glyphs = self.shape_range(text, range, face, options);
        if glyphs.is_empty() {
            vec![]
        } else {
            vec![GlyphRun {
                face: face.clone(),
                glyphs,
            }]
        }
    }
}
