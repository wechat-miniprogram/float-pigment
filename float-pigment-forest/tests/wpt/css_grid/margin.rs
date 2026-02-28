// WPT-style tests for CSS Grid item margin axis mapping
//
// References:
//   CSS Writing Modes Level 4 §2: Inline Direction and Bidirectionality
//   https://www.w3.org/TR/css-writing-modes-4/#direction
//
//   CSS Writing Modes Level 4 §4: Block Flow Direction
//   https://www.w3.org/TR/css-writing-modes-4/#block-flow
//
//   CSS Box Model §6: Margin properties
//   https://www.w3.org/TR/css-box-4/#margins
//
//   CSS Grid §10.3-10.4: Aligning with Margins
//   https://www.w3.org/TR/css-grid-1/#grid-align
//
// Margin axis mapping per writing-mode + direction:
//
//   horizontal-tb + LTR:
//     block-start = margin-top, block-end = margin-bottom
//     inline-start = margin-left, inline-end = margin-right
//
//   horizontal-tb + RTL:
//     block-start = margin-top, block-end = margin-bottom
//     inline-start = margin-right, inline-end = margin-left
//
//   vertical-lr + LTR:
//     block-start = margin-left, block-end = margin-right
//     inline-start = margin-top, inline-end = margin-bottom
//
//   vertical-lr + RTL:
//     block-start = margin-left, block-end = margin-right
//     inline-start = margin-bottom, inline-end = margin-top
//
//   vertical-rl + LTR:
//     block-start = margin-right, block-end = margin-left
//     inline-start = margin-top, inline-end = margin-bottom
//
//   vertical-rl + RTL:
//     block-start = margin-right, block-end = margin-left
//     inline-start = margin-bottom, inline-end = margin-top
//
// In this layout engine:
//   - main axis = block axis, cross axis = inline axis
//   - gen_origin(Vertical): left = offset_cross, top = offset_main
//   - gen_origin(Horizontal): left = offset_main, top = offset_cross
//   - offset_main includes main_axis_start margin (block-start margin)
//   - offset_cross includes cross_axis_start margin (inline-start margin)

use crate::*;

// ═══════════════════════════════════════════════════════════════════════════
// horizontal-tb + LTR: margin mapping
// ═══════════════════════════════════════════════════════════════════════════

// Case: horizontal-tb + LTR with margin-top and margin-left
// CSS Writing Modes §4.1: horizontal-tb block flow = top → bottom
// CSS Writing Modes §2.1: LTR inline flow = left → right
//
// margin-top = block-start margin → shifts item down in block axis
// margin-left = inline-start margin → shifts item right in inline axis
//
// Layout:
//   - Container: 400x300, 1 column (200px), 1 row (200px)
//   - Item: 60x60 with margin-top: 10px, margin-left: 20px
//   - gen_origin(Vertical):
//     offset_main = row_offset(0) + margin.top(10) = 10 → top = 10
//     offset_cross = col_offset(0) + margin.left(20) = 20 → left = 20
#[test]
fn grid_margin_horizontal_tb_ltr_top_left() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: horizontal-tb; direction: ltr; grid-template-columns: 200px; grid-template-rows: 200px; align-items: start; justify-items: start;">
          <div style="width: 60px; height: 60px; margin-top: 10px; margin-left: 20px;"
            expect_left="20" expect_top="10" expect_width="60" expect_height="60"></div>
        </div>
    "#,
        true
    )
}

// Case: horizontal-tb + LTR with margin-bottom and margin-right
// CSS Writing Modes §4.1 + §2.1: block-end = bottom, inline-end = right
//
// margin-bottom and margin-right are block-end and inline-end.
// With align-items: end / justify-items: end, item aligns to cell's end edge,
// and margin-bottom / margin-right push the item inward from those edges.
//
// Layout:
//   - Container: 400x300, 1 column (200px), 1 row (200px)
//   - Item: 60x60 with margin-bottom: 10px, margin-right: 20px
//   - align-items: end → item at bottom of cell, margin-bottom shifts up
//   - justify-items: end → item at right of cell, margin-right shifts left
//   - top = row_offset(0) + (200 - 60 - 10) = 130 (end-aligned, but margin shifts the alignment area)
//
// Note: In Grid, end-aligned items with margin-end: the margin is applied
// by the alignment offset calculation, not directly in gen_origin.
// The test verifies that margin-top (block-start) correctly maps to physical top.
// Using start alignment for clarity.
#[test]
fn grid_margin_horizontal_tb_ltr_all_margins() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: horizontal-tb; direction: ltr; grid-template-columns: 200px; grid-template-rows: 200px; align-items: start; justify-items: start;">
          <div style="width: 60px; height: 60px; margin: 15px 25px 10px 20px;"
            expect_left="20" expect_top="15" expect_width="60" expect_height="60"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// horizontal-tb + RTL: margin mapping
// ═══════════════════════════════════════════════════════════════════════════

// Case: horizontal-tb + RTL with margin-top and margin-right
// CSS Writing Modes §2.1: RTL reverses inline direction (right → left)
// CSS Writing Modes §4.1: block flow remains top → bottom
//
// In RTL:
//   inline-start = right side → margin-right is the inline-start margin
//   margin-top = block-start margin (unchanged)
//
// Layout:
//   - Container: 400x300, 1 column (200px), 1 row (200px)
//   - Item: 60x60, margin-top: 10px, margin-right: 20px
//   - RTL: inline_offset = 400 - 0 - 200 = 200 (column left edge)
//   - justify_offset_rtl(Start, 60, 200) = 200 - 60 = 140
//     (RTL start aligns to right edge within track)
//   - cross_axis_start(Vertical, Reversed) = margin.right = 20
//   - offset_main = 0 + 0(align) + margin.top(10) = 10
//   - offset_cross = 200 + 140 + 20 = 360
//   - gen_origin(V): left = 360, top = 10
#[test]
fn grid_margin_horizontal_tb_rtl_top_right() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: horizontal-tb; direction: rtl; grid-template-columns: 200px; grid-template-rows: 200px; align-items: start; justify-items: start;">
          <div style="width: 60px; height: 60px; margin-top: 10px; margin-right: 20px;"
            expect_left="360" expect_top="10" expect_width="60" expect_height="60"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// vertical-lr + LTR: margin mapping
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-lr + LTR with margin-left and margin-top
// CSS Writing Modes §4.2: vertical-lr block flow = left → right
// CSS Writing Modes §4.3: LTR inline flow = top → bottom (in vertical mode)
//
// margin-left = block-start margin → shifts item right (in block axis = horizontal)
// margin-top = inline-start margin → shifts item down (in inline axis = vertical)
//
// Layout:
//   - Container: 400x300, columns: 100px, rows: 80px
//   - Item at (row 0, col 0): 40x40 with margin-left: 15px, margin-top: 10px
//   - axis_info: dir=Horizontal, main_dir_rev=NR, cross_dir_rev=NR
//   - main_axis_start(H, NR) = margin.left = 15 (block-start)
//   - cross_axis_start(H, NR) = margin.top = 10 (inline-start)
//   - offset_main = row_offset(0) + 15 = 15
//   - offset_cross = col_offset(0) + 10 = 10
//   - gen_origin(Horizontal): left = offset_main = 15, top = offset_cross = 10
#[test]
fn grid_margin_vertical_lr_ltr_left_top() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-left: 15px; margin-top: 10px;"
            expect_left="15" expect_top="10" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// Case: vertical-lr + LTR with all four margins
// Verifies that block-start (margin-left) and inline-start (margin-top)
// are the ones affecting initial position with start alignment.
//
// Layout:
//   - Container: 400x300, columns: 100px, rows: 80px
//   - Item: 40x40 with margin: 10px(top) 25px(right) 20px(bottom) 15px(left)
//   - block-start = margin-left = 15 → left offset
//   - inline-start = margin-top = 10 → top offset
//   - gen_origin(H): left = 15, top = 10
#[test]
fn grid_margin_vertical_lr_ltr_all_margins() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin: 10px 25px 20px 15px;"
            expect_left="15" expect_top="10" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// vertical-lr + RTL: margin mapping
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-lr + RTL with margin-left and margin-bottom
// CSS Writing Modes §4.2: vertical-lr block flow = left → right (unchanged by direction)
// CSS Writing Modes §2.1: RTL reverses inline direction to bottom → top
//
// margin-left = block-start margin (same as LTR, direction doesn't affect block axis)
// margin-bottom = inline-start margin (RTL flips inline, so bottom is start)
//
// Layout:
//   - Container: 400x300, columns: 100px, rows: 80px
//   - Item: 40x40, margin-left: 15px, margin-bottom: 10px
//   - axis_info: dir=Horizontal, main_dir_rev=NR, cross_dir_rev=Reversed
//   - main_axis_start(H, NR) = margin.left = 15
//   - cross_axis_start(H, Reversed) = margin.bottom = 10
//   - RTL: inline_offset = 300 - 0 - 100 = 200 (container_content_height=300)
//   - justify_offset_rtl(Start, 40, 100) = 100 - 40 = 60
//     (RTL start aligns to inline-start edge = bottom within track)
//   - offset_main = 0 + 0(align) + 15 = 15
//   - offset_cross = 200 + 60 + 10 = 270
//   - gen_origin(H): left = 15, top = 270
#[test]
fn grid_margin_vertical_lr_rtl_left_bottom() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: rtl; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-left: 15px; margin-bottom: 10px;"
            expect_left="15" expect_top="270" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// vertical-rl + LTR: margin mapping
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-rl + LTR with margin-right and margin-top
// CSS Writing Modes §4.2: vertical-rl block flow = right → left
// CSS Writing Modes §4.3: LTR inline flow = top → bottom (in vertical mode)
//
// margin-right = block-start margin (right → left, so right is start)
// margin-top = inline-start margin
//
// Layout:
//   - Container: 400x300, columns: 100px, rows: 80px
//   - Item: 40x40, margin-right: 15px, margin-top: 10px
//   - axis_info: dir=Horizontal, main_dir_rev=Reversed, cross_dir_rev=NR
//   - main_axis_start(H, Reversed) = margin.right = 15
//   - cross_axis_start(H, NR) = margin.top = 10
//   - offset_main = 0 + 0(align) + 15 = 15
//   - offset_cross = 0 + 0(justify) + 10 = 10
//
// gen_origin(Horizontal):
//   width = offset_main = 15, height = offset_cross = 10
//   width_rev = main_dir_rev = Reversed
//   left = track_size.width(100) - offset_main(15) - item_width(40) = 45
//   top = offset_cross = 10
#[test]
fn grid_margin_vertical_rl_ltr_right_top() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-rl; direction: ltr; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-right: 15px; margin-top: 10px;"
            expect_left="45" expect_top="10" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// vertical-rl + RTL: margin mapping
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-rl + RTL with margin-right and margin-bottom
// CSS Writing Modes §4.2: vertical-rl block flow = right → left
// CSS Writing Modes §2.1: RTL reverses inline to bottom → top
//
// margin-right = block-start margin (same as LTR, direction doesn't affect block)
// margin-bottom = inline-start margin (RTL flips inline start to bottom)
//
// Layout:
//   - Container: 400x300, columns: 100px, rows: 80px
//   - Item: 40x40, margin-right: 15px, margin-bottom: 10px
//   - axis_info: dir=Horizontal, main_dir_rev=Reversed, cross_dir_rev=Reversed
//   - main_axis_start(H, Reversed) = margin.right = 15
//   - cross_axis_start(H, Reversed) = margin.bottom = 10
//   - RTL: inline_offset = 300 - 0 - 100 = 200
//   - justify_offset_rtl(Start, 40, 100) = 100 - 40 = 60
//   - offset_main = 0 + 0(align) + 15 = 15
//   - offset_cross = 200 + 60 + 10 = 270
//   - gen_origin(H, main_rev=Reversed, cross_rev=NR overridden):
//     left = track_size.width(100) - offset_main(15) - item_width(40) = 45
//     top = 270
#[test]
fn grid_margin_vertical_rl_rtl_right_bottom() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-rl; direction: rtl; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-right: 15px; margin-bottom: 10px;"
            expect_left="45" expect_top="270" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Invariant: margin block-start direction is independent of `direction`
// ═══════════════════════════════════════════════════════════════════════════

// Case: vertical-lr — direction should NOT change block-start margin mapping
// CSS Writing Modes §2.1: direction only affects inline base direction
//
// In vertical-lr, block-start = margin-left regardless of direction.
// LTR and RTL should produce the same physical `left` for the same margin-left.
//
// Layout (LTR and RTL):
//   - Container: 400x300, rows: 80px
//   - Item: 40x40, margin-left: 20px
//   - Both: left = 20 (block-start margin is always margin-left)
#[test]
fn grid_margin_vertical_lr_block_start_independent_of_direction() {
    // LTR
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-left: 20px;"
            expect_left="20" expect_top="0" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    );

    // RTL — left position must be the same as LTR
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: rtl; grid-template-columns: 100px; grid-template-rows: 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-left: 20px;"
            expect_left="20" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Margin with multiple items in different grid cells
// ═══════════════════════════════════════════════════════════════════════════

// Case: horizontal-tb + LTR, 2x2 grid with different margins per item
// CSS Grid §10.3-10.4: Margins shift items within their grid areas
//
// Layout:
//   - Container: 400x300, columns: 100px 100px, rows: 100px 100px
//   - Item 1 (col 0, row 0): margin-left: 5px, margin-top: 10px → left=5, top=10
//   - Item 2 (col 1, row 0): margin-left: 15px, margin-top: 0 → left=115, top=0
//   - Item 3 (col 0, row 1): margin-left: 0, margin-top: 20px → left=0, top=120
//   - Item 4 (col 1, row 1): margin-left: 10px, margin-top: 5px → left=110, top=105
#[test]
fn grid_margin_horizontal_tb_ltr_multi_items() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: horizontal-tb; direction: ltr; grid-template-columns: 100px 100px; grid-template-rows: 100px 100px; align-items: start; justify-items: start;">
          <div style="width: 50px; height: 50px; margin-left: 5px; margin-top: 10px;"
            expect_left="5" expect_top="10" expect_width="50" expect_height="50"></div>
          <div style="width: 50px; height: 50px; margin-left: 15px;"
            expect_left="115" expect_top="0" expect_width="50" expect_height="50"></div>
          <div style="width: 50px; height: 50px; margin-top: 20px;"
            expect_left="0" expect_top="120" expect_width="50" expect_height="50"></div>
          <div style="width: 50px; height: 50px; margin-left: 10px; margin-top: 5px;"
            expect_left="110" expect_top="105" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: vertical-lr + LTR, 2x2 grid with different margins per item
// CSS Writing Modes §4.2 + CSS Grid §10.3-10.4:
//   - In vertical-lr: columns map to top (inline axis), rows map to left (block axis)
//   - Block-start margin = margin-left, Inline-start margin = margin-top
//
// Layout:
//   - Container: 400x300, columns: 100px 100px, rows: 80px 80px
//   - Item 1 (col 0, row 0): margin-left: 5px, margin-top: 10px
//     left = row_offset(0) + margin-left(5) = 5
//     top = col_offset(0) + margin-top(10) = 10
//   - Item 2 (col 1, row 0): margin-left: 15px, margin-top: 0
//     left = row_offset(0) + 15 = 15
//     top = col_offset(1=100) + 0 = 100
//   - Item 3 (col 0, row 1): margin-left: 0, margin-top: 20px
//     left = row_offset(1=80) + 0 = 80
//     top = col_offset(0) + 20 = 20
//   - Item 4 (col 1, row 1): margin-left: 10px, margin-top: 5px
//     left = row_offset(1=80) + 10 = 90
//     top = col_offset(1=100) + 5 = 105
#[test]
fn grid_margin_vertical_lr_ltr_multi_items() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 300px; writing-mode: vertical-lr; direction: ltr; grid-template-columns: 100px 100px; grid-template-rows: 80px 80px; align-items: start; justify-items: start;">
          <div style="width: 40px; height: 40px; margin-left: 5px; margin-top: 10px;"
            expect_left="5" expect_top="10" expect_width="40" expect_height="40"></div>
          <div style="width: 40px; height: 40px; margin-left: 15px;"
            expect_left="15" expect_top="100" expect_width="40" expect_height="40"></div>
          <div style="width: 40px; height: 40px; margin-top: 20px;"
            expect_left="80" expect_top="20" expect_width="40" expect_height="40"></div>
          <div style="width: 40px; height: 40px; margin-left: 10px; margin-top: 5px;"
            expect_left="90" expect_top="105" expect_width="40" expect_height="40"></div>
        </div>
    "#,
        true
    )
}
