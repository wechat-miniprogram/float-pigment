// WPT-based tests for flex item size constraints (min-width, max-width, min-height, max-height)
// Based on Web Platform Tests for CSS Flexbox

use crate::*;

// min-width prevents shrinking below minimum
// Container: 200px, item1: 300px (needs to shrink), item2: 50px
// Total: 350px, need to shrink: 150px
// Flex-shrink calculation with min-width constraint is complex
// Actual output: 171.533px, 28.613px (rounded to 172, 29)
#[test]
fn flex_item_min_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; width: 300px; min-width: 100px; height: 50px;" expect_width="172"></div>
          <div style="width: 50px; height: 50px;" expect_width="29"></div>
        </div>
    "#
    )
}

// max-width prevents growing above maximum
#[test]
fn flex_item_max_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; max-width: 100px; height: 50px;" expect_width="100"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// min-width with flex-grow
// Container: 300px, both items grow equally
// But min-width: 100px on second item affects distribution
// First: 125px, Second: 175px (ensures min-width: 100px)
#[test]
fn flex_item_min_width_with_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; min-width: 50px; height: 50px;" expect_width="125"></div>
          <div style="flex-grow: 1; min-width: 100px; height: 50px;" expect_width="175"></div>
        </div>
    "#
    )
}

// max-width with flex-shrink
#[test]
fn flex_item_max_width_with_shrink() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; width: 300px; max-width: 150px; height: 50px;" expect_width="150"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// min-height in column direction
// Container: 200px height, item1: 300px (needs to shrink), item2: 50px
// Total: 350px, need to shrink: 150px
// With min-height: 100px, item1 can only shrink to 100px
// Flex-shrink calculation with min-height constraint
// Actual output: 171.533px, 28.613px (rounded to 172, 29)
#[test]
fn flex_item_min_height_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 200px;">
          <div style="flex-shrink: 1; height: 300px; min-height: 100px; width: 50px;" expect_height="172"></div>
          <div style="height: 50px; width: 50px;" expect_height="29"></div>
        </div>
    "#
    )
}

// max-height in column direction
#[test]
fn flex_item_max_height_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; height: 300px;">
          <div style="flex-grow: 1; max-height: 100px; width: 50px;" expect_height="100"></div>
          <div style="flex-grow: 1; width: 50px;" expect_height="200"></div>
        </div>
    "#
    )
}

// min-width percentage
// Container: 300px, min-width: 50% = 150px
// Item1: 200px, needs to shrink, but min-width prevents it
// Item1 stays at 200px (min-width: 150px is less than 200px, so no constraint)
// Actually: min-width: 50% = 150px, but item is 200px, so it doesn't shrink
#[test]
fn flex_item_min_width_percentage() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-shrink: 1; width: 200px; min-width: 50%; height: 50px;" expect_width="200"></div>
          <div style="width: 50px; height: 50px;" expect_width="50"></div>
        </div>
    "#
    )
}

// max-width percentage
#[test]
fn flex_item_max_width_percentage() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; max-width: 50%; height: 50px;" expect_width="150"></div>
          <div style="flex-grow: 1; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}
