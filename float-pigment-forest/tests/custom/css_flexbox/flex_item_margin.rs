// Tests for flex item margins in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - margin: auto absorbs remaining space in flex container
// - Auto margins can be used for alignment and spacing

use crate::*;

// Case: Auto margins on both sides of flex items
// Spec points:
// - margin-left: auto and margin-right: auto centers items
// - Remaining space distributed to auto margins
// In this test:
// - Container: 100px
// - Two items of 30px width with auto margins
// - Remaining: 100 - 60 = 40px, split among 4 auto margins = 10px each
// - Item 1: 10px margin-left, at left=10
// - Item 2: after Item 1 (30px + 10px right + 10px left), at left=60
#[test]
fn flex_item_with_margin() {
    assert_xml!(
        r#"
        <div style="height: 100px; display: flex; width: 100px;">
          <div style="width: 30px; margin-left: auto; margin-right: auto" expect_left="10" expect_width="30"></div>
          <div style="width: 30px; margin-left: auto; margin-right: auto" expect_left="60" expect_width="30"></div>
        </div>
    "#
    )
}

// Case: Auto margin on one item only
// Spec points:
// - Only auto margins absorb space
// - Item without auto margin positioned normally
// In this test:
// - Container: 100px
// - Item 1: 30px with auto margins
// - Item 2: 30px without auto margins (at right edge)
// - Remaining: 100 - 60 = 40px, split to Item 1's 2 auto margins = 20px each
// - Item 1: at left=20
// - Item 2: at left=70 (20 + 30 + 20)
#[test]
fn flex_item_with_margin_1() {
    assert_xml!(
        r#"
        <div style="height: 100px; display: flex; width: 100px;">
          <div style="width: 30px; margin-left: auto; margin-right: auto" expect_left="20" expect_width="30"></div>
          <div style="width: 30px;" expect_left="70" expect_width="30"></div>
        </div>
    "#
    )
}
