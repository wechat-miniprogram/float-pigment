// Tests for `border` properties in CSS
// Based on CSS Box Model Module Level 3:
// - Border adds to the element's total size in content-box mode
// - Border is included within the element's size in border-box mode
// - Border can be specified for individual sides (top, right, bottom, left)
// - Border width can be fixed values or percentages

use crate::*;

// Case: Fixed border with content-box sizing
// Spec points:
// - In content-box mode, border adds to width and height
// - border: 10px adds 10px on each side (20px total per axis)
// In this test:
// - Element: 10x10px content, border=10px
// - Expected: 10+20 = 30px for both width and height
#[test]
fn border_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; box-sizing: content-box; border: 10px;" expect_height="30" expect_width="30"></div>
    "#
    )
}

// Case: Fixed border with border-box sizing
// Spec points:
// - In border-box mode, border is included within declared width/height
// - Content area shrinks to accommodate border
// In this test:
// - Element: 10x10px with border-box, border=1px
// - Expected: 10x10px total (border included)
#[test]
fn border_fixed_border_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; box-sizing: border-box; border: 1px;" expect_height="10" expect_width="10"></div>
    "#
    )
}

// Case: Percentage border with content-box sizing
// Spec points:
// - Percentage border is calculated relative to parent's width
// - Border percentage applies to all four sides
// In this test:
// - Parent: 300x200px
// - Child: 10x10px content, border=10% = 30px (10% of 300px width)
// - Expected: 10+60 = 70px for both width and height
#[test]
fn border_percentage_content_box() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
          <div style="height: 10px; width: 10px; border: 10%;" expect_height="70" expect_width="70"></div>
        </div>
    "#
    )
}

// Case: border-left fixed value
// Spec points:
// - border-left only affects the left side
// In this test:
// - Element: 10x10px, border-left=20px
// - Expected width: 10+20 = 30px
#[test]
fn border_left_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-left: 20px;" expect_width="30"></div>
    "#
    )
}

// Case: border-right fixed value
// Spec points:
// - border-right only affects the right side
// In this test:
// - Element: 10x10px, border-right=20px
// - Expected width: 10+20 = 30px
#[test]
fn border_right_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-right: 20px;" expect_width="30"></div>
    "#
    )
}

// Case: border-top fixed value
// Spec points:
// - border-top only affects the top side
// In this test:
// - Element: 10x10px, border-top=20px
// - Expected height: 10+20 = 30px
#[test]
fn border_top_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-top: 20px;" expect_height="30"></div>
    "#
    )
}

// Case: border-bottom fixed value
// Spec points:
// - border-bottom only affects the bottom side
// In this test:
// - Element: 10x10px, border-bottom=20px
// - Expected height: 10+20 = 30px
#[test]
fn border_bottom_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-bottom: 20px;" expect_height="30"></div>
    "#
    )
}
