// WPT-based tests for flex-wrap property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// flex-wrap: nowrap (default behavior)
// In nowrap mode, items are compressed to fit container and stay on one line
// Each item is compressed from 100px to ~67px (200px / 3 ≈ 66.7px, rounded to 67px)
#[test]
fn flex_wrap_nowrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: nowrap; width: 200px;">
          <div style="width: 100px; height: 50px;" expect_width="67" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="67" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="67" expect_top="0"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap - basic wrapping behavior
// First two items fit on first line, third item wraps to second line
#[test]
fn flex_wrap_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap-reverse - reverse wrapping order
#[test]
fn flex_wrap_wrap_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap-reverse; width: 200px; height: 100px;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="0"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap with flex-direction: column
#[test]
fn flex_wrap_wrap_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; flex-direction: column; height: 200px; width: 375px;">
          <div style="width: 50px; height: 100px;" expect_width="50" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 100px;" expect_width="50" expect_left="0" expect_top="100"></div>
          <div style="width: 50px; height: 100px;" expect_width="50" expect_left="188" expect_top="0"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap with gap property
// Gap creates 10px spacing between items and lines
#[test]
fn flex_wrap_wrap_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; gap: 10px;">
          <div style="width: 90px; height: 50px;" expect_width="90" expect_left="0" expect_top="0"></div>
          <div style="width: 90px; height: 50px;" expect_width="90" expect_left="100" expect_top="0"></div>
          <div style="width: 90px; height: 50px;" expect_width="90" expect_left="0" expect_top="60"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap with align-content
#[test]
fn flex_wrap_wrap_with_align_content() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 150px; align-content: center;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="25"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="25"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="75"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap with justify-content
// justify-content: center affects each flex line independently
// Note: In this case, all items fit in one line, so no wrapping occurs
// Since items exactly fill the container, justify-content has no effect
// Verify that all items stay on one line
#[test]
fn flex_wrap_wrap_with_justify_content() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 300px; justify-content: center;">
          <div style="width: 100px; height: 50px;" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap with overflow items
// Items wider than container are clamped to container width
#[test]
fn flex_wrap_wrap_overflow() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 150px;">
          <div style="width: 200px; height: 50px;" expect_width="150" expect_left="0" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap with flex-grow
// flex-grow distributes space within each flex line
// Note: All items fit in one line, so no wrapping occurs
#[test]
fn flex_wrap_wrap_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px;">
          <div style="flex-grow: 1; height: 50px; min-width: 50px;" expect_top="0"></div>
          <div style="flex-grow: 1; height: 50px; min-width: 50px;" expect_top="0"></div>
          <div style="flex-grow: 1; height: 50px; min-width: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// flex-wrap: wrap-reverse with flex-direction: row-reverse
#[test]
fn flex_wrap_wrap_reverse_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap-reverse; flex-direction: row-reverse; width: 200px; height: 100px;">
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="0" expect_top="50"></div>
          <div style="width: 100px; height: 50px;" expect_width="100" expect_left="100" expect_top="0"></div>
        </div>
    "#
    )
}
