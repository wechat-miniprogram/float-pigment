// WPT-based tests for width property
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// width: fixed value
#[test]
fn width_fixed() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;" expect_width="200" expect_height="100"></div>
    "#
    )
}

// width: percentage
#[test]
fn width_percentage() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="width: 50%; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// width: auto (default)
#[test]
fn width_auto() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;" expect_width="200">
            <div style="width: auto; height: 50px;" expect_width="200">
                <div style="width: 100px; height: 50px;" expect_width="100"></div>
            </div>
        </div>
    "#
    )
}

// width: 0
#[test]
fn width_zero() {
    assert_xml!(
        r#"
        <div style="width: 0px; height: 100px;" expect_width="0" expect_height="100"></div>
    "#
    )
}

// width: 100%
#[test]
fn width_100_percent() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;" expect_width="200">
            <div style="width: 100%; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// width: with padding (content-box)
#[test]
fn width_with_padding_content_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; padding: 20px;" expect_width="240" expect_height="140">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// width: with padding (border-box)
#[test]
fn width_with_padding_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 20px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// width: with border (content-box)
#[test]
fn width_with_border_content_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border: 10px solid;" expect_width="220" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// width: with border (border-box)
#[test]
fn width_with_border_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; border: 10px solid;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// width: with min-width constraint
#[test]
fn width_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="width: 100px; min-width: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// width: with max-width constraint
#[test]
fn width_with_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 200px; max-width: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// width: percentage with min-width
#[test]
fn width_percentage_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="width: 50%; min-width: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// width: percentage with max-width
#[test]
fn width_percentage_with_max_width() {
    assert_xml!(
        r#"
        <div style="width: 400px; height: 100px;">
            <div style="width: 50%; max-width: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// width: in flex container
#[test]
fn width_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="width: 100px; height: 50px;" expect_width="100"></div>
            <div style="width: 100px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// width: auto in flex container (flex-grow)
#[test]
fn width_auto_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="width: auto; flex-grow: 1; height: 50px;" expect_width="150"></div>
            <div style="width: auto; flex-grow: 1; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}
