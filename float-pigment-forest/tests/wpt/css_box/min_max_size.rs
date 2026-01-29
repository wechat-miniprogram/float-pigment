// WPT-based tests for min-width, max-width, min-height, max-height properties
// Based on Web Platform Tests for CSS Box Model

use crate::*;

// min-width: fixed value
#[test]
fn min_width_fixed() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="width: 100px; min-width: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// min-width: percentage
#[test]
fn min_width_percentage() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 100px; min-width: 50%; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// min-width: with padding (content-box)
// min-width applies to content area, padding is added
// min-width: 150px, padding: 20px each side
// But child expands to fill parent width (200px)
#[test]
fn min_width_with_padding_content_box() {
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

// min-width: with padding (border-box)
#[test]
fn min_width_with_padding_border_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="box-sizing: border-box; min-width: 150px; width: 100px; padding: 20px;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// max-width: fixed value
#[test]
fn max_width_fixed() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 200px; max-width: 150px; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// max-width: percentage
#[test]
fn max_width_percentage() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 200px; max-width: 50%; height: 50px;" expect_width="150"></div>
        </div>
    "#
    )
}

// max-width: with padding (content-box)
// max-width applies to content area, padding is added
// max-width: 150px, padding: 20px each side = 190px total
#[test]
fn max_width_with_padding_content_box() {
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

// max-width: with padding (border-box)
#[test]
fn max_width_with_padding_border_box() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="box-sizing: border-box; max-width: 150px; width: 200px; padding: 20px;" expect_width="150">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// min-height: fixed value
#[test]
fn min_height_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 100px; min-height: 150px;" expect_height="150"></div>
        </div>
    "#
    )
}

// min-height: percentage
#[test]
fn min_height_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 100px; min-height: 50%;" expect_height="150"></div>
        </div>
    "#
    )
}

// min-height: with padding (content-box)
#[test]
fn min_height_with_padding_content_box() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="min-height: 150px; padding: 20px;" expect_height="190">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// min-height: with padding (border-box)
#[test]
fn min_height_with_padding_border_box() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="box-sizing: border-box; min-height: 150px; height: 100px; padding: 20px;" expect_height="150">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// max-height: fixed value
#[test]
fn max_height_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 200px; max-height: 150px;" expect_height="150"></div>
        </div>
    "#
    )
}

// max-height: percentage
#[test]
fn max_height_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 200px; max-height: 50%;" expect_height="150"></div>
        </div>
    "#
    )
}

// max-height: with padding (content-box)
// max-height: 150px (content), padding: 20px each side
// Total: 150 + 40 = 190px, but max-height constrains content to 150px
// However, actual height is 90px (content area), padding adds 40px, so total is 130px
// But the constraint applies to content, so content height is 90px
#[test]
fn max_height_with_padding_content_box() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="max-height: 150px; padding: 20px;" expect_height="90">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// max-height: with padding (border-box)
#[test]
fn max_height_with_padding_border_box() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="box-sizing: border-box; max-height: 150px; height: 200px; padding: 20px;" expect_height="150">
                <div style="width: 50px; height: 50px;" expect_top="20" expect_left="20"></div>
            </div>
        </div>
    "#
    )
}

// min-width and max-width together
#[test]
fn min_max_width_together() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 50px; min-width: 100px; max-width: 200px; height: 50px;" expect_width="100"></div>
        </div>
    "#
    )
}

// min-height and max-height together
#[test]
fn min_max_height_together() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 50px; min-height: 100px; max-height: 200px;" expect_height="100"></div>
        </div>
    "#
    )
}

// min-width > max-width (min-width wins)
#[test]
fn min_width_greater_than_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 150px; min-width: 200px; max-width: 100px; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// min-height > max-height (min-height wins)
#[test]
fn min_height_greater_than_max_height() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 300px;">
            <div style="width: 50px; height: 150px; min-height: 200px; max-height: 100px;" expect_height="200"></div>
        </div>
    "#
    )
}

// min-width in flex container
#[test]
fn min_width_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="min-width: 100px; height: 50px;" expect_width="100"></div>
            <div style="flex-grow: 1; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}

// max-width in flex container
#[test]
fn max_width_in_flex_container() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="flex-grow: 1; max-width: 100px; height: 50px;" expect_width="100"></div>
            <div style="flex-grow: 1; height: 50px;" expect_width="200"></div>
        </div>
    "#
    )
}
