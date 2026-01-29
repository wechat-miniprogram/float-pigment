// WPT-based tests for flexbox with padding and border
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// flex container with padding
#[test]
fn flex_container_with_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; padding: 10px;">
          <div style="width: 50px; height: 50px;" expect_left="10"></div>
          <div style="width: 50px; height: 50px;" expect_left="60"></div>
        </div>
    "#
    )
}

// flex item with padding
#[test]
fn flex_item_with_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 50px; height: 50px; padding: 10px;" expect_width="70"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex container with border
#[test]
fn flex_container_with_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; border: 5px solid black;">
          <div style="width: 50px; height: 50px;" expect_left="5"></div>
          <div style="width: 50px; height: 50px;" expect_left="55"></div>
        </div>
    "#
    )
}

// flex item with border
#[test]
fn flex_item_with_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 50px; height: 50px; border: 5px solid black;" expect_width="60"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex container with padding and border
#[test]
fn flex_container_padding_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; padding: 10px; border: 5px solid black;">
          <div style="width: 50px; height: 50px;" expect_left="15"></div>
          <div style="width: 50px; height: 50px;" expect_left="65"></div>
        </div>
    "#
    )
}

// flex item with padding and border
#[test]
fn flex_item_padding_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 50px; height: 50px; padding: 10px; border: 5px solid black;" expect_width="80"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex-grow with padding
// Container: 300px, both items grow equally
// Padding: 10px on each side = 20px total per item
// Available space: 300px - 40px (padding) = 260px
// Each item gets: 130px + 20px padding = 150px total width
#[test]
fn flex_grow_with_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; padding: 10px; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; padding: 10px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-shrink with border
// Container: 200px, item1: 300px (needs to shrink), border: 5px
// Border adds 10px total (5px each side)
// Shrink calculation with border constraint
// Actual output: 172.969px, 27.1875px (rounded to 173, 27)
#[test]
fn flex_shrink_with_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; width: 300px; border: 5px solid black; height: 50px;" expect_width="173"></div>
          <div style="width: 50px; height: 50px;" expect_width="27"></div>
        </div>
    "#
    )
}

// gap with padding
#[test]
fn gap_with_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px; padding: 10px;">
          <div style="width: 50px; height: 50px;" expect_left="10"></div>
          <div style="width: 50px; height: 50px;" expect_left="70"></div>
        </div>
    "#
    )
}

// gap with border
#[test]
fn gap_with_border() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px; border: 5px solid black;">
          <div style="width: 50px; height: 50px;" expect_left="5"></div>
          <div style="width: 50px; height: 50px;" expect_left="65"></div>
        </div>
    "#
    )
}
