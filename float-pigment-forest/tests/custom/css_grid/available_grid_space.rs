// Regression tests for CSS Grid §11.3 "available grid space".
//
// Per spec (https://www.w3.org/TR/css-grid-1/#algo-terms), independently
// per dimension, the available grid space is:
//   - the container's content box, when the container has a definite size;
//   - the active min-/max-content constraint (indefinite), when the
//     container is being sized under such a constraint;
//   - indefinite, otherwise.
//
// When available grid space is indefinite, fr tracks must be resolved via
// §11.7.1 ("Find the Size of an fr") using items' max-content contributions,
// not the ancestor's available space.
//
// These tests lock down that the grid container never inherits the
// ancestor's available space as its own definite available grid space
// in `SizingMode::Normal`, and therefore the container collapses to the
// children's contribution under indefinite sizing.

use crate::*;

// §11.3 branch: container size is indefinite (inline-grid, width auto),
// parent provides available space but it MUST NOT be taken as definite
// available grid space. fr tracks must resolve via §11.7.1.
// Expected: container width = sum of children max-content (100 + 100 = 200).
#[test]
fn grid_available_space_indefinite_inline_grid_collapses_to_children() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 1fr 1fr;" expect_width="200">
          <div style="width: 100px; height: 50px;" expect_left="0" expect_width="100"></div>
          <div style="width: 100px; height: 50px;" expect_left="100" expect_width="100"></div>
        </div>
    "#,
        true
    )
}

// §11.3 branch: container has a definite size, available grid space is
// the content box. fr tracks resolve via the definite path and split it.
#[test]
fn grid_available_space_definite_uses_content_box() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 1fr 1fr;">
          <div style="height: 50px;" expect_left="0" expect_width="150"></div>
          <div style="height: 50px;" expect_left="150" expect_width="150"></div>
        </div>
    "#,
        true
    )
}

// §11.3 branch: indefinite container with unequal fr values still uses
// §11.7.1 indefinite resolution (based on each child's max-content
// contribution scaled by its fr): hyp_1fr = max(100/1, 150/2) = 100,
// so track widths are 1fr=100, 2fr=200, and container width = 300.
// This test asserts the container width only; item-in-track sizing
// (§11.7/§11.8) is intentionally not asserted here.
#[test]
fn grid_available_space_indefinite_unequal_fr_uses_max_contribution() {
    assert_xml!(
        r#"
        <div style="display: inline-grid; grid-template-columns: 1fr 2fr;" expect_width="300">
          <div style="width: 100px; height: 50px;"></div>
          <div style="width: 150px; height: 50px;"></div>
        </div>
    "#,
        true
    )
}

// Nested case: a block flow wraps the inline-grid. The outer ancestor has
// a definite width (375 default viewport) and propagates available space
// down; if that value leaks into the inner grid's available grid space,
// fr tracks would wrongly split it. This test pins the regression fixed
// in grid/mod.rs: in Normal sizing mode, `request.max_content` of the
// inner grid MUST NOT be used as its available grid space.
#[test]
fn grid_available_space_indefinite_ignores_ancestor_available_space() {
    assert_xml!(
        r#"
        <div>
          <div style="display: inline-grid; grid-template-columns: 1fr 1fr;" expect_width="200">
            <div style="width: 100px; height: 50px;" expect_left="0" expect_width="100"></div>
            <div style="width: 100px; height: 50px;" expect_left="100" expect_width="100"></div>
          </div>
        </div>
    "#,
        true
    )
}
