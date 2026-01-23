// WPT-based tests for gap property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// gap: fixed value in row direction
#[test]
fn gap_row_fixed() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="60"></div>
          <div style="width: 50px; height: 50px;" expect_left="120"></div>
        </div>
    "#
    )
}

// gap: fixed value with wrap
// Verify gap works correctly when items wrap to new lines
#[test]
fn gap_row_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; gap: 10px;">
          <div style="width: 40px; height: 40px;" expect_left="0"></div>
          <div style="width: 40px; height: 40px;" expect_left="50"></div>
          <div style="width: 40px; height: 40px;" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// gap: percentage value
#[test]
fn gap_row_percentage() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10%;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="70"></div>
        </div>
    "#
    )
}

// gap in column direction
#[test]
fn gap_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 200px; gap: 10px;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="60"></div>
          <div style="width: 50px; height: 50px;" expect_top="120"></div>
        </div>
    "#
    )
}

// column-gap and row-gap separately
// Verify column-gap and row-gap can be set independently
#[test]
fn gap_column_row_separate() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 200px; column-gap: 20px; row-gap: 10px;">
          <div style="width: 80px; height: 80px;" expect_left="0"></div>
          <div style="width: 80px; height: 80px;" expect_left="100"></div>
          <div style="width: 80px; height: 80px;" expect_left="0" expect_top="105"></div>
        </div>
    "#
    )
}

// gap with flex-grow
#[test]
fn gap_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px;">
          <div style="flex-grow: 1; height: 50px; min-width: 20px;" expect_left="0"></div>
          <div style="flex-grow: 1; height: 50px; min-width: 20px;" expect_left="105"></div>
        </div>
    "#
    )
}

// gap with align-content
#[test]
fn gap_with_align_content() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; gap: 10px; align-content: center;">
          <div style="width: 40px; height: 40px;" expect_left="0" expect_top="60"></div>
          <div style="width: 40px; height: 40px;" expect_left="50" expect_top="60"></div>
          <div style="width: 40px; height: 40px;" expect_left="0" expect_top="110"></div>
        </div>
    "#
    )
}
