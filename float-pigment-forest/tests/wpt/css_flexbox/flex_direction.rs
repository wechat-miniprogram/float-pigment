// WPT-based tests for flex-direction property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// flex-direction: row (default)
// Verify items are laid out horizontally
#[test]
fn flex_direction_row() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// flex-direction: row-reverse
// Verify items are laid out in reverse horizontal order
#[test]
fn flex_direction_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row-reverse;">
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
        </div>
    "#
    )
}

// flex-direction: column
// Verify items are laid out vertically
#[test]
fn flex_direction_column() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; flex-direction: column;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// flex-direction: column-reverse
#[test]
fn flex_direction_column_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; flex-direction: column-reverse;">
          <div style="width: 50px; height: 50px;" expect_left="0" expect_top="150"></div>
          <div style="width: 50px; height: 50px;" expect_left="0" expect_top="100"></div>
        </div>
    "#
    )
}

// flex-direction: row with justify-content
#[test]
fn flex_direction_row_with_justify_content() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
        </div>
    "#
    )
}

// flex-direction: column with align-items
// Verify align-items works in column direction (affects horizontal alignment)
#[test]
fn flex_direction_column_with_align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; width: 200px; flex-direction: column; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="75"></div>
          <div style="width: 50px; height: 50px;" expect_left="75" expect_top="50"></div>
        </div>
    "#
    )
}
