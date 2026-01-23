// WPT-based tests for justify-content property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// justify-content: flex-start (default)
// Verify items align to the start of the main axis
#[test]
fn justify_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-start;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// justify-content: start
#[test]
fn justify_content_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: start;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// justify-content: center
#[test]
fn justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
        </div>
    "#
    )
}

// justify-content: flex-end
#[test]
fn justify_content_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-end;">
          <div style="width: 50px; height: 50px;" expect_left="200"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}

// justify-content: end
#[test]
fn justify_content_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: end;">
          <div style="width: 50px; height: 50px;" expect_left="200"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}

// justify-content: space-between
#[test]
fn justify_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; justify-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
        </div>
    "#
    )
}

// justify-content: space-around
#[test]
fn justify_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; justify-content: space-around;">
          <div style="width: 50px; height: 50px;" expect_left="25"></div>
          <div style="width: 50px; height: 50px;" expect_left="125"></div>
        </div>
    "#
    )
}

// justify-content: space-evenly
#[test]
fn justify_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; justify-content: space-evenly;">
          <div style="width: 50px; height: 50px;" expect_left="33"></div>
          <div style="width: 50px; height: 50px;" expect_left="117"></div>
        </div>
    "#
    )
}

// justify-content: left
// left keyword behaves like flex-start in LTR
#[test]
fn justify_content_left() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: left;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// justify-content: right
#[test]
fn justify_content_right() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: right;">
          <div style="width: 50px; height: 50px;" expect_left="200"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}

// justify-content with flex-direction: column
#[test]
fn justify_content_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; justify-content: center;">
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_top="150"></div>
        </div>
    "#
    )
}

// justify-content: space-between with three items
#[test]
fn justify_content_space_between_three_items() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_left="125"></div>
          <div style="width: 50px; height: 50px;" expect_left="250"></div>
        </div>
    "#
    )
}
