// WPT-style tests for the `flex-flow` shorthand property
// Inspired by WPT CSS Flexbox tests, covering combined flex-direction and flex-wrap:
// - `flex-flow` is a shorthand for `flex-direction` and `flex-wrap`
// - Syntax: `flex-flow: <flex-direction> <flex-wrap>`
// - Both values are optional; defaults apply if omitted
// - Examples: `row wrap`, `column nowrap`, `row-reverse wrap-reverse`

use crate::*;

// flex-flow: row wrap
#[test]
fn flex_flow_row_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-flow: row wrap; width: 200px;">
          <div style="width: 100px; height: 50px;" expect_left="0" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_left="100" expect_top="0"></div>
          <div style="width: 100px; height: 50px;" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

// flex-flow: column wrap
#[test]
fn flex_flow_column_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-flow: column wrap; height: 200px; width: 375px;">
          <div style="width: 50px; height: 100px;" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 100px;" expect_left="0" expect_top="100"></div>
          <div style="width: 50px; height: 100px;" expect_left="188" expect_top="0"></div>
        </div>
    "#
    )
}

// flex-flow: row-reverse nowrap
#[test]
fn flex_flow_row_reverse_nowrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-flow: row-reverse nowrap; width: 200px;">
          <div style="width: 50px; height: 50px;" expect_left="150"></div>
          <div style="width: 50px; height: 50px;" expect_left="100"></div>
        </div>
    "#
    )
}

// flex-flow: column-reverse wrap
#[test]
fn flex_flow_column_reverse_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-flow: column-reverse wrap; height: 200px; width: 375px;">
          <div style="width: 50px; height: 100px;" expect_left="0" expect_top="100"></div>
          <div style="width: 50px; height: 100px;" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 100px;" expect_left="188" expect_top="100"></div>
        </div>
    "#
    )
}
