// Tests for flex container intrinsic main sizes (§9.9.1)
// Based on CSS Flexible Box Layout Module Level 1:
// - §9.9.1.2: single-line min-content main size = sum of items' min-content contributions
// - §9.9.1.3: multi-line min-content main size = largest item's min-content contribution
//
// Trigger mechanism: an absolutely-positioned flex container with auto main size
// receives a definite max-content constraint (= containing block main size minus
// insets/margins) from special_positioned.rs, while its requested main size stays
// indefinite. When the sum of items' hypothetical main sizes exceeds that
// constraint, the `shrink_max_content` branch in flex_box.rs (§9.7) fires and
// floors the container at the sum of min-content contributions.

use crate::*;

// §9.9.1.2 — single-line flex container min-content main size.
// Spec: "For the min-content size of a single-line container, take the sum of
// the min-content contributions of all the non-collapsed flex items."
//
// Setup: an absolutely-positioned single-line (`flex-wrap: nowrap`) flex
// container with auto width. Its max-content constraint = containing block
// width (200px). Two items with `flex-shrink: 0` and definite widths whose
// sum (300px) exceeds the constraint, so the container cannot shrink below
// the sum of min-content contributions (300px).
#[test]
fn single_line_min_content_is_sum() {
    assert_xml!(
        r#"
        <div style="position: relative; width: 200px; height: 200px;">
            <div style="position: absolute; display: flex; flex-wrap: nowrap; height: 50px;">
                <div style="flex-shrink: 0; width: 150px; height: 50px;" expect_width="150" expect_height="50" expect_left="0"></div>
                <div style="flex-shrink: 0; width: 150px; height: 50px;" expect_width="150" expect_height="50" expect_left="150"></div>
            </div>
        </div>
    "#
    )
}

// §9.9.1.3 — multi-line flex container min-content main size.
// Spec: "For a multi-line container, the min-content main size is simply the
// largest min-content contribution of all the non-collapsed flex items in the
// flex container."
//
// Setup: an absolutely-positioned multi-line (`flex-wrap: wrap`) flex container
// with auto width. Its max-content constraint = containing block width (200px).
// Two items with `flex-shrink: 0` and definite widths (250px each) that exceed
// the constraint, so each item wraps onto its own line. Because the container
// is multi-line, its min-content main size is the largest single item's
// min-content (250px), NOT the sum (500px) — the container ends up at 250px
// (max(largest item, constraint) = max(250, 200) = 250). Contrast with the
// single-line case where the container would be the sum (500px).
#[test]
fn multi_line_min_content_is_largest_item() {
    assert_xml!(
        r#"
        <div style="position: relative; width: 200px; height: 300px;">
            <div style="position: absolute; display: flex; flex-wrap: wrap; height: 100px;">
                <div style="flex-shrink: 0; width: 250px; height: 50px;" expect_width="250" expect_height="50" expect_left="0" expect_top="0"></div>
                <div style="flex-shrink: 0; width: 250px; height: 50px;" expect_width="250" expect_height="50" expect_left="0" expect_top="50"></div>
            </div>
        </div>
    "#
    )
}

// §9.9.1.3 — multi-line min-content main size: largest item, not per-line sum.
//
// This test DISTINGUISHES the spec-correct container-level min-content
// (largest item across ALL lines) from the previous per-line `sum_of_min_content`
// (sum of items on each line). It uses a multi-line container where one line
// has MULTIPLE items (so the per-line sum > the largest single item), and the
// largest item exceeds the available (max-content) constraint.
//
// Setup: an absolutely-positioned multi-line (`flex-wrap: wrap`) flex container
// with auto width. Its max-content constraint = containing block width (100px).
// Three items with `flex-grow: 1; flex-shrink: 0`:
//   - Two 40px items (fit on line 1: 40+40=80 ≤ 100)
//   - One 150px item (line 2: 150 > 100, wraps alone)
//
// max-content size = 40+40+150 = 230 > 100 (available) → shrink branch fires.
// min-content size (multi-line, §9.9.1.3) = largest item = 150.
// container_main_inner = max(150, min(230, 100)) = max(150, 100) = 150.
//
// §9.7 line 1: target = 150, used_flex_factor = 80, growing = true (80 < 150).
//   free space = 150 - 80 = 70, distributed equally → each item = 40 + 35 = 75.
// §9.7 line 2: target = 150, used_flex_factor = 150, no grow/shrink → 150.
//
// Under the OLD per-line code, line 1's target would have been
// `max(per_line_min_content=80, available=100)` = 100, so line 1 items would
// grow to 50 each (free space 20, +10 each). The new container-level value
// (150) makes line 1 items grow to 75 each — the distinguishing assertion.
#[test]
fn multi_line_min_content_largest_not_sum() {
    assert_xml!(
        r#"
        <div style="position: relative; width: 100px; height: 300px;">
            <div style="position: absolute; display: flex; flex-wrap: wrap; height: 100px;">
                <div style="flex-grow: 1; flex-shrink: 0; width: 40px; height: 50px;" expect_width="75" expect_height="50" expect_left="0" expect_top="0"></div>
                <div style="flex-grow: 1; flex-shrink: 0; width: 40px; height: 50px;" expect_width="75" expect_height="50" expect_left="75" expect_top="0"></div>
                <div style="flex-grow: 1; flex-shrink: 0; width: 150px; height: 50px;" expect_width="150" expect_height="50" expect_left="0" expect_top="50"></div>
            </div>
        </div>
    "#
    )
}
