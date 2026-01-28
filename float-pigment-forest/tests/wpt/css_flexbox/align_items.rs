// WPT-style tests for the `align-items` property
// Inspired by WPT CSS Flexbox tests, covering cross-axis alignment:
// - `align-items` aligns flex items along the cross axis (perpendicular to main axis)
// - Values: stretch (default), flex-start, start, center, flex-end, end, baseline
// - The property applies to all flex items in the container (unless overridden by align-self)
// - In row direction, cross axis is vertical; in column direction, cross axis is horizontal

use crate::*;

// Case: `align-items: stretch` (default behavior)
// Spec points:
// - Items are stretched to fill the cross axis of the flex container
// - Items without explicit cross-axis size (height in row, width in column) stretch to container size
// - Items with explicit cross-axis size maintain their size
// In this test:
// - Container: width=300, height=100, align-items=stretch (default)
// - First item: width=50, no height → stretches to expect_height=100
// - Second item: width=50, height=50 → maintains expect_height=50
#[test]
fn align_items_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
          <div style="width: 50px;" expect_height="100"></div>
          <div style="width: 50px; height: 50px;" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: `align-items: flex-start`
// Spec points:
// - Items are aligned to the start of the cross axis
// - Items maintain their intrinsic cross-axis size
// In this test:
// - Container: width=300, height=100, align-items=flex-start
// - Both items: expect_top=0 (aligned to top)
// - Items maintain their specified heights (50px, 30px)
#[test]
fn align_items_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 30px;" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `align-items: start`
// Spec points:
// - Behaves like `flex-start` in LTR writing mode
// - Items are aligned to the start of the cross axis
// In this test:
// - Same behavior as flex-start: expect_top=0 for both items
#[test]
fn align_items_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: start;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 30px;" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `align-items: center`
// Spec points:
// - Items are centered along the cross axis
// - Each item is centered independently based on its own cross-axis size
// In this test:
// - Container: width=300, height=100, align-items=center
// - First item: height=50, centered → expect_top=25 ((100-50)/2)
// - Second item: height=30, centered → expect_top=35 ((100-30)/2)
#[test]
fn align_items_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
          <div style="width: 50px; height: 30px;" expect_top="35"></div>
        </div>
    "#
    )
}

// Case: `align-items: flex-end`
// Spec points:
// - Items are aligned to the end of the cross axis
// - Items maintain their intrinsic cross-axis size
// In this test:
// - Container: width=300, height=100, align-items=flex-end
// - First item: height=50, at bottom → expect_top=50 (100-50)
// - Second item: height=30, at bottom → expect_top=70 (100-30)
#[test]
fn align_items_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-end;">
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
          <div style="width: 50px; height: 30px;" expect_top="70"></div>
        </div>
    "#
    )
}

// Case: `align-items: end`
// Spec points:
// - Behaves like `flex-end` in LTR writing mode
// - Items are aligned to the end of the cross axis
// In this test:
// - Same behavior as flex-end: expect_top=50, 70
#[test]
fn align_items_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: end;">
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
          <div style="width: 50px; height: 30px;" expect_top="70"></div>
        </div>
    "#
    )
}

// Case: `align-items: baseline`
// Spec points:
// - Items are aligned such that their baselines align
// - The baseline is determined by the first line of text or the bottom edge for replaced elements
// - Items are shifted so their baselines align at the same cross-axis position
// Engine behavior:
// - Text content has a baseline at approximately 4px from top (font-size=16, baseline offset)
// - Non-text items align their bottom edge to the baseline
// In this test:
// - First text item: expect_top=4 (baseline alignment)
// - Second item (height=20): expect_top=0 (bottom edge aligns with baseline)
// - Third text item: expect_top=4 (baseline alignment)
#[test]
fn align_items_baseline() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: baseline;">
          <div expect_top="4">xxx</div>
          <div style="height: 20px; width: 10px;" expect_top="0"></div>
          <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

// Case: `align-items: center` with `flex-direction: column`
// Spec points:
// - With `flex-direction: column`, the cross axis is horizontal
// - `align-items: center` centers items horizontally
// In this test:
// - Container: height=300, width=200, flex-direction=column, align-items=center
// - First item: width=50, centered → expect_left=75 ((200-50)/2)
// - Second item: width=100, centered → expect_left=50 ((200-100)/2)
#[test]
fn align_items_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; width: 200px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="75"></div>
          <div style="width: 100px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: `align-items: stretch` with different item heights
// Spec points:
// - Items without explicit cross-axis size stretch to fill the container
// - Items with explicit cross-axis size maintain their size
// In this test:
// - Container: width=300, height=100, align-items=stretch
// - First item: no height → expect_height=100 (stretches)
// - Second item: height=50 → expect_height=50 (maintains)
// - Third item: no height → expect_height=100 (stretches)
#[test]
fn align_items_stretch_different_heights() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: stretch;">
          <div style="width: 50px;" expect_height="100"></div>
          <div style="width: 50px; height: 50px;" expect_height="50"></div>
          <div style="width: 50px;" expect_height="100"></div>
        </div>
    "#
    )
}
