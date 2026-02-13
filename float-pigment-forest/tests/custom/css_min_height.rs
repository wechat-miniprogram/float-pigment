// Tests for `min-height` property in CSS
// Based on CSS Box Sizing Module Level 3:
// - min-height sets the minimum height constraint
// - If content/height is less than min-height, the element expands
// - min-height takes precedence over max-height

use crate::*;

// Case: min-height with no explicit height
// Spec points:
// - min-height ensures element is at least the specified height
// In this test:
// - Element: min-height=100px, no height specified
// - Expected: height=100px (expanded to min)
#[test]
fn min_height_fixed() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="100">
              <div style="min-height: 100px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}

// Case: min-height greater than explicit height
// Spec points:
// - When min-height > height, element expands to min-height
// In this test:
// - Element: min-height=100px, height=10px
// - Expected: height=100px (expanded)
#[test]
fn min_height_fixed_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="100">
              <div style="min-height: 100px; height: 10px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}

// Case: min-height less than explicit height
// Spec points:
// - When min-height < height, min-height has no effect
// In this test:
// - Element: min-height=10px, height=100px
// - Expected: height=100px (no change)
#[test]
fn min_height_fixed_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="100">
              <div style="min-height: 10px; height: 100px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}

// Case: Percentage min-height
// Spec points:
// - Percentage min-height is relative to parent height
// In this test:
// - Parent: height=100px
// - Child: min-height=50% = 50px
#[test]
fn min_height_percentage() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="min-height: 50%;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: Percentage min-height greater than explicit height
// Spec points:
// - min-height percentage constraint wins over explicit height
// In this test:
// - Parent: height=100px
// - Child: min-height=50% = 50px, height=10px
// - Expected: height=50px (expanded to min)
#[test]
fn min_height_percentage_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="min-height: 50%; height: 10px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// Case: Percentage min-height less than explicit height
// Spec points:
// - When min-height < height, explicit height is used
// In this test:
// - Parent: height=100px
// - Child: min-height=50% = 50px, height=100px
// - Expected: height=100px (no change)
#[test]
fn min_height_percentage_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="min-height: 50%; height: 100px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}
