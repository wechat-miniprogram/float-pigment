// WPT-style tests for the `flex-shrink` property
// Inspired by WPT CSS Flexbox tests, covering main-axis space compression:
// - `flex-shrink` determines how flex items shrink when there isn't enough space
// - Default value is 1 (can shrink)
// - When items overflow the container, they shrink proportionally based on flex-shrink values
// - Items with flex-shrink=0 do not shrink below their base size

use crate::*;

// flex-shrink: 1 (default, can shrink)
#[test]
fn flex_shrink_one() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="width: 150px; height: 50px;" expect_width="100"></div>
          <div style="width: 150px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex-shrink: 0 (cannot shrink)
#[test]
fn flex_shrink_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 0; width: 200px; height: 50px;" expect_width="200"></div>
          <div style="flex-shrink: 1; width: 100px; height: 50px;" expect_width="0"></div>
        </div>
    "#
    )
}

// flex-shrink: different values (proportional shrinking)
#[test]
fn flex_shrink_proportional() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; width: 200px; height: 50px;" expect_width="80"></div>
          <div style="flex-shrink: 1; width: 300px; height: 50px;" expect_width="120"></div>
        </div>
    "#
    )
}

// flex-shrink: multiple items with different values
#[test]
fn flex_shrink_multiple_items() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; width: 200px; height: 50px;" expect_width="120"></div>
          <div style="flex-shrink: 0; width: 20px; height: 50px;" expect_width="20"></div>
          <div style="flex-shrink: 2; width: 300px; height: 50px;" expect_width="60"></div>
        </div>
    "#
    )
}

// flex-shrink: with flex-basis
#[test]
fn flex_shrink_with_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; flex-basis: 150px; height: 50px;" expect_width="100"></div>
          <div style="flex-shrink: 1; flex-basis: 150px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex-shrink: in column direction
#[test]
fn flex_shrink_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 200px;">
          <div style="height: 150px; width: 50px;" expect_height="100"></div>
          <div style="height: 150px; width: 50px;" expect_height="100"></div>
        </div>
    "#
    )
}

// flex-shrink: with gap
// Container: 200px, gap: 10px, available: 190px
// Both items: 150px, total: 300px, need to shrink: 110px
// Shrunk equally: 150 - 55 = 95px each
#[test]
fn flex_shrink_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; gap: 10px;">
          <div style="width: 150px; height: 50px;" expect_width="95"></div>
          <div style="width: 150px; height: 50px;" expect_width="95"></div>
        </div>
    "#
    )
}

// flex-shrink: 0 prevents shrinking
#[test]
fn flex_shrink_zero_prevents_shrinking() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px;">
          <div style="flex-shrink: 0; width: 80px; height: 50px;" expect_width="80"></div>
          <div style="flex-shrink: 1; width: 50px; height: 50px;" expect_width="20"></div>
        </div>
    "#
    )
}
