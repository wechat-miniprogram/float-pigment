//! Glyph rasterization abstraction.

use euclid::UnknownUnit;

use crate::font::FontFace;

/// The pixel data of a rasterized glyph.
#[derive(Debug, Clone, PartialEq)]
pub enum GlyphBitmapData {
    /// 8-bit alpha mask, row-major, tightly packed.
    Alpha(Vec<u8>),
    /// 32-bit RGBA, row-major, tightly packed.
    Rgba {
        /// The pixel data.
        data: Vec<u8>,
        /// Whether the color channels are premultiplied with alpha.
        premultiplied: bool,
    },
    /// Undecoded image data (e.g. PNG from a bitmap strike), to be decoded by the host.
    Encoded {
        /// The MIME type of the data, e.g. `image/png`.
        mime_type: Box<str>,
        /// The encoded data.
        data: Vec<u8>,
    },
}

/// A rasterized glyph bitmap.
#[derive(Debug, Clone, PartialEq)]
pub struct GlyphBitmap {
    /// The bitmap origin relative to the glyph origin (baseline start point).
    pub origin: euclid::Point2D<f32, UnknownUnit>,
    /// The bitmap size in pixels.
    pub size: euclid::Size2D<u32, UnknownUnit>,
    /// The pixel data.
    pub data: GlyphBitmapData,
}

/// The glyph rasterization service, implemented by platform specific crates
/// or the built-in pure-Rust rasterizer.
pub trait Rasterizer {
    /// The font face type of this backend.
    type Face: FontFace;

    /// Rasterize a glyph at the given size.
    ///
    /// `subpixel_offset` is the fractional part of the glyph origin,
    /// which affects the antialiasing result.
    fn rasterize(
        &self,
        face: &Self::Face,
        glyph_id: u32,
        size: f32,
        subpixel_offset: (f32, f32),
    ) -> Option<GlyphBitmap>;
}
