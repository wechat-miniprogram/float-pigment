// WPT-style tests for CSS writing-mode + direction in Grid Layout
//
// References:
//   CSS Writing Modes Level 4 §2: Inline Direction and Bidirectionality
//   https://www.w3.org/TR/css-writing-modes-4/#direction
//
//   CSS Writing Modes Level 4 §4: Block Flow Direction
//   https://www.w3.org/TR/css-writing-modes-4/#block-flow
//
//   CSS Grid Layout Module Level 1 §10: Alignment
//   https://www.w3.org/TR/css-grid-1/#grid-align
//
// In this layout engine:
//   - grid-template-columns controls physical horizontal (width) track sizing
//   - grid-template-rows controls physical vertical (height) track sizing
//   - gen_origin maps block/inline offsets to physical coordinates via AxisInfo
//   - main axis = block axis, cross axis = inline axis
//
// Axis mapping per writing-mode:
//   horizontal-tb (dir=Vertical):
//     left = inline_offset (from columns), top = block_offset (from rows)
//   vertical-lr (dir=Horizontal):
//     left = block_offset (from rows), top = inline_offset (from columns)
//   vertical-rl (dir=Horizontal, main_reversed):
//     left = reversed block_offset (from rows), top = inline_offset (from columns)
//
// The `direction` property only affects the inline (cross) axis:
//   RTL reverses inline axis, causing column tracks to flow in reverse order.

use crate::*;

// ═══════════════════════════════════════════════════════════════════════════
// writing-mode: horizontal-tb (default) with direction
// ═══════════════════════════════════════════════════════════════════════════

// Case: horizontal-tb + LTR (default) with explicit rows
// W3C CSS Writing Modes Level 4 §4.1:
//   - Block flow: top → bottom
//   - Inline flow: left → right
//
// Layout:
//   - Container: 400x300, 2 columns (100px), 2 rows (100px)
//   - Item 1: left=0, top=0
//   - Item 2: left=100, top=0
//   - Item 3: left=0, top=100
//   - Item 4: left=100, top=100
#[test]
fn grid_horizontal_tb_ltr_basic() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: horizontal-tb; direction: ltr; grid-template-columns: 100px 100px; grid-template-rows: 100px 100px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="100"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="100"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="100"></div>
          <div expect_left="100" expect_top="100" expect_width="100" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: horizontal-tb + RTL with explicit rows
// W3C CSS Writing Modes Level 4 §2.1:
//   - `direction: rtl` reverses the inline axis (right → left)
//   - Block axis (top → bottom) is unaffected
//
// Layout:
//   - Container: 400x300, 2 columns (100px), 2 rows (100px)
//   - RTL: columns placed from right edge
//   - Col 0: left = 400 - 100 = 300
//   - Col 1: left = 400 - 200 = 200
//   - Row 0: top=0, Row 1: top=100
#[test]
fn grid_horizontal_tb_rtl_basic() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: horizontal-tb; direction: rtl; grid-template-columns: 100px 100px; grid-template-rows: 100px 100px;">
          <div expect_left="300" expect_top="0" expect_width="100" expect_height="100"></div>
          <div expect_left="200" expect_top="0" expect_width="100" expect_height="100"></div>
          <div expect_left="300" expect_top="100" expect_width="100" expect_height="100"></div>
          <div expect_left="200" expect_top="100" expect_width="100" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// writing-mode: vertical-lr
// ═══════════════════════════════════════════════════════════════════════════
// W3C CSS Writing Modes Level 4 §4.2:
//   - Block flow: left → right
//   - Inline flow: top → bottom (LTR) or bottom → top (RTL)
//
// In this engine with vertical-lr (dir=Horizontal):
//   gen_origin: left = block_offset (from rows), top = inline_offset (from columns)
//   main_dir_rev = NotReversed, cross_dir_rev from direction

// Case: vertical-lr + LTR with explicit tracks
// Layout:
//   - Container: 400x300
//   - Columns: 100px 100px → mapped to top positions via gen_origin
//   - Rows: 80px 80px → mapped to left positions via gen_origin
//   - Block axis (left→right): left = row_offset
//   - Inline axis (top→bottom): top = col_offset
//
//   Item 1 (col 0, row 0): left=0, top=0, width=80(from row track), height=100(from col track)
//   Item 2 (col 1, row 0): left=0, top=100
//   Item 3 (col 0, row 1): left=80, top=0
//   Item 4 (col 1, row 1): left=80, top=100
//
// Note: In vertical writing modes, gen_origin swaps the axis mapping:
//   physical width comes from row track size, physical height from column track size.
//   But current engine does not rotate item dimensions — items keep their
//   physical width/height. So item width = column track (inline = 100px),
//   height = row track (block = 80px).
#[test]
fn grid_vertical_lr_ltr_explicit_tracks() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px 100px; grid-template-rows: 80px 80px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="80"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="0" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="100" expect_width="100" expect_height="80"></div>
        </div>
    "#,
        true
    )
}

// Case: vertical-lr + RTL with explicit tracks
// W3C CSS Writing Modes Level 4 §2.1:
//   - `direction: rtl` reverses inline axis (bottom → top)
//   - Block axis (left → right) unaffected
//
// Layout:
//   - Container: 400x300, Columns: 100px 100px, Rows: 80px 80px
//   - Inline reversed: columns placed from bottom of container height (300px)
//   - Container content height for inline = 300px
//   - Col 0: top = 300 - 100 = 200
//   - Col 1: top = 300 - 100 - 100 = 100
//   - Block axis normal: left=0 (row 0), left=80 (row 1)
#[test]
fn grid_vertical_lr_rtl_explicit_tracks() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: rtl; grid-template-columns: 100px 100px; grid-template-rows: 80px 80px;">
          <div expect_left="0" expect_top="200" expect_width="100" expect_height="80"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="200" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="100" expect_width="100" expect_height="80"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Verify that direction only affects inline axis (cross_dir_rev),
// not block axis (main_dir_rev)
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-lr - direction should NOT change block (horizontal) positions
// W3C CSS Writing Modes Level 4 §2.1:
//   - direction only affects inline base direction
//   - Block flow direction is unaffected
//
// In vertical-lr, block axis = left→right
// With direction: ltr vs rtl, block positions (left) should stay the same.
// Only inline positions (top) should differ.
//
// Layout with explicit rows (80px 80px):
//   - LTR: top = 0, 100 (inline top→bottom)
//   - RTL: top = 200, 100 (inline bottom→top: 300-100=200, 300-100-100=100)
//   - Both: left = 0, 80 (block left→right, unchanged)
#[test]
fn grid_vertical_lr_direction_only_affects_inline() {
    // LTR
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px 100px; grid-template-rows: 80px 80px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="80"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="0" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="100" expect_width="100" expect_height="80"></div>
        </div>
    "#,
        true
    );

    // RTL - left positions must be the same as LTR; only top changes
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: rtl; grid-template-columns: 100px 100px; grid-template-rows: 80px 80px;">
          <div expect_left="0" expect_top="200" expect_width="100" expect_height="80"></div>
          <div expect_left="0" expect_top="100" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="200" expect_width="100" expect_height="80"></div>
          <div expect_left="80" expect_top="100" expect_width="100" expect_height="80"></div>
        </div>
    "#,
        true
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// writing-mode: vertical-lr with gap
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-lr + LTR with column-gap
// W3C CSS Grid §10.1 + CSS Writing Modes Level 4 §4.2:
//   - column-gap inserts physical horizontal gaps between columns
//   - In vertical-lr: columns map to vertical direction via gen_origin
//   - Column offsets with gap: col 0 at top=0, col 1 at top=100+20=120
//
// Layout:
//   - Container: 400x300, columns: 100px 100px, column-gap: 20px
//   - Col 0: top=0, Col 1: top=120
#[test]
fn grid_vertical_lr_ltr_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px 100px; column-gap: 20px;">
          <div style="height: 50px;" expect_left="0" expect_top="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="0" expect_top="120" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: vertical-lr + RTL with column-gap
// Layout:
//   - Inline reversed: columns placed from bottom of container (300px)
//   - Total column space: 100 + 20 + 100 = 220px
//   - Col 0: top = 300 - 100 = 200
//   - Col 1: top = 300 - 100 - 20 - 100 = 80
#[test]
fn grid_vertical_lr_rtl_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: rtl; grid-template-columns: 100px 100px; column-gap: 20px;">
          <div style="height: 50px;" expect_left="0" expect_top="200" expect_width="100"></div>
          <div style="height: 50px;" expect_left="0" expect_top="80" expect_width="100"></div>
        </div>
    "#,
        true
    )
}
