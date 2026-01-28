// WPT-style tests for `align-self: baseline`
// Inspired by WPT CSS Flexbox tests, covering baseline alignment:
// - `align-self: baseline` aligns items such that their baselines align
// - The baseline is determined by the first line of text or the bottom edge for replaced elements
// - Items are shifted so their baselines align at the same cross-axis position
// - This is useful for aligning text content across flex items

use crate::*;

// align-self: baseline - basic baseline alignment
#[test]
fn align_self_baseline() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: start">
          <div style="align-self: baseline" expect_top="0">xxx</div>
          <div style="height: 10px; width: 10px; align-self: baseline" expect_top="6"></div>
          <div style="align-self: baseline" expect_top="0">xxx</div>
        </div>
    "#
    )
}

// align-self: baseline with different font sizes
#[test]
fn align_self_baseline_different_sizes() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: start">
          <div style="align-self: baseline; font-size: 20px;" expect_top="4">xxx</div>
          <div style="height: 20px; width: 10px; align-self: baseline" expect_top="0"></div>
          <div style="align-self: baseline; font-size: 16px;" expect_top="4">xxx</div>
        </div>
    "#
    )
}

// align-self: baseline with flex container
#[test]
fn align_self_baseline_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: start">
          <div style="align-self: baseline" expect_top="4">xxx</div>
          <div style="display: flex; height: 20px; width: 10px; align-self: baseline" expect_top="0"></div>
          <div style="align-self: baseline" expect_top="4">xxx</div>
        </div>
    "#
    )
}

// align-self: baseline with inline-block
#[test]
fn align_self_baseline_inline_block() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: start">
          <div style="align-self: baseline" expect_top="4">xxx</div>
          <div style="display: inline-block; height: 20px; width: 10px; align-self: baseline" expect_top="0"></div>
          <div style="align-self: baseline" expect_top="4">xxx</div>
        </div>
    "#
    )
}

// align-self: baseline with margin-top
#[test]
fn align_self_baseline_margin_top() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: start">
          <div style="align-self: baseline; margin-top: 10px;" expect_top="10">xxx</div>
          <div style="margin-top: 10px; height: 10px; width: 10px; align-self: baseline" expect_top="10">xxx</div>
          <div style="align-self: baseline" expect_top="10">xxx</div>
        </div>
    "#
    )
}

// align-self: baseline overriding align-items
#[test]
fn align_self_baseline_override_align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center">
          <div style="height: 50px; width: 50px;" expect_top="25"></div>
          <div style="align-self: baseline; height: 30px; width: 50px;" expect_top="0">xxx</div>
          <div style="height: 50px; width: 50px;" expect_top="25"></div>
        </div>
    "#
    )
}

// align-self: baseline in column direction
// In column direction, baseline alignment works on the horizontal axis
// All items align to the same left baseline
#[test]
fn align_self_baseline_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; align-items: start">
          <div style="align-self: baseline" expect_left="0">xxx</div>
          <div style="width: 10px; height: 10px; align-self: baseline" expect_left="0"></div>
          <div style="align-self: baseline" expect_left="0">xxx</div>
        </div>
    "#
    )
}
