// Tests for `align-content` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - align-content aligns flex lines within the flex container
// - Only applies when flex-wrap is enabled and there are multiple lines
// - Values: flex-start, flex-end, center, space-between, space-around, space-evenly, start, end

use crate::*;

// Case: align-content: flex-start
// Spec points:
// - Lines packed to start of cross axis
// In this test:
// - Container: width=50px, height=600px, flex-wrap=wrap
// - Three items of 50x50px (single item per line due to width)
// - Lines at top: 0, 50, 100
#[test]
fn flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: flex-start; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="0" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="100" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: start
// Spec points:
// - Same as flex-start for LTR writing mode
// In this test:
// - Same as flex-start test
#[test]
fn start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: start; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="0" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="100" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: flex-end
// Spec points:
// - Lines packed to end of cross axis
// In this test:
// - Container: height=600px, 3 items of 50px each = 150px total
// - Lines at bottom: 600-150=450, 500, 550
#[test]
fn flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: flex-end; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="450" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="500" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="550" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: end
// Spec points:
// - Same as flex-end for LTR writing mode
// In this test:
// - Same as flex-end test
#[test]
fn end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: end; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="450" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="500" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="550" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: center
// Spec points:
// - Lines centered in cross axis
// In this test:
// - Container: height=500px, 3 items of 100px = 300px total
// - Centering offset: (500-300)/2 = 100px
// - Lines at: 100, 200, 300
#[test]
fn center() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: center; width: 50px; height: 500px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="100" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="200" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="300" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: space-between
// Spec points:
// - First line at start, last line at end, space distributed between
// In this test:
// - Container: height=500px, 2 items of 100px
// - First at 0, last at 400
#[test]
fn space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: space-between; width: 50px; height: 500px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="0" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="400" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: space-around
// Spec points:
// - Equal space around each line (half space at edges)
// In this test:
// - Container: height=600px, 2 items of 100px = 200px content
// - Remaining: 400px, space per item = 200px, half at edges = 100px
// - First at 100, second at 400
#[test]
fn space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: space-around; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="100" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="400" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: space-evenly
// Spec points:
// - Equal space between and around lines
// In this test:
// - Container: height=500px, 2 items of 100px = 200px content
// - Remaining: 300px, 3 gaps = 100px each
// - First at 100, second at 300
#[test]
fn space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: space-evenly; width: 50px; height: 500px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="100" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="300" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-content: flex-end with flex-wrap in column direction
// Spec points:
// - In column wrap mode, cross axis is horizontal
// - flex-end aligns columns to the right
// In this test:
// - Container: width=100px, height=100px, flex-direction=column, wrap
// - 5 items that fit in single column
// - All items at left=50 (right half of container)
#[test]
fn flex_end_with_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; height: 100px; flex-wrap: wrap; flex-direction: column; align-content: flex-end;">
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="0"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="10"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="20"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="30"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="40"></div>
        </div>
    "#
    )
}
