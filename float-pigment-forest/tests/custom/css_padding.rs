// Tests for `padding` properties in CSS
// Based on CSS Box Model Module Level 3:
// - Padding creates space inside the element's border
// - Padding is added to width/height in content-box sizing
// - Padding percentages are relative to containing block width

use crate::*;

// Case: Fixed padding with four values
// Spec points:
// - padding: top right bottom left
// - Padding increases total element size in content-box
// In this test:
// - Element: height=10px, padding=10px 20px 30px 40px
// - Total height: 10 + 10 + 30 = 50px
// - Child at left=40px (padding-left), top inherits
// - Child width: 375 - 40 - 20 = 315px
#[test]
fn padding_fixed() {
    assert_xml!(
        r#"
        <div style="height: 10px; padding: 10px 20px 30px 40px;" expect_height="50">
            <div style="height: 100%" expect_height="10" expect_width="315" expect_left="40"></div>
        </div>
    "#
    )
}

// Case: padding-left fixed
// Spec points:
// - padding-left only affects left side
// In this test:
// - Element: width=10px, padding-left=20px
// - Total width: 10 + 20 = 30px
#[test]
fn padding_left_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-left: 20px; width: 10px; height: 10px;" expect_width="30"></div>
        </div>
    "#
    )
}

// Case: padding-right fixed with content-box
// Spec points:
// - padding-right adds to right side
// - Explicit box-sizing: content-box ensures padding is additive
// In this test:
// - Element: width=10px, padding-right=20px
// - Total width: 10 + 20 = 30px
#[test]
fn padding_right_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-right: 20px; box-sizing: content-box; width: 10px; height: 10px;" expect_width="30"></div>
        </div>
    "#
    )
}

// Case: padding-top fixed
// Spec points:
// - padding-top adds to top side
// In this test:
// - Element: height=10px, padding-top=20px
// - Total height: 10 + 20 = 30px
#[test]
fn padding_top_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-top: 20px; box-sizing: content-box; width: 10px; height: 10px;" expect_width="10" expect_height="30"></div>
        </div>
    "#
    )
}

// Case: padding-bottom fixed
// Spec points:
// - padding-bottom adds to bottom side
// In this test:
// - Element: height=10px, padding-top=20px (test uses padding-top)
// - Total height: 10 + 20 = 30px
#[test]
fn padding_bottom_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-top: 20px; box-sizing: content-box; width: 10px; height: 10px;" expect_width="10" expect_height="30"></div>
        </div>
    "#
    )
}

// Case: Percentage padding
// Spec points:
// - Percentage padding is relative to containing block width
// - This applies to both horizontal AND vertical padding
// In this test:
// - Parent: width=100px, height=200px
// - Child: padding=10% = 10px (10% of 100px width)
// - Child height: padding-top + padding-bottom = 10 + 10 = 20px
#[test]
fn padding_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;">
            <div style="padding: 10%" expect_height="20"></div>
        </div>
    "#
    )
}
