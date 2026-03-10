// WPT-style tests for the `fr` (flexible length) unit in CSS Grid Layout
// Reference: CSS Grid Layout Module Level 1
// https://www.w3.org/TR/css-grid-1/#fr-unit
//
// The `fr` unit represents a fraction of the leftover space in the grid container.
//
// Key behaviors:
// - fr tracks share the remaining space after fixed tracks are sized
// - The space is distributed proportionally based on fr values
// - 1fr + 2fr + 1fr = 4fr total, so 1fr gets 1/4 of remaining space

use crate::*;

// Case: Equal fr columns (1fr 1fr 1fr)
// Spec points:
//   - Equal fr values result in equal track sizes
//   - All remaining space is distributed equally
// In this test:
//   - Container: width=300px, 3 columns of 1fr each
//   - Each column: 300 / 3 = 100px
//   - Column positions: 0, 100, 200
#[test]
fn grid_fr_equal_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 1fr 1fr 1fr;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Different fr values (1fr 2fr 1fr)
// Spec points:
//   - fr values determine proportional distribution
//   - 1fr : 2fr : 1fr = 1:2:1 ratio
// In this test:
//   - Container: width=400px, columns: 1fr, 2fr, 1fr (total 4fr)
//   - Each fr unit: 400 / 4 = 100px
//   - Column widths: 100px, 200px, 100px
//   - Column positions: 0, 100, 300
#[test]
fn grid_fr_proportional() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: 1fr 2fr 1fr;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="200"></div>
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Mixed fixed and fr columns (100px 1fr 100px)
// Spec points:
//   - Fixed tracks are sized first
//   - fr tracks share the remaining space
// In this test:
//   - Container: width=400px, columns: 100px, 1fr, 100px
//   - Fixed: 100 + 100 = 200px
//   - Remaining for fr: 400 - 200 = 200px
//   - 1fr column: 200px
//   - Column positions: 0, 100, 300
#[test]
fn grid_fr_mixed_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: 100px 1fr 100px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="200"></div>
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Mixed fixed and multiple fr columns (100px 1fr 2fr)
// Spec points:
//   - After fixed sizing, remaining space is distributed by fr ratio
// In this test:
//   - Container: width=400px, columns: 100px, 1fr, 2fr
//   - Fixed: 100px
//   - Remaining: 400 - 100 = 300px, total fr = 3
//   - 1fr = 100px, 2fr = 200px
//   - Column positions: 0, 100, 200
#[test]
fn grid_fr_mixed_fixed_multiple_fr() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: 100px 1fr 2fr;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: fr rows
// Spec points:
//   - fr unit works the same way for rows
//   - Container must have explicit height for fr rows
// In this test:
//   - Container: height=300px, rows: 1fr 2fr
//   - Total fr = 3, each fr = 100px
//   - Row heights: 100px, 200px
//   - Row positions: 0, 100
#[test]
fn grid_fr_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: 1fr 2fr;">
          <div expect_top="0" expect_height="100"></div>
          <div expect_top="100" expect_height="200"></div>
        </div>
    "#,
        true
    )
}

// Case: fr with gap
// Spec points:
//   - Gap is subtracted before distributing space to fr tracks
// In this test:
//   - Container: width=330px, columns: 1fr 1fr 1fr, gap=10px
//   - Total gap: 10 * 2 = 20px
//   - Remaining for fr: 330 - 20 = 310px (≈103.33px each, but let's check)
//   - Actually: 310 / 3 ≈ 103.33px
//   - Let's use 320px for cleaner math: (320 - 20) / 3 = 100px
#[test]
fn grid_fr_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 320px; grid-template-columns: 1fr 1fr 1fr; column-gap: 10px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="110" expect_width="100"></div>
          <div style="height: 50px;" expect_left="220" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Single fr column takes all space
// Spec points:
//   - A single 1fr takes all available space
// In this test:
//   - Container: width=300px, columns: 1fr
//   - 1fr takes entire 300px
#[test]
fn grid_fr_single_column() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 1fr;">
          <div style="height: 50px;" expect_left="0" expect_width="300"></div>
        </div>
    "#,
        true
    )
}

// Case: fr with percentage columns
// Spec points:
//   - Percentage is resolved first, then fr shares remaining
// In this test:
//   - Container: width=400px, columns: 25%, 1fr, 25%
//   - Percentage columns: 100px each
//   - Remaining for fr: 400 - 200 = 200px
//   - Column positions: 0, 100, 300
#[test]
fn grid_fr_with_percentage() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: 25% 1fr 25%;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="200"></div>
          <div style="height: 50px;" expect_left="300" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: 2x2 grid with fr columns and rows
// Spec points:
//   - fr works independently on both axes
// In this test:
//   - Container: width=200px, height=150px
//   - Columns: 1fr 1fr (100px each)
//   - Rows: 1fr 2fr (50px, 100px)
#[test]
fn grid_fr_2x2() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; height: 150px; grid-template-columns: 1fr 1fr; grid-template-rows: 1fr 2fr;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="100" expect_height="100"></div>
          <div expect_left="100" expect_top="50" expect_width="100" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: fr with larger values (3fr 5fr 2fr)
// Spec points:
//   - Any positive fr value is valid
//   - Distribution is proportional regardless of absolute values
// In this test:
//   - Container: width=500px, columns: 3fr 5fr 2fr (total 10fr)
//   - Each fr unit: 500 / 10 = 50px
//   - Column widths: 150px, 250px, 100px
//   - Column positions: 0, 150, 400
#[test]
fn grid_fr_larger_values() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 500px; grid-template-columns: 3fr 5fr 2fr;">
          <div style="height: 50px;" expect_left="0" expect_width="150"></div>
          <div style="height: 50px;" expect_left="150" expect_width="250"></div>
          <div style="height: 50px;" expect_left="400" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: fr combined with gap and fixed columns
// Spec points:
//   - Complex scenario: gap + fixed + fr all work together
// In this test:
//   - Container: width=360px, columns: 100px 1fr 1fr, gap=20px
//   - Total gap: 20 * 2 = 40px
//   - Available for sizing: 360 - 40 = 320px
//   - Fixed: 100px, remaining for fr: 320 - 100 = 220px
//   - Each fr: 220 / 2 = 110px
//   - Column positions: 0, 120 (100+20), 250 (100+20+110+20)
#[test]
fn grid_fr_complex() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 360px; grid-template-columns: 100px 1fr 1fr; column-gap: 20px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="120" expect_width="110"></div>
          <div style="height: 50px;" expect_left="250" expect_width="110"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════
// Indefinite free space: §11.7.1 "Find the Size of an fr"
// https://www.w3.org/TR/css-grid-1/#algo-find-fr-size
//
// When the available space for fr tracks is indefinite (e.g. inline-grid
// without explicit width), fr sizes are derived from items' max-content
// contributions rather than distributing remaining space.
// ═══════════════════════════════════════════════════════════════════════

// Case: Equal fr columns with indefinite width (inline-grid)
// Spec points:
//   - §11.7.1: When free space is indefinite, fr = max(max-content / fr_value)
//   - display: inline-grid → container shrinks to fit content
// In this test:
//   - Container: inline-grid, columns: 1fr 1fr
//   - Item 1: width=100px → max-content=100, hypothetical 1fr = 100/1 = 100
//   - Item 2: width=100px → max-content=100, hypothetical 1fr = 100/1 = 100
//   - Unified 1fr = max(100, 100) = 100
//   - Both columns: 100px each
//   - Container width: 200px
#[test]
fn grid_fr_indefinite_equal_columns() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 1fr 1fr;" expect_width="200">
          <div style="width: 100px; height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="width: 100px; height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Proportional fr with indefinite width
// Spec points:
//   - §11.7.1: hypothetical_1fr = max(max-content / fr_value) across all fr tracks
//   - Items without explicit width stretch to track width
// In this test:
//   - Container: inline-grid, columns: 1fr 2fr
//   - Item 1 (1fr): padding-left=100px → max-content=100, hypothetical 1fr = 100/1 = 100
//   - Item 2 (2fr): padding-left=200px → max-content=200, hypothetical 1fr = 200/2 = 100
//   - Unified 1fr = max(100, 100) = 100
//   - Column 1: 100×1 = 100px, Column 2: 100×2 = 200px
//   - Container width: 300px
#[test]
fn grid_fr_indefinite_proportional() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 1fr 2fr;" expect_width="300">
          <div style="padding-left: 100px; height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="padding-left: 200px; height: 50px;" expect_left="100" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: Proportional fr with unequal hypothetical values
// Spec points:
//   - §11.7.1: The largest hypothetical_1fr dominates
// In this test:
//   - Container: inline-grid, columns: 1fr 2fr
//   - Item 1 (1fr): width=150px → hypothetical 1fr = 150/1 = 150
//   - Item 2 (2fr): width=200px → hypothetical 1fr = 200/2 = 100
//   - Unified 1fr = max(150, 100) = 150
//   - Column 1: 150px (item stretches), Column 2: 300px (item has explicit width 200)
//   - Container width: 450px
#[test]
fn grid_fr_indefinite_largest_hypothetical() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 1fr 2fr;" expect_width="450">
          <div style="width: 150px; height: 50px;" expect_left="0" expect_width="150"></div>
          <div style="width: 200px; height: 50px;" expect_left="150" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: Mixed fixed + fr with indefinite width
// Spec points:
//   - Fixed columns keep their explicit size
//   - §11.7.1 applies to fr tracks even when mixed with fixed tracks
//   - The free space being indefinite is determined by the container,
//     not by the presence of fixed tracks
// In this test:
//   - Container: inline-grid, columns: 80px 1fr 1fr
//   - Fixed column: 80px
//   - Item 2 (1fr): padding-left=100px → max-content=100, hypothetical 1fr = 100/1 = 100
//   - Item 3 (1fr): padding-left=60px  → max-content=60, hypothetical 1fr = 60/1 = 60
//   - Unified 1fr = max(100, 60) = 100
//   - Column sizes: 80, 100, 100
//   - Container width: 280px
//   - Item 3 stretches from 60 to 100 (no explicit width, stretch to track width)
#[test]
fn grid_fr_indefinite_mixed_fixed() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 80px 1fr 1fr;" expect_width="280">
          <div style="height: 50px;" expect_left="0" expect_width="80"></div>
          <div style="padding-left: 100px; height: 50px;" expect_left="80" expect_width="100"></div>
          <div style="padding-left: 60px; height: 50px;" expect_left="180" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: fr with indefinite height (rows)
// Spec points:
//   - §11.7.1 also applies to row direction when height is indefinite
// In this test:
//   - Container: inline-grid without height, columns: 100px, rows: 1fr 2fr
//   - Item 1 (1fr): padding-top=60px → max-content height=60, hypothetical 1fr = 60/1 = 60
//   - Item 2 (2fr): padding-top=80px → max-content height=80, hypothetical 1fr = 80/2 = 40
//   - Unified 1fr = max(60, 40) = 60
//   - Row 1: 60×1 = 60px, Row 2: 60×2 = 120px
#[test]
fn grid_fr_indefinite_rows() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 100px; grid-template-rows: 1fr 2fr;">
          <div style="padding-top: 60px;" expect_top="0" expect_height="60"></div>
          <div style="padding-top: 80px;" expect_top="60" expect_height="120"></div>
        </div>
    "#,
        true
    )
}
