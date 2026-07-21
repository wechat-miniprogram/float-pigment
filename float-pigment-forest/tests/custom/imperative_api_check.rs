// Handwritten imperative re-implementation of cases/custom_css_grid/grid.html.
// Verifies that the Task 2-4 imperative API (create_node / create_text /
// set_style / append + layout_imperative + getters) produces layout identical
// to the legacy from_str + data-expect-* path.
//
// If this test FAILs, the discrepancy points at a bug in build_dfs (Task 3):
// props inheritance, is_block_tag handling, or Text node preparation. Report
// the actual vs expected values — do NOT tweak assertions or build_dfs here.

use crate::{NodeHandle, TestCtx};

/// Mirrors cases/custom_css_grid/grid.html exactly:
///   <div style="display: grid; width: 600px;
///               grid-template-columns: auto 100px auto;
///               grid-template-rows: 30px 40px;">
///     <div data-expect-height="30" data-expect-left="0">header1</div>
///     <div data-expect-height="30" data-expect-left="250">header2</div>
///     <div data-expect-height="30" data-expect-left="350">header3</div>
///     <div data-expect-height="40" data-expect-left="0">content1</div>
///     <div style="width: 23px; height: 23px"
///          data-expect-height="23" data-expect-left="250">content2</div>
///     <div data-expect-height="40" data-expect-left="350">content3</div>
///     <div data-expect-height="32" data-expect-left="0">content4</div>
///     <div data-expect-height="32" data-expect-left="250">content5</div>
///     <div data-expect-height="32" data-expect-left="350">content6</div>
///   </div>
#[test]
fn imperative_grid_equivalent_to_from_str() {
    let mut ctx = TestCtx::new();

    // Root: grid container with the exact style from grid.html.
    let n0 = ctx.create_node("div");
    ctx.set_style(
        n0,
        "display: grid; width: 600px; \
         grid-template-columns: auto 100px auto; \
         grid-template-rows: 30px 40px;",
    );

    // Helper: append a div with text content under parent. Returns the div
    // handle so we can assert against it.
    fn append_div_with_text(
        ctx: &mut TestCtx,
        parent: NodeHandle,
        text: &str,
    ) -> NodeHandle {
        let div = ctx.create_node("div");
        let t = ctx.create_text(text);
        ctx.append(div, t);
        ctx.append(parent, div);
        div
    }

    // Row 1 (template row height 30px)
    let n1 = append_div_with_text(&mut ctx, n0, "header1"); // left=0
    let n2 = append_div_with_text(&mut ctx, n0, "header2"); // left=250
    let n3 = append_div_with_text(&mut ctx, n0, "header3"); // left=350

    // Row 2 (template row height 40px)
    let n4 = append_div_with_text(&mut ctx, n0, "content1"); // left=0
    // content2 has an explicit inline style overriding width/height.
    let n5 = ctx.create_node("div");
    ctx.set_style(n5, "width: 23px; height: 23px");
    let t5 = ctx.create_text("content2");
    ctx.append(n5, t5);
    ctx.append(n0, n5); // left=250, height=23
    let n6 = append_div_with_text(&mut ctx, n0, "content3"); // left=350

    // Row 3 (auto row; intrinsic height 32 — derived by layout, asserted below)
    let n7 = append_div_with_text(&mut ctx, n0, "content4"); // left=0
    let n8 = append_div_with_text(&mut ctx, n0, "content5"); // left=250
    let n9 = append_div_with_text(&mut ctx, n0, "content6"); // left=350

    ctx.layout_imperative();

    // Row 1 — all three sit on the 30px template row.
    assert_eq!(ctx.height(n1), 30.0, "header1 height");
    assert_eq!(ctx.left(n1), 0.0, "header1 left");
    assert_eq!(ctx.height(n2), 30.0, "header2 height");
    assert_eq!(ctx.left(n2), 250.0, "header2 left");
    assert_eq!(ctx.height(n3), 30.0, "header3 height");
    assert_eq!(ctx.left(n3), 350.0, "header3 left");

    // Row 2 — 40px template row, except content2 which sets its own height.
    assert_eq!(ctx.height(n4), 40.0, "content1 height");
    assert_eq!(ctx.left(n4), 0.0, "content1 left");
    assert_eq!(ctx.height(n5), 23.0, "content2 height (inline style override)");
    assert_eq!(ctx.left(n5), 250.0, "content2 left");
    assert_eq!(ctx.height(n6), 40.0, "content3 height");
    assert_eq!(ctx.left(n6), 350.0, "content3 left");

    // Row 3 — auto row; intrinsic height comes out at 32px per data-expect.
    assert_eq!(ctx.height(n7), 32.0, "content4 height");
    assert_eq!(ctx.left(n7), 0.0, "content4 left");
    assert_eq!(ctx.height(n8), 32.0, "content5 height");
    assert_eq!(ctx.left(n8), 250.0, "content5 left");
    assert_eq!(ctx.height(n9), 32.0, "content6 height");
    assert_eq!(ctx.left(n9), 350.0, "content6 left");
}
