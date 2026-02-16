// Tests for text rendering with font-size in CSS
// Based on CSS Fonts Module Level 4:
// - font-size determines the size of the font
// - Affects line height and text dimensions

use crate::*;

// Case: Text with font-size
// Spec points:
// - font-size affects line height
// In this test:
// - Container with font-size=30px
// - Expected height matches font-size (30px)
#[test]
fn text_with_font_size() {
    assert_xml!(
        r#"
        <div style="font-size: 30px;" expect_height="30">
          XX
        </div>
    "#,
        true
    )
}

// Case: Text with font-size in inline container
// Spec points:
// - Inline element inherits font-size
// - Inline width determined by text content at that font size
// In this test:
// - Container: font-size=30px
// - Inline child: contains "XX", width = 2 chars * 30px = 60px
#[test]
fn text_with_font_size_2() {
    assert_xml!(
        r#"
        <div style="font-size: 30px;" expect_height="30">
          <div style="display: inline" expect_width="60">XX</div>
        </div>
    "#,
        true
    )
}
