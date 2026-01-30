// Tests for `height` property in CSS
// Based on CSS Box Sizing Module Level 3:
// - height: <length> - specifies a fixed height
// - height: <percentage> - relative to containing block's height
// - height: auto - determined by content or constraints

use crate::*;

// Case: Fixed height
// Spec points:
// - Explicit pixel height is used directly
// In this test:
// - Element: height=100px
// - Expected: height=100px
#[test]
fn height_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_height="100"></div>
    "#
    )
}

// Case: Percentage height
// Spec points:
// - Percentage height is relative to parent's height
// In this test:
// - Parent: height=100px
// - Child: height=50% = 50px
#[test]
fn height_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_height="100">
          <div style="height: 50%" expect_height="50"></div>
        </div>
    "#
    )
}

// Case: Auto height
// Spec points:
// - height: auto is determined by content
// - If content has explicit height, auto expands to fit
// - If content has percentage height and parent is auto, percentage resolves to 0
// In this test:
// - Child 1: height=auto, contains 50px high element, expands to 50px
// - Child 2: height=auto, contains 50% high element, but 50% of auto = 0
#[test]
fn height_auto() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_height="100">
          <div style="height: auto" expect_height="50">
            <div style="height: 50px" expect_height="50"></div>
          </div>
          <div style="height: auto" expect_height="0">
            <div style="height: 50%" expect_height="0"></div>
          </div>
        </div>
    "#
    )
}
