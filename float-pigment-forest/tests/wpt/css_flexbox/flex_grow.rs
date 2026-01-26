// WPT-based tests for flex-grow property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// flex-grow: 0 (default, no growth)
#[test]
fn flex_grow_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex-grow: 1 (equal distribution)
#[test]
fn flex_grow_one() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-grow: different values (proportional distribution)
#[test]
fn flex_grow_proportional() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 50px;" expect_width="100"></div>
          <div style="flex-grow: 2; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// flex-grow: 0 with fixed width
#[test]
fn flex_grow_zero_with_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="250"></div>
        </div>
    "#
    )
}

// flex-grow: multiple items with different values
// Container: 300px, fixed: 50px, free: 250px
// Grow ratio: 1:2, total grow: 3
// Item 2: 50 + 250/3 = 83.33px (rounded to 83)
// Item 3: 0 + 250*2/3 = 166.67px (rounded to 167)
#[test]
fn flex_grow_multiple_items() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="83"></div>
          <div style="flex-grow: 2; height: 50px;" expect_width="167"></div>
        </div>
    "#
    )
}

// flex-grow: with flex-basis
// Container: 300px, both items have basis: 50px, total: 100px
// Free space: 200px, divided equally: 100px each
// Final width: 50 + 100 = 150px each
#[test]
fn flex_grow_with_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; flex-basis: 50px; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; flex-basis: 50px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-grow: in column direction
#[test]
fn flex_grow_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px;">
          <div style="flex-grow: 1; width: 50px;" expect_height="150"></div>
          <div style="flex-grow: 1; width: 50px;" expect_height="150"></div>
        </div>
    "#
    )
}

// flex-grow: with gap
// Container: 300px, gap: 10px, available: 290px
// Divided equally: 145px each
#[test]
fn flex_grow_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; gap: 10px;">
          <div style="flex-grow: 1; height: 50px;" expect_width="145"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="145"></div>
        </div>
    "#
    )
}
