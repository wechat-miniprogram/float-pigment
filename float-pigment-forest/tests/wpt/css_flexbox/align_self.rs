// WPT-style tests for the `align-self` property
// Inspired by WPT CSS Flexbox tests, covering individual item cross-axis alignment:
// - `align-self` overrides the container's `align-items` for individual flex items
// - Values: auto (inherits from align-items), stretch, flex-start, start, center, flex-end, end, baseline
// - Allows fine-grained control over how individual items align along the cross axis
// - Note: `align-self: baseline` is tested separately in `align_self_baseline.rs`

use crate::*;

// align-self: auto (inherits from align-items)
#[test]
fn align_self_auto() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
          <div style="width: 50px; height: 30px;" expect_top="35"></div>
        </div>
    "#
    )
}

// align-self: stretch
#[test]
fn align_self_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="width: 50px; align-self: stretch;" expect_height="100"></div>
          <div style="width: 50px; height: 50px;" expect_height="50"></div>
        </div>
    "#
    )
}

// align-self: flex-start
#[test]
fn align_self_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center;">
          <div style="width: 50px; height: 50px; align-self: flex-start;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
        </div>
    "#
    )
}

// align-self: start
#[test]
fn align_self_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center;">
          <div style="width: 50px; height: 50px; align-self: start;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
        </div>
    "#
    )
}

// align-self: center
#[test]
fn align_self_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="width: 50px; height: 50px; align-self: center;" expect_top="25"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// align-self: flex-end
#[test]
fn align_self_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="width: 50px; height: 50px; align-self: flex-end;" expect_top="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// align-self: end
#[test]
fn align_self_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="width: 50px; height: 50px; align-self: end;" expect_top="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
        </div>
    "#
    )
}

// align-self: with flex-direction: column
#[test]
fn align_self_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; width: 200px; align-items: flex-start;">
          <div style="width: 50px; height: 50px; align-self: center;" expect_left="75"></div>
          <div style="width: 50px; height: 50px;" expect_left="0"></div>
        </div>
    "#
    )
}

// align-self: multiple items with different values
#[test]
fn align_self_multiple() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center;">
          <div style="width: 50px; height: 50px; align-self: flex-start;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
          <div style="width: 50px; height: 50px; align-self: flex-end;" expect_top="50"></div>
        </div>
    "#
    )
}
