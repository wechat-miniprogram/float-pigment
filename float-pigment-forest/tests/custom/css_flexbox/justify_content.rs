// Tests for `justify-content` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - justify-content aligns items along the main axis
// - Values: flex-start, flex-end, center, space-between, space-around, space-evenly, start, end, left, right

use crate::*;

// Case: justify-content: start
// Spec points:
// - Items packed to start of main axis
// In this test:
// - Items at left=0 and left=50
#[test]
fn justify_content_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: start">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: justify-content: flex-start
// Spec points:
// - Same as start for LTR
// In this test:
// - Same as start test
#[test]
fn justify_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-start">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: justify-content: center
// Spec points:
// - Items centered on main axis
// In this test:
// - Container: 300px, items: 100px total
// - Offset: (300-100)/2 = 100px
// - Items at left=100 and left=150
#[test]
fn justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: center">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="100"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="150"></div>
        </div>
    "#
    )
}

// Case: justify-content: end
// Spec points:
// - Items packed to end of main axis
// In this test:
// - Container: 300px, items: 100px total
// - Items at left=200 and left=250
#[test]
fn justify_content_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: end">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="200"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="250"></div>
        </div>
    "#
    )
}

// Case: justify-content: flex-end
// Spec points:
// - Same as end for LTR
// In this test:
// - Same as end test
#[test]
fn justify_content_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-end">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="200"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="250"></div>
        </div>
    "#
    )
}

// Case: justify-content: left
// Spec points:
// - Items packed to left edge (physical)
// In this test:
// - Items at left=0 and left=50
#[test]
fn justify_content_left() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: left">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: justify-content: right
// Spec points:
// - Items packed to right edge (physical)
// In this test:
// - Container: 300px, items: 100px total
// - Items at left=200 and left=250
#[test]
fn justify_content_right() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: right">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="200"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="250"></div>
        </div>
    "#
    )
}

// Case: justify-content: space-between
// Spec points:
// - First item at start, last at end, space distributed between
// In this test:
// - Container: 100px, items: 40px total
// - First at left=0, last at left=80
#[test]
fn justify_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; justify-content: space-between">
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="80"></div>
        </div>
    "#
    )
}

// Case: justify-content: space-around
// Spec points:
// - Equal space around each item (half space at edges)
// In this test:
// - Container: 120px, items: 40px total, remaining: 80px
// - Space per item: 40px (20px on each side)
// - First at left=20, second at left=80
#[test]
fn justify_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 120px; justify-content: space-around">
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="20"></div>
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="80"></div>
        </div>
    "#
    )
}

// Case: justify-content: space-evenly
// Spec points:
// - Equal space between and around items
// In this test:
// - Container: 170px, items: 80px total, remaining: 90px
// - 3 gaps = 30px each
// - First at left=30, second at left=100
#[test]
fn justify_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 170px; justify-content: space-evenly">
            <div style="height: 50px; width: 40px;" expect_width="40" expect_height="50" expect_left="30"></div>
            <div style="height: 50px; width: 40px;" expect_width="40" expect_height="50" expect_left="100"></div>
        </div>
    "#
    )
}
