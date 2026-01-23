// WPT-based tests for align-items property
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// align-items: stretch (default)
#[test]
fn align_items_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
          <div style="width: 50px;" expect_height="100"></div>
          <div style="width: 50px; height: 50px;" expect_height="50"></div>
        </div>
    "#
    )
}

// align-items: flex-start
#[test]
fn align_items_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-start;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 30px;" expect_top="0"></div>
        </div>
    "#
    )
}

// align-items: start
#[test]
fn align_items_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: start;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 30px;" expect_top="0"></div>
        </div>
    "#
    )
}

// align-items: center
#[test]
fn align_items_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
          <div style="width: 50px; height: 30px;" expect_top="35"></div>
        </div>
    "#
    )
}

// align-items: flex-end
#[test]
fn align_items_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: flex-end;">
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
          <div style="width: 50px; height: 30px;" expect_top="70"></div>
        </div>
    "#
    )
}

// align-items: end
#[test]
fn align_items_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: end;">
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
          <div style="width: 50px; height: 30px;" expect_top="70"></div>
        </div>
    "#
    )
}

// align-items: baseline
#[test]
fn align_items_baseline() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; align-items: baseline;">
          <div expect_top="4">xxx</div>
          <div style="height: 20px; width: 10px;" expect_top="0"></div>
          <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

// align-items with flex-direction: column
#[test]
fn align_items_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px; width: 200px; align-items: center;">
          <div style="width: 50px; height: 50px;" expect_left="75"></div>
          <div style="width: 100px; height: 50px;" expect_left="50"></div>
        </div>
    "#
    )
}

// align-items: stretch with different heights
#[test]
fn align_items_stretch_different_heights() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; align-items: stretch;">
          <div style="width: 50px;" expect_height="100"></div>
          <div style="width: 50px; height: 50px;" expect_height="50"></div>
          <div style="width: 50px;" expect_height="100"></div>
        </div>
    "#
    )
}
