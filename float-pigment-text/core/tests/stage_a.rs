//! Stage A tests: itemize, shaping, font fallback, with hand-crafted fonts.

mod common;

use std::sync::Arc;

use common::{build_font, FontSpec};
use float_pigment_text_layout::font::{
    FamilyName, FontFace, FontProperties, FontProvider, FontQuery, Weight,
};
use float_pigment_text_layout::model::{
    Block, BlockProperty, Inline, InlineProperty, InlineText, Spacing,
};
use float_pigment_text_layout::rust_font::{MemoryFontProvider, ParsedFont, RustShaper};
use float_pigment_text_layout::shaper::{Language, Script};
use float_pigment_text_layout::ShapedBlock;

fn latin_spec() -> FontSpec {
    FontSpec {
        glyphs: vec![('A', 600), ('B', 700), (' ', 250)],
        ..Default::default()
    }
}

fn cjk_spec() -> FontSpec {
    FontSpec {
        glyphs: vec![('中', 1000), ('文', 1000), ('\u{1F642}', 1000)],
        ..Default::default()
    }
}

fn register(
    provider: &mut MemoryFontProvider,
    spec: &FontSpec,
    families: Box<[FamilyName]>,
    properties: FontProperties,
) {
    let data = Arc::new(build_font(spec));
    provider
        .register(data, 0, families, properties)
        .expect("test font should parse");
}

fn text_block(content: &str, query: FontQuery) -> Block<f32> {
    Block {
        property: BlockProperty {
            font_query: query,
            font_size: 16.0,
            ..Default::default()
        },
        inlines: vec![Inline::Text(InlineText {
            content: Arc::from(content),
            property: InlineProperty::default(),
            spacing: Spacing::default(),
        })],
    }
}

fn query(families: Box<[FamilyName]>) -> FontQuery {
    FontQuery {
        families,
        properties: FontProperties::default(),
    }
}

fn assert_close(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-4, "{a} != {b}");
}

#[test]
fn basic_shaping_metrics_and_flags() {
    let spec = latin_spec();
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &spec,
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let block = text_block("A A", query(Box::new([FamilyName::SansSerif])));
    let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);

    // font metrics at size 16 with upem 1000, ascent 800, descent -200, line gap 200
    let metrics = shaped.faces()[0].metrics(16.0);
    assert_close(metrics.ascent, 12.8);
    assert_close(metrics.descent, 3.2);
    assert_close(metrics.line_gap, 3.2);

    let glyphs = shaped.glyphs();
    assert_eq!(glyphs.len(), 3);
    assert_eq!(glyphs[0].glyph_id, spec.glyph_id_of('A'));
    assert_eq!(glyphs[0].cluster, 0);
    assert_close(glyphs[0].advance.x, 9.6);
    assert!(!glyphs[0].flags.whitespace());
    assert_eq!(glyphs[1].glyph_id, spec.glyph_id_of(' '));
    assert_eq!(glyphs[1].cluster, 1);
    assert_close(glyphs[1].advance.x, 4.0);
    assert!(glyphs[1].flags.whitespace());
    assert_eq!(glyphs[2].glyph_id, spec.glyph_id_of('A'));
    assert_eq!(glyphs[2].cluster, 2);
    assert!(glyphs
        .iter()
        .all(|g| !g.flags.missing() && !g.flags.colored()));
}

#[test]
fn missing_glyph_kept_with_flag() {
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &latin_spec(),
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let block = text_block("A中", query(Box::new([FamilyName::SansSerif])));
    let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);

    let glyphs = shaped.glyphs();
    assert_eq!(glyphs.len(), 2);
    assert!(!glyphs[0].flags.missing());
    assert!(glyphs[1].flags.missing());
    assert_eq!(glyphs[1].glyph_id, 0);
    assert_eq!(glyphs[1].cluster, 1);
}

#[test]
fn fallback_to_covering_font() {
    let cjk = cjk_spec();
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &latin_spec(),
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    register(
        &mut provider,
        &cjk,
        Box::new([FamilyName::Title("CJK".into())]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let block = text_block("A中🙂", query(Box::new([FamilyName::SansSerif])));
    let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);

    let glyphs = shaped.glyphs();
    assert_eq!(glyphs.len(), 3);
    assert_eq!(shaped.faces().len(), 2);
    assert_eq!(glyphs[1].face_index, 1);
    assert_eq!(glyphs[1].glyph_id, cjk.glyph_id_of('中'));
    assert_eq!(glyphs[1].cluster, 1);
    assert_close(glyphs[1].advance.x, 16.0);
    // non-BMP char via format 12 cmap, 4 UTF-8 bytes
    assert_eq!(glyphs[2].face_index, 1);
    assert_eq!(glyphs[2].glyph_id, cjk.glyph_id_of('\u{1F642}'));
    assert_eq!(glyphs[2].cluster, 4);
    assert!(glyphs.iter().all(|g| !g.flags.missing()));
}

#[test]
fn rtl_items_have_descending_clusters() {
    let spec = FontSpec {
        glyphs: vec![('A', 600), ('\u{5D4}', 500), ('\u{5D5}', 500)], // ה ו
        ..Default::default()
    };
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &spec,
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let block = text_block("A\u{5D4}\u{5D5}", query(Box::new([FamilyName::SansSerif])));
    let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);

    let glyphs = shaped.glyphs();
    assert_eq!(glyphs.len(), 3);
    // LTR item first, then the RTL item in visual order (descending clusters)
    assert_eq!(glyphs[0].cluster, 0);
    assert_eq!(glyphs[1].cluster, 3);
    assert_eq!(glyphs[2].cluster, 1);
    assert_eq!(glyphs[1].glyph_id, spec.glyph_id_of('\u{5D5}'));
    assert_eq!(glyphs[2].glyph_id, spec.glyph_id_of('\u{5D4}'));
}

#[test]
fn colored_glyph_detected() {
    let spec = FontSpec {
        glyphs: vec![('A', 600), ('E', 600)],
        colored: vec!['E'],
        ..Default::default()
    };
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &spec,
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let block = text_block("AE", query(Box::new([FamilyName::SansSerif])));
    let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);

    let glyphs = shaped.glyphs();
    assert!(!glyphs[0].flags.colored());
    assert!(glyphs[1].flags.colored());
}

#[test]
fn font_matching_follows_css_rules() {
    use float_pigment_text_layout::font::{FontStyle, Stretch};
    let spec = latin_spec();
    let mut provider = MemoryFontProvider::new();
    let props = |weight: f32, style: FontStyle| FontProperties {
        style,
        weight: Weight(weight),
        stretch: Stretch::NORMAL,
    };
    register(
        &mut provider,
        &spec,
        Box::new([FamilyName::Title("T".into())]),
        props(400.0, FontStyle::Normal),
    );
    register(
        &mut provider,
        &spec,
        Box::new([FamilyName::Title("T".into())]),
        props(700.0, FontStyle::Normal),
    );
    register(
        &mut provider,
        &spec,
        Box::new([FamilyName::Title("T".into())]),
        props(400.0, FontStyle::Italic),
    );

    let first_face_id = |p: &MemoryFontProvider, weight: f32, style: FontStyle| {
        let q = FontQuery {
            families: Box::new([FamilyName::Title("T".into())]),
            properties: props(weight, style),
        };
        p.resolve(&q).faces()[0].id()
    };

    let normal = first_face_id(&provider, 400.0, FontStyle::Normal);
    let bold = first_face_id(&provider, 700.0, FontStyle::Normal);
    let italic = first_face_id(&provider, 400.0, FontStyle::Italic);
    assert_ne!(normal, bold);
    assert_ne!(normal, italic);
    // weight 600: above ascending wins over below
    assert_eq!(first_face_id(&provider, 600.0, FontStyle::Normal), bold);
    // weight 300: below has nothing, so ascending picks 400
    assert_eq!(first_face_id(&provider, 300.0, FontStyle::Normal), normal);
    // italic request: italic face preferred over normal
    assert_eq!(first_face_id(&provider, 400.0, FontStyle::Italic), italic);
}

#[test]
fn han_fallback_prefers_content_language() {
    let zh_spec = FontSpec {
        glyphs: vec![('中', 1000)],
        design_languages: vec!["zh-Hans"],
        ..Default::default()
    };
    let ja_spec = FontSpec {
        glyphs: vec![('中', 900)],
        design_languages: vec!["ja"],
        ..Default::default()
    };
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &latin_spec(),
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    // register the ja font first: naive registration order would pick it
    register(
        &mut provider,
        &ja_spec,
        Box::new([FamilyName::Title("JA".into())]),
        Default::default(),
    );
    register(
        &mut provider,
        &zh_spec,
        Box::new([FamilyName::Title("ZH".into())]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let shaped_with = |language: &str| {
        let mut block = text_block("中", query(Box::new([FamilyName::SansSerif])));
        block.property.language = language.parse().ok();
        ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper)
    };

    let zh = shaped_with("zh-CN");
    assert_eq!(zh.glyphs().len(), 1);
    assert!(!zh.glyphs()[0].flags.missing());
    assert_close(zh.glyphs()[0].advance.x, 16.0); // zh font advance 1000

    let ja = shaped_with("ja-JP");
    assert!(!ja.glyphs()[0].flags.missing());
    assert_close(ja.glyphs()[0].advance.x, 14.4); // ja font advance 900

    // non-Han content language falls back in registration order (ja first)
    let en = shaped_with("en-US");
    assert!(!en.glyphs()[0].flags.missing());
    assert_close(en.glyphs()[0].advance.x, 14.4);
}

#[test]
fn han_fallback_distinguishes_zh_varieties() {
    let hans_spec = FontSpec {
        glyphs: vec![('骨', 1000)],
        design_languages: vec!["zh-Hans"],
        ..Default::default()
    };
    let hant_spec = FontSpec {
        glyphs: vec![('骨', 900)],
        design_languages: vec!["zh-Hant"],
        ..Default::default()
    };
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &latin_spec(),
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    // register Hans first: variety-aware ordering must override registration order
    register(
        &mut provider,
        &hans_spec,
        Box::new([FamilyName::Title("HANS".into())]),
        Default::default(),
    );
    register(
        &mut provider,
        &hant_spec,
        Box::new([FamilyName::Title("HANT".into())]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let advance_with = |language: &str| {
        let mut block = text_block("骨", query(Box::new([FamilyName::SansSerif])));
        block.property.language = language.parse().ok();
        let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);
        assert!(!shaped.glyphs()[0].flags.missing());
        shaped.glyphs()[0].advance.x
    };

    // simplified conventions: CN/SG and bare zh
    assert_close(advance_with("zh-CN"), 16.0);
    assert_close(advance_with("zh-SG"), 16.0);
    assert_close(advance_with("zh"), 16.0);
    // traditional conventions: TW/HK/MO and the explicit script tag
    assert_close(advance_with("zh-TW"), 14.4);
    assert_close(advance_with("zh-HK"), 14.4);
    assert_close(advance_with("zh-Hant"), 14.4);
}

#[test]
fn language_tag_validation() {
    let lang: Language = "zh-CN".parse().unwrap();
    assert_eq!(lang.as_str(), "zh-CN");
    assert_eq!(format!("{lang}"), "zh-CN");
    assert!("zh-Hans-CN".parse::<Language>().is_ok());
    assert!("a-very-long-private-use-tag".parse::<Language>().is_err());
    assert!("中文".parse::<Language>().is_err());
}

#[test]
fn block_script_and_language_reach_shaping() {
    let mut provider = MemoryFontProvider::new();
    register(
        &mut provider,
        &latin_spec(),
        Box::new([FamilyName::SansSerif]),
        Default::default(),
    );
    let shaper = RustShaper::new();

    let mut block = text_block("A", query(Box::new([FamilyName::SansSerif])));
    block.property.script = Some(Script(*b"Latn"));
    block.property.language = "en-US".parse().ok();
    let shaped = ShapedBlock::<f32, ParsedFont>::new(&block, &provider, &shaper);
    assert_eq!(shaped.glyphs().len(), 1);
}
