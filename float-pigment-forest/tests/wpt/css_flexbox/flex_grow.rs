// WPT-style tests for the `flex-grow` property
// Inspired by WPT CSS Flexbox tests, covering main-axis space distribution:
// - `flex-grow` determines how flex items grow to fill available space along the main axis
// - Default value is 0 (no growth)
// - Positive values distribute free space proportionally among items
// - Free space = container size - sum of flex items' base sizes (flex-basis or width/height)

use crate::*;

// Case: `flex-grow: 0` (default, no growth)
// Spec points:
// - When flex-grow is 0, items do not grow to fill available space
// - Items maintain their base size (flex-basis or specified width/height)
// In this test:
// - Container: width=300
// - Two items: width=50 each (total 100px)
// - Free space: 300 - 100 = 200px (not distributed)
// - Items maintain their width: expect_width=50
#[test]
fn flex_grow_zero() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// Case: `flex-grow: 1` (equal distribution)
// Spec points:
// - When all items have the same flex-grow value, free space is distributed equally
// - Each item grows by the same amount
// In this test:
// - Container: width=300
// - Two items with flex-grow=1, no base width specified
// - Free space: 300px
// - Each item gets: 300 / 2 = 150px
// - Items: expect_width=150 each
#[test]
fn flex_grow_one() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// Case: `flex-grow` with different values (proportional distribution)
// Spec points:
// - Free space is distributed proportionally based on flex-grow ratios
// - Items with larger flex-grow values receive more space
// In this test:
// - Container: width=300
// - First item: flex-grow=1
// - Second item: flex-grow=2
// - Total grow factor: 1 + 2 = 3
// - First item: 300 * 1/3 = 100px
// - Second item: 300 * 2/3 = 200px
#[test]
fn flex_grow_proportional() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 50px;" expect_width="100"></div>
          <div style="flex-grow: 2; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// Case: `flex-grow: 0` with fixed width
// Spec points:
// - Items with flex-grow=0 maintain their specified width
// - Items with flex-grow>0 grow to fill remaining space
// In this test:
// - Container: width=300
// - First item: flex-grow=0, width=50 → expect_width=50
// - Second item: flex-grow=1 → gets remaining 250px
#[test]
fn flex_grow_zero_with_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="250"></div>
        </div>
    "#
    )
}

// Case: `flex-grow` with multiple items and different values
// Spec points:
// - Free space is calculated after accounting for items with fixed sizes
// - Space is distributed proportionally based on flex-grow ratios
// In this test:
// - Container: width=300
// - First item: flex-grow=0, width=50 (fixed)
// - Second item: flex-grow=1
// - Third item: flex-grow=2
// - Free space: 300 - 50 = 250px
// - Total grow factor: 1 + 2 = 3
// - Second item: 50 (base) + 250/3 ≈ 83px
// - Third item: 0 (base) + 250*2/3 ≈ 167px
#[test]
fn flex_grow_multiple_items() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 50px; height: 50px;" expect_width="50"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="83"></div>
          <div style="flex-grow: 2; height: 50px;" expect_width="167"></div>
        </div>
    "#
    )
}

// Case: `flex-grow` with `flex-basis`
// Spec points:
// - flex-basis sets the initial main-axis size before flex-grow distribution
// - Free space is calculated after all flex-basis values are applied
// In this test:
// - Container: width=300
// - Both items: flex-basis=50px, flex-grow=1
// - Total base size: 50 + 50 = 100px
// - Free space: 300 - 100 = 200px
// - Each item grows by: 200 / 2 = 100px
// - Final width: 50 + 100 = 150px each
#[test]
fn flex_grow_with_basis() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; flex-basis: 50px; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; flex-basis: 50px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// Case: `flex-grow` in column direction
// Spec points:
// - flex-grow works along the main axis, which is vertical in column direction
// - Items grow in height instead of width
// In this test:
// - Container: height=300, flex-direction=column
// - Two items with flex-grow=1
// - Each item gets: 300 / 2 = 150px height
#[test]
fn flex_grow_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px;">
          <div style="flex-grow: 1; width: 50px;" expect_height="150"></div>
          <div style="flex-grow: 1; width: 50px;" expect_height="150"></div>
        </div>
    "#
    )
}

// Case: `flex-grow` with `gap`
// Spec points:
// - Gap reduces the available space for flex-grow distribution
// - Free space = container size - gap - sum of base sizes
// In this test:
// - Container: width=300, gap=10px
// - Available space: 300 - 10 = 290px
// - Two items with flex-grow=1
// - Each item gets: 290 / 2 = 145px
#[test]
fn flex_grow_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; gap: 10px;">
          <div style="flex-grow: 1; height: 50px;" expect_width="145"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="145"></div>
        </div>
    "#
    )
}
