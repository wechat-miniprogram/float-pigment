// Tests for `max-width` property in CSS
// Based on CSS Box Sizing Module Level 3:
// - max-width sets the maximum width constraint
// - If content/width exceeds max-width, the element is clamped
// - max-width takes precedence over width but min-width takes precedence over max-width

use crate::*;

// Case: max-width constrains stretched block
// Spec points:
// - Block element stretches to parent width by default
// - max-width clamps the stretched width
// In this test:
// - Parent: width=300px
// - Child: max-width=100px, stretches to 300px then clamped to 100px
#[test]
fn max_width_fixed() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 100px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

// Case: Percentage max-width
// Spec points:
// - Percentage max-width is relative to parent width
// In this test:
// - Parent: width=300px
// - Child: max-width=50% = 150px
#[test]
fn max_width_percentage() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 50%; height: 50px;" expect_width="150"></div>
          </div>
      "#
    )
}

// Case: max-width less than explicit width (clamped)
// Spec points:
// - max-width clamps explicit width value
// In this test:
// - Child: max-width=100px, width=200px
// - Expected: width=100px (clamped)
#[test]
fn max_width_fixed_lt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 100px; width: 200px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

// Case: max-width greater than explicit width (no clamping)
// Spec points:
// - When max-width > width, no clamping occurs
// In this test:
// - Child: max-width=300px, width=200px
// - Expected: width=200px (not clamped)
#[test]
fn max_width_fixed_gt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 300px; width: 200px; height: 50px;" expect_width="200"></div>
          </div>
      "#
    )
}

// Case: max-width with overflowing child content
// Spec points:
// - max-width clips parent but child can overflow
// In this test:
// - Parent: max-width=200px
// - Child: width=300px (overflows parent)
// - Parent clamped to 200px, child renders at 300px (overflow)
#[test]
fn max_width_fixed_lt_child_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 200px; height: 50px;" expect_width="200">
              <div style="width: 300px; height: 50px" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}

// Case: max-width greater than child content
// Spec points:
// - When max-width > child content, max-width doesn't affect layout
// In this test:
// - Parent: max-width=400px, child width=300px
// - Parent shrinks to fit child at 300px (not stretched to 400px)
#[test]
fn max_width_fixed_gt_child_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 400px; height: 50px;" expect_width="300">
              <div style="width: 300px; height: 50px" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}
