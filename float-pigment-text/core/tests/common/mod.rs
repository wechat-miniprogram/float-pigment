//! Minimal hand-crafted TTF builder for tests.
//!
//! The generated fonts contain only the tables required for shaping and
//! metrics (head, hhea, maxp, hmtx, cmap, glyf, loca, OS/2, post),
//! plus optional COLR/CPAL for colored glyph tests.
//! All glyphs have empty outlines, so the fonts are trivially
//! copyright-free and fully deterministic.

pub struct FontSpec {
    /// (char, advance in font units); glyph ids are assigned from 1 in order.
    pub glyphs: Vec<(char, u16)>,
    /// Chars whose glyph gets a COLR v0 entry.
    pub colored: Vec<char>,
    /// Design languages, written to the `meta` table `dlng` entry.
    pub design_languages: Vec<&'static str>,
    pub units_per_em: u16,
    pub ascent: i16,
    pub descent: i16,
    pub line_gap: i16,
}

impl Default for FontSpec {
    fn default() -> Self {
        Self {
            glyphs: vec![],
            colored: vec![],
            design_languages: vec![],
            units_per_em: 1000,
            ascent: 800,
            descent: -200,
            line_gap: 200,
        }
    }
}

impl FontSpec {
    /// The glyph id of a char (1-based, following the spec order).
    pub fn glyph_id_of(&self, c: char) -> u16 {
        self.glyphs.iter().position(|(ch, _)| *ch == c).unwrap() as u16 + 1
    }

    /// The advance of a char at the given font size.
    #[allow(dead_code)] // shared test helper, unused in some test binaries
    pub fn advance_of(&self, c: char, size: f32) -> f32 {
        let (_, advance) = self.glyphs.iter().find(|(ch, _)| *ch == c).unwrap();
        *advance as f32 * size / self.units_per_em as f32
    }
}

fn u16be(v: &mut Vec<u8>, x: u16) {
    v.extend(x.to_be_bytes());
}

fn i16be(v: &mut Vec<u8>, x: i16) {
    v.extend(x.to_be_bytes());
}

fn u32be(v: &mut Vec<u8>, x: u32) {
    v.extend(x.to_be_bytes());
}

fn i32be(v: &mut Vec<u8>, x: i32) {
    v.extend(x.to_be_bytes());
}

fn u64be(v: &mut Vec<u8>, x: u64) {
    v.extend(x.to_be_bytes());
}

fn build_head(units_per_em: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(54);
    u32be(&mut v, 0x00010000); // version
    u32be(&mut v, 0x00010000); // fontRevision
    u32be(&mut v, 0); // checkSumAdjustment (not verified by parsers)
    u32be(&mut v, 0x5F0F3CF5); // magicNumber
    u16be(&mut v, 0); // flags
    u16be(&mut v, units_per_em);
    u64be(&mut v, 0); // created
    u64be(&mut v, 0); // modified
    i16be(&mut v, 0); // xMin
    i16be(&mut v, 0); // yMin
    i16be(&mut v, 0); // xMax
    i16be(&mut v, 0); // yMax
    u16be(&mut v, 0); // macStyle
    u16be(&mut v, 8); // lowestRecPPEM
    i16be(&mut v, 2); // fontDirectionHint
    i16be(&mut v, 0); // indexToLocFormat: short
    i16be(&mut v, 0); // glyphDataFormat
    v
}

fn build_hhea(spec: &FontSpec, num_glyphs: u16, advance_max: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(36);
    u32be(&mut v, 0x00010000); // version
    i16be(&mut v, spec.ascent);
    i16be(&mut v, spec.descent);
    i16be(&mut v, spec.line_gap);
    u16be(&mut v, advance_max);
    i16be(&mut v, 0); // minLeftSideBearing
    i16be(&mut v, 0); // minRightSideBearing
    i16be(&mut v, 0); // xMaxExtent
    i16be(&mut v, 1); // caretSlopeRise
    i16be(&mut v, 0); // caretSlopeRun
    i16be(&mut v, 0); // caretOffset
    for _ in 0..4 {
        i16be(&mut v, 0); // reserved
    }
    i16be(&mut v, 0); // metricDataFormat
    u16be(&mut v, num_glyphs); // numberOfHMetrics
    v
}

fn build_maxp(num_glyphs: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(32);
    u32be(&mut v, 0x00010000); // version 1.0
    u16be(&mut v, num_glyphs);
    u16be(&mut v, 0); // maxPoints
    u16be(&mut v, 0); // maxContours
    u16be(&mut v, 0); // maxCompositePoints
    u16be(&mut v, 0); // maxCompositeContours
    u16be(&mut v, 2); // maxZones
    u16be(&mut v, 0); // maxTwilightPoints
    u16be(&mut v, 0); // maxStorage
    u16be(&mut v, 0); // maxFunctionDefs
    u16be(&mut v, 0); // maxInstructionDefs
    u16be(&mut v, 0); // maxStackElements
    u16be(&mut v, 0); // maxSizeOfInstructions
    u16be(&mut v, 0); // maxComponentElements
    u16be(&mut v, 0); // maxComponentDepth
    v
}

fn build_hmtx(advances: &[u16]) -> Vec<u8> {
    let mut v = Vec::with_capacity(advances.len() * 4);
    for advance in advances {
        u16be(&mut v, *advance);
        i16be(&mut v, 0); // lsb
    }
    v
}

fn build_glyf_and_loca(num_glyphs: u16) -> (Vec<u8>, Vec<u8>) {
    // every glyph is an empty contour (10-byte header)
    let mut glyf = Vec::with_capacity(num_glyphs as usize * 10);
    let mut loca = Vec::with_capacity((num_glyphs as usize + 1) * 2);
    for i in 0..num_glyphs {
        u16be(&mut loca, i * 5); // short loca: offset / 2
        i16be(&mut glyf, 0); // numberOfContours
        i16be(&mut glyf, 0); // xMin
        i16be(&mut glyf, 0); // yMin
        i16be(&mut glyf, 0); // xMax
        i16be(&mut glyf, 0); // yMax
    }
    u16be(&mut loca, num_glyphs * 5);
    (glyf, loca)
}

fn build_cmap_format4(entries: &[(u16, u16)]) -> Vec<u8> {
    // one segment per char, plus the 0xFFFF terminator
    let seg_count = entries.len() as u16 + 1;
    let mut pow2 = 1u16;
    let mut entry_selector = 0u16;
    while pow2 * 2 <= seg_count {
        pow2 *= 2;
        entry_selector += 1;
    }
    let search_range = pow2 * 2;
    let mut v = Vec::new();
    u16be(&mut v, 4); // format
    u16be(&mut v, 16 + seg_count * 8); // length
    u16be(&mut v, 0); // language
    u16be(&mut v, seg_count * 2); // segCountX2
    u16be(&mut v, search_range);
    u16be(&mut v, entry_selector);
    u16be(&mut v, seg_count * 2 - search_range); // rangeShift
    for (c, _) in entries {
        u16be(&mut v, *c); // endCode
    }
    u16be(&mut v, 0xFFFF);
    u16be(&mut v, 0); // reservedPad
    for (c, _) in entries {
        u16be(&mut v, *c); // startCode
    }
    u16be(&mut v, 0xFFFF);
    for (c, gid) in entries {
        u16be(&mut v, gid.wrapping_sub(*c)); // idDelta
    }
    u16be(&mut v, 1);
    for _ in entries {
        u16be(&mut v, 0); // idRangeOffset
    }
    u16be(&mut v, 0);
    v
}

fn build_cmap_format12(entries: &[(u32, u32)]) -> Vec<u8> {
    let mut v = Vec::new();
    u16be(&mut v, 12); // format
    u16be(&mut v, 0); // reserved
    u32be(&mut v, 16 + entries.len() as u32 * 12); // length
    u32be(&mut v, 0); // language
    u32be(&mut v, entries.len() as u32); // nGroups
    for (c, gid) in entries {
        u32be(&mut v, *c); // startCharCode
        u32be(&mut v, *c); // endCharCode
        u32be(&mut v, *gid); // startGlyphID
    }
    v
}

fn build_cmap(spec: &FontSpec) -> Vec<u8> {
    let mut bmp: Vec<(u16, u16)> = spec
        .glyphs
        .iter()
        .enumerate()
        .filter_map(|(i, (c, _))| {
            let c = *c as u32;
            (c <= 0xFFFF).then(|| (c as u16, i as u16 + 1))
        })
        .collect();
    bmp.sort();
    // format 12 covers the full repertoire (including BMP chars), matching
    // what real fonts do: it is the preferred subtable for UCS-4 consumers,
    // while format 4 remains for BMP-only consumers
    let has_non_bmp = spec.glyphs.iter().any(|(c, _)| *c as u32 > 0xFFFF);
    let full: Vec<(u32, u32)> = spec
        .glyphs
        .iter()
        .enumerate()
        .map(|(i, (c, _))| (*c as u32, i as u32 + 1))
        .collect();

    let format4 = (!bmp.is_empty()).then(|| build_cmap_format4(&bmp));
    let format12 = has_non_bmp.then(|| build_cmap_format12(&full));
    let num_tables = format4.iter().count() + format12.iter().count();
    let mut v = Vec::new();
    u16be(&mut v, 0); // version
    u16be(&mut v, num_tables as u16);
    let mut offset = 4 + 8 * num_tables as u32;
    if let Some(f4) = &format4 {
        u16be(&mut v, 3); // platformID: Windows
        u16be(&mut v, 1); // encodingID: Unicode BMP
        u32be(&mut v, offset);
        offset += f4.len() as u32;
    }
    if let Some(_f12) = &format12 {
        u16be(&mut v, 3); // platformID: Windows
        u16be(&mut v, 10); // encodingID: Unicode full repertoire
        u32be(&mut v, offset);
    }
    if let Some(f4) = format4 {
        v.extend(f4);
    }
    if let Some(f12) = format12 {
        v.extend(f12);
    }
    v
}

fn build_os2(spec: &FontSpec) -> Vec<u8> {
    let mut v = Vec::with_capacity(78);
    u16be(&mut v, 0); // version
    i16be(&mut v, 500); // xAvgCharWidth
    u16be(&mut v, 400); // usWeightClass
    u16be(&mut v, 5); // usWidthClass
    u16be(&mut v, 0); // fsType
    for _ in 0..6 {
        i16be(&mut v, 0); // subscript/superscript metrics
    }
    i16be(&mut v, 0); // yStrikeoutSize
    i16be(&mut v, 0); // yStrikeoutPosition
    i16be(&mut v, 0); // sFamilyClass
    v.extend([0u8; 10]); // panose
    for _ in 0..4 {
        u32be(&mut v, 0); // ulUnicodeRange
    }
    v.extend(*b"TEST"); // achVendID
    u16be(&mut v, 0x40); // fsSelection: REGULAR
    u16be(&mut v, 0x20); // usFirstCharIndex
    u16be(&mut v, 0xFFFF); // usLastCharIndex
    i16be(&mut v, spec.ascent); // sTypoAscender
    i16be(&mut v, spec.descent); // sTypoDescender
    i16be(&mut v, spec.line_gap); // sTypoLineGap
    u16be(&mut v, spec.ascent as u16); // usWinAscent
    u16be(&mut v, (-spec.descent) as u16); // usWinDescent
    v
}

fn build_post() -> Vec<u8> {
    let mut v = Vec::with_capacity(32);
    u32be(&mut v, 0x00030000); // version 3.0
    i32be(&mut v, 0); // italicAngle
    i16be(&mut v, 0); // underlinePosition
    i16be(&mut v, 0); // underlineThickness
    u32be(&mut v, 0); // isFixedPitch
    u32be(&mut v, 0); // minMemType42
    u32be(&mut v, 0); // maxMemType42
    u32be(&mut v, 0); // minMemType1
    u32be(&mut v, 0); // maxMemType1
    v
}

fn build_colr(spec: &FontSpec) -> Vec<u8> {
    let mut v = Vec::new();
    let num = spec.colored.len() as u16;
    u16be(&mut v, 0); // version 0
    u16be(&mut v, num); // numBaseGlyphRecords
    u32be(&mut v, 14); // baseGlyphRecordsOffset
    u32be(&mut v, 14 + num as u32 * 6); // layerRecordsOffset
    u16be(&mut v, num); // numLayerRecords
    for (i, c) in spec.colored.iter().enumerate() {
        u16be(&mut v, spec.glyph_id_of(*c)); // base glyph
        u16be(&mut v, i as u16); // firstLayerIndex
        u16be(&mut v, 1); // numLayers
    }
    for _ in &spec.colored {
        u16be(&mut v, 0); // layer glyph: .notdef (unused by detection)
        u16be(&mut v, 0); // paletteIndex
    }
    v
}

fn build_cpal() -> Vec<u8> {
    let mut v = Vec::new();
    u16be(&mut v, 0); // version
    u16be(&mut v, 1); // numPaletteEntries
    u16be(&mut v, 1); // numPalettes
    u16be(&mut v, 1); // numColorRecords
    u32be(&mut v, 12); // colorRecordsArrayOffset
    u16be(&mut v, 0); // colorRecordIndices[0]
    v.extend([0, 0, 255, 255]); // BGRA: red
    v
}

fn build_meta(spec: &FontSpec) -> Vec<u8> {
    // dlng data is a comma-separated list of language tags
    let data = spec.design_languages.join(",").into_bytes();
    let mut v = Vec::new();
    u32be(&mut v, 1); // version
    u32be(&mut v, 0); // flags
    u32be(&mut v, 0); // reserved
    u32be(&mut v, 1); // dataMapsCount
    v.extend(*b"dlng"); // tag
    u32be(&mut v, 16 + 12); // data offset from the table start
    u32be(&mut v, data.len() as u32);
    v.extend(data);
    v
}

/// Build a TTF from the spec.
pub fn build_font(spec: &FontSpec) -> Vec<u8> {
    let num_glyphs = spec.glyphs.len() as u16 + 1;
    let mut advances = vec![500u16]; // .notdef
    advances.extend(spec.glyphs.iter().map(|(_, a)| *a));
    let advance_max = advances.iter().copied().max().unwrap();

    let (glyf, loca) = build_glyf_and_loca(num_glyphs);
    let mut tables: Vec<([u8; 4], Vec<u8>)> = vec![
        (*b"OS/2", build_os2(spec)),
        (*b"cmap", build_cmap(spec)),
        (*b"glyf", glyf),
        (*b"head", build_head(spec.units_per_em)),
        (*b"hhea", build_hhea(spec, num_glyphs, advance_max)),
        (*b"hmtx", build_hmtx(&advances)),
        (*b"loca", loca),
        (*b"maxp", build_maxp(num_glyphs)),
        (*b"post", build_post()),
    ];
    if !spec.colored.is_empty() {
        tables.push((*b"COLR", build_colr(spec)));
        tables.push((*b"CPAL", build_cpal()));
    }
    if !spec.design_languages.is_empty() {
        tables.push((*b"meta", build_meta(spec)));
    }
    tables.sort_by_key(|(tag, _)| *tag);

    let num_tables = tables.len() as u16;
    let mut pow2 = 1u16;
    let mut entry_selector = 0u16;
    while pow2 * 2 <= num_tables {
        pow2 *= 2;
        entry_selector += 1;
    }

    let mut v = Vec::new();
    u32be(&mut v, 0x00010000); // sfntVersion
    u16be(&mut v, num_tables);
    u16be(&mut v, pow2 * 16); // searchRange
    u16be(&mut v, entry_selector);
    u16be(&mut v, num_tables * 16 - pow2 * 16); // rangeShift
    let mut offset = 12 + 16 * num_tables as u32;
    for (tag, data) in &tables {
        v.extend(*tag);
        u32be(&mut v, 0); // checksum (not verified by parsers)
        u32be(&mut v, offset);
        u32be(&mut v, data.len() as u32);
        offset += (data.len() as u32).div_ceil(4) * 4;
    }
    for (_, data) in &tables {
        v.extend(data);
        while v.len() % 4 != 0 {
            v.push(0);
        }
    }
    v
}
