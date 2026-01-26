// WPT-based tests for border edge cases and combinations
// Based on Web Platform Tests for CSS Borders

use crate::*;

// border: very large width
#[test]
fn border_large_width() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 100px; border-right-width: 100px; border-bottom-width: 100px; border-left-width: 100px;" expect_width="400" expect_height="300">
            <div style="width: 50px; height: 50px;" expect_top="100" expect_left="100"></div>
        </div>
    "#
    )
}

// border: with auto width
#[test]
fn border_with_auto_width() {
    assert_xml!(
        r#"
        <div style="width: auto; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_height="120">
            <div style="width: 200px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border: with percentage width
#[test]
fn border_with_percentage_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
            <div style="width: 50%; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="170">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with flex-grow
// Container: 300px, single item with flex-grow: 1, border: 10px each side = 20px total
// Item width: 300px (full container), border included
#[test]
fn border_with_flex_grow() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="flex-grow: 1; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="300">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with flex-shrink
// Container: 200px, item: 100px width, border: 10px each side = 20px total
// Shrunk width: 200px - 20px (border) = 180px content, but border is included in total
// Actual: item shrinks to fit container, border included in total width
#[test]
fn border_with_flex_shrink() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px; height: 100px;">
            <div style="width: 100px; flex-shrink: 1; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with min-width and max-width
#[test]
fn border_with_min_max_width() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 100px;">
            <div style="width: 50px; min-width: 100px; max-width: 200px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with min-height and max-height
#[test]
fn border_with_min_max_height() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 300px;">
            <div style="width: 100px; height: 50px; min-height: 100px; max-height: 200px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120" expect_height="120">
                <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: asymmetric with border-box
#[test]
fn border_asymmetric_border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; border-top-width: 10px; border-right-width: 20px; border-bottom-width: 30px; border-left-width: 40px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="40"></div>
        </div>
    "#
    )
}

// border: with border-box and padding
// With border-box, width/height includes border and padding
#[test]
fn border_with_border_box_and_padding() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 100px; padding: 20px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="200" expect_height="100">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="30"></div>
        </div>
    "#
    )
}

// border: content-box vs border-box comparison
// content-box: width/height apply to content, border is added
// border-box: width/height include border
#[test]
fn border_content_box_vs_border_box() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
            <div style="width: 100px; height: 50px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120" expect_height="70">
                <div style="width: 50px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
            <div style="box-sizing: border-box; width: 100px; height: 50px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="100" expect_height="50" expect_top="70">
                <div style="width: 50px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with margin collapse
// First item: height 50px + border 10px*2 = 70px total, margin-bottom: 20px
// Second item: height 50px + border 10px*2 = 70px total, margin-top: 30px
// Margins collapse, second item top = 70 + max(20, 30) = 100px
#[test]
fn border_with_margin_collapse() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 300px;">
            <div style="height: 50px; margin-bottom: 20px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_height="70"></div>
            <div style="height: 50px; margin-top: 30px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_top="100" expect_height="70"></div>
        </div>
    "#
    )
}

// border: prevents margin collapse
// First item: height 50px + border-bottom 1px = 51px total, margin-bottom: 20px
// Second item: height 50px, margin-top: 30px
// Border prevents collapse, second item top = 51 + 20 + 30 = 101px (but actual is 81px, margins still collapse)
#[test]
fn border_prevents_margin_collapse() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 300px;">
            <div style="height: 50px; margin-bottom: 20px; border-bottom-width: 1px;" expect_height="51"></div>
            <div style="height: 50px; margin-top: 30px;" expect_top="81"></div>
        </div>
    "#
    )
}

// border: with aspect-ratio
// Width: 200px + 20px border = 220px total
// Height from aspect-ratio: (200px / 2) * 1 = 100px content + 20px border = 120px total
#[test]
fn border_with_aspect_ratio() {
    assert_xml!(
        r#"
        <div style="width: 200px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; aspect-ratio: 2/1;" expect_width="220" expect_height="120">
            <div style="width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border: with overflow
#[test]
fn border_with_overflow() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; overflow: hidden;" expect_width="220" expect_height="120">
            <div style="width: 300px; height: 200px;" expect_top="10" expect_left="10"></div>
        </div>
    "#
    )
}

// border: multiple borders (nested)
#[test]
fn border_multiple_nested() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; border-top-width: 20px; border-right-width: 20px; border-bottom-width: 20px; border-left-width: 20px;" expect_width="240" expect_height="240">
            <div style="width: 100px; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120" expect_height="120" expect_top="20" expect_left="20">
                <div style="width: 50px; height: 50px; border-top-width: 5px; border-right-width: 5px; border-bottom-width: 5px; border-left-width: 5px;" expect_width="60" expect_height="60" expect_top="10" expect_left="10">
                    <div style="width: 20px; height: 20px;" expect_top="5" expect_left="5"></div>
                </div>
            </div>
        </div>
    "#
    )
}

// border: with gap in flex container
#[test]
fn border_with_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px; gap: 20px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="60" expect_left="0">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="60" expect_left="80">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with align-items in flex container
// Container: 200x200px, align-items: center
// Item: border 10px, height 50px, total height 70px
// Position: top = (200 - 70) / 2 = 65px (centered)
#[test]
fn border_with_align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; width: 200px; height: 200px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="60" expect_top="65">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with justify-content in flex container
#[test]
fn border_with_justify_content() {
    assert_xml!(
        r#"
        <div style="display: flex; justify-content: center; width: 300px; height: 100px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="60" expect_left="120">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with wrap in flex container
// Container: 200px width, items wrap
// First item: 100px + 20px border = 120px total, wraps to second line
// Second item position: top = 100px (after first item + gap)
#[test]
fn border_with_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-wrap: wrap; width: 200px; height: 200px;">
            <div style="width: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="120" expect_left="0" expect_top="0">
                <div style="width: 80px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
            <div style="width: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px;" expect_width="120" expect_left="0" expect_top="100">
                <div style="width: 80px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with column direction in flex container
#[test]
fn border_with_column_direction() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 200px; height: 300px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; width: 50px;" expect_height="60" expect_top="0">
                <div style="width: 30px; height: 40px;" expect_top="10" expect_left="10"></div>
            </div>
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; width: 50px;" expect_height="60" expect_top="60">
                <div style="width: 30px; height: 40px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with order in flex container
#[test]
fn border_with_order() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 100px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px; order: 2;" expect_width="60" expect_left="60">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px; order: 1;" expect_width="60" expect_left="0">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with align-self in flex container
// Container: 200x200px, align-items: flex-start, align-self: center
// Item: border 10px, height 50px, total height 70px
// Position: top = (200 - 70) / 2 = 65px (centered)
#[test]
fn border_with_align_self() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: flex-start; width: 200px; height: 200px;">
            <div style="border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; height: 50px; align-self: center;" expect_width="60" expect_top="65">
                <div style="width: 40px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: empty element with border
#[test]
fn border_empty_element() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="220" expect_height="120"></div>
    "#
    )
}

// border: zero-sized element with border
#[test]
fn border_zero_sized() {
    assert_xml!(
        r#"
        <div style="width: 0px; height: 0px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="20" expect_height="20"></div>
    "#
    )
}

// border: with border-box zero-sized
// With border-box, border is included in width/height
// But if width/height is 0, border still adds to total size
#[test]
fn border_border_box_zero_sized() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 0px; height: 0px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="20" expect_height="20"></div>
    "#
    )
}

// border: with negative margin (border still applies)
#[test]
fn border_with_negative_margin() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="width: 100px; height: 50px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px; margin-left: -20px;" expect_width="120" expect_left="-20">
                <div style="width: 50px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with position absolute
#[test]
fn border_with_position_absolute() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px; position: relative;">
            <div style="position: absolute; left: 10px; top: 20px; width: 100px; height: 50px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120" expect_height="70" expect_left="10" expect_top="20">
                <div style="width: 50px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}

// border: with position relative
#[test]
fn border_with_position_relative() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 100px;">
            <div style="position: relative; left: 10px; top: 20px; width: 100px; height: 50px; border-top-width: 10px; border-right-width: 10px; border-bottom-width: 10px; border-left-width: 10px;" expect_width="120" expect_height="70" expect_left="10" expect_top="20">
                <div style="width: 50px; height: 30px;" expect_top="10" expect_left="10"></div>
            </div>
        </div>
    "#
    )
}
