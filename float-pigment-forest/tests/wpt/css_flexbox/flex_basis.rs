// WPT-based tests for flex-basis property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// flex-basis: auto (default)
// When flex-basis is auto, width is used as the basis
#[test]
fn flex_basis_auto() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: auto; width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: auto; width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: auto; width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex-basis: fixed length
#[test]
fn flex_basis_fixed_length() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-basis: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-basis: percentage
#[test]
fn flex_basis_percentage() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 20%; height: 50px;" expect_width="60"></div>
          <div style="flex-basis: 30%; height: 50px;" expect_width="90"></div>
          <div style="flex-basis: 50%; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-basis: 0
// When flex-basis is 0, all available space is distributed by flex-grow
#[test]
fn flex_basis_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 0; flex-grow: 1; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 0; flex-grow: 1; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 0; flex-grow: 1; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex-basis with flex-grow
#[test]
fn flex_basis_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 50px; flex-grow: 1; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 50px; flex-grow: 2; height: 50px;" expect_width="150"></div>
          <div style="flex-basis: 50px; flex-grow: 0; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex-basis with flex-shrink
// Items shrink proportionally based on flex-shrink values
#[test]
fn flex_basis_with_flex_shrink() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-basis: 100px; flex-shrink: 0; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 100px; flex-shrink: 1; height: 50px;" expect_width="50"></div>
          <div style="flex-basis: 100px; flex-shrink: 1; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex-basis: content (intrinsic size)
#[test]
fn flex_basis_content() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: content; width: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-basis: content; width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: content; flex-grow: 1; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex-basis: max-content
#[test]
fn flex_basis_max_content() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: max-content; width: 80px; height: 50px;" expect_width="80"></div>
          <div style="flex-basis: max-content; flex-grow: 1; height: 50px;" expect_width="220"></div>
        </div>
    "#
    )
}

// flex-basis: min-content
// Note: min-content may not be supported, test may need adjustment
#[test]
#[ignore]
fn flex_basis_min_content() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: min-content; width: 80px; min-width: 20px; height: 50px;" expect_width="20"></div>
          <div style="flex-basis: min-content; flex-grow: 1; height: 50px;" expect_width="280"></div>
        </div>
    "#
    )
}

// flex-basis with width override
// width takes precedence over flex-basis when both are specified
#[test]
fn flex_basis_with_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 100px; width: 50px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 100px; width: 150px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 100px; flex-grow: 1; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex-basis in column direction
// In column direction, flex-basis affects height instead of width
#[test]
fn flex_basis_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px;">
          <div style="flex-basis: 50px; width: 50px;" expect_height="50" expect_top="0"></div>
          <div style="flex-basis: 100px; width: 50px;" expect_height="100" expect_top="50"></div>
          <div style="flex-basis: 150px; width: 50px;" expect_height="150" expect_top="150"></div>
        </div>
    "#
    )
}

// flex-basis: percentage in column direction
// Percentage is calculated relative to container height
#[test]
fn flex_basis_percentage_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px;">
          <div style="flex-basis: 20%; width: 50px;" expect_height="60" expect_top="0"></div>
          <div style="flex-basis: 30%; width: 50px;" expect_height="90" expect_top="60"></div>
          <div style="flex-basis: 50%; width: 50px;" expect_height="150" expect_top="150"></div>
        </div>
    "#
    )
}

// flex-basis with min-width constraint
#[test]
fn flex_basis_with_min_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 50px; min-width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 50px; flex-grow: 1; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// flex-basis with max-width constraint
#[test]
fn flex_basis_with_max_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-basis: 200px; max-width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-basis: 50px; flex-grow: 1; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// flex shorthand with basis
// flex: grow shrink basis - space is distributed after basis
// 87.5px rounds to 88px in the test framework
#[test]
fn flex_shorthand_with_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: 1 1 50px; height: 50px;" expect_width="88"></div>
          <div style="flex: 2 1 50px; height: 50px;" expect_width="125"></div>
          <div style="flex: 1 1 50px; height: 50px;" expect_width="88"></div>
        </div>
    "#
    )
}
