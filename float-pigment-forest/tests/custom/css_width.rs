// Tests for `width` property in CSS
// Based on CSS Box Sizing Module Level 3:
// - width: <length> - specifies a fixed width
// - width: <percentage> - relative to containing block's width
// - width: auto - determined by content and constraints

use crate::*;

// Case: Fixed width
// Spec points:
// - Explicit pixel width is used directly
// In this test:
// - Element: width=100px
// - Expected: width=100px
#[test]
fn width_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_width="100"> </div>
    "#
    )
}

// Case: Percentage width
// Spec points:
// - Percentage width is relative to parent's width
// In this test:
// - Parent: width=100px
// - Child: width=50% = 50px
#[test]
fn width_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_width="100">
          <div style="width: 50%; height: 100px;" expect_width="50"></div>
        </div>
    "#
    )
}

// Case: Auto width
// Spec points:
// - width: auto typically stretches to fill container (for block elements)
// - Children can specify larger widths and overflow
// - Percentage children respect parent's computed width
// In this test:
// - Both children have width=auto
// - First child: contains 300px child, still 100px (block doesn't shrink)
// - Second child: contains 50% child = 50px
#[test]
fn width_auto() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_width="100">
          <div style="width: auto; height: 100px;" expect_width="100">
            <div style="width: 300px; height: 100px;" expect_width="300"> </div>
          </div>
          <div style="width: auto; height: 100px;" expect_width="100">
            <div style="width: 50%; height: 100px;" expect_width="50"> </div>
          </div>
        </div>
    "#
    )
}
