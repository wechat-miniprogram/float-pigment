// WPT-style tests for grid-auto-rows and grid-auto-columns
// Reference: CSS Grid Layout Module Level 1 §7.6
// https://www.w3.org/TR/css-grid-1/#auto-tracks

use crate::{assert_xml, TestCtx};

// Case: grid-auto-rows with fixed value
// Spec points:
//   - grid-auto-rows specifies size of implicitly-created row tracks
//   - When items overflow explicit grid, implicit rows are created
// In this test:
//   - Container: 2 explicit columns, 0 explicit rows
//   - 3 items create 2 implicit rows
//   - grid-auto-rows: 50px sets each implicit row to 50px
#[test]
fn grid_auto_rows_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 100px 100px; grid-auto-rows: 50px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-columns with fixed value
// Spec points:
//   - grid-auto-columns specifies size of implicitly-created column tracks
//   - With grid-auto-flow: column, items fill columns first
// In this test:
//   - Container: 0 explicit columns, 2 explicit rows
//   - 3 items with column flow create 2 implicit columns
//   - grid-auto-columns: 80px sets each implicit column to 80px
#[test]
fn grid_auto_columns_fixed() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 100px; grid-template-rows: 50px 50px; grid-auto-flow: column; grid-auto-columns: 80px;">
          <div expect_left="0" expect_top="0" expect_width="80" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="80" expect_height="50"></div>
          <div expect_left="80" expect_top="0" expect_width="80" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-rows with multiple values (cycling)
// Spec points:
//   - Multiple values in grid-auto-rows cycle for successive implicit tracks
//   - First implicit row uses first value, second uses second, etc.
// In this test:
//   - grid-auto-rows: 30px 60px
//   - Row 0 (implicit index 0) = 30px
//   - Row 1 (implicit index 1) = 60px
#[test]
fn grid_auto_rows_multiple() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 100px 100px; grid-auto-rows: 30px 60px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="30"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="30"></div>
          <div expect_left="0" expect_top="30" expect_width="100" expect_height="60"></div>
          <div expect_left="100" expect_top="30" expect_width="100" expect_height="60"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-columns with multiple values (cycling)
// Spec points:
//   - Multiple values in grid-auto-columns cycle for successive implicit tracks
//   - First implicit column uses first value, second uses second, etc.
// In this test:
//   - grid-auto-columns: 50px 100px
//   - Column 0 (implicit index 0) = 50px
//   - Column 1 (implicit index 1) = 100px
#[test]
fn grid_auto_columns_multiple() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 100px; grid-template-rows: 50px 50px; grid-auto-flow: column; grid-auto-columns: 50px 100px;">
          <div expect_left="0" expect_top="0" expect_width="50" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="50" expect_height="50"></div>
          <div expect_left="50" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="50" expect_top="50" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-rows: auto (default behavior)
// Spec points:
//   - auto sizes the track to fit its content
//   - Row height determined by tallest item in that row
// In this test:
//   - grid-auto-rows: auto
//   - Items have height: 40px
//   - Implicit row height = 40px (content height)
#[test]
fn grid_auto_rows_auto() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 100px 100px; grid-auto-rows: auto;">
          <div style="height: 40px;" expect_left="0" expect_top="0" expect_width="100" expect_height="40"></div>
          <div style="height: 40px;" expect_left="100" expect_top="0" expect_width="100" expect_height="40"></div>
        </div>
    "#,
        true
    )
}

// Case: explicit + implicit rows combination
// Spec points:
//   - Implicit tracks are created after explicit tracks
//   - Explicit tracks use grid-template-rows values
//   - Implicit tracks use grid-auto-rows values
// In this test:
//   - grid-template-rows: 80px (1 explicit row)
//   - grid-auto-rows: 50px
//   - Row 0 (explicit) = 80px, Row 1 (implicit) = 50px
#[test]
fn grid_explicit_and_implicit_rows() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; grid-template-columns: 100px 100px; grid-template-rows: 80px; grid-auto-rows: 50px;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="80"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="80"></div>
          <div expect_left="0" expect_top="80" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="80" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: explicit + implicit columns combination
// Spec points:
//   - Implicit tracks are created after explicit tracks
//   - Explicit tracks use grid-template-columns values
//   - Implicit tracks use grid-auto-columns values
// In this test:
//   - grid-template-columns: 120px (1 explicit column)
//   - grid-auto-columns: 80px
//   - Column 0 (explicit) = 120px, Column 1 (implicit) = 80px
#[test]
fn grid_explicit_and_implicit_columns() {
    assert_xml!(
        r#"
        <div style="display: grid; height: 100px; grid-template-rows: 50px 50px; grid-template-columns: 120px; grid-auto-flow: column; grid-auto-columns: 80px;">
          <div expect_left="0" expect_top="0" expect_width="120" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="120" expect_height="50"></div>
          <div expect_left="120" expect_top="0" expect_width="80" expect_height="50"></div>
          <div expect_left="120" expect_top="50" expect_width="80" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-rows with percentage
// Spec points:
//   - Percentage values resolve against grid container's content box
//   - Requires container to have definite height
// In this test:
//   - Container height: 200px
//   - grid-auto-rows: 25% = 200px × 25% = 50px
#[test]
fn grid_auto_rows_percentage() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 200px; height: 200px; grid-template-columns: 100px 100px; grid-auto-rows: 25%;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="100" expect_top="0" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}

// Case: grid-auto-columns with percentage
// Spec points:
//   - Percentage values resolve against grid container's content box
//   - Requires container to have definite width
// In this test:
//   - Container width: 400px
//   - grid-auto-columns: 25% = 400px × 25% = 100px
#[test]
fn grid_auto_columns_percentage() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 400px; height: 100px; grid-template-rows: 50px 50px; grid-auto-flow: column; grid-auto-columns: 25%;">
          <div expect_left="0" expect_top="0" expect_width="100" expect_height="50"></div>
          <div expect_left="0" expect_top="50" expect_width="100" expect_height="50"></div>
        </div>
    "#,
        true
    )
}
