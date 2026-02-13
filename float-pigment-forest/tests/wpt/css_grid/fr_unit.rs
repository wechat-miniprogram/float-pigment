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
