//! Stage B tests: line breaking and justification, with hand-crafted fonts.

mod common;

use std::num::NonZeroU32;
use std::sync::Arc;

use common::{build_font, FontSpec};
use float_pigment_text_layout::font::{FamilyName, FontProperties, FontQuery};
use float_pigment_text_layout::layout::LayoutResult;
use float_pigment_text_layout::model::{
    Block, BlockProperty, Inline, InlineItem, InlineProperty, InlineText, Spacing, TextOrientation,
    TextOverflow,
};
use float_pigment_text_layout::rust_font::{MemoryFontProvider, ParsedFont, RustShaper};
use float_pigment_text_layout::ShapedBlock;
use float_pigment_css::typing::{Direction, Overflow, TextAlign, WhiteSpace, WordBreak, WritingMode};

fn spec() -> FontSpec {
    // uniform 500-advance glyphs (8px at size 16), space 250 (4px)
    FontSpec {
        glyphs: vec![
            ('A', 500),
            ('B', 500),
            ('C', 500),
            ('D', 500),
            (' ', 250),
            ('\u{5D4}', 500), // ה
            ('\u{5D5}', 500), // ו
            ('…', 400),
        ],
        ..Default::default()
    }
}

fn setup() -> (MemoryFontProvider, RustShaper, FontQuery) {
    let mut provider = MemoryFontProvider::new();
    let data = Arc::new(build_font(&spec()));
    provider
        .register(data, 0, Box::new([FamilyName::SansSerif]), Default::default())
        .expect("test font should parse");
    let query = FontQuery {
        families: Box::new([FamilyName::SansSerif]),
        properties: FontProperties::default(),
    };
    (provider, RustShaper::new(), query)
}

fn layout_text_with(
    provider: &MemoryFontProvider,
    shaper: &RustShaper,
    query: &FontQuery,
    content: &str,
    width: f32,
    property: BlockProperty<f32>,
    inline_property: InlineProperty<f32>,
) -> LayoutResult<f32, ParsedFont> {
    let block: Block<f32> = Block {
        property: BlockProperty {
            font_query: query.clone(),
            font_size: 16.0,
            ..property
        },
        inlines: vec![Inline::Text(InlineText {
            content: Arc::from(content),
            property: inline_property,
            spacing: Spacing::default(),
        })],
    };
    let shaped = Arc::new(ShapedBlock::<f32, ParsedFont>::new(&block, provider, shaper));
    shaped.layout(width)
}

fn layout_text(
    provider: &MemoryFontProvider,
    shaper: &RustShaper,
    query: &FontQuery,
    content: &str,
    width: f32,
    property: BlockProperty<f32>,
) -> LayoutResult<f32, ParsedFont> {
    layout_text_with(provider, shaper, query, content, width, property, InlineProperty::default())
}

fn assert_close(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-4, "{a} != {b}");
}

#[test]
fn single_line_positions() {
    let (provider, shaper, query) = setup();
    let result = layout_text(&provider, &shaper, &query, "A B", 1000.0, Default::default());
    assert_eq!(result.lines.len(), 1);
    assert_eq!(result.glyphs.len(), 3);
    // advances: A=8, space=4
    assert_close(result.glyphs[0].origin.x, 0.0);
    assert_close(result.glyphs[1].origin.x, 8.0);
    assert_close(result.glyphs[2].origin.x, 12.0);
    assert_close(result.size.width, 20.0);
    // line height: ascent 12.8 + descent 3.2 + no gap on first line
    assert_close(result.size.height, 16.0);
    assert_close(result.lines[0].baseline_offset, 12.8);
}

#[test]
fn wraps_at_space_and_trims() {
    let (provider, shaper, query) = setup();
    // "AA AA" = 8*4 + 4 = 36px; width 34px fits only "AA " (trimmed to 16)
    let result = layout_text(&provider, &shaper, &query, "AA AA", 34.0, Default::default());
    assert_eq!(result.lines.len(), 2);
    assert_eq!(result.lines[0].glyph_range.len(), 2);
    assert_eq!(result.lines[1].glyph_range.len(), 2);
    assert_close(result.lines[0].inline_end, 16.0);
    assert_close(result.size.width, 16.0);
    assert_close(result.size.height, 16.0 * 2.0 + 3.2); // line gap 200 units
}

#[test]
fn collapsed_whitespace() {
    let (provider, shaper, query) = setup();
    let single = layout_text(&provider, &shaper, &query, "A A", 1000.0, Default::default());
    let double = layout_text(&provider, &shaper, &query, "A  A", 1000.0, Default::default());
    assert_eq!(double.glyphs.len(), single.glyphs.len());
    assert_close(double.size.width, single.size.width);
}

#[test]
fn pre_keeps_whitespace_and_newline_breaks() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::Pre,
        ..Default::default()
    };
    let result = layout_text_with(&provider, &shaper, &query, "A  A\nB", 1000.0, Default::default(), inline_property);
    // note: Pre keeps both spaces, and \n forces a break
    assert_eq!(result.lines.len(), 2);
    assert!(result.lines[0].glyph_range.len() >= 4);
    assert_eq!(result.lines[1].glyph_range.len(), 1);
}

#[test]
fn nowrap_never_wraps() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::NoWrap,
        ..Default::default()
    };
    let result = layout_text_with(&provider, &shaper, &query, "AA AA", 34.0, Default::default(), inline_property);
    assert_eq!(result.lines.len(), 1);
    assert_close(result.size.width, 36.0);
}

#[test]
fn ellipsis_truncates() {
    let (provider, shaper, query) = setup();
    let property = BlockProperty {
        text_overflow: TextOverflow::Ellipsis,
        overflow_inline: Overflow::Hidden,
        ..Default::default()
    };
    // "AAA AAA" content 56px+; width 30px, ellipsis 6.4px
    let result = layout_text(&provider, &shaper, &query, "AAA AAA", 30.0, property);
    assert_eq!(result.lines.len(), 1);
    // 3 A's (24px) + ellipsis (6.4px) = 30.4 > 30 → truncate to 2 A's
    assert_eq!(result.glyphs.len(), 3);
    assert_close(result.glyphs[2].origin.x, 16.0);
    // the last glyph is the ellipsis from extra glyphs
    assert_eq!(result.shaped_glyph(result.glyphs[2].glyph_ref).glyph_id, spec().glyph_id_of('…'));
}

#[test]
fn line_clamp_two_lines() {
    let (provider, shaper, query) = setup();
    let property = BlockProperty {
        text_overflow: TextOverflow::Ellipsis,
        overflow_inline: Overflow::Hidden,
        line_clamp: NonZeroU32::new(2),
        ..Default::default()
    };
    let result = layout_text(&provider, &shaper, &query, "AA AA AA AA", 34.0, property);
    assert_eq!(result.lines.len(), 2);
    // second line: "AA" + ellipsis
    let second = result.lines[1].glyph_range.clone();
    assert_eq!(second.len(), 3);
}

#[test]
fn rtl_visual_order() {
    let (provider, shaper, query) = setup();
    // logical order: ה(0) ו(2) A(4); base direction RTL from the Hebrew
    let result = layout_text(&provider, &shaper, &query, "\u{5D4}\u{5D5}A", 1000.0, Default::default());
    assert_eq!(result.lines.len(), 1);
    assert_eq!(result.glyphs.len(), 3);
    // visual left-to-right: A, ו, ה — i.e. glyph_refs [2, 0, 1]
    let refs: Vec<u32> = result.glyphs.iter().map(|g| g.glyph_ref).collect();
    assert_eq!(refs, vec![2, 0, 1]);
    assert_close(result.glyphs[0].origin.x, 0.0);
    assert_close(result.glyphs[1].origin.x, 8.0);
    assert_close(result.glyphs[2].origin.x, 16.0);
}

#[test]
fn ltr_with_rtl_run_visual_order() {
    let (provider, shaper, query) = setup();
    // logical order: A(0) ה(1) ו(3); base direction LTR from the Latin
    let result = layout_text(&provider, &shaper, &query, "A\u{5D4}\u{5D5}", 1000.0, Default::default());
    assert_eq!(result.glyphs.len(), 3);
    // visual left-to-right: A, ו, ה — clusters [0, 3, 1]
    let clusters: Vec<u32> = result
        .glyphs
        .iter()
        .map(|g| result.shaped_glyph(g.glyph_ref).cluster)
        .collect();
    assert_eq!(clusters, vec![0, 3, 1]);
    assert_close(result.glyphs[1].origin.x, 8.0);
    assert_close(result.glyphs[2].origin.x, 16.0);
}

#[test]
fn inline_item_positioned() {
    let (provider, shaper, query) = setup();
    let block: Block<f32> = Block {
        property: BlockProperty {
            font_query: query,
            font_size: 16.0,
            ..Default::default()
        },
        inlines: vec![
            Inline::Text(InlineText {
                content: Arc::from("A"),
                property: InlineProperty::default(),
                spacing: Spacing::default(),
            }),
            Inline::Item(InlineItem {
                size: euclid::Size2D::new(10.0, 20.0),
                baseline_ascent: None,
                property: InlineProperty::default(),
                spacing: Spacing::default(),
            }),
            Inline::Text(InlineText {
                content: Arc::from("B"),
                property: InlineProperty::default(),
                spacing: Spacing::default(),
            }),
        ],
    };
    let shaped = Arc::new(ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper));
    let result = shaped.layout(1000.0);
    assert_eq!(result.items.len(), 1);
    assert_close(result.items[0].rect.origin.x, 8.0); // after "A"
    // glyph after the item
    let last = result.glyphs.last().unwrap();
    assert_close(last.origin.x, 18.0);
    // item bottom aligned to baseline: baseline 12.8, height 20 → ascent extended
    assert!(result.size.height >= 20.0);
}

#[test]
fn text_align_right_and_center() {
    let (provider, shaper, query) = setup();
    let right = layout_text(
        &provider, &shaper, &query, "A", 100.0,
        BlockProperty {
            text_align: TextAlign::Right,
            ..Default::default()
        },
    );
    assert_close(right.glyphs[0].origin.x, 92.0);
    let center = layout_text(
        &provider, &shaper, &query, "A", 100.0,
        BlockProperty {
            text_align: TextAlign::Center,
            ..Default::default()
        },
    );
    assert_close(center.glyphs[0].origin.x, 46.0);
}

#[test]
fn justify_stretches_word_gaps() {
    let (provider, shaper, query) = setup();
    // "AA AA" content 36px in 68px: one word gap absorbs 32px
    let result = layout_text(
        &provider, &shaper, &query, "AA AA", 68.0,
        BlockProperty {
            text_align: TextAlign::Justify,
            ..Default::default()
        },
    );
    assert_eq!(result.lines.len(), 1);
    // all glyphs including the space: A A sp A A
    assert_eq!(result.glyphs.len(), 5);
    // the second word starts at 16 + 4 + 32 = 52
    assert_close(result.glyphs[3].origin.x, 52.0);
}

#[test]
fn custom_ellipsis_text() {
    let (provider, shaper, query) = setup();
    let property = BlockProperty {
        text_overflow: TextOverflow::Custom("BB".into()),
        overflow_inline: Overflow::Hidden,
        ..Default::default()
    };
    // "AAA AAA" width 30: truncate until "BB" (16px) fits
    let result = layout_text(&provider, &shaper, &query, "AAA AAA", 30.0, property);
    assert_eq!(result.lines.len(), 1);
    // "A" (8px) + "BB" (16px) = 24px
    assert_eq!(result.glyphs.len(), 3);
    assert_eq!(result.shaped_glyph(result.glyphs[1].glyph_ref).glyph_id, spec().glyph_id_of('B'));
    // ellipsis glyphs have no source cluster
    assert_eq!(result.shaped_glyph(result.glyphs[1].glyph_ref).cluster, u32::MAX);
}

#[test]
fn trailing_whitespace_overflows_without_wrapping() {
    let (provider, shaper, query) = setup();
    // "AA " content 20px in 18px: the trailing space overflows but is
    // trimmed instead of causing a wrap
    let result = layout_text(&provider, &shaper, &query, "AA ", 18.0, Default::default());
    assert_eq!(result.lines.len(), 1);
    assert_close(result.size.width, 16.0);
    // a line consisting of only spaces vanishes
    let spaces = layout_text(&provider, &shaper, &query, "A   ", 12.0, Default::default());
    assert_eq!(spaces.lines.len(), 1);
    assert_close(spaces.size.width, 8.0);
}

#[test]
fn white_space_pre_wrap() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::PreWrap,
        ..Default::default()
    };
    // PreWrap: spaces kept, wrapping and newline breaks allowed
    let result = layout_text_with(&provider, &shaper, &query, "A  A\nB", 20.0, Default::default(), inline_property);
    // line 1: "A  " (16px, spaces kept), then "A" wraps, "B" after newline
    assert_eq!(result.lines.len(), 3);
    assert_eq!(result.lines[0].glyph_range.len(), 3);
    assert_eq!(result.lines[1].glyph_range.len(), 1);
    assert_eq!(result.lines[2].glyph_range.len(), 1);
}

#[test]
fn white_space_pre_line() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::PreLine,
        ..Default::default()
    };
    // PreLine: spaces collapsed, newline breaks allowed
    let collapsed = layout_text_with(&provider, &shaper, &query, "A  A\nB", 1000.0, Default::default(), inline_property.clone());
    assert_eq!(collapsed.lines.len(), 2);
    assert_eq!(collapsed.lines[0].glyph_range.len(), 3); // "A A"
    let single = layout_text(&provider, &shaper, &query, "A A", 1000.0, Default::default());
    assert_close(collapsed.lines[0].inline_end, single.lines[0].inline_end);
}

#[test]
fn white_space_wx_pre_edit() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::WxPreEdit,
        ..Default::default()
    };
    // WxPreEdit behaves like PreWrap: spaces kept
    let result = layout_text_with(&provider, &shaper, &query, "A  A", 1000.0, Default::default(), inline_property);
    assert_eq!(result.lines.len(), 1);
    assert_eq!(result.lines[0].glyph_range.len(), 4);
    assert_close(result.size.width, 24.0);
}

#[test]
fn word_break_break_all() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        word_break: WordBreak::BreakAll,
        ..Default::default()
    };
    // BreakAll: breaks between any graphemes, even mid-word
    let result = layout_text_with(&provider, &shaper, &query, "AAAA", 20.0, Default::default(), inline_property);
    assert_eq!(result.lines.len(), 2);
    assert_eq!(result.lines[0].glyph_range.len(), 2);
    assert_eq!(result.lines[1].glyph_range.len(), 2);
}

#[test]
fn word_break_keep_all() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        word_break: WordBreak::KeepAll,
        ..Default::default()
    };
    // KeepAll: no breaks between letters, the content overflows
    let result = layout_text_with(&provider, &shaper, &query, "AAAA", 20.0, Default::default(), inline_property.clone());
    assert_eq!(result.lines.len(), 1);
    assert_close(result.size.width, 32.0);
    // but breaks at spaces are still allowed
    let with_space = layout_text_with(&provider, &shaper, &query, "AA AA", 20.0, Default::default(), inline_property);
    assert_eq!(with_space.lines.len(), 2);
}

#[test]
fn word_break_break_word_emergency() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        word_break: WordBreak::BreakWord,
        ..Default::default()
    };
    // BreakWord: emergency breaks when no legal break point exists
    let result = layout_text_with(&provider, &shaper, &query, "AAAA", 10.0, Default::default(), inline_property);
    assert_eq!(result.lines.len(), 4);
}

#[test]
fn direction_combinations() {
    let (provider, shaper, query) = setup();
    let clusters_of = |direction: Direction, content: &str| {
        let result = layout_text(
            &provider, &shaper, &query, content, 1000.0,
            BlockProperty {
                direction,
                ..Default::default()
            },
        );
        result
            .glyphs
            .iter()
            .map(|g| result.shaped_glyph(g.glyph_ref).cluster)
            .collect::<Vec<u32>>()
    };
    // auto: Latin content → LTR
    assert_eq!(clusters_of(Direction::Auto, "AB"), vec![0, 1]);
    // auto: Hebrew content → RTL, Hebrew alone stays logical==visual reversed
    assert_eq!(clusters_of(Direction::Auto, "\u{5D4}\u{5D5}"), vec![2, 0]);
    // forced LTR with Hebrew: level-1 run reversed to visual order
    assert_eq!(clusters_of(Direction::LTR, "\u{5D4}\u{5D5}"), vec![2, 0]);
    // forced RTL with Latin: base direction RTL, visual order unchanged
    assert_eq!(clusters_of(Direction::RTL, "AB"), vec![0, 1]);
    // forced RTL with mixed content: level array [2,1,1] resolves to [ו,ה,A]
    assert_eq!(clusters_of(Direction::RTL, "A\u{5D4}\u{5D5}"), vec![3, 1, 0]);
}

#[test]
fn direction_affects_start_end_alignment() {
    let (provider, shaper, query) = setup();
    let start_x = |direction: Direction| {
        let result = layout_text(
            &provider, &shaper, &query, "A", 100.0,
            BlockProperty {
                direction,
                text_align: TextAlign::Start,
                ..Default::default()
            },
        );
        result.glyphs[0].origin.x
    };
    assert_close(start_x(Direction::LTR), 0.0);
    assert_close(start_x(Direction::RTL), 92.0);
    let end_x = |direction: Direction| {
        let result = layout_text(
            &provider, &shaper, &query, "A", 100.0,
            BlockProperty {
                direction,
                text_align: TextAlign::End,
                ..Default::default()
            },
        );
        result.glyphs[0].origin.x
    };
    assert_close(end_x(Direction::LTR), 92.0);
    assert_close(end_x(Direction::RTL), 0.0);
}

#[test]
fn baselines_and_glyph_origins() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::Pre,
        ..Default::default()
    };
    // two lines from a newline: baselines stack with line gap
    let result = layout_text_with(&provider, &shaper, &query, "AA\nAA", 1000.0, Default::default(), inline_property);
    assert_eq!(result.lines.len(), 2);
    assert_close(result.lines[0].baseline_offset, 12.8);
    assert_close(result.lines[1].baseline_offset, 32.0);
    // glyph origins sit on the baseline (no shaper offsets in the test font)
    assert_close(result.glyphs[0].origin.y, 12.8);
    assert_close(result.glyphs[1].origin.y, 12.8);
    assert_close(result.glyphs[2].origin.y, 32.0);
    assert_close(result.size.height, 35.2);
}

#[test]
fn line_height_half_leading() {
    let (provider, shaper, query) = setup();
    let inline_property = InlineProperty {
        white_space: WhiteSpace::Pre,
        ..Default::default()
    };
    let property = BlockProperty {
        line_height: Some(40.0),
        ..Default::default()
    };
    let result = layout_text_with(&provider, &shaper, &query, "AA\nAA", 1000.0, property, inline_property);
    assert_eq!(result.lines.len(), 2);
    // half leading: (40 - 16) / 2 = 12 added above and below
    assert_close(result.lines[0].baseline_offset, 24.8);
    assert_close(result.lines[1].baseline_offset, 64.8);
    assert_close(result.size.height, 80.0);
    assert_close(result.glyphs[0].origin.y, 24.8);
}

#[test]
fn writing_mode_and_orientation_combinations() {
    let (provider, shaper, query) = setup();
    let layout_with_mode = |writing_mode: WritingMode, text_orientation, direction: Direction| {
        layout_text(
            &provider, &shaper, &query, "AA", 1000.0,
            BlockProperty {
                writing_mode,
                text_orientation,
                direction,
                ..Default::default()
            },
        )
    };
    // horizontal: orientation does not affect the layout
    let horizontal = layout_with_mode(WritingMode::HorizontalTb, TextOrientation::Upright, Direction::LTR);
    assert_close(horizontal.size.width, 16.0);
    assert_close(horizontal.size.height, 16.0);
    // vertical-lr mixed: glyphs advance downward, lines stack left-to-right
    let vlr = layout_with_mode(WritingMode::VerticalLr, TextOrientation::Mixed, Direction::LTR);
    assert_eq!(vlr.lines.len(), 1);
    assert_eq!(vlr.glyphs.len(), 2);
    assert!(vlr.glyphs[1].origin.y > vlr.glyphs[0].origin.y);
    assert!(vlr.size.width > 0.0 && vlr.size.height > vlr.size.width);
    // vertical-rl mixed: the line is placed from the right edge
    let vrl = layout_with_mode(WritingMode::VerticalRl, TextOrientation::Mixed, Direction::LTR);
    assert_eq!(vrl.lines.len(), 1);
    assert_close(vrl.lines[0].baseline_offset, vrl.size.width - vlr.lines[0].baseline_offset);
    // vertical-lr sideways: horizontal shaping, advances go downward too
    let sideways = layout_with_mode(WritingMode::VerticalLr, TextOrientation::Sideways, Direction::LTR);
    assert_eq!(sideways.glyphs.len(), 2);
    assert!(sideways.glyphs[1].origin.y > sideways.glyphs[0].origin.y);
    // upright behaves like mixed in the current implementation
    let upright = layout_with_mode(WritingMode::VerticalLr, TextOrientation::Upright, Direction::LTR);
    assert_eq!(upright.glyphs.len(), vlr.glyphs.len());
}
