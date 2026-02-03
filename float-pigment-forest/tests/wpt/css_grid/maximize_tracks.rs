// WPT-style tests for Maximize Tracks (§11.6)
// Reference: CSS Grid Layout Module Level 1
// https://www.w3.org/TR/css-grid-1/#algo-grow-tracks
//
// When the container has a definite size and there is free space left over
// after track sizing, auto tracks are expanded to fill the remaining space.

use crate::*;

// ═══════════════════════════════════════════════════════════════════════════
// Maximize Tracks - Columns (§11.6)
// ═══════════════════════════════════════════════════════════════════════════

// Case: Single auto column fills container width
// Spec points:
//   - Auto track expands to fill container when there's free space
// In this test:
//   - Container: width=300px, 1 auto column
//   - Item: expands to fill 300px
#[test]
fn maximize_tracks_single_auto_column() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto;">
          <div style="height: 50px;" expect_left="0" expect_width="300"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto column with fixed column
// Spec points:
//   - Auto track takes remaining space after fixed tracks
// In this test:
//   - Container: width=300px, columns: 100px auto
//   - Auto column: 300 - 100 = 200px
#[test]
fn maximize_tracks_auto_with_fixed_column() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px auto;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple auto columns share free space equally
// Spec points:
//   - Free space is distributed equally among auto tracks
// In this test:
//   - Container: width=300px, columns: auto auto auto
//   - Each auto column: 300 / 3 = 100px
#[test]
fn maximize_tracks_multiple_auto_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto auto auto;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto columns with gap
// Spec points:
//   - Gap is subtracted before distributing free space
// In this test:
//   - Container: width=320px, gap=20px, columns: auto auto
//   - Free space: 320 - 20 = 300px, each auto: 150px
#[test]
fn maximize_tracks_auto_columns_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 320px; column-gap: 20px; grid-template-columns: auto auto;">
          <div style="height: 50px;" expect_left="0" expect_width="150"></div>
          <div style="height: 50px;" expect_left="170" expect_width="150"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto column between fixed columns
// Spec points:
//   - Auto track fills space between fixed tracks
// In this test:
//   - Container: width=300px, columns: 50px auto 50px
//   - Auto column: 300 - 50 - 50 = 200px
#[test]
fn maximize_tracks_auto_between_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 50px auto 50px;">
          <div style="height: 50px;" expect_left="0" expect_width="50"></div>
          <div style="height: 50px;" expect_left="50" expect_width="200"></div>
          <div style="height: 50px;" expect_left="250" expect_width="50"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Maximize Tracks - Rows (§11.6)
// ═══════════════════════════════════════════════════════════════════════════

// Case: Single auto row fills container height
// Spec points:
//   - Auto row expands to fill container when there's free space
// In this test:
//   - Container: height=200px, 1 auto row
//   - Item: expands to fill 200px
#[test]
fn maximize_tracks_single_auto_row() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 200px; grid-template-columns: 100px; grid-template-rows: auto;">
          <div style="width: 50px;" expect_top="0" expect_height="200"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto row with fixed row
// Spec points:
//   - Auto row takes remaining space after fixed rows
// In this test:
//   - Container: height=200px, rows: 50px auto
//   - Auto row: 200 - 50 = 150px
#[test]
fn maximize_tracks_auto_with_fixed_row() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 200px; grid-template-columns: 100px; grid-template-rows: 50px auto;">
          <div style="width: 50px;" expect_top="0" expect_height="50"></div>
          <div style="width: 50px;" expect_top="50" expect_height="150"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple auto rows share free space
// Spec points:
//   - Free space is distributed equally among auto rows
// In this test:
//   - Container: height=300px, rows: auto auto auto
//   - Each auto row: 300 / 3 = 100px
#[test]
fn maximize_tracks_multiple_auto_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 300px; grid-template-columns: 100px; grid-template-rows: auto auto auto;">
          <div style="width: 50px;" expect_top="0" expect_height="100"></div>
          <div style="width: 50px;" expect_top="100" expect_height="100"></div>
          <div style="width: 50px;" expect_top="200" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto rows with gap
// Spec points:
//   - Gap is subtracted before distributing free space
// In this test:
//   - Container: height=220px, gap=20px, rows: auto auto
//   - Free space: 220 - 20 = 200px, each auto: 100px
#[test]
fn maximize_tracks_auto_rows_with_gap() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 220px; row-gap: 20px; grid-template-columns: 100px; grid-template-rows: auto auto;">
          <div style="width: 50px;" expect_top="0" expect_height="100"></div>
          <div style="width: 50px;" expect_top="120" expect_height="100"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Maximize Tracks - No maximization cases
// ═══════════════════════════════════════════════════════════════════════════

// Case: No free space (tracks fill container)
// Spec points:
//   - When there's no free space, no maximization occurs
// In this test:
//   - Container: width=200px, columns: 100px 100px
//   - No free space to distribute
#[test]
fn maximize_tracks_no_free_space() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 100px 100px;">
          <div style="height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto width container (no definite size)
// Spec points:
//   - Maximize tracks only applies when container has definite size
// In this test:
//   - Container: no width specified (auto)
//   - Auto column sizes to content, not maximized
#[test]
fn maximize_tracks_auto_container_width() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: auto;">
          <div style="width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// Case: Auto height container (no definite size)
// Spec points:
//   - Maximize tracks only applies when container has definite size
// In this test:
//   - Container: no height specified (auto)
//   - Auto row sizes to content, not maximized
#[test]
fn maximize_tracks_auto_container_height() {
    assert_xml!(
        r#"
        <div style="display: grid; grid-template-columns: 100px; grid-template-rows: auto;">
          <div style="width: 50px; height: 80px;" expect_height="80"></div>
        </div>
    "#,
        true
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Maximize Tracks - Combined with fr units
// ═══════════════════════════════════════════════════════════════════════════

// Case: fr units with auto
// W3C §11.7 Expand Flexible Tracks:
//   - auto tracks size to content first (base size)
//   - fr units then distribute remaining free space
// In this test:
//   - Container: width=300px, columns: auto 1fr
//   - auto: 100px (content), free space: 300 - 100 = 200px
//   - 1fr: takes all 200px
#[test]
fn maximize_tracks_auto_with_fr() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: auto 1fr;">
          <div style="width: 100px; height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="200"></div>
        </div>
    "#,
        true
    )
}

// Case: Multiple fr units with auto
// W3C §11.7 Expand Flexible Tracks:
//   - auto tracks size to content first
//   - fr units share remaining free space proportionally
// In this test:
//   - Container: width=400px, columns: auto 1fr 2fr
//   - auto: 100px (content), free space: 400 - 100 = 300px
//   - 1fr: 300 / 3 = 100px, 2fr: 300 * 2 / 3 = 200px
#[test]
fn maximize_tracks_auto_with_multiple_fr() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; grid-template-columns: auto 1fr 2fr;">
          <div style="width: 100px; height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="height: 50px;" expect_left="100" expect_width="100"></div>
          <div style="height: 50px;" expect_left="200" expect_width="200"></div>
        </div>
    "#,
        true
    )
}
