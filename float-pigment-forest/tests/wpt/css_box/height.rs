// WPT-based tests for height property
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// height: fixed value
#[test]
fn height_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_width="100" expect_height="200"></div>
    "#
    )
}

// height: percentage
#[test]
fn height_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;" expect_height="300">
            <div style="width: 50px; height: 50%;" expect_height="150"></div>
        </div>
    "#
    )
}

// height: auto (default)
#[test]
fn height_auto() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200">
            <div style="width: 50px; height: auto;" expect_height="50">
                <div style="width: 50px; height: 50px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

// height: 0
#[test]
fn height_zero() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 0px;" expect_width="100" expect_height="0"></div>
    "#
    )
}

// height: 100%
#[test]
fn height_100_percent() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_height="200">
            <div style="width: 50px; height: 100%;" expect_height="200"></div>
        </div>
    "#
    )
}

// height: with padding (content-box)
#[test]
fn height_with_padding_content_box() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px; padding: 20px;" expect_width="140" expect_height="240">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// height: with padding (border-box)
#[test]
fn height_with_padding_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 100px; height: 200px; padding: 20px;" expect_width="100" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
        </div>
    "#
    )
}

// height: with border (content-box)
#[test]
fn height_with_border_content_box() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px; border: 10px solid;" expect_width="120" expect_height="220">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// height: with border (border-box)
#[test]
fn height_with_border_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 100px; height: 200px; border: 10px solid;" expect_width="100" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// height: with min-height constraint
#[test]
fn height_with_min_height() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 100px; min-height: 150px;" expect_height="150"></div>
        </div>
    "#
    )
}

// height: with max-height constraint
#[test]
fn height_with_max_height() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 200px; max-height: 150px;" expect_height="150"></div>
        </div>
    "#
    )
}

// height: percentage with min-height
#[test]
fn height_percentage_with_min_height() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;">
            <div style="width: 50px; height: 50%; min-height: 150px;" expect_height="150"></div>
        </div>
    "#
    )
}

// height: percentage with max-height
#[test]
fn height_percentage_with_max_height() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 400px;">
            <div style="width: 50px; height: 50%; max-height: 150px;" expect_height="150"></div>
        </div>
    "#
    )
}

// height: in flex container (column direction)
#[test]
fn height_in_flex_container_column() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 300px;">
            <div style="width: 50px; height: 100px;" expect_height="100"></div>
            <div style="width: 50px; height: 100px;" expect_height="100"></div>
        </div>
    "#
    )
}

// height: auto in flex container (flex-grow)
#[test]
fn height_auto_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 300px;">
            <div style="width: 50px; height: auto; flex-grow: 1;" expect_height="150"></div>
            <div style="width: 50px; height: auto; flex-grow: 1;" expect_height="150"></div>
        </div>
    "#
    )
}

// height: auto with content
#[test]
fn height_auto_with_content() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: auto;" expect_height="50">
            <div style="width: 50px; height: 50px;" expect_height="50"></div>
        </div>
    "#
    )
}

// height: auto with multiple children
#[test]
fn height_auto_with_multiple_children() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: auto;" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_height="50"></div>
            <div style="width: 50px; height: 50px;" expect_height="50" expect_top="50"></div>
        </div>
    "#
    )
}
