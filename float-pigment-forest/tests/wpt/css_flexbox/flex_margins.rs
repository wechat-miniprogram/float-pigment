// WPT-style tests for flex item margins
// Inspired by WPT CSS Flexbox tests, covering margin behavior in flex containers:
// - Margins on flex items participate in space distribution
// - `margin: auto` absorbs all available space in that direction, useful for centering
// - Margins do not collapse in flex containers
// - Percentage margins are resolved relative to the containing block's width

use crate::*;

// margin: auto centers item
#[test]
fn flex_item_margin_auto_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="width: 50px; height: 50px; margin-left: auto; margin-right: auto;" expect_left="75"></div>
        </div>
    "#
    )
}

// margin-left: auto pushes item to the right
#[test]
fn flex_item_margin_left_auto() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px; margin-left: auto;" expect_left="150"></div>
        </div>
    "#
    )
}

// margin-right: auto doesn't affect positioning in row direction
// margin-right: auto in row direction doesn't push items apart
#[test]
fn flex_item_margin_right_auto() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="width: 50px; height: 50px; margin-right: auto;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
        </div>
    "#
    )
}

// margin: auto with multiple items
// Each item with margin: auto centers itself in its allocated space
#[test]
fn flex_item_margin_auto_multiple() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="width: 30px; height: 50px; margin-left: auto; margin-right: auto;" expect_left="35"></div>
          <div style="width: 30px; height: 50px; margin-left: auto; margin-right: auto;" expect_left="135"></div>
        </div>
    "#
    )
}

// margin: auto in column direction
#[test]
fn flex_item_margin_auto_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 200px; width: 100px;">
          <div style="width: 50px; height: 50px; margin-top: auto; margin-bottom: auto;" expect_top="75"></div>
        </div>
    "#
    )
}

// margin with fixed values
#[test]
fn flex_item_margin_fixed() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="width: 50px; height: 50px; margin-left: 10px;" expect_left="10"></div>
          <div style="width: 50px; height: 50px; margin-left: 20px;" expect_left="80"></div>
        </div>
    "#
    )
}

// margin with gap
#[test]
fn flex_item_margin_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px; gap: 10px;">
          <div style="width: 50px; height: 50px; margin-left: 10px;" expect_left="10"></div>
          <div style="width: 50px; height: 50px; margin-left: 10px;" expect_left="80"></div>
        </div>
    "#
    )
}
