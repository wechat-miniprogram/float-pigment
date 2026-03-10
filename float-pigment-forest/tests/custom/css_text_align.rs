// Tests for `text-align` property in CSS
// Based on CSS Text Module Level 3:
// - text-align controls inline content alignment within block
// - Values: start, end, left, right, center
// - Affects inline and inline-block children

use crate::*;

// Case: text-align: center
// Spec points:
// - Inline content is centered within the containing block
// In this test:
// - Container: width=300px, text-align=center
// - Two inline-blocks of 100px each
// - Remaining space = 300 - 200 = 100px, offset = 50px
// - First at left=50, second at left=150
#[test]
fn text_align_1() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: center">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="50"></div>
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="150"></div>
        </div>
    "#
    )
}

// Case: text-align: end
// Spec points:
// - Inline content aligned to end edge (right in LTR)
// In this test:
// - Container: width=300px, text-align=end
// - Two inline-blocks of 100px each
// - Aligned to right: first at left=100, second at left=200
#[test]
fn text_align_2() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: end">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="100"></div>
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="200"></div>
        </div>
    "#
    )
}

// Case: text-align: start
// Spec points:
// - Inline content aligned to start edge (left in LTR)
// - This is the default behavior
// In this test:
// - Container: width=300px, text-align=start
// - Two inline-blocks positioned at left=0 and left=100
#[test]
fn text_align_3() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: start">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="0"></div>
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="100"></div>
        </div>
    "#
    )
}

// Case: text-align: center with nested container
// Spec points:
// - text-align inherits to descendants
// - Each block applies alignment to its own inline content
// In this test:
// - Outer: 300px, center - inline-block at center (100px)
// - Inner: 100px, center - inline-block at center (25px)
#[test]
fn text_align_4() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: center">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="100"></div>
          <div style="width: 100px; text-align: center">
            <div style="display: inline-block; width: 50px; height: 30px;" expect_left="25"></div>
          </div>
        </div>
    "#
    )
}

// Case: text-align: center with overflow
// Spec points:
// - When content is wider than container, overflow occurs
// - Content starts at left edge (left=0), not negative
// In this test:
// - Container: width=20px, text-align=center
// - Inline-block: width=100px (overflows)
// - Expected: left=0 (no negative offset)
#[test]
fn text_align_5() {
    assert_xml!(
        r#"
        <div style="width: 20px; text-align: center">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="0"></div>
        </div>
    "#
    )
}
