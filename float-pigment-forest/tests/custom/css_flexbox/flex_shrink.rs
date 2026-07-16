// Tests for `flex-shrink` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - flex-shrink determines how much an item shrinks when container is too small
// - Shrink amount proportional to flex-shrink * flex-basis
// - Default value is 1 (items can shrink)

use crate::*;

// Case: flex-shrink: 0 and 1
// Spec points:
// - Item with flex-shrink: 0 doesn't shrink
// - Item with flex-shrink: 1 absorbs all overflow
// In this test:
// - Container: 100px
// - Item 1: flex-shrink=0, width=200px (doesn't shrink, overflows)
// - Item 2: flex-shrink=1, width=100px, shrinks to 0px
#[test]
fn flex_shrink_0_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px;">
          <div style="flex-shrink: 0; height: 100px; width: 200px;" expect_width="200"></div>
          <div style="flex-shrink: 1; height: 100px; width: 100px;" expect_width="0"></div>
        </div>
    "#
    )
}

// Case: flex-shrink: 1 and 1 with different bases
// Spec points:
// - Shrink proportional to flex-shrink * flex-basis
// - Item 1: shrink ratio = 1 * 200 = 200
// - Item 2: shrink ratio = 1 * 300 = 300
// - Total ratio = 500, overflow = 300px
// - Item 1 shrinks: 300 * (200/500) = 120, final = 80px
// - Item 2 shrinks: 300 * (300/500) = 180, final = 120px
// In this test:
// - Container: 200px, total content: 500px, overflow: 300px
// - Item 1: 200px base, shrinks to 80px
// - Item 2: 300px base, shrinks to 120px
#[test]
fn flex_shrink_1_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; height: 100px; width: 200px;" expect_width="80"></div>
          <div style="flex-shrink: 1; height: 100px; width: 300px;" expect_width="120"></div>
        </div>
    "#
    )
}

// Case: flex-shrink: 1, 0, and 2
// Spec points:
// - Item with flex-shrink: 0 doesn't shrink
// - Others shrink proportionally
// In this test:
// - Container: 200px, total needed: 520px
// - Item 2: flex-shrink=0, stays at 20px
// - Remaining items must fit in 180px (from 500px needed)
// - Overflow for shrinkable items: 500 - 180 = 320px
// - Item 1 ratio: 1 * 200 = 200
// - Item 3 ratio: 2 * 300 = 600
// - Total ratio: 800
// - Item 1 shrinks: 320 * (200/800) = 80, final = 120px
// - Item 3 shrinks: 320 * (600/800) = 240, final = 60px
#[test]
fn flex_shrink_1_0_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; height: 100px; width: 200px;" expect_width="120"></div>
          <div style="flex-shrink: 0; height: 100px; width: 20px;" expect_width="20"></div>
          <div style="flex-shrink: 2; height: 100px; width: 300px;" expect_width="60"></div>
        </div>
    "#
    )
}

// Case: flex-shrink with max-width clamping the declared main size
// Spec points (§9.2.3 vs §9.2.4):
// - The flex base size uses the declared main size UNCLAMPED (§9.2.3).
// - Clamping to min/max happens only at the hypothetical main size step (§9.2.4).
// - Therefore the scaled flex shrink factor (§9.7, = flex-shrink × inner flex base
//   size) uses the unclamped declared size, not the max-clamped size.
// In this test:
// - Container: 100px, both items width=200 max-width=100 (declared > max)
// - flex-shrink 1 and 2 -> scaled factors 1*200=200 and 2*200=400 (sum 600)
// - free space = 100 - (200+200) = -300
// - item1 target = 200 + (-300)*(200/600) = 100, clamped to [0,100] = 100
// - item2 target = 200 + (-300)*(400/600) = 0, clamped to [0,100] = 0
// - (If flex base size were wrongly clamped to 100, targets would be ~66.67 and ~33.33.)
#[test]
fn flex_shrink_max_width_unclamped_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px;">
          <div style="flex-shrink: 1; max-width: 100px; height: 100px; width: 200px;" expect_width="100"></div>
          <div style="flex-shrink: 2; max-width: 100px; height: 100px; width: 200px;" expect_width="0"></div>
        </div>
    "#
    )
}
