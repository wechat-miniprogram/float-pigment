// Tests for `position` property in CSS
// Based on CSS Positioned Layout Module Level 3:
// - position: relative - offsets from normal flow position
// - position: absolute - removed from flow, positioned relative to containing block
// - position: fixed - positioned relative to viewport
// - Positioned elements use top/right/bottom/left for placement

use crate::*;

// Case: relative positioning with left/top
// Spec points:
// - Relative positioning offsets element from normal position
// - Element still occupies original space in flow
// In this test:
// - Two siblings with position=relative, left=10, top=10
// - First at (10, 10), second at (10, 60) - maintaining flow
#[test]
fn relative_left_top_fixed() {
    assert_xml!(
        r#"
          <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="10"></div>
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="60"></div>
          </div>
      "#
    )
}

// Case: absolute positioning with fixed size
// Spec points:
// - Absolute elements are removed from normal flow
// - Parent height doesn't include absolute children
// In this test:
// - Absolute child doesn't contribute to parent height
// - Parent expect_height=0
#[test]
fn absolute_size_fixed() {
    assert_xml!(
        r#"
          <div expect_height="0">
            <div style="position: absolute; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="10"></div>
          </div>
      "#
    )
}

// Case: absolute positioning with percentage size
// Spec points:
// - Percentage width/height relative to containing block
// In this test:
// - Parent: 100x200px
// - Child: 10% width = 10px, 10% height = 20px
#[test]
fn absolute_size_percentage() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10%; width: 10%; left: 10px; top: 10px;" expect_height="20" expect_width="10" expect_left="10" expect_top="10"></div>
          </div>
      "#
    )
}

// Case: absolute with fixed left/top
// Spec points:
// - left/top position absolute element within containing block
// In this test:
// - Child at left=10, top=10
#[test]
fn absolute_left_top_fixed() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; left: 10px; top: 10px; " expect_left="10" expect_top="10"></div>
          </div>
      "#
    )
}

// Case: absolute with right/bottom
// Spec points:
// - right/bottom position from opposite edges
// In this test:
// - Parent: 100x200px
// - Child: 10x10px, right=10, bottom=10
// - Expected: left = 100-10-10 = 80, top = 200-10-10 = 180
#[test]
fn absolute_right_bottom_fixed() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; right: 10px; bottom: 10px; " expect_left="80" expect_top="180"></div>
          </div>
      "#
    )
}

// Case: absolute with percentage left/top
// Spec points:
// - Percentage offsets are relative to containing block
// In this test:
// - Parent: 100x200px
// - Child: left=50% = 50px, top=50% = 100px
#[test]
fn absolute_left_top_percentage() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; left: 50%; top: 50%;" expect_left="50" expect_top="100"></div>
          </div>
      "#
    )
}

// Case: absolute with percentage right/bottom
// Spec points:
// - Percentage right/bottom calculated from edges
// In this test:
// - Parent: 100x200px
// - Child: 10x10px, right=50% = 50px, bottom=50% = 100px
// - Expected: left = 100-10-50 = 40, top = 200-10-100 = 90
#[test]
fn absolute_right_bottom_percentage() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; right: 50%; bottom: 50%;" expect_left="40" expect_top="90"></div>
          </div>
      "#
    )
}

// Case: absolute with all edges = 0
// Spec points:
// - Setting all edges to 0 stretches element to fill container
// In this test:
// - Child stretches to 100x200px (same as parent)
#[test]
fn absolute_zero() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="height: 10px;"></div>
            <div style="position: absolute; left: 0; top: 0; bottom: 0; right: 0;" expect_width="100" expect_height="200" expect_top="0"></div>
          </div>
      "#
    )
}

// Case: absolute element in flex container
// Spec points:
// - Absolute elements don't participate in flex layout
// - Other flex children ignore absolute sibling
// In this test:
// - Absolute child removed from flow
// - Two normal children each get 50px width
#[test]
fn absolute_in_flex() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 100px; display: flex;">
            <div style="flex-grow: 1;" expect_width="50" expect_left="0"></div>
            <div style="flex-grow: 1; position: absolute; left: 20px; top: 20px; width: 10px; height: 10px;" expect_top="20" expect_left="20"></div>
            <div style="flex-grow: 1;" expect_width="50" expect_left="50"></div>
          </div>
      "#
    )
}

// Case: fixed positioning with margin
// Spec points:
// - Fixed elements are positioned relative to viewport
// - Margins offset from the positioned location
// In this test:
// - Fixed element with margin-left=100, margin-top=100
// - Positioned at (100, 100)
#[test]
fn fixed_with_margin() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; margin-left: 100px; margin-top: 100px" expect_left="100" expect_top="100" expect_width="32" expect_height="16">XX</div>
        </div>
    "#
    )
}

// Case: fixed with specified left/right
// Spec points:
// - When both left and right are specified, width is computed
// - Width = viewport width - left - right
// In this test:
// - left=100, right=100, viewport=375
// - Width = 375 - 100 - 100 = 175px
#[test]
fn fixed_with_specified_left_right() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; left: 100px; right: 100px;" expect_left="100" expect_width="175" expect_height="16">hello</div>
        </div>
    "#
    )
}

// Case: fixed with left/right and margin
// Spec points:
// - Margins are applied after positioning
// In this test:
// - left=25, margin-left=25, right=50
// - Final left = 25 + 25 = 50
// - Width = 375 - 50 - 50 = 275px
#[test]
fn fixed_with_specified_left_right_and_margin() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; left: 25px; margin-left: 25px; right: 50px; height: 50px;" expect_left="100" expect_width="275" expect_height="50" expect_top="0" expect_left="50"></div>
        </div>
    "#
    )
}

// Case: fixed with top/bottom
// Spec points:
// - Height computed when both top and bottom specified
// - Height = viewport height - top - bottom
// In this test:
// - top=100, bottom=100, viewport height=750
// - Height = 750 - 100 - 100 = 550px
#[test]
fn fixed_with_specified_top_bottom() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; top: 100px; bottom: 100px;" expect_top="100" expect_height=550" expect_width="0">hello</div>
        </div>
    "#
    )
}

// Case: fixed with explicit width
// Spec points:
// - Explicit width takes precedence
// - Margins don't affect computed width
// In this test:
// - Fixed element with width=100, positioned with margin
#[test]
fn fixed_with_specified_width() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; width: 100px; height: 100px; margin-left: 100px; margin-right: 100px;" expect_top="0"  expect_left="100" expect_height=16" expect_width="100"></div>
        </div>
    "#
    )
}

// Case: fixed with percentage top
// Spec points:
// - Percentage top is relative to viewport height
// In this test:
// - top=50% of 750px = 375px
#[test]
fn fixed_with_percentage_top() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; width: 100px; height: 100px; top: 50%" expect_top="375"></div>
        </div>
    "#
    )
}

// Case: fixed in flex container with align-items
// Spec points:
// - Fixed elements respect parent's align-items for initial position
// - Then fixed positioning takes over
// In this test:
// - Various flex containers with different align-items
// - Fixed children positioned based on alignment then fixed constraints
#[test]
fn fixed_in_flex_container() {
    assert_xml!(
        r#"
        <div>
          <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: center">
            <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0"  expect_left="100" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: flex-start">
            <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: flex-end">
            <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0"  expect_left="200" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; width: 300px; height: 300px; align-items: flex-start">
            <div style="position: fixed; width: 100px; height: 100px;" expect_top="0"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; width: 300px; height: 300px; align-items: flex-end">
            <div style="position: fixed; width: 100px; height: 100px;" expect_top="200"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; width: 300px; height: 300px; align-items: center">
            <div style="position: fixed; width: 100px; height: 100px;" expect_top="100"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
        </div>
       
    "#
    )
}

// Case: fixed with all edges specified
// Spec points:
// - Element stretches to fill specified area
// In this test:
// - First: all 0, fills viewport (375x750)
// - Second: inset 100px on top/bottom, fills 375x550
#[test]
fn fixed_with_specified_top_bottom_left_right() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; top: 0px; bottom: 0px; left: 0px; right: 0px;" expect_top="0" expect_left="0" expect_width="375" expect_height="750">hello</div>
          <div style="position: fixed; top: 100px; bottom: 100px; left: 0px; right: 0px;" expect_top="100" expect_left="0" expect_width="375" expect_height="550">hello</div>
        </div>
    "#
    );
}

// Case: fixed with complex margin and positioning
// Spec points:
// - Margins combined with left/right constraints
// In this test:
// - left=30, right=30, margin-left=10, margin-right=10
// - Final left = 30 + 10 = 40
// - Width = 375 - 30 - 30 - 10 - 10 = 295px
#[test]
fn fixed_complex() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; left: 30px; right: 30px; margin-left: 10px; margin-right: 10px;" expect_top="0"  expect_left="40" expect_height=16" expect_width="295"></div>
        </div>
    "#
    )
}

// Case: absolute in flex with align-items
// Spec points:
// - Absolute elements in flex respect cross-axis alignment
// - height: 100% relative to containing block's content box
// In this test:
// - Flex container with border, height=100% resolves to content height
#[test]
fn absolute_flex_align_items() {
    assert_xml!(
        r#"
          <div style="display: flex; box-sizing: border-box; width: 100%;height: 40px; border: 2px solid #07c160; align-items: center;">
            <div style="position: absolute; right: 0; width: 60px; height: 100%; align-items: center;" expect_top="2" expect_height="36"></div>
          </div>
      "#
    )
}

// Case: absolute in border-box element
// Spec points:
// - Absolute percentage dimensions relative to padding box
// In this test:
// - Parent: border-box, 200x100, border=2px
// - Child: height=100% = 96px (100 - 4px border)
#[test]
fn absolute_in_border_box() {
    assert_xml!(
        r#"
          <div style="box-sizing: border-box; width: 200px; height: 100px; border: 2px solid #07c160;">
            <div style="position: absolute; left: 10px; height: 100%; right: 10px;" expect_top="2" expect_height="96" expect_left="12" expect_width="176" expect_right="12"></div>
          </div>
      "#
    )
}

// Case: absolute in flex with padding and align-items
// Spec points:
// - Padding affects positioning of absolute children
// In this test:
// - Container: flex, align-items=center, padding-top=20
// - Absolute child centered within content area
#[test]
fn absolute_flex_align_items_1() {
    assert_xml!(
        r#"
          <div style="display:flex; align-items:center; padding-top:20px; width:300px; height: 68px">
            <div style="width:113px; height:32px" expect_top="38"></div>
            <div style="position:absolute; left:60px; width:60px; height:48px" expect_left="60" expect_top="30"></div>
          </div>
      "#
    )
}

// Case: absolute in inline container
// Spec points:
// - Absolute children in inline inherit inline's dimensions (often 0)
// In this test:
// - Inline has no explicit size, so percentage children resolve to 0
#[test]
fn absolute_in_inline_1() {
    assert_xml!(
        r#"
          <div style="width: 200px;" expect_width="200" expect_height="0">
            <div style="display: inline" expect_width="0" expect_height="0">
              <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="0"></div>
              <div style="position: absolute; width: 100%; height: 100%;" expect_height="0" expect_width="0"></div>
              <div style="position: absolute; width: 30px; height: 100%;" expect_height="0" expect_width="30"></div>
            </div>
          </div>
      "#
    )
}

// Case: absolute in inline-block containers
// Spec points:
// - inline-block establishes a containing block for absolutes
// - Percentage dimensions relative to inline-block size
// In this test:
// - First inline-block: 0x0 (no content)
// - Second inline-block: 100x100, absolutes size relative to it
#[test]
fn absolute_in_inline_2() {
    assert_xml!(
        r#"
          <div style="width: 200px;" expect_width="200" expect_height="100">
            <div style="display: inline-block" expect_width="0" expect_height="0">
              <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="0"></div>
              <div style="position: absolute; width: 100%; height: 100%;" expect_height="0" expect_width="0"></div>
              <div style="position: absolute; width: 30px; height: 100%;" expect_height="0" expect_width="30"></div>            
            </div>
            <div style="display: inline-block; width: 100px; height: 100px; " expect_width="100" expect_height="100">
              <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="100"></div>
              <div style="position: absolute; width: 100%; height: 100%;" expect_height="100" expect_width="100"></div>
              <div style="position: absolute; width: 30px; height: 100%;" expect_height="100" expect_width="30"></div>            
            </div>
          </div>
      "#
    )
}

// Case: relative in inline container
// Spec points:
// - Relative positioning works within inline context
// In this test:
// - Relative children maintain flow but offset visually
#[test]
fn relative_in_inline_1() {
    assert_xml!(
        r#"
          <div style="width: 200px; position: absolute" expect_width="200" expect_height="600">
            <div style="display: inline" expect_width="300" expect_height="300">
              <div style="display: flex; position: relative; width: 300px; height: 300px;" expect_height="300" expect_width="300"></div>
            </div>
            <div style="display: inline" expect_width="300" expect_height="300">
              <div style="display: flex; position: relative; left: 100px; top: 100px; width: 300px; height: 300px;" expect_left="100" expect_top="100" expect_height="300" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}

// Case: inline element with relative positioning
// Spec points:
// - Inline element can be relatively positioned
// - Offsets apply to the inline box
// In this test:
// - Inline containers with position=relative, top=100
#[test]
fn inline_as_relative() {
    assert_xml!(
        r#"
          <div style="width: 200px; position: absolute" expect_width="200" expect_height="600">
            <div style="display: inline; position: relative; top: 100px;" expect_width="300" expect_height="300" expect_top="100">
              <div style="display: flex; position: relative; width: 300px; height: 300px;" expect_height="300" expect_width="300"></div>
            </div>
            <div style="display: inline; position: relative; top: 100px;" expect_width="300" expect_height="300" expect_top="400">
              <div style="display: flex; position: relative; left: 100px; top: 100px; width: 300px; height: 300px;" expect_left="100" expect_top="100" expect_height="300" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}

// Case: absolute with max-width constraint
// Spec points:
// - max-width constrains absolute element's width
// - Children can still overflow
// In this test:
// - Absolute parent with max-width=20px
// - Child width=30px overflows
// - Text wraps within 20px constraint
#[test]
fn absolute_with_max_width() {
    assert_xml!(
        r#"
          <div style="position: absolute; max-width: 20px; left: 0; top: 100px;">
            <div style="height: 10px; width: 30px;"></div>      
            <div style="background: blue; opacity: 0.5" expect_width="20" expect_height="32">
              <div style="display: inline" expect_width="16">XX</div>
            </div>
          </div>
      "#
    )
}
