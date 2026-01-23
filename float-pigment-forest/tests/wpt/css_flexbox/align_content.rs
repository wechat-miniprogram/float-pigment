// WPT-based tests for align-content property
// Based on Web Platform Tests for CSS Flexbox
// Note: align-content only works when flex-wrap is enabled and there are multiple lines

use crate::*;

// align-content: flex-start
#[test]
fn align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: flex-start;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
        </div>
    "#
    )
}

// align-content: start
#[test]
fn align_content_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: start;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
        </div>
    "#
    )
}

// align-content: center
#[test]
fn align_content_center() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: center;">
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="50"></div>
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
        </div>
    "#
    )
}

// align-content: flex-end
#[test]
fn align_content_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: flex-end;">
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_top="150"></div>
        </div>
    "#
    )
}

// align-content: end
#[test]
fn align_content_end() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: end;">
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
          <div style="width: 50px; height: 50px;" expect_top="150"></div>
        </div>
    "#
    )
}

// align-content: space-between
#[test]
fn align_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: space-between;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="150"></div>
        </div>
    "#
    )
}

// align-content: space-around
#[test]
fn align_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: space-around;">
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
          <div style="width: 50px; height: 50px;" expect_top="25"></div>
          <div style="width: 50px; height: 50px;" expect_top="125"></div>
        </div>
    "#
    )
}

// align-content: space-evenly
#[test]
fn align_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: space-evenly;">
          <div style="width: 50px; height: 50px;" expect_top="33"></div>
          <div style="width: 50px; height: 50px;" expect_top="33"></div>
          <div style="width: 50px; height: 50px;" expect_top="117"></div>
        </div>
    "#
    )
}

// align-content: stretch (default)
#[test]
fn align_content_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 100px; height: 200px; align-content: stretch;">
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_top="100"></div>
        </div>
    "#
    )
}

// align-content with flex-direction: column
#[test]
fn align_content_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; flex-direction: column; height: 100px; width: 200px; align-content: center;">
          <div style="width: 50px; height: 30px;" expect_left="75"></div>
          <div style="width: 50px; height: 30px;" expect_left="75"></div>
          <div style="width: 50px; height: 30px;" expect_left="75"></div>
        </div>
    "#
    )
}
