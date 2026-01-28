// WPT-style tests for border-width properties
// Inspired by WPT CSS Borders tests, covering individual border width properties:
// - `border-top-width`, `border-right-width`, `border-bottom-width`, `border-left-width`
// - Can be fixed lengths, percentages, or keywords (thin=1px, medium=3px, thick=5px)
// - Border width affects the box model: with `content-box`, width/height exclude borders
// - With `border-box`, width/height include borders
// - Percentage values are resolved relative to the containing block's width
// Note: This engine only supports border-width properties, not border-style or border-color

use crate::*;

// border-width: fixed value (all sides)
#[test]
fn border_width_fixed() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="220" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border-width: asymmetric (top right bottom left)
#[test]
fn border_width_asymmetric() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 10px; border-right-width: 20px; border-bottom-width: 30px; border-left-width: 40px;" expect_width="260" expect_height="140">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="40"></div>
        </div>
    "#
    )
}

// border-width: thin (1px)
#[test]
fn border_width_thin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: thin; border-right-width: thin; border-bottom-width: thin; border-left-width: thin;" expect_width="202" expect_height="102">
            <div style="width: 50px; height: 50px;" expect_top="1" expect_left="1"></div>
        </div>
    "#
    )
}

// border-width: medium (3px default)
#[test]
fn border_width_medium() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: medium; border-right-width: medium; border-bottom-width: medium; border-left-width: medium;" expect_width="206" expect_height="106">
            <div style="width: 50px; height: 50px;" expect_top="3" expect_left="3"></div>
        </div>
    "#
    )
}

// border-width: thick (5px)
#[test]
fn border_width_thick() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: thick; border-right-width: thick; border-bottom-width: thick; border-left-width: thick;" expect_width="210" expect_height="110">
            <div style="width: 50px; height: 50px;" expect_top="5" expect_left="5"></div>
        </div>
    "#
    )
}

// border-width: 0
#[test]
fn border_width_zero() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 0; border-right-width: 0; border-bottom-width: 0; border-left-width: 0;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// border-width: percentage (relative to containing block width)
#[test]
fn border_width_percentage() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="width: 100px; height: 100px; border-top-width: 10%; border-right-width: 10%; border-bottom-width: 10%; border-left-width: 10%;" expect_width="140" expect_height="140">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// border-width: with border-box
#[test]
fn border_width_with_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border-width: with border-box and padding
// With border-box, width/height includes border and padding
// width: 200px includes border (10px*2) and padding (20px*2)
// Total: 200px, content area = 200 - 20 - 20 = 160px
#[test]
fn border_width_with_border_box_and_padding() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 20px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// border-width: with border-box and percentage width
// With border-box, percentage width includes border
#[test]
fn border_width_with_border_box_percentage() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
            <div style="box-sizing: border-box; width: 50%; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="150" expect_height="100">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: with padding
#[test]
fn border_width_with_padding() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 20px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="260" expect_height="160">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// border-width: individual properties
#[test]
fn border_width_individual() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 5px; border-right-width: 10px; border-bottom-width: 15px; border-left-width: 20px;" expect_width="230" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="5" expect_left="20"></div>
        </div>
    "#
    )
}

// border-width: with min-width constraint
#[test]
fn border_width_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="min-width: 150px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="200" expect_height="70">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: with min-width constraint and border-box
// min-width applies to total box including border
#[test]
fn border_width_with_min_width_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="box-sizing: border-box; min-width: 150px; width: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="150" expect_height="70">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: with max-width constraint
#[test]
fn border_width_with_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="max-width: 150px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="170">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: with max-width constraint and border-box
// max-width applies to total box including border
#[test]
fn border_width_with_max_width_border_box() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="box-sizing: border-box; max-width: 150px; width: 200px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: in flex container
#[test]
fn border_width_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="60" expect_left="0">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: in flex container with border-box
// With border-box, flex item width includes border
#[test]
fn border_width_in_flex_container_border_box() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="box-sizing: border-box; flex-grow: 1; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="300">
                <div style="width: 50px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border-width: nested elements
#[test]
fn border_width_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="220" expect_height="220">
            <div style="width: 100px; height: 100px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="110" expect_height="110" expect_top="10" expect_left="10">
                <div style="width: 50px; height: 50px;" expect_top="5" expect_left="5"></div>
            </div>
        </div>
    "#
    )
}
