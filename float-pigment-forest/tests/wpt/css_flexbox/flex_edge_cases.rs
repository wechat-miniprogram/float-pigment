// WPT-based tests for flexbox edge cases and special scenarios
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// Empty flex container
#[test]
fn flex_container_empty() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;"></div>
    "#
    )
}

// Single flex item
#[test]
fn flex_container_single_item() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// Nested flex containers
#[test]
fn flex_nested() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
          <div style="display: flex; flex-grow: 1; height: 50px;">
            <div style="width: 50px; height: 50px;" expect_width="50"></div>
            <div style="width: 50px; height: 50px;" expect_width="50"></div>
          </div>
        </div>
    "#
    )
}

// Zero width container
#[test]
fn flex_container_zero_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 0px; height: 100px;">
          <div style="width: 50px; height: 50px;" expect_width="0"></div>
        </div>
    "#
    )
}

// Zero height container
// Even with zero height container, flex items maintain their intrinsic height
// unless constrained by align-items: stretch
#[test]
fn flex_container_zero_height() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 0px;">
          <div style="width: 50px; height: 50px;" expect_height="50"></div>
        </div>
    "#
    )
}

// Very large flex-grow value
#[test]
fn flex_grow_large_value() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 100; height: 50px;" expect_width="300"></div>
        </div>
    "#
    )
}

// Zero flex-grow
#[test]
fn flex_grow_zero_value() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; height: 50px;" expect_width="0"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="300"></div>
        </div>
    "#
    )
}

// Zero flex-shrink
#[test]
fn flex_shrink_zero_value() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 0; width: 300px; height: 50px;" expect_width="300"></div>
          <div style="width: 50px; height: 50px;" expect_width="0"></div>
        </div>
    "#
    )
}

// All items have flex-grow: 0
#[test]
fn flex_grow_all_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-grow: 0; width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// All items have flex-shrink: 0
#[test]
fn flex_shrink_all_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 0; width: 200px; height: 50px;" expect_width="200"></div>
          <div style="flex-shrink: 0; width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}
