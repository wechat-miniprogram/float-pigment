//! The content model of a text block.

use std::num::NonZeroU32;
use std::sync::Arc;

use float_pigment_css::length_num::LengthNum;
use float_pigment_css::typing::{
    Direction, Overflow, TextAlign, WhiteSpace, WordBreak, WritingMode,
};

use crate::font::FontQuery;
use crate::geometry::Size;
use crate::shaper::{Language, Script};

/// The behavior when text overflows the block in the inline direction.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextOverflow {
    /// Clipped without any marker.
    #[default]
    Clip,
    /// Truncated with an ellipsis (`…`).
    Ellipsis,
    /// Truncated with a custom marker text.
    Custom(Box<str>),
}

/// The orientation of glyphs relative to the baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextOrientation {
    /// Horizontal scripts are upright; vertical scripts are sideways.
    #[default]
    Mixed,
    /// All glyphs are upright.
    Upright,
    /// All glyphs are sideways.
    Sideways,
}

/// Vertical align of an inline item relative to the line baseline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalAlign<L: LengthNum> {
    /// Aligned with the baseline, plus an offset.
    Baseline {
        /// The offset from the baseline.
        offset: L,
    },
    /// Aligned with the top of the line.
    Top,
    /// Aligned with the middle of the line.
    Middle,
    /// Aligned with the bottom of the line.
    Bottom,
    /// Aligned with the top of the text.
    TextTop,
    /// Aligned with the bottom of the text.
    TextBottom,
}

impl<L: LengthNum> Default for VerticalAlign<L> {
    fn default() -> Self {
        Self::Baseline { offset: L::zero() }
    }
}

/// Spacing around an inline.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Spacing<L: LengthNum> {
    /// The spacing before the inline content in the inline direction.
    pub inline_start: L,
    /// The spacing after the inline content in the inline direction.
    pub inline_end: L,
    /// The spacing above the inline content in the block direction.
    pub block_start: L,
    /// The spacing below the inline content in the block direction.
    pub block_end: L,
}

/// Layout properties of a text block.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockProperty<L: LengthNum> {
    /// The indent of the first line.
    pub text_indent: L,
    /// The horizontal alignment.
    pub text_align: TextAlign,
    /// The line height; `None` means deriving from the content.
    pub line_height: Option<L>,
    /// The base direction of the block.
    pub direction: Direction,
    /// The writing mode of the block.
    pub writing_mode: WritingMode,
    /// The text orientation of the block.
    pub text_orientation: TextOrientation,
    /// The behavior when text overflows the block in the inline direction.
    pub text_overflow: TextOverflow,
    /// The overflow mode in the inline direction.
    pub overflow_inline: Overflow,
    /// The maximum line count before ellipsis.
    pub line_clamp: Option<NonZeroU32>,
    /// The default font of the block.
    pub font_query: FontQuery,
    /// The default font size of the block.
    pub font_size: L,
    /// The script of the content; `None` means guessing from the text.
    pub script: Option<Script>,
    /// The BCP 47 language tag of the content.
    pub language: Option<Language>,
}

impl<L: LengthNum> Default for BlockProperty<L> {
    fn default() -> Self {
        Self {
            text_indent: L::zero(),
            text_align: TextAlign::Left,
            line_height: None,
            direction: Direction::Auto,
            writing_mode: WritingMode::HorizontalTb,
            text_orientation: TextOrientation::Mixed,
            text_overflow: TextOverflow::Clip,
            overflow_inline: Overflow::Visible,
            line_clamp: None,
            font_query: FontQuery::default(),
            font_size: L::from_i32(16),
            script: None,
            language: None,
        }
    }
}

/// Layout properties of an inline.
#[derive(Debug, Clone, PartialEq)]
pub struct InlineProperty<L: LengthNum> {
    /// The vertical alignment relative to the line baseline.
    pub vertical_align: VerticalAlign<L>,
    /// The white space handling mode.
    pub white_space: WhiteSpace,
    /// The word breaking mode.
    pub word_break: WordBreak,
    /// The extra spacing between glyphs.
    pub letter_spacing: L,
    /// The extra spacing between words.
    pub word_spacing: L,
    /// The font size; `None` means inheriting from the block.
    pub font_size: Option<L>,
    /// The base direction; `Auto` means inheriting from the block.
    pub direction: Direction,
    /// The font; `None` means inheriting from the block.
    pub font_query: Option<FontQuery>,
}

impl<L: LengthNum> Default for InlineProperty<L> {
    fn default() -> Self {
        Self {
            vertical_align: VerticalAlign::default(),
            white_space: WhiteSpace::Normal,
            word_break: WordBreak::BreakWord,
            letter_spacing: L::zero(),
            word_spacing: L::zero(),
            font_size: None,
            direction: Direction::Auto,
            font_query: None,
        }
    }
}

/// A text segment inside a block.
#[derive(Debug, Clone, PartialEq)]
pub struct InlineText<L: LengthNum> {
    /// The text content.
    pub content: Arc<str>,
    /// The layout properties.
    pub property: InlineProperty<L>,
    /// The spacing around the text.
    pub spacing: Spacing<L>,
}

/// An opaque inline object inside a block, e.g. an inline image or node tree.
#[derive(Debug, Clone, PartialEq)]
pub struct InlineItem<L: LengthNum> {
    /// The size of the item.
    pub size: Size<L>,
    /// The distance from the item top to its baseline; `None` means the item bottom.
    pub baseline_ascent: Option<L>,
    /// The layout properties (only vertical align and spacing related fields are used).
    pub property: InlineProperty<L>,
    /// The spacing around the item.
    pub spacing: Spacing<L>,
}

/// An inline inside a block.
#[derive(Debug, Clone, PartialEq)]
pub enum Inline<L: LengthNum> {
    /// A text segment.
    Text(InlineText<L>),
    /// An opaque inline object.
    Item(InlineItem<L>),
}

/// A text block to be laid out.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block<L: LengthNum> {
    /// The layout properties of the block.
    pub property: BlockProperty<L>,
    /// The inline contents of the block.
    pub inlines: Vec<Inline<L>>,
}
