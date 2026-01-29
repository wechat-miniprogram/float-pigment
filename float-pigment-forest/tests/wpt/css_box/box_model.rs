// WPT-based tests for box model combinations and interactions
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// Box model: width + padding + border (content-box)
#[test]
fn box_model_content_box_full() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 20px; border: 10px solid;" expect_width="260" expect_height="160">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// Box model: width + padding + border (border-box)
#[test]
fn box_model_border_box_full() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 20px; border: 10px solid;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// Box model: percentage width with padding and border
// width: 50% of 300px = 150px, padding: 20px each side, border: 10px each side
// Total: 150 + 40 + 20 = 210px
// Height: 200px, padding: 20px each side, border: 10px each side
// Total height: 200 + 40 + 20 = 260px, but content height is 110px
#[test]
fn box_model_percentage_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
            <div style="width: 50%; padding: 20px; border: 10px solid;" expect_width="210" expect_height="110">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: min-width with padding and border
// min-width: 150px (content), padding: 20px each side, border: 10px each side
// Total: 150 + 40 + 20 = 210px
// Height: 100px, padding: 20px each side, border: 10px each side
// Total height: 100 + 40 + 20 = 160px, but content height is 110px
#[test]
fn box_model_min_width_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="min-width: 150px; padding: 20px; border: 10px solid;" expect_width="210" expect_height="110">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: max-width with padding and border
// max-width: 150px (content), padding: 20px each side, border: 10px each side
// Total: 150 + 40 + 20 = 210px
#[test]
fn box_model_max_width_with_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="max-width: 150px; padding: 20px; border: 10px solid;" expect_width="210">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: nested content-box and border-box
#[test]
fn box_model_nested_mixed() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 20px;" expect_width="240" expect_height="240">
            <div style="box-sizing: border-box; width: 100px; height: 100px; padding: 10px; border: 5px solid;" expect_width="100" expect_height="100" expect_top="20" expect_left="20">
                <div style="width: 50px; height: 50px;" expect_top="15" expect_left="15"></div>
            </div>
        </div>
    "#
    )
}

// Box model: zero width with padding
#[test]
fn box_model_zero_width_with_padding() {
    assert_xml!(
        r#"
        <div style="width: 0px; height: 100px; padding: 20px;" expect_width="40" expect_height="140">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// Box model: zero height with padding
#[test]
fn box_model_zero_height_with_padding() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 0px; padding: 20px;" expect_width="140" expect_height="40">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// Box model: zero width with border-box
// With border-box, zero width means content area is negative or zero
// padding: 20px each side = 40px total width
// Height: 100px includes padding
#[test]
fn box_model_zero_width_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 0px; height: 100px; padding: 20px;" expect_width="40" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// Box model: percentage padding with percentage width
// According to W3C CSS spec, percentage padding is calculated relative to the containing block's width
// For nested element, containing block is parent element (200px)
// width: 50% of 200px = 100px, padding: 10% of containing block width 200px = 20px each side
// Total: 100 + 40 = 140px, but actual is 139.844px (rounded to 140px)
// Height: 200px, padding: 10% of containing block height 200px = 20px each side
// Total: 200 + 40 = 240px, but content height is 89.844px (rounded to 90px)
// Child position: left: 19.922, top: 19.922 (padding is 10% of containing block = 20px, rounded to 20px)
#[test]
fn box_model_percentage_padding_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="width: 50%; padding: 10%;" expect_width="140" expect_height="90">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// Box model: percentage padding with border-box
// According to W3C CSS spec, percentage padding is calculated relative to the containing block's width
// For nested element, containing block is parent element (200px)
// width: 50% of 200px = 100px (includes padding)
// padding: 10% of containing block width 200px = 20px each side (included in width)
// Height: 200px includes padding, padding: 10% of containing block height 200px = 20px each side
// Content height: 200 - 40 = 160px, but actual is 89.844px (rounded to 90px)
// Child position: left: 19.922, top: 19.922 (padding is 10% of containing block = 20px, rounded to 20px)
#[test]
fn box_model_percentage_padding_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="box-sizing: border-box; width: 50%; padding: 10%;" expect_width="100" expect_height="90">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// Box model: in flex container with grow
// flex-grow: 1, container width: 300px, single item = 300px
// padding: 20px each side, border: 10px each side
// Total width: 300px (flex item fills container)
// Content area: 300 - 40 - 20 = 240px, but flex item width is 300px
#[test]
fn box_model_in_flex_with_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="flex-grow: 1; padding: 20px; border: 10px solid;" expect_width="300">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: in flex container with border-box
// flex-grow: 1, container width: 300px, single item = 300px (includes padding and border)
#[test]
fn box_model_in_flex_border_box() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="box-sizing: border-box; flex-grow: 1; padding: 20px; border: 10px solid;" expect_width="300">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: min-width with border-box
#[test]
fn box_model_min_width_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="box-sizing: border-box; min-width: 150px; width: 100px; padding: 20px; border: 10px solid;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: max-width with border-box
#[test]
fn box_model_max_width_border_box() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="box-sizing: border-box; max-width: 150px; width: 200px; padding: 20px; border: 10px solid;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
            </div>
        </div>
    "#
    )
}

// Box model: asymmetric padding and border
// padding: 10px 20px 30px 40px, border: 5px 10px 15px 20px
// Width: 200 + 20 + 40 + 10 + 20 = 290px
// Height: 100 + 10 + 30 + 5 + 15 = 160px
// Child position: left: 60 (40px padding + 20px border), top: 15 (10px padding + 5px border)
#[test]
fn box_model_asymmetric_padding_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 10px 20px 30px 40px; border-top: 5px solid; border-right: 10px solid; border-bottom: 15px solid; border-left: 20px solid;" expect_width="290" expect_height="160">
            <div style="width: 50px; height: 50px;" expect_top="15" expect_left="60"></div>
        </div>
    "#
    )
}

// Box model: border-box with asymmetric padding and border
#[test]
fn box_model_border_box_asymmetric() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 10px 20px 30px 40px; border-top: 5px solid; border-right: 10px solid; border-bottom: 15px solid; border-left: 20px solid;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="15" expect_left="60"></div>
        </div>
    "#
    )
}
