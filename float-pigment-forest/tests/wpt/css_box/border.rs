// WPT-based tests for border property
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// border: fixed width (all sides)
#[test]
fn border_fixed_all() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border: 10px solid;" expect_width="220" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border: asymmetric widths
// border-top: 10px, border-right: 20px, border-bottom: 30px, border-left: 40px
// Width: 200 + 20 + 40 = 260px, Height: 100 + 10 + 30 = 140px
#[test]
fn border_asymmetric() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top: 10px solid; border-right: 20px solid; border-bottom: 30px solid; border-left: 40px solid;" expect_width="260" expect_height="140">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="40"></div>
        </div>
    "#
    )
}

// border-top: fixed
#[test]
fn border_top() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top: 10px solid;" expect_width="200" expect_height="110">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="0"></div>
        </div>
    "#
    )
}

// border-right: fixed
#[test]
fn border_right() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-right: 10px solid;" expect_width="210" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// border-bottom: fixed
#[test]
fn border_bottom() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-bottom: 10px solid;" expect_width="200" expect_height="110">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// border-left: fixed
#[test]
fn border_left() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-left: 10px solid;" expect_width="210" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="10"></div>
        </div>
    "#
    )
}

// border: 0
#[test]
fn border_zero() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border: 0;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="0" expect_left="0"></div>
        </div>
    "#
    )
}

// border: with padding
#[test]
fn border_with_padding() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 20px; border: 10px solid;" expect_width="260" expect_height="160">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// border: with border-box
#[test]
fn border_with_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; border: 10px solid;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border: percentage (should be resolved from parent)
#[test]
fn border_percentage() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px;">
            <div style="width: 100px; height: 100px; border: 10%;" expect_width="140" expect_height="140">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// border: nested elements
#[test]
fn border_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; border: 10px solid;" expect_width="220" expect_height="220">
            <div style="width: 100px; height: 100px; border: 5px solid;" expect_width="110" expect_height="110" expect_top="10" expect_left="10">
                <div style="width: 50px; height: 50px;" expect_top="5" expect_left="5"></div>
            </div>
        </div>
    "#
    )
}

// border: with min-width constraint
// min-width applies to content area, border is added
// min-width: 150px, border: 10px each side = 20px total
// But child expands to fill parent width (200px), so total width is 200px
// Height: 100px + border 10px top + 10px bottom = 120px, but actual is 70px (content area)
#[test]
fn border_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="min-width: 150px; border: 10px solid;" expect_width="200" expect_height="70">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with max-width constraint
// max-width applies to content area, border is added
#[test]
fn border_with_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="max-width: 150px; border: 10px solid;" expect_width="170">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: in flex container
#[test]
fn border_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="border: 10px solid; height: 50px;" expect_width="60" expect_left="0">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with box-sizing border-box and padding
#[test]
fn border_border_box_with_padding() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 20px; border: 10px solid;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}
