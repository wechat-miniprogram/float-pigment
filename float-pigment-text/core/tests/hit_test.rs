//! Hit testing and selection rect tests, with hand-crafted fonts.

mod common;

use std::sync::Arc;

use common::{build_font, FontSpec};
use float_pigment_css::typing::{Direction, TextAlign, WhiteSpace, WritingMode};
use float_pigment_text_layout::font::{FamilyName, FontProperties, FontQuery};
use float_pigment_text_layout::layout::{CaretAffinity, LayoutResult};
use float_pigment_text_layout::model::TextOrientation;
use float_pigment_text_layout::model::{
    Block, BlockProperty, Inline, InlineProperty, InlineText, Spacing,
};
use float_pigment_text_layout::rust_font::{MemoryFontProvider, ParsedFont, RustShaper};
use float_pigment_text_layout::ShapedBlock;

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
            ('中', 500),
        ],
        ..Default::default()
    }
}

#[test]
fn hit_rtl_break_at_space() {
    // "ABה וCD" with LTR base: bytes A0 B1 ה2..4 sp4 ו5..7 C7 D8;
    // the RTL run is "ה ו" (bytes 2..7). Breaks at the space:
    // line 0 is "ABה" (visual A B ה), line 1 is "וCD" (visual ו C D)
    let result = layout_text("AB\u{5D4} \u{5D5}CD", 25.0, Default::default());
    assert_eq!(result.lines.len(), 2);
    assert_eq!(result.glyphs.len(), 6); // the space is trimmed
                                        // with an LTR base direction, the RTL run ו at the line start renders
                                        // at the visual LEFT of the line (UAX#9 L2 is applied per line)
    let first = &result.glyphs[result.line_glyph_range(1).start as usize];
    assert_eq!(result.shaped_glyph(first.glyph_ref).cluster, 5); // ו
    assert_close(first.origin.x, 0.0);
    // line 0: ה at [16, 24), RTL
    assert_eq!(hit(&result, 18.0, 5.0), (0, 4, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 22.0, 5.0), (0, 2, CaretAffinity::Upstream));
    // line 1: ו at [0, 8) RTL, C at [8, 16), D at [16, 24) LTR
    assert_eq!(hit(&result, 2.0, 25.0), (0, 7, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 6.0, 25.0), (0, 5, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 10.0, 25.0), (0, 7, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 18.0, 25.0), (0, 8, CaretAffinity::Upstream));
}

#[test]
fn rect_rtl_break_at_space() {
    let result = layout_text("AB\u{5D4} \u{5D5}CD", 25.0, Default::default());
    // the RTL run "ה ו" (bytes 2..7) spans both lines; the space glyph is
    // trimmed away, so only ה and ו contribute rects
    let rects = result.rect_for_range(0, 2..7);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 16.0); // ה on line 0
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.width, 8.0);
    assert_close(rects[1].origin.x, 0.0); // ו on line 1
    assert_close(rects[1].origin.y, 19.2);
    assert_close(rects[1].size.width, 8.0);
    // the whole text: one rect per line
    let rects = result.rect_for_range(0, 0..9);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].size.width, 24.0);
    assert_close(rects[1].size.width, 24.0);
}

#[test]
fn hit_and_rect_vertical_mixed_cjk() {
    // mixed orientation: Latin sideways (8px along y), CJK upright
    // (16px along y with synthesized vertical metrics)
    let result = layout_text(
        "A中",
        1000.0,
        BlockProperty {
            writing_mode: WritingMode::VerticalLr,
            text_orientation: TextOrientation::Mixed,
            ..Default::default()
        },
    );
    assert_eq!(result.lines.len(), 1);
    assert_close(result.size.height, 24.0);
    // A occupies y [0, 8), 中 (bytes 1..4) occupies y [8, 24)
    assert_eq!(hit(&result, 5.0, 2.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 6.0), (0, 1, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 5.0, 10.0), (0, 1, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 20.0), (0, 4, CaretAffinity::Downstream));

    let rects = result.rect_for_range(0, 0..1);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.width, 16.0);
    assert_close(rects[0].size.height, 8.0);
    let rects = result.rect_for_range(0, 1..4);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].size.width, 16.0);
    assert_close(rects[0].origin.y, 8.0);
    assert_close(rects[0].size.height, 16.0);
}

#[test]
fn hit_and_rect_vertical_upright() {
    // upright orientation: Latin is shaped vertically too, 16px along y
    let result = layout_text(
        "AB",
        1000.0,
        BlockProperty {
            writing_mode: WritingMode::VerticalLr,
            text_orientation: TextOrientation::Upright,
            ..Default::default()
        },
    );
    assert_close(result.size.height, 32.0);
    assert_eq!(hit(&result, 5.0, 3.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 10.0), (0, 1, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 5.0, 20.0), (0, 1, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 30.0), (0, 2, CaretAffinity::Downstream));

    let rects = result.rect_for_range(0, 1..2);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].origin.y, 16.0);
    assert_close(rects[0].size.width, 16.0);
    assert_close(rects[0].size.height, 16.0);
}

#[test]
fn hit_and_rect_vertical_sideways() {
    // sideways orientation: all glyphs shaped horizontally, 8px along y
    let result = layout_text(
        "AB",
        1000.0,
        BlockProperty {
            writing_mode: WritingMode::VerticalLr,
            text_orientation: TextOrientation::Sideways,
            ..Default::default()
        },
    );
    assert_close(result.size.height, 16.0);
    assert_eq!(hit(&result, 5.0, 3.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 10.0), (0, 1, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 13.0), (0, 2, CaretAffinity::Downstream));

    let rects = result.rect_for_range(0, 0..2);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.width, 16.0);
    assert_close(rects[0].size.height, 16.0);
}

#[test]
fn hit_and_rect_vertical_lr_nearest_line() {
    // two lines stack left-to-right in vertical-lr
    let result = layout_text(
        "A B",
        10.0,
        BlockProperty {
            writing_mode: WritingMode::VerticalLr,
            ..Default::default()
        },
    );
    assert_eq!(result.lines.len(), 2);
    assert_close(result.size.width, 35.2);
    // line 0 extent x [0, 16), line 1 extent x [19.2, 35.2)
    assert_eq!(hit(&result, -5.0, 3.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 18.0, 3.0), (0, 2, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 100.0, 3.0), (0, 2, CaretAffinity::Upstream));
    // far away corners: clamp to the nearest line and the nearest edge
    assert_eq!(
        hit(&result, -1000.0, -1000.0),
        (0, 0, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, 1000.0, -1000.0),
        (0, 2, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, -1000.0, 1000.0),
        (0, 1, CaretAffinity::Downstream)
    );
    assert_eq!(
        hit(&result, 1000.0, 1000.0),
        (0, 3, CaretAffinity::Downstream)
    );

    let rects = result.rect_for_range(0, 0..3);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.width, 16.0);
    assert_close(rects[0].size.height, 8.0);
    assert_close(rects[1].origin.x, 19.2);
    assert_close(rects[1].origin.y, 0.0);
    assert_close(rects[1].size.width, 16.0);
    assert_close(rects[1].size.height, 8.0);
}

fn layout_text(
    content: &str,
    width: f32,
    property: BlockProperty<f32>,
) -> LayoutResult<f32, ParsedFont> {
    let mut provider = MemoryFontProvider::new();
    let data = Arc::new(build_font(&spec()));
    provider
        .register(
            data,
            0,
            Box::new([FamilyName::SansSerif]),
            Default::default(),
        )
        .expect("test font should parse");
    let query = FontQuery {
        families: Box::new([FamilyName::SansSerif]),
        properties: FontProperties::default(),
    };
    let block: Block<f32> = Block {
        property: BlockProperty {
            font_query: query,
            font_size: 16.0,
            ..property
        },
        inlines: vec![Inline::Text(InlineText {
            content: Arc::from(content),
            property: InlineProperty::default(),
            spacing: Spacing::default(),
        })],
    };
    let shaped = Arc::new(ShapedBlock::<f32, ParsedFont>::new(
        &block,
        &provider,
        &RustShaper::new(),
    ));
    shaped.layout(width)
}

fn layout_ws(content: &str, width: f32, white_space: WhiteSpace) -> LayoutResult<f32, ParsedFont> {
    let mut provider = MemoryFontProvider::new();
    let data = Arc::new(build_font(&spec()));
    provider
        .register(
            data,
            0,
            Box::new([FamilyName::SansSerif]),
            Default::default(),
        )
        .expect("test font should parse");
    let query = FontQuery {
        families: Box::new([FamilyName::SansSerif]),
        properties: FontProperties::default(),
    };
    let block: Block<f32> = Block {
        property: BlockProperty {
            font_query: query,
            font_size: 16.0,
            ..Default::default()
        },
        inlines: vec![Inline::Text(InlineText {
            content: Arc::from(content),
            property: InlineProperty {
                white_space,
                ..Default::default()
            },
            spacing: Spacing::default(),
        })],
    };
    let shaped = Arc::new(ShapedBlock::<f32, ParsedFont>::new(
        &block,
        &provider,
        &RustShaper::new(),
    ));
    shaped.layout(width)
}

fn assert_close(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-4, "{a} != {b}");
}

fn hit(result: &LayoutResult<f32, ParsedFont>, x: f32, y: f32) -> (u32, u32, CaretAffinity) {
    let pos = result
        .hit_test(float_pigment_text_layout::layout::Point::new(x, y))
        .expect("hit test should return a position");
    (pos.inline_index, pos.utf8_index, pos.affinity)
}

#[test]
fn hit_ltr_single_line() {
    // "AB": A at [0, 8), B at [8, 16)
    let result = layout_text("AB", 1000.0, Default::default());
    assert_eq!(hit(&result, 0.0, 5.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 3.0, 5.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 6.0, 5.0), (0, 1, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 10.0, 5.0), (0, 1, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 14.0, 5.0), (0, 2, CaretAffinity::Downstream));
    // out of the content: clamp to the edges
    assert_eq!(hit(&result, -10.0, 5.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 100.0, 5.0), (0, 2, CaretAffinity::Downstream));
}

#[test]
fn hit_selects_nearest_line() {
    // "A B" in a 10px box wraps: line 0 has "A", line 1 has "B"
    let result = layout_text("A B", 10.0, Default::default());
    assert_eq!(result.lines.len(), 2);
    // line 1 block range is [19.2, 35.2]
    assert_eq!(hit(&result, 2.0, 20.0), (0, 2, CaretAffinity::Upstream));
    // below the last line: clamp to it
    assert_eq!(hit(&result, 2.0, 100.0), (0, 2, CaretAffinity::Upstream));
    // above the first line: clamp to it
    assert_eq!(hit(&result, 2.0, -10.0), (0, 0, CaretAffinity::Upstream));
}

#[test]
fn hit_rtl_visual_halves() {
    // RTL "הו": ה (bytes 0..2) renders right, ו (bytes 2..4) renders left
    let result = layout_text(
        "\u{5D4}\u{5D5}",
        1000.0,
        BlockProperty {
            direction: Direction::RTL,
            ..Default::default()
        },
    );
    assert_eq!(result.glyphs.len(), 2);
    assert_close(result.glyphs[0].origin.x, 0.0); // ו at [0, 8)
    assert_close(result.glyphs[1].origin.x, 8.0); // ה at [8, 16)
                                                  // leading visual half of ו is its logical end
    assert_eq!(hit(&result, 2.0, 5.0), (0, 4, CaretAffinity::Downstream));
    // trailing visual half of ו is its logical start
    assert_eq!(hit(&result, 6.0, 5.0), (0, 2, CaretAffinity::Upstream));
    // trailing visual half of ה is its logical start
    assert_eq!(hit(&result, 14.0, 5.0), (0, 0, CaretAffinity::Upstream));
    // beyond the visual end: the logical start of the last unit
    assert_eq!(hit(&result, 100.0, 5.0), (0, 0, CaretAffinity::Upstream));
    // before the visual start: the logical end of the first unit
    assert_eq!(hit(&result, -10.0, 5.0), (0, 4, CaretAffinity::Downstream));
}

#[test]
fn hit_far_away_horizontal() {
    // "A B" in a 10px box wraps: line 0 has "A", line 1 has "B";
    // far away points clamp to the nearest line and the nearest edge
    let result = layout_text("A B", 10.0, Default::default());
    assert_eq!(
        hit(&result, -1000.0, -1000.0),
        (0, 0, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, 1000.0, -1000.0),
        (0, 1, CaretAffinity::Downstream)
    );
    assert_eq!(
        hit(&result, -1000.0, 1000.0),
        (0, 2, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, 1000.0, 1000.0),
        (0, 3, CaretAffinity::Downstream)
    );
}

#[test]
fn hit_empty_text() {
    let result = layout_text("", 1000.0, Default::default());
    assert_eq!(hit(&result, 5.0, 5.0), (0, 0, CaretAffinity::Upstream));
}

#[test]
fn rect_ltr_ranges() {
    // "A B": A at [0, 8), space at [8, 12), B at [12, 20)
    let result = layout_text("A B", 1000.0, Default::default());
    let rects = result.rect_for_range(0, 0..3);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].size.width, 20.0);
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.height, 16.0);

    let rects = result.rect_for_range(0, 2..3);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 12.0);
    assert_close(rects[0].size.width, 8.0);

    // empty and out-of-range ranges produce no rects
    assert!(result.rect_for_range(0, 1..1).is_empty());
    let clamped = result.rect_for_range(0, 2..100);
    assert_eq!(clamped.len(), 1);
    assert_close(clamped[0].origin.x, 12.0);
    assert!(result.rect_for_range(5, 0..1).is_empty());
}

#[test]
fn rect_multiline_range() {
    // "A B" wrapped into two lines; selecting the whole text gives one
    // rect per line
    let result = layout_text("A B", 10.0, Default::default());
    let rects = result.rect_for_range(0, 0..3);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].size.width, 8.0);
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.height, 16.0);
    // line 1: gap 3.2, baseline 32, extent [19.2, 35.2]
    assert_close(rects[1].origin.x, 0.0);
    assert_close(rects[1].size.width, 8.0);
    assert_close(rects[1].origin.y, 19.2);
    assert_close(rects[1].size.height, 16.0);
}

#[test]
fn rect_rtl_range() {
    // RTL "הו": ו at [0, 8), ה at [8, 16); selecting only ה (bytes 0..2)
    let result = layout_text(
        "\u{5D4}\u{5D5}",
        1000.0,
        BlockProperty {
            direction: Direction::RTL,
            ..Default::default()
        },
    );
    let rects = result.rect_for_range(0, 0..2);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 8.0);
    assert_close(rects[0].size.width, 8.0);
}

#[test]
fn hit_bidi_mixed_line() {
    // "ABהוCD" with LTR base: bytes A0 B1 ה2..4 ו4..6 C6 D7,
    // visual order A B ו ה C D, each 8px wide
    let result = layout_text("AB\u{5D4}\u{5D5}CD", 1000.0, Default::default());
    assert_eq!(result.glyphs.len(), 6);
    assert_close(result.glyphs[2].origin.x, 16.0); // ו
    assert_close(result.glyphs[3].origin.x, 24.0); // ה
                                                   // LTR halves behave as usual
    assert_eq!(hit(&result, 10.0, 5.0), (0, 1, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 13.0, 5.0), (0, 2, CaretAffinity::Downstream));
    // ו at [16, 24): RTL, leading visual half maps to its logical end
    assert_eq!(hit(&result, 18.0, 5.0), (0, 6, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 22.0, 5.0), (0, 4, CaretAffinity::Upstream));
    // ה at [24, 32): RTL
    assert_eq!(hit(&result, 26.0, 5.0), (0, 4, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 30.0, 5.0), (0, 2, CaretAffinity::Upstream));
    // after the RTL run, LTR resumes
    assert_eq!(hit(&result, 34.0, 5.0), (0, 6, CaretAffinity::Upstream));
}

#[test]
fn rect_bidi_mixed_line() {
    let result = layout_text("AB\u{5D4}\u{5D5}CD", 1000.0, Default::default());
    // "Bה" (bytes 1..4) is visually discontinuous: B at [8, 16), ה at [24, 32)
    let rects = result.rect_for_range(0, 1..4);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 8.0);
    assert_close(rects[0].size.width, 8.0);
    assert_close(rects[1].origin.x, 24.0);
    assert_close(rects[1].size.width, 8.0);
    // "Bהו" (bytes 1..6) is visually contiguous: merged into one rect
    let rects = result.rect_for_range(0, 1..6);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 8.0);
    assert_close(rects[0].size.width, 24.0);
}

#[test]
fn hit_vertical_lr() {
    // Latin is sideways in vertical writing: 8px advance along y
    let result = layout_text(
        "AB",
        1000.0,
        BlockProperty {
            writing_mode: WritingMode::VerticalLr,
            ..Default::default()
        },
    );
    assert_eq!(result.lines.len(), 1);
    assert_close(result.glyphs[0].origin.y, 0.0);
    assert_close(result.glyphs[1].origin.y, 8.0);
    // the block axis is physical x
    assert_eq!(hit(&result, 5.0, 3.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 6.0), (0, 1, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 5.0, 10.0), (0, 1, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 5.0, 100.0), (0, 2, CaretAffinity::Downstream));
}

#[test]
fn hit_and_rect_vertical_rl() {
    // "A B" wraps into two lines (sideways: A 8px, space 4px, B 8px);
    // vertical-rl stacks line 1 to the left of line 0
    let result = layout_text(
        "A B",
        10.0,
        BlockProperty {
            writing_mode: WritingMode::VerticalRl,
            ..Default::default()
        },
    );
    assert_eq!(result.lines.len(), 2);
    // total block size: 16 + 3.2 gap + 16 = 35.2
    assert_close(result.size.width, 35.2);
    // hit on the right side: line 0 ("A")
    assert_eq!(hit(&result, 30.0, 3.0), (0, 0, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 30.0, 6.0), (0, 1, CaretAffinity::Downstream));
    // hit on the left side: line 1 ("B", utf8 offset 2)
    assert_eq!(hit(&result, 5.0, 3.0), (0, 2, CaretAffinity::Upstream));
    // beyond the left edge: clamp to line 1
    assert_eq!(hit(&result, -10.0, 3.0), (0, 2, CaretAffinity::Upstream));
    // beyond the right edge: clamp to line 0
    assert_eq!(hit(&result, 100.0, 3.0), (0, 0, CaretAffinity::Upstream));
    // far away corners: clamp to the nearest line and the nearest edge
    assert_eq!(
        hit(&result, 1000.0, -1000.0),
        (0, 0, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, -1000.0, -1000.0),
        (0, 2, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, 1000.0, 1000.0),
        (0, 1, CaretAffinity::Downstream)
    );
    assert_eq!(
        hit(&result, -1000.0, 1000.0),
        (0, 3, CaretAffinity::Downstream)
    );

    // selection rects: line 0 is at the right, line 1 at the left
    let rects = result.rect_for_range(0, 0..3);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 19.2); // 35.2 - 16
    assert_close(rects[0].origin.y, 0.0);
    assert_close(rects[0].size.width, 16.0);
    assert_close(rects[0].size.height, 8.0);
    assert_close(rects[1].origin.x, 0.0);
    assert_close(rects[1].origin.y, 0.0);
    assert_close(rects[1].size.width, 16.0);
    assert_close(rects[1].size.height, 8.0);
}

fn rtl_block() -> BlockProperty<f32> {
    BlockProperty {
        direction: Direction::RTL,
        text_align: TextAlign::Start,
        ..Default::default()
    }
}

#[test]
fn hit_rtl_base_pure_rtl_break() {
    // RTL base, pure Hebrew "הו הו": bytes ה0..2 ו2..4 sp4 ה5..7 ו7..9,
    // wraps at the space into two lines of "הו" (width 16, aligned to the
    // right of the 20px box). Visual order per line: ו then ה.
    let result = layout_text("הו הו", 20.0, rtl_block());
    assert_eq!(result.lines.len(), 2);
    assert_close(result.glyphs[0].origin.x, 4.0); // ו at [4, 12)
    assert_close(result.glyphs[1].origin.x, 12.0); // ה at [12, 20)
                                                   // line 0: ו (bytes 2..4), ה (bytes 0..2)
    assert_eq!(hit(&result, 6.0, 5.0), (0, 4, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 10.0, 5.0), (0, 2, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 14.0, 5.0), (0, 2, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 18.0, 5.0), (0, 0, CaretAffinity::Upstream));
    // line 1: ו (bytes 7..9), ה (bytes 5..7)
    assert_eq!(hit(&result, 6.0, 25.0), (0, 9, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 10.0, 25.0), (0, 7, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 14.0, 25.0), (0, 7, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 18.0, 25.0), (0, 5, CaretAffinity::Upstream));

    // whole words: one rect per line
    let rects = result.rect_for_range(0, 0..4);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 4.0);
    assert_close(rects[0].size.width, 16.0);
    let rects = result.rect_for_range(0, 5..9);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 4.0);
    assert_close(rects[0].origin.y, 19.2);
    // the run "ו ה" (bytes 2..7) spans both lines: the trimmed space
    // contributes nothing
    let rects = result.rect_for_range(0, 2..7);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 4.0); // ו on line 0
    assert_close(rects[0].size.width, 8.0);
    assert_close(rects[1].origin.x, 12.0); // ה on line 1
    assert_close(rects[1].origin.y, 19.2);
    assert_close(rects[1].size.width, 8.0);
}

#[test]
fn hit_rtl_base_embedded_ltr() {
    // RTL base "הו AB" (bytes ה0..2 ו2..4 sp4 A5 B6): visual order is
    // A B _ ו ה (the embedded LTR run stays LTR, placed at the left)
    let result = layout_text("הו AB", 36.0, rtl_block());
    assert_eq!(result.lines.len(), 1);
    assert_close(result.glyphs[0].origin.x, 0.0); // A
    assert_close(result.glyphs[1].origin.x, 8.0); // B
    assert_close(result.glyphs[2].origin.x, 16.0); // space
    assert_close(result.glyphs[3].origin.x, 20.0); // ו
    assert_close(result.glyphs[4].origin.x, 28.0); // ה
    assert_eq!(hit(&result, 2.0, 5.0), (0, 5, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 6.0, 5.0), (0, 6, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 10.0, 5.0), (0, 6, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 14.0, 5.0), (0, 7, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 22.0, 5.0), (0, 4, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 26.0, 5.0), (0, 2, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 30.0, 5.0), (0, 2, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 34.0, 5.0), (0, 0, CaretAffinity::Upstream));

    // "הו" (bytes 0..4) is at the right
    let rects = result.rect_for_range(0, 0..4);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 20.0);
    assert_close(rects[0].size.width, 16.0);
    // "AB" (bytes 5..7) is at the left
    let rects = result.rect_for_range(0, 5..7);
    assert_eq!(rects.len(), 1);
    assert_close(rects[0].origin.x, 0.0);
    assert_close(rects[0].size.width, 16.0);
    // "ו A" (bytes 2..6) is visually discontinuous
    let rects = result.rect_for_range(0, 2..6);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 0.0); // A
    assert_close(rects[0].size.width, 8.0);
    assert_close(rects[1].origin.x, 16.0); // space and ו
    assert_close(rects[1].size.width, 12.0);
}

#[test]
fn hit_rtl_base_break_inside_ltr() {
    // RTL base "הו A B" (bytes ה0..2 ו2..4 sp4 A5 sp6 B7), width 28:
    // breaks inside the embedded LTR run at its internal space,
    // line 0 is "הו A", line 1 is "B"
    let result = layout_text("הו A B", 28.0, rtl_block());
    assert_eq!(result.lines.len(), 2);
    assert_eq!(result.glyphs.len(), 5); // trailing and mid-break spaces trimmed
                                        // line 0 visual: A _ ו ה
    assert_close(result.glyphs[0].origin.x, 0.0); // A at [0, 8)
    assert_close(result.glyphs[1].origin.x, 8.0); // space at [8, 12)
    assert_close(result.glyphs[2].origin.x, 12.0); // ו at [12, 20)
    assert_close(result.glyphs[3].origin.x, 20.0); // ה at [20, 28)
                                                   // line 1: B, aligned to the right edge (free = 20)
    let b = &result.glyphs[result.line_glyph_range(1).start as usize];
    assert_close(b.origin.x, 20.0);
    // hits on line 0
    assert_eq!(hit(&result, 2.0, 5.0), (0, 5, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 6.0, 5.0), (0, 6, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 14.0, 5.0), (0, 4, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 18.0, 5.0), (0, 2, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 22.0, 5.0), (0, 2, CaretAffinity::Downstream));
    assert_eq!(hit(&result, 27.0, 5.0), (0, 0, CaretAffinity::Upstream));
    // hits on line 1
    assert_eq!(hit(&result, 22.0, 25.0), (0, 7, CaretAffinity::Upstream));
    assert_eq!(hit(&result, 26.0, 25.0), (0, 8, CaretAffinity::Downstream));

    // "A B" (bytes 5..8): one rect per line
    let rects = result.rect_for_range(0, 5..8);
    assert_eq!(rects.len(), 2);
    assert_close(rects[0].origin.x, 0.0); // A on line 0
    assert_close(rects[0].size.width, 8.0);
    assert_close(rects[1].origin.x, 20.0); // B on line 1
    assert_close(rects[1].origin.y, 19.2);
    assert_close(rects[1].size.width, 8.0);
}

#[test]
fn hit_pre_wrap_empty_line() {
    // "A\n\nB": the newline between A and B produces a caret-less empty
    // line; hits on it should still resolve to its text position
    let result = layout_ws("A\n\nB", 1000.0, WhiteSpace::PreWrap);
    assert_eq!(result.lines.len(), 3);
    let mid_empty = result.lines[1].baseline_offset - result.lines[1].text_ascent + 1.0;
    // any point on the empty line resolves to the position between the
    // two newlines (bytes: A0 \n1 \n2 B3)
    assert_eq!(
        hit(&result, 0.0, mid_empty),
        (0, 2, CaretAffinity::Upstream)
    );
    assert_eq!(
        hit(&result, 5.0, mid_empty),
        (0, 2, CaretAffinity::Upstream)
    );
    // the real lines keep their usual behavior
    assert_eq!(hit(&result, 3.0, 5.0), (0, 0, CaretAffinity::Upstream));
    let third_line = result.lines[2].baseline_offset - result.lines[2].text_ascent + 1.0;
    assert_eq!(hit(&result, 3.0, third_line), (0, 3, CaretAffinity::Upstream));
}

#[test]
fn hit_pre_wrap_trailing_newline() {
    // "A\n": the trailing newline produces a caret-less last line
    let result = layout_ws("A\n", 1000.0, WhiteSpace::PreWrap);
    assert_eq!(result.lines.len(), 2);
    let last_line = result.lines[1].baseline_offset - result.lines[1].text_ascent + 1.0;
    assert_eq!(hit(&result, 5.0, last_line), (0, 2, CaretAffinity::Upstream));
}

#[test]
fn hit_normal_collapsed_newlines() {
    // with normal white space, "\n\n" collapses to a single space:
    // no empty line, one wrapped line break instead
    let result = layout_ws("A\n\nB", 10.0, WhiteSpace::Normal);
    assert_eq!(result.lines.len(), 2);
    assert_eq!(hit(&result, 2.0, 20.0), (0, 3, CaretAffinity::Upstream));
}
