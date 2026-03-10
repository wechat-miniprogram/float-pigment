// Tests for `min-width` and `min-height` in CSS
// Based on CSS Box Sizing Module Level 3:
// - min-width sets the minimum width constraint
// - If content/width is less than min-width, the element expands
// - min-width takes precedence over max-width

use crate::*;

// Case: min-width greater than container
// Spec points:
// - min-width can cause overflow when larger than container
// In this test:
// - Parent: width=300px
// - Child: min-width=400px, overflows parent
// - Expected: width=400px
#[test]
fn min_width_fixed() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 400px; height: 50px;" expect_width="400"></div>
          </div>
      "#
    )
}

// Case: Percentage min-width
// Spec points:
// - Percentage min-width is relative to parent width
// In this test:
// - Parent: width=300px
// - Child: min-width=50% = 150px, stretches to 300px (> 150px)
// - Expected: width=300px (stretched, since stretch > min)
#[test]
fn min_width_percentage() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 50%; height: 50px;" expect_width="300"></div>
          </div>
      "#
    )
}

// Case: min-width in flex container
// Spec points:
// - In flex, min-width constrains shrinking
// In this test:
// - Flex container: width=300px
// - Child: min-width=100px, no other size
// - Expected: width=100px
#[test]
fn min_width_fixed_in_flex() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px; display: flex" expect_width="300">
            <div style="min-width: 100px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

// Case: Percentage min-width in flex container
// Spec points:
// - Percentage min-width works in flex context
// In this test:
// - Flex container: width=300px
// - Child: min-width=50% = 150px
// - Expected: width=150px
#[test]
fn min_width_percentage_in_flex() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px; display: flex" expect_width="300">
            <div style="min-width: 50%; height: 50px;" expect_width="150"></div>
          </div>
      "#
    )
}

// Case: min-width less than explicit width
// Spec points:
// - When min-width < width, explicit width is used
// In this test:
// - Child: min-width=50px, width=100px
// - Expected: width=100px (no change)
#[test]
fn min_width_fixed_lt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 50px; width: 100px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

// Case: min-width greater than explicit width
// Spec points:
// - When min-width > width, element expands to min-width
// In this test:
// - Child: min-width=150px, width=100px
// - Expected: width=150px (expanded)
#[test]
fn min_width_fixed_gt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 150px; width: 100px; height: 50px;" expect_width="150"></div>
          </div>
      "#
    )
}

// Case: min-height with flex align-items: center
// Spec points:
// - min-height affects cross-axis centering in flex
// In this test:
// - Container: min-height=300px, align-items=center
// - Child centered vertically at (300-100)/2 = 100
#[test]
fn min_height_flex_align_items_center() {
    assert_xml!(
        r#"
          <div style="display:flex;  width: 300px; min-height: 300px; align-items: center;">
            <div style="width: 100px; height: 100px;" expect_top="100"></div>
          </div>
      "#
    )
}

// Case: min-height with flex justify-content: center (column)
// Spec points:
// - In flex column, min-height affects main-axis centering
// In this test:
// - Container: flex-direction=column, min-height=300px, justify-content=center
// - Child centered at (300-100)/2 = 100
#[test]
fn min_height_flex_justify_content_center() {
    assert_xml!(
        r#"
          <div style="display:flex; flex-direction: column; width: 300px; min-height: 300px; justify-content: center;">
            <div style="width: 100px; height: 100px;" expect_top="100"></div>
          </div>
      "#
    )
}
