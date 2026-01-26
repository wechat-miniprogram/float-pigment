// WPT-based tests for box-sizing property
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// box-sizing: content-box (default)
// Width/height apply to content area only, padding and border are added
#[test]
fn box_sizing_content_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 20px; border: 10px solid;" expect_width="260" expect_height="260">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// box-sizing: border-box
// Width/height include padding and border
#[test]
fn box_sizing_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding: 20px; border: 10px solid;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// box-sizing: content-box with percentage padding
// According to W3C CSS spec, percentage padding is calculated relative to the containing block's width
// For root's direct child, containing block is root element (375px in test framework)
// Container: 200px, padding: 10% of containing block width 375px = 37.5px each side
// Total width: 200 + 37.5*2 = 275px (rounded to 275px)
#[test]
fn box_sizing_content_box_percentage_padding() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 10%;" expect_width="275" expect_height="275">
            <div style="width: 50px; height: 50px;" expect_top="37" expect_left="37"></div>
        </div>
    "#
    )
}

// box-sizing: border-box with percentage padding
// According to W3C CSS spec, percentage padding is calculated relative to the containing block's width
// With border-box, padding is included in width
// For root's direct child, containing block is root element (375px in test framework)
// width: 200px includes padding, padding: 10% of containing block width 375px = 37.5px each side
#[test]
fn box_sizing_border_box_percentage_padding() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding: 10%;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="37" expect_left="37"></div>
        </div>
    "#
    )
}

// box-sizing: content-box with asymmetric padding
#[test]
fn box_sizing_content_box_asymmetric_padding() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 10px 20px 30px 40px;" expect_width="260" expect_height="240">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="40"></div>
        </div>
    "#
    )
}

// box-sizing: border-box with asymmetric padding
#[test]
fn box_sizing_border_box_asymmetric_padding() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding: 10px 20px 30px 40px;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="40"></div>
        </div>
    "#
    )
}

// box-sizing: content-box with border only
#[test]
fn box_sizing_content_box_border_only() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; border: 10px solid;" expect_width="220" expect_height="220">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// box-sizing: border-box with border only
#[test]
fn box_sizing_border_box_border_only() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; border: 10px solid;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// box-sizing: content-box with min-width
// min-width applies to content area
// min-width: 150px, padding: 20px each side, border: 10px each side
// Total width: 150 + 40 + 20 = 210px, but expands to fill parent (200px) = 200px
// Total height: 200 + 40 + 20 = 260px, but actual is 110px (content area)
#[test]
fn box_sizing_content_box_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="min-width: 150px; padding: 20px; border: 10px solid;" expect_width="210" expect_height="110">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// box-sizing: border-box with min-width
// min-width applies to total box including padding and border
#[test]
fn box_sizing_border_box_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="box-sizing: border-box; min-width: 150px; width: 100px; padding: 20px; border: 10px solid;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// box-sizing: content-box with max-width
// max-width applies to content area
// max-width: 150px, padding: 20px each side, border: 10px each side
// Total: 150 + 40 + 20 = 210px
#[test]
fn box_sizing_content_box_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
            <div style="max-width: 150px; padding: 20px; border: 10px solid;" expect_width="210" expect_height="110">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// box-sizing: border-box with max-width
// max-width applies to total box including padding and border
#[test]
fn box_sizing_border_box_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
            <div style="box-sizing: border-box; max-width: 150px; width: 200px; padding: 20px; border: 10px solid;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// box-sizing: nested content-box
#[test]
fn box_sizing_nested_content_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 20px;" expect_width="240" expect_height="240">
            <div style="width: 100px; height: 100px; padding: 10px;" expect_width="120" expect_height="120" expect_top="20" expect_left="20">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// box-sizing: nested border-box
#[test]
fn box_sizing_nested_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding: 20px;" expect_width="200" expect_height="200">
            <div style="box-sizing: border-box; width: 100px; height: 100px; padding: 10px;" expect_width="100" expect_height="100" expect_top="20" expect_left="20">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// box-sizing: mixed content-box and border-box
#[test]
fn box_sizing_mixed() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 20px;" expect_width="240" expect_height="240">
            <div style="box-sizing: border-box; width: 100px; height: 100px; padding: 10px;" expect_width="100" expect_height="100" expect_top="20" expect_left="20">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}
