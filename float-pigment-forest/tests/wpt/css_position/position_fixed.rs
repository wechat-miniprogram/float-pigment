// WPT-style tests for `position: fixed`
// Inspired by WPT CSS Position tests:
// - fixed-position elements are positioned relative to the viewport (initial containing block)
// - they do not move when the document is scrolled and are taken out of normal flow

use crate::*;

// Case: `position: fixed` with margin
// Spec / engine behavior:
// - fixed-position box is offset from the viewport by margin-left/margin-top when no explicit left/top is given
// In this test:
// - margin-left:100, margin-top:100 → expect_left=100, expect_top=100
// - content "XX" is measured by the inline text model as 32x16
#[test]
fn position_fixed_with_margin() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; margin-left: 100px; margin-top: 100px;" expect_left="100" expect_top="100" expect_width="32" expect_height="16">XX</div>
        </div>
    "#
    )
}

// Case: `position: fixed` with left/right
// Spec meaning:
// - left/right specify insets from the viewport edges
// - width is determined by available space or content when not specified
// In this test:
// - left=100, right=100 in a 375px wide viewport
// - used width = 375 - 100 - 100 = 175, height=16 from measured text "hello"
#[test]
fn position_fixed_left_right() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; left: 100px; right: 100px;" expect_left="100" expect_width="175" expect_height="16">hello</div>
        </div>
    "#
    )
}

// Case: `position: fixed` with top/bottom
// Spec meaning:
// - top/bottom specify insets from viewport top/bottom
// - used height = viewport_height - top - bottom
// In this test:
// - first: top=0, bottom=0 → height=750
// - second: top=100, bottom=100 → height=750 - 200 = 550
// - width spans full viewport width 375, and left=0 in both cases
#[test]
fn position_fixed_top_bottom() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; top: 0px; bottom: 0px; left: 0px; right: 0px;" expect_top="0" expect_left="0" expect_width="375" expect_height="750">hello</div>
            <div style="position: fixed; top: 100px; bottom: 100px; left: 0px; right: 0px;" expect_top="100" expect_left="0" expect_width="375" expect_height="550">hello</div>
        </div>
    "#
    );
}

// Case: `position: fixed` inside a flex container (alignment)
// Spec / engine behavior:
// - fixed-position elements ignore flex layout and are positioned relative to the viewport
// - here the flex container is centered with width=300, so a 100px wide fixed box is centered at left=100
// We assert the fixed box at top=0, left=100 with size 100x100
#[test]
fn position_fixed_in_flex_align() {
    assert_xml!(
        r#"
        <div>
            <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: center;">
                <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0" expect_left="100" expect_height="100" expect_width="100"></div>
            </div>
        </div>
    "#
    )
}

// Case: `position: fixed` with percentage top
// Spec meaning:
// - top:50% is resolved against the viewport height
// In this test:
// - viewport height=750 → top=0.5 * 750 = 375
#[test]
fn position_fixed_percentage_top() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; width: 100px; height: 100px; top: 50%;" expect_top="375"></div>
        </div>
    "#
    )
}

// Case: `position: fixed` with left/right and margin
// Spec / engine behavior:
// - when both left and margin-left are given, this engine adds them to compute the final offset
// - right defines the opposite inset but width is determined from left/right/margins
// In this test:
// - left=25, margin-left=25 → expect_left=50
// - right=50 implies width 375 - 50 (right) - 50 (left) = 275
#[test]
fn position_fixed_left_right_margin() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; left: 25px; margin-left: 25px; right: 50px; height: 50px;" expect_left="50" expect_width="275" expect_height="50" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: `position: fixed` with specified width
// Spec meaning:
// - explicit width overrides left/right-based intrinsic width computation
// - margins shift the fixed box relative to the viewport
// In this test:
// - width=100, margin-left=100 → expect_left=100, width=100, height=100
#[test]
fn position_fixed_with_width() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; width: 100px; height: 100px; margin-left: 100px; margin-right: 100px;" expect_top="0" expect_left="100" expect_height="100" expect_width="100"></div>
        </div>
    "#
    )
}

// Case: `position: fixed` with top/bottom only (no horizontal insets)
// Spec / engine behavior:
// - top/bottom define the vertical insets, so height = 750 - (top+bottom)
// - width is determined purely by inline text content in this engine
// In this test:
// - top=100, bottom=100 → height=550
// - text "hello" width is measured as 80px by the text engine
#[test]
fn position_fixed_top_bottom_only() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; top: 100px; bottom: 100px;" expect_top="100" expect_height="550" expect_width="80">hello</div>
        </div>
    "#
    )
}

// Case: complex `position: fixed` with left/right and margin, no content
// Engine behavior:
// - left/right and margins combine to produce the final horizontal insets
// - in this engine, with no explicit height or content, the fixed box height is 0
// In this test:
// - left=30, margin-left=10 → final left=40
// - width = viewport_width - left - right - margin_right = 295
#[test]
fn position_fixed_complex() {
    assert_xml!(
        r#"
        <div>
            <div style="position: fixed; left: 30px; right: 30px; margin-left: 10px; margin-right: 10px;" expect_top="0" expect_left="40" expect_height="0" expect_width="295"></div>
        </div>
    "#
    )
}
