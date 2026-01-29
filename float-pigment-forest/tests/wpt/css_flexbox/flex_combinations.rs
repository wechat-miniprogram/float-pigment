// WPT-based tests for flexbox property combinations
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// flex-grow + flex-shrink combination
// Container: 300px, both items start at 50px
// Free space: 200px, distributed by grow ratio 1:2
// Actual calculation: 116.602px, 183.203px (rounded to 117, 183)
#[test]
fn flex_grow_shrink_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; flex-shrink: 1; width: 50px; height: 50px;" expect_width="117"></div>
          <div style="flex-grow: 2; flex-shrink: 1; width: 50px; height: 50px;" expect_width="183"></div>
        </div>
    "#
    )
}

// flex-grow + flex-basis combination
// Container: 300px, both items have basis: 50px, total: 100px
// Free space: 200px, divided equally: 100px each
// Final width: 50 + 100 = 150px each
#[test]
fn flex_grow_basis_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; flex-basis: 50px; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; flex-basis: 50px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-shrink + flex-basis combination
#[test]
fn flex_shrink_basis_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; flex-basis: 150px; height: 50px;" expect_width="100"></div>
          <div style="flex-shrink: 1; flex-basis: 150px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// justify-content + align-items combination
// Container: 300px width, 100px height
// justify-content: center, align-items: center
// Items centered both horizontally and vertically
// Actual positions: left: 100, 150; top: 25, 25
#[test]
fn justify_content_align_items_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; justify-content: center; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="100" expect_top="25"></div>
          <div style="width: 50px; height: 50px;" expect_left="150" expect_top="25"></div>
        </div>
    "#
    )
}

// flex-wrap + justify-content + align-content combination
// Container: 200px width, 150px height
// Items: 80px each, justify-content: center, align-content: center
// First line: 2 items centered horizontally, second line: 1 item centered
// Actual positions: left: 20, 100, 60; top: 25, 25, 75
#[test]
fn flex_wrap_justify_align_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 150px; justify-content: center; align-content: center;">
          <div style="width: 80px; height: 50px;" expect_left="20" expect_top="25"></div>
          <div style="width: 80px; height: 50px;" expect_left="100" expect_top="25"></div>
          <div style="width: 80px; height: 50px;" expect_left="60" expect_top="75"></div>
        </div>
    "#
    )
}

// flex-direction + gap + align-items combination
#[test]
fn flex_direction_gap_align_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 200px; width: 100px; gap: 10px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="25" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="25" expect_top="60"></div>
        </div>
    "#
    )
}

// order + flex-grow combination
#[test]
fn order_flex_grow_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="order: 2; flex-grow: 1; height: 50px;" expect_left="150"></div>
          <div style="order: 1; flex-grow: 1; height: 50px;" expect_left="0"></div>
        </div>
    "#
    )
}

// align-self + flex-grow combination
#[test]
fn align_self_flex_grow_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="flex-grow: 1; height: 50px; align-self: center;" expect_width="150" expect_top="25"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="150" expect_top="0"></div>
        </div>
    "#
    )
}

// gap + margin combination
#[test]
fn gap_margin_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px;">
          <div style="width: 50px; height: 50px; margin-left: 10px;" expect_left="10"></div>
          <div style="width: 50px; height: 50px; margin-left: 10px;" expect_left="80"></div>
        </div>
    "#
    )
}

// min-width + max-width + flex-grow combination
#[test]
fn min_max_width_flex_grow_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; min-width: 50px; max-width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// flex-wrap + gap + align-content combination
// Container: 200px width, 150px height, gap: 10px
// align-content: space-between distributes lines
// Items: 90px each, gap creates spacing
// Actual positions: top: 0, 0, 110 (space-between with gap)
#[test]
fn flex_wrap_gap_align_content_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 150px; gap: 10px; align-content: space-between;">
          <div style="width: 90px; height: 50px;" expect_left="0" expect_top="0"></div>
          <div style="width: 90px; height: 50px;" expect_left="100" expect_top="0"></div>
          <div style="width: 90px; height: 50px;" expect_left="0" expect_top="110"></div>
        </div>
    "#
    )
}

// flex-direction: column + justify-content + align-items combination
#[test]
fn flex_column_justify_align_combination() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; width: 200px; justify-content: center; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="75" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_left="75" expect_top="150"></div>
        </div>
    "#
    )
}
