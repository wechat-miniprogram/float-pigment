// WPT-style tests for the `order` property
// Inspired by WPT CSS Flexbox tests, covering visual order control:
// - `order` controls the order in which flex items appear visually
// - Default value is 0
// - Items are sorted by their order value (ascending), then by document order
// - Negative values are allowed and come before 0
// - Order affects visual layout but not the DOM order

use crate::*;

// order: default (0)
// Without explicit order, items appear in document order
#[test]
fn order_default() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 50px; height: 50px;"></div>
          <div style="width: 50px; height: 50px;"></div>
          <div style="width: 50px; height: 50px;"></div>
        </div>
    "#
    )
}

// order: reorder items
#[test]
fn order_reorder() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 50px; height: 50px; order: 3;" expect_left="100"></div>
          <div style="width: 50px; height: 50px; order: 1;" expect_left="0"></div>
          <div style="width: 50px; height: 50px; order: 2;" expect_left="50"></div>
        </div>
    "#
    )
}

// order: negative values
// Items with order: -1 come first, then order: 0, then order: 1
#[test]
fn order_negative() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 50px; height: 50px; order: 1;" expect_left="100"></div>
          <div style="width: 50px; height: 50px; order: -1;" expect_left="0"></div>
          <div style="width: 50px; height: 50px; order: 0;" expect_left="50"></div>
        </div>
    "#
    )
}

// order with flex-direction: row-reverse
// In row-reverse, items are ordered by order value, then reversed
#[test]
fn order_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; flex-direction: row-reverse;">
          <div style="width: 50px; height: 50px; order: 1;" expect_left="50"></div>
          <div style="width: 50px; height: 50px; order: -1;" expect_left="150"></div>
          <div style="width: 50px; height: 50px; order: 0;" expect_left="100"></div>
        </div>
    "#
    )
}

// order with flex-direction: column
#[test]
fn order_column() {
    assert_xml!(
        r#"
        <div style="display: flex; height: 200px; flex-direction: column;">
          <div style="width: 50px; height: 50px; order: 2;" expect_top="50"></div>
          <div style="width: 50px; height: 50px; order: 1;" expect_top="0"></div>
          <div style="width: 50px; height: 50px; order: 3;" expect_top="100"></div>
        </div>
    "#
    )
}

// order with flex-wrap
// Items are ordered by order value, then wrap
#[test]
fn order_with_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px;">
          <div style="width: 50px; height: 50px; order: 2;" expect_left="50" expect_top="0"></div>
          <div style="width: 50px; height: 50px; order: 1;" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 50px; order: 3;" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}
