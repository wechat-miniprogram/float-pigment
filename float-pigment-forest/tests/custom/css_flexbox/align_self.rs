// Tests for `align-self` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - align-self overrides align-items for individual flex items
// - Values: auto, flex-start, flex-end, center, baseline, stretch, start, end, self-start, self-end

use crate::*;

// Case: align-self: center
// Spec points:
// - Individual item centered on cross axis
// - Other items use default stretch
// In this test:
// - Item 1 & 3: align-self=center, centered at top=50
// - Item 2: default (stretch implied but has explicit height)
// - Item 4: stretch to container height
#[test]
fn align_self_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px;">
          <div style="height: 50px; width: 50px; align-self: center" expect_height="50" expect_top="50"></div>
          <div style="height: 50px; width: 50px;" expect_top="0"></div>
          <div style="width: 50px; align-self: center"  expect_height="50" expect_top="50">
            <div style="height: 50px; width: 50px"></div>
          </div>
          <div style="width: 50px;" expect_height="150" expect_top="0">
            <div style="height: 50px; width: 50px" expect_height="50"></div>
          </div>
        </div>
    "#
    )
}

// Case: align-self: stretch
// Spec points:
// - Item stretches to fill cross axis
// - Overrides align-items: start
// In this test:
// - Container: align-items=start
// - Item 1: align-self=stretch, height=150
// - Item 2: follows align-items, height=50
#[test]
fn align_self_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px; align-items: start">
          <div style="width: 50px; align-self: stretch" expect_height="150" expect_top="0">
            <div style="width: 50px; height: 50px;"></div>
          </div>
          <div style="height: 50px; width: 50px;" expect_height="50" expect_top="0"></div>

        </div>
    "#
    )
}

// Case: align-self: start, flex-start, self-start
// Spec points:
// - All align to start of cross axis
// - self-start uses item's writing mode for direction
// In this test:
// - Container: align-items=center (default would center)
// - Items with start/flex-start/self-start at top=0
// - Default item centered at top=50
#[test]
fn align_self_self_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px; align-items: center">
          <div style="height: 50px; width: 50px; align-self: start" expect_height="50" expect_top="0"></div>
          <div style="height: 50px; width: 50px;" expect_top="50"></div>
          <div style="width: 50px; align-self: flex-start" expect_height="50" expect_top="0">
            <div style="height: 50px; width: 50px"></div>
          </div>
          <div style="width: 50px; align-self: self-start" expect_height="50" expect_top="0">
            <div style="height: 50px; width: 50px"></div>
          </div>
        </div>
    "#
    )
}

// Case: align-self: end, flex-end, self-end
// Spec points:
// - All align to end of cross axis
// In this test:
// - Container: align-items=center
// - Items with end/flex-end/self-end at top=100 (150-50)
// - Default item centered at top=50
#[test]
fn align_self_self_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px; align-items: center">
          <div style="height: 50px; width: 50px; align-self: end" expect_height="50" expect_top="100"></div>
          <div style="height: 50px; width: 50px;" expect_top="50"></div>
          <div style="width: 50px; align-self: flex-end" expect_height="50" expect_top="100">
            <div style="height: 50px; width: 50px"></div>
          </div>
          <div style="width: 50px; align-self: self-end" expect_height="50" expect_top="100">
            <div style="height: 50px; width: 50px"></div>
          </div>
        </div>
    "#
    )
}
