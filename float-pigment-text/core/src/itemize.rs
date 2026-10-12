//! Step 1: split the block content into items (bidi paragraphs, style runs).

use std::ops::Range;

use float_pigment_css::length_num::LengthNum;
use float_pigment_css::typing::Direction;
use unicode_bidi::{BidiInfo, Level};

use crate::model::{Block, Inline};
use crate::shaper::ResolvedDirection;

/// A text item with uniform style and direction.
#[derive(Debug, Clone)]
pub(crate) struct TextItem<L: LengthNum> {
    /// The UTF-8 byte range in the flattened text.
    pub(crate) flattened_range: Range<u32>,
    /// The index of the source `Inline::Text` in the block.
    pub(crate) text_index: u32,
    /// The bidi level resolved by UAX#9.
    pub(crate) bidi_level: u8,
    /// The resolved base direction.
    pub(crate) direction: ResolvedDirection,
    /// The resolved font size.
    pub(crate) font_size: L,
}

/// The flattened text of a block, in which each inline item is
/// represented by a U+FFFC object replacement character.
#[derive(Debug, Clone, Default)]
pub(crate) struct FlattenedText {
    pub(crate) text: Box<str>,
    /// Maps flattened U+FFFC UTF-8 offsets to `Inline::Item` indexes.
    pub(crate) item_positions: Vec<(u32, u32)>,
    /// The flattened UTF-8 range of each `Inline::Text`.
    pub(crate) text_ranges: Vec<Range<u32>>,
}

/// The itemization result.
#[derive(Debug, Clone)]
pub(crate) struct Itemized<L: LengthNum> {
    pub(crate) flattened: FlattenedText,
    /// The text items, sorted by flattened position.
    pub(crate) text_items: Vec<TextItem<L>>,
    /// The bidi level of each `Inline::Item`.
    pub(crate) item_levels: Vec<u8>,
    /// The resolved base direction of the block.
    pub(crate) base_direction: ResolvedDirection,
}

impl<L: LengthNum> Default for Itemized<L> {
    fn default() -> Self {
        Self {
            flattened: FlattenedText::default(),
            text_items: vec![],
            item_levels: vec![],
            base_direction: ResolvedDirection::LTR,
        }
    }
}

pub(crate) fn itemize<L: LengthNum>(block: &Block<L>) -> Itemized<L> {
    let mut text = String::new();
    let mut item_positions = Vec::new();
    let mut text_ranges = Vec::new();
    for inline in &block.inlines {
        match inline {
            Inline::Text(t) => {
                let start = text.len() as u32;
                text.push_str(&t.content);
                text_ranges.push(start..text.len() as u32);
            }
            Inline::Item(_) => {
                item_positions.push((text.len() as u32, item_positions.len() as u32));
                text.push('\u{FFFC}');
            }
        }
    }

    let base_level = match block.property.direction {
        Direction::Auto => None,
        Direction::LTR => Some(Level::ltr()),
        Direction::RTL => Some(Level::rtl()),
    };
    let bidi_info = BidiInfo::new(&text, base_level);

    let item_levels = item_positions
        .iter()
        .map(|(offset, _)| bidi_info.levels[*offset as usize].number())
        .collect();

    let mut text_items = Vec::new();
    for (text_index, inline) in block.inlines.iter().filter_map(|i| match i {
        Inline::Text(t) => Some(t),
        Inline::Item(_) => None,
    }).enumerate() {
        let range = text_ranges[text_index].clone();
        if range.is_empty() {
            continue;
        }
        let font_size = inline.property.font_size.unwrap_or(block.property.font_size);
        let mut run_start = range.start as usize;
        let mut run_level = bidi_info.levels[run_start];
        for offset in range.start as usize + 1..range.end as usize {
            let level = bidi_info.levels[offset];
            if level != run_level {
                text_items.push(TextItem {
                    flattened_range: run_start as u32..offset as u32,
                    text_index: text_index as u32,
                    bidi_level: run_level.number(),
                    direction: if run_level.is_rtl() {
                        ResolvedDirection::RTL
                    } else {
                        ResolvedDirection::LTR
                    },
                    font_size,
                });
                run_start = offset;
                run_level = level;
            }
        }
        text_items.push(TextItem {
            flattened_range: run_start as u32..range.end,
            text_index: text_index as u32,
            bidi_level: run_level.number(),
            direction: if run_level.is_rtl() {
                ResolvedDirection::RTL
            } else {
                ResolvedDirection::LTR
            },
            font_size,
        });
    }

    let base_direction = bidi_info
        .paragraphs
        .first()
        .map(|para| {
            if para.level.is_rtl() {
                ResolvedDirection::RTL
            } else {
                ResolvedDirection::LTR
            }
        })
        .unwrap_or(match block.property.direction {
            Direction::RTL => ResolvedDirection::RTL,
            _ => ResolvedDirection::LTR,
        });

    Itemized {
        flattened: FlattenedText {
            text: text.into_boxed_str(),
            item_positions,
            text_ranges,
        },
        text_items,
        item_levels,
        base_direction,
    }
}
