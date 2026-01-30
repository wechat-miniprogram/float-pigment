// Tests for `align-items` property in CSS Flexbox
// Based on CSS Flexible Box Layout Module Level 1:
// - align-items sets default alignment for flex items along cross axis
// - Values: flex-start, flex-end, center, baseline, stretch, start, end

use crate::*;

// Case: align-items: center
// Spec points:
// - Items centered along cross axis
// In this test:
// - Container: 100x100px
// - Item: 50x50px, centered at (50-50)/2 = 0 wait that's wrong
// - Actually: (100-50)/2 = 25 for top
#[test]
fn align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; height: 100px; width: 100px;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="25" expect_left="0"></div>
        </div>
    "#
    )
}

// Case: align-items: stretch (default)
// Spec points:
// - Items stretch to fill cross axis
// - Items without explicit height stretch to tallest item
// In this test:
// - Container: auto height, first item 80px tall
// - Second and third items stretch to 80px
#[test]
fn align_items_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; margin-top: 100px;">
          <div style="flex: 33; height: 80px;"></div>
          <div style="flex: 33;" expect_height="80" expect_width="125"></div>
          <div style="flex: 33;" expect_height="80" expect_width="125">
            <div style="width: 100%; height: 100%; expect_height="80" expect_width="125"></div>
          </div>
        </div>
    "#
    )
}

// Case: align-items: start
// Spec points:
// - Items aligned to start of cross axis
// In this test:
// - All items at top=0
#[test]
fn align_items_start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: start; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="0" ></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="0"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: align-items: flex-start
// Spec points:
// - Same as start for LTR
// In this test:
// - All items at top=0
#[test]
fn align_items_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: flex-start; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="0" ></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="0"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="0"></div>
        </div>
    "#
    )
}

// Case: align-items: center (multiple items)
// Spec points:
// - Each item individually centered
// In this test:
// - Container: 100px high
// - Item 1: 80px, top=(100-80)/2=10
// - Item 2: 60px, top=(100-60)/2=20
// - Item 3: 50px, top=(100-50)/2=25
#[test]
fn align_items_center() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="10"></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="20"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="25"></div>
        </div>
    "#
    )
}

// Case: align-items: end
// Spec points:
// - Items aligned to end of cross axis
// In this test:
// - Container: 100px high
// - Item 1: 80px, top=100-80=20
// - Item 2: 60px, top=100-60=40
// - Item 3: 50px, top=100-50=50
#[test]
fn align_items_end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: end; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="20"></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="40"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: align-items: flex-end
// Spec points:
// - Same as end for LTR
// In this test:
// - Same as align_items_end
#[test]
fn align_items_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: flex-end; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="20"></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="40"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="50"></div>
        </div>
    "#
    )
}

// Case: align-items: baseline (text baseline alignment)
// Spec points:
// - Items aligned by their text baseline
// In this test:
// - Text items at top=0 (baseline aligned)
// - Box without text offset to align its top with baseline
#[test]
fn align_items_baseline() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="0">xxx</div>
            <div style="height: 10px; width: 10px;" expect_top="6"></div>
            <div expect_top="0">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline (taller box)
// Spec points:
// - Taller element sets the baseline reference
// In this test:
// - 20px box determines baseline, text items offset
#[test]
fn align_items_baseline_1() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="4">xxx</div>
            <div style="height: 20px; width: 10px;" expect_top="0"></div>
            <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline with flex child
// Spec points:
// - Flex containers participate in baseline alignment
// In this test:
// - Flex child (20px) at top, text children offset
#[test]
fn align_items_baseline_2() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="4">xxx</div>
            <div style="display: flex; height: 20px; width: 10px;" expect_top="0"></div>
            <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline with inline-block
// Spec points:
// - Inline-block elements participate in baseline alignment
// In this test:
// - Same as baseline_2 but with inline-block
#[test]
fn align_items_baseline_3() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="4">xxx</div>
            <div style="display: inline-block; height: 20px; width: 10px;" expect_top="0"></div>
            <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline (smaller flex child)
// Spec points:
// - Smaller flex child offset to align baseline
// In this test:
// - 10px flex child offset to align with text
#[test]
fn align_items_baseline_4() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="0">xxx</div>
            <div style="display: flex; height: 10px; width: 10px;" expect_top="6"></div>
            <div expect_top="0">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline (smaller inline-block)
// Spec points:
// - Smaller inline-block offset to align baseline
#[test]
fn align_items_baseline_5() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="0">xxx</div>
            <div style="display: inline-block; height: 10px; width: 10px;" expect_top="6"></div>
            <div expect_top="0">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline with margin-top
// Spec points:
// - Margin affects baseline positioning
// In this test:
// - All items with margin-top=10, aligned at top=10
#[test]
fn align_items_baseline_margin_top() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="10">xxx</div>
            <div style="margin-top: 10px; height: 10px; width: 10px;" expect_top="10">xxx</div>
            <div expect_top="10">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: baseline with varying margin-top
// Spec points:
// - Largest margin-top shifts all baselines
// In this test:
// - First item margin-top=20 (largest)
// - All items align at baseline at top=20
#[test]
fn align_items_baseline_max_margin_top() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div style="margin-top: 20px;"expect_top="20">xxx</div>
            <div style="margin-top: 10px; height: 10px; width: 10px;" expect_top="20">xxx</div>
            <div expect_top="20">xxx</div>
        </div>
    "#
    )
}

// Case: align-items: center with min-height
// Spec points:
// - min-height expands container, centering uses expanded size
// In this test:
// - Container: min-height=60px, height=10px (resolved to 60px)
// - Items centered: (60-10)/2=25, (60-20)/2=20
#[test]
fn align_items_center_with_min_height() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; min-height: 60px; height: 10px;" expect_height="60">
            <div style="height: 10px; width: 10px;" expect_top="25"></div>
            <div style="height: 20px; width: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: align-items: center with max-height
// Spec points:
// - max-height clamps container, centering uses clamped size
// In this test:
// - Container: max-height=60px, height=100px (clamped to 60px)
// - Items centered: (60-10)/2=25, (60-20)/2=20
#[test]
fn align_items_center_with_max_height() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; max-height: 60px; height: 100px;" expect_height="60">
            <div style="height: 10px; width: 10px;" expect_top="25"></div>
            <div style="height: 20px; width: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: align-items: center with min-width in column direction
// Spec points:
// - In column flex, cross axis is horizontal
// - min-width affects container width for centering
// In this test:
// - Container: min-width=100px
// - Items centered: (100-10)/2=45, (100-20)/2=40
#[test]
fn align_items_center_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 10px;">
            <div style="display: flex; flex-direction: column; align-items: center; height: 100px; min-width: 100px;" expect_width="100">
                <div style="height: 10px; width: 10px;" expect_left="45"></div>
                <div style="height: 20px; width: 20px;" expect_left="40"></div>
            </div>
        </div>
    "#
    )
}

// Case: align-items: center with max-width
// Spec points:
// - max-width clamps container width
// In this test:
// - Container: max-width=100px, stretches to 100px then clamped
// - Items centered horizontally
#[test]
fn align_items_center_with_max_width() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; align-items: center; height: 100px; max-width: 100px;" expect_width="100">
            <div style="height: 10px; width: 10px;" expect_left="45"></div>
            <div style="height: 20px; width: 20px;" expect_left="40"></div>
        </div>
    "#
    )
}

// Case: align-items: center with both min and max height
// Spec points:
// - min-height takes precedence over height < min
// In this test:
// - height=30 < min-height=60, uses 60px
#[test]
fn align_items_center_with_min_max_limit() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; min-height: 60px; height: 30px; max-height: 300px;" expect_height="60">
            <div style="height: 10px; width: 10px;" expect_top="25"></div>
            <div style="height: 20px; width: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

// Case: align-items: center with min/max width in column direction
// Spec points:
// - min-width > width, uses min-width
// In this test:
// - width=30 < min-width=100, uses 100px
#[test]
fn align_items_center_with_min_max_limit_2() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; align-items: center; height: 100px; min-width: 100px; width: 30px; max-width: 300px;" expect_width="100">
            <div style="height: 10px; width: 10px;" expect_left="45"></div>
            <div style="height: 20px; width: 20px;" expect_left="40"></div>
        </div>
    "#
    )
}

// Case: align-items: center with align-content: flex-start and min-height
// Spec points:
// - Single line flex uses align-items for cross-axis alignment
// - min-height expands container
// In this test:
// - Container: min-height=100px, items centered vertically
#[test]
fn align_items_center_with_align_content_flex_start_and_min_height() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; min-height: 100px; align-content: flex-start" expect_height="100" expect_top="0">
            <div style="display: flex; height: 10px; width: 10px;" expect_top="45"></div>
            <div style="display: flex; height: 20px; width: 20px;" expect_top="40"></div>
        </div>
    "#,
        true
    )
}

// Case: align-items: center with align-content: flex-start, min-height, and wrap
// Spec points:
// - With wrap, align-content affects line positioning
// - align-items centers items within their line
// In this test:
// - Items in single line, align-content packs to top
// - Items centered within that line
#[test]
fn align_items_center_with_align_content_flex_start_with_min_height_with_wrap() {
    assert_xml!(
        r#"
            <div style="display: flex; align-items: center; min-height: 100px; align-content: flex-start; flex-wrap: wrap" expect_height="100" expect_top="0">
                <div style="display: flex; height: 10px; width: 10px;" expect_top="5"></div>
                <div style="display: flex; height: 20px; width: 20px;" expect_top="0"></div>
            </div>
        "#,
        true
    )
}
