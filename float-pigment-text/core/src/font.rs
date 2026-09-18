//! Font lookup abstraction.

use crate::shaper::Language;

/// The name of a font family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FamilyName {
    /// A specific family name.
    Title(Box<str>),
    /// The generic serif family.
    Serif,
    /// The generic sans-serif family.
    SansSerif,
    /// The generic monospace family.
    Monospace,
    /// The generic cursive family.
    Cursive,
    /// The generic fantasy family.
    Fantasy,
    /// The system UI family.
    SystemUi,
}

/// The font style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontStyle {
    /// Normal style.
    #[default]
    Normal,
    /// Italic style.
    Italic,
    /// Oblique style.
    Oblique,
}

/// The font weight, e.g. 400 for normal and 700 for bold.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Weight(pub f32);

impl Weight {
    /// The normal weight.
    pub const NORMAL: Weight = Weight(400.0);
    /// The bold weight.
    pub const BOLD: Weight = Weight(700.0);
}

/// The font stretch ratio, 1.0 for normal.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Stretch(pub f32);

impl Stretch {
    /// The normal stretch.
    pub const NORMAL: Stretch = Stretch(1.0);
}

/// The desired properties of a font face.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FontProperties {
    /// The style.
    pub style: FontStyle,
    /// The weight.
    pub weight: Weight,
    /// The stretch.
    pub stretch: Stretch,
}

/// A font lookup request.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontQuery {
    /// The desired families, in priority order.
    pub families: Box<[FamilyName]>,
    /// The desired properties.
    pub properties: FontProperties,
}

/// The identifier of a font face, unique within a `FontProvider`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontFaceId(pub u64);

/// The vertical metrics of a font face at a specific size.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FontMetrics {
    /// The distance above the baseline (usually positive).
    pub ascent: f32,
    /// The distance below the baseline (positive).
    pub descent: f32,
    /// The suggested gap between two lines.
    pub line_gap: f32,
}

/// A resolved font face.
///
/// The concrete representation is defined by the implementing backend,
/// e.g. a `CTFont` wrapper or a parsed in-memory font.
/// It should be cheap to clone, e.g. wrapping an `Arc`.
pub trait FontFace: Clone + std::fmt::Debug {
    /// The identifier of this font face.
    fn id(&self) -> FontFaceId;

    /// The vertical metrics at the given size.
    fn metrics(&self, size: f32) -> FontMetrics;
}

/// An ordered list of font faces resolved from a `FontQuery`.
#[derive(Debug, Clone, Default)]
pub struct ResolvedFontList<F: FontFace> {
    faces: Vec<F>,
}

impl<F: FontFace> ResolvedFontList<F> {
    /// Build a list from faces in priority order.
    pub fn from_faces(faces: Vec<F>) -> Self {
        Self { faces }
    }

    /// The resolved faces, in priority order.
    pub fn faces(&self) -> &[F] {
        &self.faces
    }

    /// Append fallback faces to the list.
    pub fn append_fallbacks(&mut self, fallbacks: Vec<F>) {
        self.faces.extend(fallbacks);
    }
}

/// The font lookup service, implemented by platform specific crates.
pub trait FontProvider {
    /// The font face type of this backend.
    type Face: FontFace;

    /// Resolve a `FontQuery` into an ordered font face list.
    fn resolve(&self, query: &FontQuery) -> ResolvedFontList<Self::Face>;

    /// Find faces that can render the given character,
    /// used when the resolved list yields missing glyphs.
    ///
    /// `language` is the content language: for Han characters the covering
    /// faces should be ordered to prefer the variant designed for the
    /// content language (Han unification).
    fn fallback_for(
        &self,
        sample: char,
        language: Option<Language>,
        resolved: &ResolvedFontList<Self::Face>,
    ) -> Vec<Self::Face>;
}
