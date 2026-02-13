// Tests for `max-height` property in CSS
// Based on CSS Box Sizing Module Level 3:
// - max-height sets the maximum height constraint
// - If content/height exceeds max-height, the element is clamped
// - max-height takes precedence over height but min-height takes precedence over max-height

use crate::*;

// Case: max-height greater than height (no clamping)
// Spec points:
// - When max-height > height, max-height has no effect
// In this test:
// - Element: max-height=100px, height=50px
// - Expected: height=50px (not clamped)
#[test]
fn max_height_fixed_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="50">
              <div style="max-height: 100px; height: 50px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: max-height less than height (clamped)
// Spec points:
// - When max-height < height, element is clamped to max-height
// In this test:
// - Element: max-height=50px, height=100px
// - Expected: height=50px (clamped)
#[test]
fn max_height_fixed_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="50">
              <div style="max-height: 50px; height: 100px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: Percentage max-height less than height (clamped)
// Spec points:
// - Percentage max-height is relative to parent height
// In this test:
// - Parent: height=100px
// - Child: max-height=50% = 50px, height=100px
// - Expected: height=50px (clamped)
#[test]
fn max_height_percentage_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="max-height: 50%; height: 100px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: Percentage max-height greater than height (no clamping)
// Spec points:
// - When percentage max-height > height, no clamping occurs
// In this test:
// - Parent: height=100px
// - Child: max-height=50% = 50px, height=20px
// - Expected: height=20px (not clamped, 20 < 50)
#[test]
fn max_height_percentage_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="max-height: 50%; height: 20px;" expect_height="20"></div>
            </div>
        </div>
    "#
    )
}
