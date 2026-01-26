// WPT-based tests for padding property
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// padding: fixed value (all sides)
#[test]
fn padding_fixed_all() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 20px;" expect_width="240" expect_height="140">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// padding: asymmetric (top right bottom left)
#[test]
fn padding_asymmetric() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 10px 20px 30px 40px;" expect_width="260" expect_height="140">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="40"></div>
        </div>
    "#
    )
}

// padding: percentage
// According to W3C CSS spec, percentage padding is calculated relative to the containing block's width
// For root's direct child, containing block is root element (375px in test framework)
// Container: 200px, padding: 10% of containing block width 375px = 37.5px each side
// Total width: 200 + 37.5*2 = 275px (rounded to 275px)
// Total height: 200 + 37.5*2 = 275px (rounded to 275px)
// Child position: left: 37.354, top: 37.354 (padding is 10% of containing block width 375px = 37.5px, rounded to 37px)
#[test]
fn padding_percentage() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding: 10%;" expect_width="275" expect_height="275">
            <div style="width: 50px; height: 50px;" expect_top="37" expect_left="37"></div>
        </div>
    "#
    )
}

// padding-top: fixed
#[test]
fn padding_top() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding-top: 20px;" expect_width="200" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="0"></div>
        </div>
    "#
    )
}

// padding-right: fixed
#[test]
fn padding_right() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding-right: 20px;" expect_width="220" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// padding-bottom: fixed
#[test]
fn padding_bottom() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding-bottom: 20px;" expect_width="200" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// padding-left: fixed
#[test]
fn padding_left() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding-left: 20px;" expect_width="220" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="20"></div>
        </div>
    "#
    )
}

// padding: 0
#[test]
fn padding_zero() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 0;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// padding: with border-box
#[test]
fn padding_with_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 20px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// padding: with border
#[test]
fn padding_with_border() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 20px; border: 10px solid;" expect_width="260" expect_height="160">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// padding: percentage with border-box
// According to W3C CSS spec, percentage padding is calculated relative to the containing block's width
// With border-box, padding is included in the width
// For root's direct child, containing block is root element (375px in test framework)
// width: 200px includes padding, padding: 10% of containing block width 375px = 37.5px each side
#[test]
fn padding_percentage_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding: 10%;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="37" expect_left="37"></div>
        </div>
    "#
    )
}

// padding: nested elements
#[test]
fn padding_nested() {
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

// padding: in flex container
#[test]
fn padding_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="padding: 20px; height: 50px;" expect_width="60" expect_left="0">
                <div style="width: 20px; height: 10px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// padding: with min-width constraint
// min-width applies to content area, padding is added
// min-width: 150px, padding: 20px each side = 190px total
// But if parent width is 200px, child expands to fill
#[test]
fn padding_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="min-width: 150px; padding: 20px;" expect_width="200">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// padding: with max-width constraint
// max-width applies to content area, padding is added
// max-width: 150px, padding: 20px each side = 190px total
#[test]
fn padding_with_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="max-width: 150px; padding: 20px;" expect_width="190">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}
