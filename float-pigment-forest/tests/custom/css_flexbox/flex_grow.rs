// Tests for `flex-grow` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - flex-grow determines how much an item grows relative to others
// - Remaining space is distributed proportionally to flex-grow values
// - Default value is 0 (no growth)

use crate::*;

// Case: flex-grow: 0 and 1
// Spec points:
// - Item with flex-grow: 0 doesn't grow
// - Item with flex-grow: 1 takes all remaining space
// In this test:
// - Container: 300px
// - Item 1: flex-grow=0, width=0 (no base width)
// - Item 2: flex-grow=1, width=300px (all space)
#[test]
fn flex_grow_0_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; height: 10px;" expect_width="0"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="300"></div>
        </div>
    "#
    )
}

// Case: flex-grow: 1 and 1
// Spec points:
// - Equal flex-grow values split space equally
// In this test:
// - Container: 300px
// - Both items: flex-grow=1, each gets 150px
#[test]
fn flex_grow_1_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 10px;" expect_width="150"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="150"></div>
        </div>
    "#
    )
}

// Case: flex-grow: 1 and 2
// Spec points:
// - Space distributed proportionally (1:2 ratio)
// In this test:
// - Container: 300px
// - Item 1: 1/3 = 100px
// - Item 2: 2/3 = 200px
#[test]
fn flex_grow_1_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 10px;" expect_width="100"></div>
          <div style="flex-grow: 2; height: 10px;" expect_width="200"></div>
        </div>
    "#
    )
}

// Case: flex-grow: 0, 1, and 2
// Spec points:
// - flex-grow: 0 takes no extra space
// - Remaining items split proportionally
// In this test:
// - Container: 300px
// - Item 1: flex-grow=0, width=0
// - Item 2: 1/3 of 300 = 100px
// - Item 3: 2/3 of 300 = 200px
#[test]
fn flex_grow_0_1_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; height: 10px;" expect_width="0"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="100"></div>
          <div style="flex-grow: 2; height: 10px;" expect_width="200"></div>
        </div>
    "#
    )
}

// Case: flex-grow with base width
// Spec points:
// - Base width is deducted from container before distributing
// In this test:
// - Container: 300px
// - Item 1: flex-grow=0, width=30px (fixed)
// - Remaining: 300-30 = 270px
// - Item 2: 1/3 of 270 = 90px
// - Item 3: 2/3 of 270 = 180px
#[test]
fn flex_grow_0_width_1_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 30px; height: 10px;" expect_width="30"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="90"></div>
          <div style="flex-grow: 2; height: 10px;" expect_width="180"></div>
        </div>
    "#
    )
}

// Case: flex-grow: 0 with explicit widths
// Spec points:
// - flex-grow: 0 respects explicit width
// - Items don't grow beyond their specified width
// In this test:
// - All items flex-grow=0
// - Widths: 10px, 20px, 0px (no width specified)
#[test]
fn flex_grow_0_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 10px; height: 10px;" expect_width="10"></div>
          <div style="flex-grow: 0; width: 20px; height: 10px;" expect_width="20"></div>
          <div style="flex-grow: 0; height: 10px;" expect_width="0"></div>
        </div>
    "#
    )
}
