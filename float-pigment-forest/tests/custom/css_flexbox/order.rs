// Tests for `order` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - order controls visual order of flex items
// - Items sorted by order value, then by source order
// - Default value is 0, negative values allowed

use crate::*;

// Case: Basic order (1, 2, 3, 4)
// Spec points:
// - Items rendered in order value sequence
// In this test:
// - Items with increasing order values
// - Positions: 0, 10, 30, 60
#[test]
fn order() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 2;  width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 3;  width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 4;  width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

// Case: Equal order values (source order as tiebreaker)
// Spec points:
// - Items with same order maintain source order
// In this test:
// - Items 1 & 2 both have order=1
// - They appear in source order (10px then 20px)
#[test]
fn order_1() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 1;  width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 3;  width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 4;  width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

// Case: Multiple equal order values
// Spec points:
// - Multiple groups of same order maintain source order within group
// In this test:
// - Two items with order=1, two with order=2
#[test]
fn order_2() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 1;  width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 2;  width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 2;  width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

// Case: Reverse order (4, 3, 2, 1)
// Spec points:
// - Items reordered by decreasing order value
// In this test:
// - Source order: 10, 20, 30, 40px
// - Visual order: 40, 30, 20, 10px (reversed)
#[test]
fn order_3() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 4; width: 10px; height: 10px;" expect_left="90"></div>
            <div style="order: 3; width: 20px; height: 10px;" expect_left="70"></div>
            <div style="order: 2; width: 30px; height: 10px;" expect_left="40"></div>
            <div style="order: 1; width: 40px; height: 10px;" expect_left="0"></div>
          </div>
      "#
    )
}

// Case: Negative and large order values
// Spec points:
// - Negative order values are valid
// - Items sorted by numeric value
// In this test:
// - order: -100, 0, 1, 100
// - Visual: -100 first, then 0, 1, 100
#[test]
fn order_4() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: -100; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 0; width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 1; width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 100; width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

// Case: Mixed order values
// Spec points:
// - Complex ordering with duplicates
// In this test:
// - order: 1, 3, 2, 4, 2
// - Visual: 1(10), 2(30), 2(50), 3(20), 4(40)
// - Positions: 0, 10, 40, 90, 110
#[test]
fn order_5() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 3;  width: 20px; height: 10px;" expect_left="90"></div>
            <div style="order: 2;  width: 30px; height: 10px;" expect_left="10"></div>
            <div style="order: 4;  width: 40px; height: 10px;" expect_left="110"></div>
            <div style="order: 2;  width: 50px; height: 10px;" expect_left="40"></div>
          </div>
      "#
    )
}

// Case: Order with absolute/fixed positioned items
// Spec points:
// - Absolute/fixed items don't participate in flex layout
// - Order only affects in-flow flex items
// In this test:
// - Items 2 (fixed) and 3 (absolute) removed from flow
// - Only items 1, 4, 5 participate in flex layout
#[test]
fn order_6() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 3; position: fixed; width: 20px; height: 10px; left: 0px; height: 0px;" expect_left="0"></div>
            <div style="order: 2; position: absolute; width: 30px; height: 10px; left: 0px; height: 0px;" expect_left="0"></div>
            <div style="order: 4; width: 40px; height: 10px;" expect_left="60"></div>
            <div style="order: 2; width: 50px; height: 10px;" expect_left="10"></div>
          </div>
      "#
    )
}
