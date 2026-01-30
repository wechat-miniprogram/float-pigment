// Tests for `display` property in CSS
// Based on CSS Display Module Level 3:
// - display: block - generates a block-level box
// - display: none - removes element from layout entirely
// - display: flex - generates a flex container
// - display: inline - generates inline-level boxes

use crate::*;

// Case: display: block (default)
// Spec points:
// - Block elements stack vertically
// - Block elements stretch to fill container width by default
// In this test:
// - First child: width=200px, height=40px
// - Second child: width=auto (stretches to 375px), positioned at top=40
#[test]
fn display_block() {
    assert_xml!(
        r#"
            <div style="height: 80px;">
                <div>
                    <div style="height: 40px; width: 200px; margin-right: 100px;" expect_width="200"></div>
                    <div style="height: 40px;" expect_width="375" expect_top="40"></div>
                </div>
            </div>
        "#
    )
}

// Case: display: none
// Spec points:
// - Element with display: none takes no space
// - Element's computed width/height are effectively 0
// - Subsequent siblings positioned as if element doesn't exist
// In this test:
// - First child: display=none, renders as 0x0
// - Second child: positioned at top=0 (as if first doesn't exist)
#[test]
fn display_none() {
    assert_xml!(
        r#"
            <div style="height: 300px;" expect_height="300">
                <div style="display: none; width: 100px; height: 100px;" expect_top="0" expect_height="0" expect_width="0"></div>
                <div style="width: 100px; height: 100px;" expect_height="100" expect_width="100" expect_top="0"></div>
            </div>
        "#
    )
}

// Case: display: flex
// Spec points:
// - Flex container lays out children in a row by default
// - flex-grow distributes remaining space proportionally
// In this test:
// - Container: flex, width=100px (explicit in markup)
// - Child 1: 50x50px fixed
// - Child 2: 30x50px base, flex-grow=1, grows to fill remaining space (50px)
#[test]
fn display_flex() {
    assert_xml!(
        r#"
            <div style="height: 300px;" expect_height="300">
               <div style="display: flex; width="100px" expect_height="50">
                    <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50"></div>
                    <div style="height: 50px; width: 30px; flex-grow: 1;" expect_width="50" expect_height="50"></div>
               </div>
            </div>
        "#
    )
}
