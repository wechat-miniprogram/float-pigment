// Tests for CSS Grid Layout basic functionality
// Based on CSS Grid Layout Module Level 1 specification:
// - `display: grid` establishes a grid formatting context
// - `grid-template-columns` and `grid-template-rows` define explicit grid tracks
// - Grid items are placed into cells according to the grid auto-placement algorithm
// - `auto` track sizing allows tracks to size based on content

mod gap;

use crate::*;

// Case: Multi-row grid with mixed auto and fixed column sizes
// Spec points:
// - `auto` columns share remaining space equally after fixed columns
// - Grid items wrap to new rows when columns are filled
// - Row heights are determined by the tallest item in each row
// In this test:
// - Container: width=600px, 3 columns (auto, 100px, auto), 2 explicit rows (30px, 40px)
// - Available for auto columns: 600 - 100 = 500px, each auto = 250px
// - Row 1: items at left=0, 250, 350
// - Row 2: items at left=0, 250, 350
// - Row 3 (implicit): items at left=0, 250, 350, height=32px (default text height)
#[test]
fn grid() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto; grid-template-rows: 30px 40px;">
          <div expect_height="30" expect_left="0">header1</div>
          <div expect_height="30" expect_left="250">header2</div>
          <div expect_height="30" expect_left="350">header3</div>
          <div expect_height="40" expect_left="0">content1</div>
          <div style="width: 23px; height: 23px" expect_height="23" expect_left="250">content2</div>
          <div expect_height="40" expect_left="350">content3</div>
          <div expect_height="32" expect_left="0">content4</div>
          <div expect_height="32" expect_left="250">content5</div>
          <div expect_height="32" expect_left="350">content6</div>
        </div>
    "#,
        true
    )
}

// Case: Single row grid with auto columns
// Spec points:
// - When only columns are specified, items fill a single row
// - Container height is determined by item heights
// In this test:
// - Container: width=600px, 3 columns (auto, 100px, auto)
// - Available for auto columns: 600 - 100 = 500px, each auto = 250px
// - All items in one row, height=32px (default text height)
#[test]
fn grid_1() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto;" expect_height="32">
          <div expect_width="250" expect_height="32">header1</div>
          <div expect_width="100" expect_height="32">header2</div>
          <div expect_width="250" expect_height="32">header3</div>
        </div>
    "#,
        true
    )
}

// Case: Grid with tall nested content
// Spec points:
// - Row height expands to fit the tallest item
// - All items in the same row get the same height
// In this test:
// - Container: width=600px, 3 columns (auto, 100px, auto)
// - Second column contains a 300px tall child
// - All items in the row expand to height=300px
#[test]
fn grid_2() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto;" expect_height="300">
          <div expect_width="250" expect_height="300">header1</div>
          <div expect_width="100">
            <div style="height: 300px;" expect_height="300"></div>
          </div>
          <div expect_width="250" expect_height="300">header3</div>
        </div>
    "#,
        true
    )
}

// Case: Grid with overflow to implicit rows
// Spec points:
// - Items exceeding explicit grid create implicit rows
// - Implicit rows are sized based on content
// In this test:
// - Container: width=600px, 3 columns (auto, 100px, auto)
// - 4 items: first 3 fill row 1 (300px tall), 4th goes to row 2 (16px)
// - Total height: 300 + 16 = 316px
#[test]
fn grid_3() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto;" expect_height="316">
          <div expect_width="250" expect_height="300">header1</div>
          <div expect_width="100">
            <div style="height: 300px;" expect_height="300"></div>
          </div>
          <div expect_width="250" expect_height="300">header3</div>
          <div expect_width="250" expect_height="16">foote</div>
        </div>
    "#,
        true
    )
}

// Case: Grid items with margin
// Spec points:
// - Item margins are applied within the grid cell
// - Margins offset the item from the cell edges
// In this test:
// - Container: 2 columns of 100px each
// - Items have margin-top=10px, margin-left=10px
// - First item: expect_left=10, expect_top=10
// - Second item: expect_left=110 (100 cell + 10 margin), expect_top=10
#[test]
fn grid_item_with_margin() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px 100px" >
          <div style="margin-top: 10px; margin-left: 10px; width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
          <div style="margin-top: 10px; margin-left: 10px; width: 50px; height: 50px;" expect_top="10" expect_left="110"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid items with padding and border
// Spec points:
// - Padding and border increase the item's total size
// - Row height accommodates the largest item including padding/border
// In this test:
// - Container: 2 columns of 100px each
// - First item: width=50px + padding=20px + border=1px = 71px (but content-box so 70+1=71)
// - Second item: height=100px + border=1px = 101px
// - Row height: max(first item height, 101) = 101px
#[test]
fn grid_item_with_border() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px 100px" >
          <div style="width: 50px; padding: 10px; border-bottom: 1px solid black;" expect_height="101" expect_width="70"></div>
          <div style="width: 50px; height: 100px; border-bottom: 1px solid black;" expect_height="101" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// Case: Grid with auto columns and large min-content
// Spec points:
// - Auto columns respect the min-content size of their items
// - When min-content exceeds available space, columns may overflow
// In this test:
// - Container: width=100px, 2 auto columns
// - Each item contains a 100px wide child (min-content = 100px)
// - Each column gets at least 100px (min-content)
#[test]
fn grid_item_min_content_size() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 100px; grid-template-columns: auto auto" >
          <div expect_width="100">
            <div style="width: 100px; height: 100px;" expect_width="100" expect_height="100"></div>
          </div>
          <div expect_width="100">
            <div style="width: 100px; height: 100px;" expect_width="100" expect_height="100"></div>
          </div>       
        </div>
    "#,
        true
    )
}

// Case: Grid with auto columns and text content
// Spec points:
// - Auto columns with text content size based on text width
// - Text establishes min-content size for the column
// In this test:
// - Container: width=20px, 2 auto columns
// - Each item contains text that determines column width
#[test]
fn grid_item_min_content_size_2() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 20px; grid-template-columns: auto auto" >
          <div>
            <div>hello</div>
          </div>
          <div>
            <div>world</div>
          </div>       
        </div>
    "#,
        true
    )
}
