// WPT-style tests for CSS Grid alignment properties
// Reference: CSS Box Alignment Module Level 3
// https://www.w3.org/TR/css-align-3/
//
// Grid alignment properties:
// - `align-items` (container): Default alignment for all grid items in block axis
// - `align-self` (item): Override alignment for a specific grid item in block axis
// - Values: start, end, center, stretch (default)

use crate::*;

// Case: align-items: start
// Spec points:
//   - Items align to the start of the cell in the block axis
//   - Items do not stretch to fill the cell height
// In this test:
//   - Container: 2 columns, align-items=start
//   - Row height: 100px (from second item)
//   - First item: height=50px, aligns to top of cell (expect_top=0)
#[test]
fn grid_align_items_start() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: start;">
          <div style="width: 50px; height: 50px;" expect_top="0" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items: end
// Spec points:
//   - Items align to the end of the cell in the block axis
// In this test:
//   - Container: 2 columns, align-items=end
//   - Row height: 100px (from second item)
//   - First item: height=50px, aligns to bottom of cell (expect_top=50)
#[test]
fn grid_align_items_end() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: end;">
          <div style="width: 50px; height: 50px;" expect_top="50" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items: center
// Spec points:
//   - Items align to the center of the cell in the block axis
// In this test:
//   - Container: 2 columns, align-items=center
//   - Row height: 100px (from second item)
//   - First item: height=50px, centered (expect_top=25)
#[test]
fn grid_align_items_center() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items: stretch (default)
// Spec points:
//   - Items stretch to fill the cell in the block axis
//   - This is the default behavior for grid items
// In this test:
//   - Container: 2 columns, align-items=stretch
//   - Row height: 100px (from second item)
//   - First item: no explicit height, stretches to fill cell
#[test]
fn grid_align_items_stretch() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: stretch;">
          <div style="width: 50px;" expect_top="0" expect_height="100"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-self overrides align-items
// Spec points:
//   - align-self on an item overrides the container's align-items
// In this test:
//   - Container: align-items=start
//   - First item: align-self=end, should align to bottom
//   - Second item: uses container's align-items=start
#[test]
fn grid_align_self_override() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: start;">
          <div style="width: 50px; height: 50px; align-self: end;" expect_top="50" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-self: center
// Spec points:
//   - align-self: center centers the item within its cell
// In this test:
//   - Container: align-items=start (default behavior)
//   - First item: align-self=center, centered in cell
#[test]
fn grid_align_self_center() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: start;">
          <div style="width: 50px; height: 50px; align-self: center;" expect_top="25" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple rows with align-items: center
// Spec points:
//   - align-items applies to all rows
//   - Each row's items are centered within their row's height
// In this test:
//   - Container: 2 columns, 2 rows, align-items=center
//   - Row 1 height: 100px, Row 2 height: 80px
//   - Items centered in their respective rows
#[test]
fn grid_align_items_center_multiple_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: center;">
          <div style="width: 50px; height: 40px;" expect_top="30" expect_height="40"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
          <div style="width: 50px; height: 80px;" expect_top="100" expect_height="80"></div>
          <div style="width: 50px; height: 40px;" expect_top="120" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items with gap
// Spec points:
//   - Gap does not affect alignment within cells
//   - Items are still aligned within their cell boundaries
// In this test:
//   - Container: 2 columns, gap=20px, align-items=center
//   - Row height: 100px
//   - First item: height=50px, centered (expect_top=25)
#[test]
fn grid_align_items_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; gap: 20px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items: flex-start (same as start in grid)
// Spec points:
//   - flex-start behaves like start in grid context
// In this test:
//   - Container: align-items=flex-start
//   - Items align to the top of their cells
#[test]
fn grid_align_items_flex_start() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: flex-start;">
          <div style="width: 50px; height: 50px;" expect_top="0" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items: flex-end (same as end in grid)
// Spec points:
//   - flex-end behaves like end in grid context
// In this test:
//   - Container: align-items=flex-end
//   - Items align to the bottom of their cells
#[test]
fn grid_align_items_flex_end() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; align-items: flex-end;">
          <div style="width: 50px; height: 50px;" expect_top="50" expect_height="50"></div>
          <div style="width: 50px; height: 100px;" expect_top="0" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: align-self on multiple items
// Spec points:
//   - Each item can have its own align-self value
// In this test:
//   - Container: 3 columns, 1 row (height=100px)
//   - First: align-self=start (top)
//   - Second: align-self=center (middle)
//   - Third: align-self=end (bottom)
#[test]
fn grid_align_self_multiple() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px 100px; grid-template-rows: 100px;">
          <div style="width: 50px; height: 50px; align-self: start;" expect_top="0" expect_height="50"></div>
          <div style="width: 50px; height: 50px; align-self: center;" expect_top="25" expect_height="50"></div>
          <div style="width: 50px; height: 50px; align-self: end;" expect_top="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items with fr rows
// Spec points:
//   - align-items works with fr-sized rows
// In this test:
//   - Container: height=200px, rows: 1fr 1fr (100px each)
//   - align-items=center
//   - Items centered within their fr-sized cells
#[test]
fn grid_align_items_with_fr_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 200px; grid-template-columns: 100px; grid-template-rows: 1fr 1fr; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="125" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// =============================================
// justify-items tests (inline axis / horizontal)
// =============================================

// Case: justify-items: start
// Spec points:
//   - Items align to the start of the cell in the inline axis
//   - Items do not stretch to fill the cell width
// In this test:
//   - Container: 2 columns (100px each), justify-items=start
//   - First item: width=50px, aligns to left of cell (expect_left=0)
//   - Second item: width=50px, aligns to left of second cell (expect_left=100)
#[test]
fn grid_justify_items_start() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: start;">
          <div style="width: 50px; height: 50px;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="100" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items: end
// Spec points:
//   - Items align to the end of the cell in the inline axis
// In this test:
//   - Container: 2 columns (100px each), justify-items=end
//   - First item: width=50px, aligns to right of cell (expect_left=50)
//   - Second item: width=50px, aligns to right of second cell (expect_left=150)
#[test]
fn grid_justify_items_end() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: end;">
          <div style="width: 50px; height: 50px;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="150" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items: center
// Spec points:
//   - Items align to the center of the cell in the inline axis
// In this test:
//   - Container: 2 columns (100px each), justify-items=center
//   - First item: width=50px, centered (expect_left=25)
//   - Second item: width=50px, centered (expect_left=125)
#[test]
fn grid_justify_items_center() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="25" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="125" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items: stretch (default)
// Spec points:
//   - Items stretch to fill the cell width (when no explicit width)
// In this test:
//   - Container: 2 columns (100px each), justify-items=stretch
//   - Items without explicit width should fill the cell
#[test]
fn grid_justify_items_stretch() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: stretch;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items with fr columns
// Spec points:
//   - justify-items works with fr-sized columns
// In this test:
//   - Container: width=300px, columns: 1fr 2fr (100px and 200px)
//   - justify-items=center
//   - Items centered within their fr-sized cells
#[test]
fn grid_justify_items_with_fr_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 1fr 2fr; justify-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="25" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="175" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items with gap
// Spec points:
//   - justify-items works correctly with column-gap
// In this test:
//   - Container: 2 columns (100px each) with 20px gap, justify-items=end
//   - First item at right of first cell (left=50)
//   - Second item at right of second cell (left=170)
#[test]
fn grid_justify_items_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; column-gap: 20px; justify-items: end;">
          <div style="width: 50px; height: 50px;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="170" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items: left
// Spec points:
//   - Same as start for LTR direction
// In this test:
//   - Container: justify-items=left
//   - Items align to left of cells
#[test]
fn grid_justify_items_left() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: left;">
          <div style="width: 50px; height: 50px;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="100" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-items: right
// Spec points:
//   - Same as end for LTR direction
// In this test:
//   - Container: justify-items=right
//   - Items align to right of cells
#[test]
fn grid_justify_items_right() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: right;">
          <div style="width: 50px; height: 50px;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="150" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// =============================================
// Combined align-items and justify-items tests
// =============================================

// Case: Combined alignment (center, center)
// Spec points:
//   - Both align-items and justify-items can be used together
// In this test:
//   - Container: 100px x 100px cell, align-items=center, justify-items=center
//   - Item: 50px x 50px, centered both horizontally and vertically
#[test]
fn grid_align_justify_center() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: 100px; align-items: center; justify-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25" expect_left="25" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Combined alignment (end, start)
// Spec points:
//   - Different alignments on different axes
// In this test:
//   - Container: 100px x 100px cell
//   - align-items=end (item at bottom)
//   - justify-items=start (item at left)
#[test]
fn grid_align_end_justify_start() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: 100px; align-items: end; justify-items: start;">
          <div style="width: 50px; height: 50px;" expect_top="50" expect_left="0" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Combined alignment (start, end)
// Spec points:
//   - Different alignments on different axes
// In this test:
//   - Container: 100px x 100px cell
//   - align-items=start (item at top)
//   - justify-items=end (item at right)
#[test]
fn grid_align_start_justify_end() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: 100px; align-items: start; justify-items: end;">
          <div style="width: 50px; height: 50px;" expect_top="0" expect_left="50" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-self overrides align-items with justify-items
// Spec points:
//   - align-self overrides align-items for individual items
//   - justify-items still applies to all items
// In this test:
//   - Container: align-items=start, justify-items=center
//   - Item 1: align-self=end (at bottom, centered horizontally)
//   - Item 2: uses align-items=start (at top, centered horizontally)
#[test]
fn grid_align_self_with_justify_items() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; grid-template-rows: 100px; align-items: start; justify-items: center;">
          <div style="width: 50px; height: 50px; align-self: end;" expect_top="50" expect_left="25" expect_width="50" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="0" expect_left="125" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// =============================================
// justify-self tests (item-level inline axis alignment)
// =============================================

// Case: justify-self: start
// Spec points:
//   - justify-self overrides justify-items for individual items
// In this test:
//   - Container: justify-items=center
//   - Item 1: justify-self=start, aligns to left of cell
//   - Item 2: uses justify-items=center
#[test]
fn grid_justify_self_start() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: center;">
          <div style="width: 50px; height: 50px; justify-self: start;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="125" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-self: end
// Spec points:
//   - justify-self=end aligns item to the right of its cell
// In this test:
//   - Container: justify-items=start (default)
//   - Item 1: justify-self=end, aligns to right of cell (left=50)
//   - Item 2: uses justify-items, aligns to left (left=100)
#[test]
fn grid_justify_self_end() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; justify-items: start;">
          <div style="width: 50px; height: 50px; justify-self: end;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="100" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-self: center
// Spec points:
//   - justify-self=center centers item horizontally in its cell
// In this test:
//   - Container: justify-items=start
//   - Item: justify-self=center, centered (left=25)
#[test]
fn grid_justify_self_center() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; justify-items: start;">
          <div style="width: 50px; height: 50px; justify-self: center;" expect_left="25" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-self: auto
// Spec points:
//   - justify-self=auto inherits from justify-items
// In this test:
//   - Container: justify-items=end
//   - Item: justify-self=auto (should use justify-items=end)
#[test]
fn grid_justify_self_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; justify-items: end;">
          <div style="width: 50px; height: 50px; justify-self: auto;" expect_left="50" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple items with different justify-self values
// Spec points:
//   - Each item can have its own justify-self value
// In this test:
//   - Container: 3 columns (100px each)
//   - Item 1: justify-self=start (left=0)
//   - Item 2: justify-self=center (left=125)
//   - Item 3: justify-self=end (left=250)
#[test]
fn grid_justify_self_multiple() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px 100px;">
          <div style="width: 50px; height: 50px; justify-self: start;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: center;" expect_left="125" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: end;" expect_left="250" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-self: left/right
// Spec points:
//   - left/right are physical direction keywords
// In this test:
//   - Item 1: justify-self=left (left=0)
//   - Item 2: justify-self=right (left=150)
#[test]
fn grid_justify_self_left_right() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px;">
          <div style="width: 50px; height: 50px; justify-self: left;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: right;" expect_left="150" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Combined align-self and justify-self
// Spec points:
//   - Both align-self and justify-self can be used on the same item
// In this test:
//   - Container: 100px x 100px cell
//   - Item: align-self=end, justify-self=end (bottom-right corner)
#[test]
fn grid_align_self_justify_self_combined() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: 100px;">
          <div style="width: 50px; height: 50px; align-self: end; justify-self: end;" expect_top="50" expect_left="50" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-self with fr columns
// Spec points:
//   - justify-self works with fr-sized columns
// In this test:
//   - Container: width=300px, columns: 1fr 2fr (100px and 200px)
//   - Item 1: justify-self=end in 100px column (left=50)
//   - Item 2: justify-self=center in 200px column (left=175)
#[test]
fn grid_justify_self_with_fr_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 1fr 2fr;">
          <div style="width: 50px; height: 50px; justify-self: end;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: center;" expect_left="175" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-self with gap
// Spec points:
//   - justify-self works correctly with column-gap
// In this test:
//   - Container: 2 columns (100px each) with 20px gap
//   - Item 1: justify-self=center in first column (left=25)
//   - Item 2: justify-self=end in second column (left=170)
#[test]
fn grid_justify_self_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px 100px; column-gap: 20px;">
          <div style="width: 50px; height: 50px; justify-self: center;" expect_left="25" expect_width="50"></div>
          <div style="width: 50px; height: 50px; justify-self: end;" expect_left="170" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// =============================================
// align-content tests (track alignment in block axis)
// =============================================

// Case: align-content: start
// Spec points:
//   - Tracks align to the start of the container in the block axis
// In this test:
//   - Container: height=300px, 2 rows (50px each), total=100px
//   - Tracks at top of container (top=0)
#[test]
fn grid_align_content_start() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 50px 50px; align-content: start;">
          <div style="width: 50px; height: 50px;" expect_top="0" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-content: end
// Spec points:
//   - Tracks align to the end of the container in the block axis
// In this test:
//   - Container: height=300px, 2 rows (50px each), total=100px
//   - Available space: 200px, tracks at bottom (first row top=200)
#[test]
fn grid_align_content_end() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 50px 50px; align-content: end;">
          <div style="width: 50px; height: 50px;" expect_top="200" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="250" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-content: center
// Spec points:
//   - Tracks are centered in the container in the block axis
// In this test:
//   - Container: height=300px, 2 rows (50px each), total=100px
//   - Available space: 200px, centered (first row top=100)
#[test]
fn grid_align_content_center() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 50px 50px; align-content: center;">
          <div style="width: 50px; height: 50px;" expect_top="100" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="150" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-content: space-between
// Spec points:
//   - Tracks are evenly distributed with first at start and last at end
// In this test:
//   - Container: height=300px, 2 rows (50px each), total=100px
//   - Available space: 200px distributed between tracks
//   - First row: top=0, Second row: top=250 (0+50+200)
#[test]
fn grid_align_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 50px 50px; align-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_top="0" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="250" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-content: space-around
// Spec points:
//   - Tracks have equal space around them (half space at edges)
// In this test:
//   - Container: height=300px, 2 rows (50px each), total=100px
//   - Available space: 200px, each track gets 100px around
//   - First row: top=50 (100/2), Second row: top=150 (50+50+100/2)
#[test]
fn grid_align_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 50px 50px; align-content: space-around;">
          <div style="width: 50px; height: 50px;" expect_top="50" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="200" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-content: space-evenly
// Spec points:
//   - Tracks have equal space between and around them
// In this test:
//   - Container: height=250px, 2 rows (50px each), total=100px
//   - Available space: 150px, divided into 3 equal parts (50px each)
//   - First row: top=50, Second row: top=150
#[test]
fn grid_align_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 250px; grid-template-columns: 100px; grid-template-rows: 50px 50px; align-content: space-evenly;">
          <div style="width: 50px; height: 50px;" expect_top="50" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="150" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// =============================================
// justify-content tests (track alignment in inline axis)
// =============================================

// Case: justify-content: start
// Spec points:
//   - Tracks align to the start of the container in the inline axis
// In this test:
//   - Container: width=300px, 2 columns (50px each), total=100px
//   - Tracks at left of container (left=0)
#[test]
fn grid_justify_content_start() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px 50px; justify-content: start;">
          <div style="width: 50px; height: 50px;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="50" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-content: end
// Spec points:
//   - Tracks align to the end of the container in the inline axis
// In this test:
//   - Container: width=300px, 2 columns (50px each), total=100px
//   - Available space: 200px, tracks at right (first column left=200)
#[test]
fn grid_justify_content_end() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px 50px; justify-content: end;">
          <div style="width: 50px; height: 50px;" expect_left="200" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="250" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-content: center
// Spec points:
//   - Tracks are centered in the container in the inline axis
// In this test:
//   - Container: width=300px, 2 columns (50px each), total=100px
//   - Available space: 200px, centered (first column left=100)
#[test]
fn grid_justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px 50px; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_left="100" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="150" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-content: space-between
// Spec points:
//   - Tracks are evenly distributed with first at start and last at end
// In this test:
//   - Container: width=300px, 2 columns (50px each), total=100px
//   - Available space: 200px distributed between tracks
//   - First column: left=0, Second column: left=250 (0+50+200)
#[test]
fn grid_justify_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px 50px; justify-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_left="0" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="250" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-content: space-around
// Spec points:
//   - Tracks have equal space around them (half space at edges)
// In this test:
//   - Container: width=300px, 2 columns (50px each), total=100px
//   - Available space: 200px, each track gets 100px around
//   - First column: left=50 (100/2), Second column: left=200 (50+50+100)
#[test]
fn grid_justify_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px 50px; justify-content: space-around;">
          <div style="width: 50px; height: 50px;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="200" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: justify-content: space-evenly
// Spec points:
//   - Tracks have equal space between and around them
// In this test:
//   - Container: width=250px, 2 columns (50px each), total=100px
//   - Available space: 150px, divided into 3 equal parts (50px each)
//   - First column: left=50, Second column: left=150
#[test]
fn grid_justify_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 250px; grid-template-columns: 50px 50px; justify-content: space-evenly;">
          <div style="width: 50px; height: 50px;" expect_left="50" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="150" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Combined align-content and justify-content (center, center)
// Spec points:
//   - Both content alignments can be used together
// In this test:
//   - Container: 300x300px, tracks: 100x100px
//   - Both centered: top=100, left=100
#[test]
fn grid_align_justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; height: 300px; grid-template-columns: 100px; grid-template-rows: 100px; align-content: center; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_top="100" expect_left="100" expect_width="50" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: align-content with gap
// Spec points:
//   - Content alignment works with existing gap
// In this test:
//   - Container: height=300px, 2 rows (50px) + 20px gap = 120px
//   - align-content: center, available space: 180px
//   - First row: top=90 (180/2)
#[test]
fn grid_align_content_center_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 50px 50px; row-gap: 20px; align-content: center;">
          <div style="width: 50px; height: 50px;" expect_top="90" expect_height="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="160" expect_height="50"></div>
        </div>
    "#,
        true
    )
}
