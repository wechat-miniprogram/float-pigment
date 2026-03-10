// Tests for `box-sizing` property in CSS
// Based on CSS Box Sizing Module Level 3:
// - content-box: width/height apply to content area only; padding/border add to total
// - border-box: width/height include padding and border; content area shrinks
// - padding-box: width/height include padding; border adds to total (non-standard)

use crate::*;

// Case: content-box sizing
// Spec points:
// - width/height specify content box dimensions
// - padding is added outside the content box
// In this test:
// - Element: 200x200px content, padding-left=20px, padding-top=30px
// - Expected: width=220px, height=230px
// - Child positioned at (20, 30) due to padding
#[test]
fn content_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding-left: 20px; padding-top: 30px;" expect_width="220" expect_height="230">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="20"></div>
        </div>
    "#
    )
}

// Case: border-box sizing
// Spec points:
// - width/height specify border box dimensions
// - padding is included within declared dimensions
// In this test:
// - Element: 200x200px total (border-box), padding-left=20px, padding-top=30px
// - Expected: width=200px, height=200px (unchanged)
// - Child positioned at (20, 30) due to padding
#[test]
fn border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding-left: 20px; padding-top: 30px;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="20"></div>
        </div>
    "#
    )
}
