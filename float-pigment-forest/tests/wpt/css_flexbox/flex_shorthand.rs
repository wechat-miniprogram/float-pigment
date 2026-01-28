// WPT-style tests for the `flex` shorthand property
// Inspired by WPT CSS Flexbox tests, covering combined flex-grow, flex-shrink, and flex-basis:
// - `flex` is a shorthand for `flex-grow`, `flex-shrink`, and `flex-basis`
// - Syntax: `flex: <grow> <shrink> <basis>` or keywords: `initial`, `auto`, `none`, or a positive number
// - `flex: 1` is equivalent to `flex: 1 1 0%`
// - `flex: auto` is equivalent to `flex: 1 1 auto`
// - `flex: none` is equivalent to `flex: 0 0 auto`
// - `flex: initial` is equivalent to `flex: 0 1 auto`

use crate::*;

// flex: initial (0 1 auto)
#[test]
fn flex_initial() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: initial; width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex: initial; width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex: auto (1 1 auto)
#[test]
fn flex_auto() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: auto; width: 50px; height: 50px;" expect_width="150"></div>
          <div style="flex: auto; width: 50px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex: none (0 0 auto)
#[test]
fn flex_none() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: none; width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex: none; width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// flex: positive number (grow shrink basis)
#[test]
fn flex_positive_number() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: 1; height: 50px;" expect_width="150"></div>
          <div style="flex: 1; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex: with explicit basis
#[test]
fn flex_with_basis() {
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

// flex: with zero basis
#[test]
fn flex_zero_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: 1 1 0px; height: 50px;" expect_width="100"></div>
          <div style="flex: 2 1 0px; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// flex: percentage basis
// Container: 300px, basis: 20% = 60px each
// Free space: 300 - 120 = 180px, divided equally: 90px each
// Final width: 60 + 90 = 150px each
#[test]
fn flex_percentage_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: 1 1 20%; height: 50px;" expect_width="150"></div>
          <div style="flex: 1 1 20%; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// flex: with shrink 0
#[test]
fn flex_shrink_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex: 1 0 300px; height: 50px;" expect_width="300"></div>
          <div style="width: 50px; height: 50px;" expect_width="0"></div>
        </div>
    "#
    )
}

// flex: with grow 0
#[test]
fn flex_grow_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex: 0 1 50px; height: 50px;" expect_width="50"></div>
          <div style="flex: 0 1 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// flex: in column direction
#[test]
fn flex_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px;">
          <div style="flex: 1; width: 50px;" expect_height="150"></div>
          <div style="flex: 1; width: 50px;" expect_height="150"></div>
        </div>
    "#
    )
}
