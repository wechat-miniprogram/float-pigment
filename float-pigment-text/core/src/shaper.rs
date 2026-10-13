//! Shaping abstraction: mapping Unicode text to positioned glyphs.

use euclid::UnknownUnit;

use crate::font::FontFace;

/// The resolved base direction of a text item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolvedDirection {
    /// Left-to-right.
    LTR,
    /// Right-to-left.
    RTL,
}

/// An ISO 15924 script tag, e.g. `Latn`, `Hans`, `Arab`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Script(pub [u8; 4]);

/// A BCP 47 language tag, stored inline without heap allocation.
///
/// Tags longer than 15 bytes (practically only private-use extensions,
/// which are irrelevant to OpenType language systems) are rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Language {
    buf: [u8; 15],
    len: u8,
}

impl Language {
    /// The tag as a string slice.
    pub fn as_str(&self) -> &str {
        // SAFETY: `FromStr` is the only constructor and the fields are
        // private, so `len <= buf.len()` and `buf[..len]` always holds
        // ASCII bytes (thus valid UTF-8).
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.len as usize]) }
    }
}

/// An error returned when parsing a BCP 47 language tag fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLanguageError;

impl std::fmt::Display for ParseLanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid BCP 47 language tag")
    }
}

impl std::error::Error for ParseLanguageError {}

impl std::str::FromStr for Language {
    type Err = ParseLanguageError;

    /// Parse a BCP 47 language tag.
    ///
    /// Fails if the tag is not ASCII or exceeds 15 bytes.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() || s.len() > 15 {
            return Err(ParseLanguageError);
        }
        let mut buf = [0u8; 15];
        buf[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Self {
            buf,
            len: s.len() as u8,
        })
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Options for shaping a single text item.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeOptions {
    /// The font size.
    pub size: f32,
    /// The resolved base direction of the item.
    pub direction: ResolvedDirection,
    /// The script; `None` means guessing from the text.
    pub script: Option<Script>,
    /// The BCP 47 language tag.
    pub language: Option<Language>,
    /// Whether the glyphs advance in the block direction (vertical text).
    pub vertical: bool,
}

/// Boolean markers of a glyph, packed for compact storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct GlyphFlags(u8);

impl GlyphFlags {
    /// No markers.
    pub const NONE: Self = Self(0);
    /// The glyph has its own colors (e.g. emoji), ignoring font color.
    pub const COLORED: Self = Self(1 << 0);
    /// The glyph is a `.notdef` placeholder, i.e. the font lacks the character.
    pub const MISSING: Self = Self(1 << 1);
    /// The glyph is a whitespace.
    pub const WHITESPACE: Self = Self(1 << 2);
    /// The glyph is shaped horizontally within vertical text
    /// (`text-orientation: mixed` for horizontal scripts);
    /// it should be rendered rotated 90 degrees.
    pub const SIDEWAYS: Self = Self(1 << 3);

    /// Whether all markers in `other` are set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// The union of two marker sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether the glyph has its own colors.
    pub const fn colored(self) -> bool {
        self.contains(Self::COLORED)
    }

    /// Whether the glyph is a `.notdef` placeholder.
    pub const fn missing(self) -> bool {
        self.contains(Self::MISSING)
    }

    /// Whether the glyph is a whitespace.
    pub const fn whitespace(self) -> bool {
        self.contains(Self::WHITESPACE)
    }

    /// Whether the glyph is sideways within vertical text.
    pub const fn sideways(self) -> bool {
        self.contains(Self::SIDEWAYS)
    }
}

/// A shaped glyph.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    /// The advance of the pen position.
    pub advance: euclid::Vector2D<f32, UnknownUnit>,
    /// The origin offset relative to the pen position.
    pub offset: euclid::Vector2D<f32, UnknownUnit>,
    /// The UTF-8 byte offset of the cluster start in the flattened text.
    pub cluster: u32,
    /// The glyph id in the font (uint16 per the OpenType spec).
    pub glyph_id: u16,
    /// The index of the used face in `ShapedBlock::faces`.
    pub face_index: u16,
    /// The glyph markers.
    pub flags: GlyphFlags,
}

/// A run of glyphs shaped with a single font face.
#[derive(Debug, Clone)]
pub struct GlyphRun<F: FontFace> {
    /// The font face used by this run.
    pub face: F,
    /// The glyphs of this run, in logical order.
    pub glyphs: Vec<ShapedGlyph>,
}

/// The shaping service, implemented by platform specific crates or the built-in pure-Rust shaper.
pub trait Shaper {
    /// The font face type of this backend.
    type Face: FontFace;

    /// Shape `text[range]` with the given face, using the rest of `text`
    /// as shaping context. Glyph clusters in the result are absolute
    /// UTF-8 offsets in `text`.
    ///
    /// The implementation may use additional faces beyond the given one
    /// (e.g. platform cascade lists); actually used faces are reported
    /// per run. Glyphs that cannot be produced are kept in the result
    /// with `GlyphFlags::MISSING` set.
    fn shape(
        &self,
        text: &str,
        range: std::ops::Range<usize>,
        face: &Self::Face,
        options: &ShapeOptions,
    ) -> Vec<GlyphRun<Self::Face>>;
}
