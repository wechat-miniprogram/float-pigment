// An abs-pos box with `height: auto` takes its content height, not the space left in
// its containing block (CSS2.1 §10.6.4); shrink-to-fit is inline-axis only.

use crate::{NodeHandle, TestCtx};

const PILL: &str = "height: 28px; display: flex; align-items: center; padding: 0 6px; box-sizing: border-box; align-self: flex-start;";
const FLOATING: &str = "position: absolute; left: 16px; right: 16px; bottom: 200px; display: flex; align-items: end;";

/// bottom bar (height 0) > floating area (abs-pos) > column flex > [pill, info]
fn build(ctx: &mut TestCtx, floating_style: &str) -> (NodeHandle, NodeHandle, NodeHandle) {
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 375px; height: 600px; position: relative;");
    let bottom = ctx.create_node("div");
    ctx.set_style(
        bottom,
        "position: absolute; bottom: 0; left: 0; right: 0; padding: 0 16px;",
    );
    ctx.append(root, bottom);
    let floating = ctx.create_node("div");
    ctx.set_style(floating, floating_style);
    ctx.append(bottom, floating);
    let column = ctx.create_node("div");
    ctx.set_style(
        column,
        "display: flex; flex-direction: column; flex: 1; margin-bottom: 16px; overflow: hidden;",
    );
    ctx.append(floating, column);
    let pill = ctx.create_node("div");
    ctx.set_style(pill, PILL);
    ctx.append(column, pill);
    let text = ctx.create_text("28");
    ctx.set_style(text, "font-size: 14px; line-height: 18px;");
    ctx.append(pill, text);
    let info = ctx.create_node("div");
    ctx.set_style(info, "height: 18px; margin-top: 8px; align-self: flex-start;");
    ctx.append(column, info);
    (floating, column, pill)
}

#[test]
fn abs_auto_height_does_not_constrain_block_axis() {
    let mut ctx = TestCtx::new();
    let (floating, column, pill) = build(&mut ctx, FLOATING);
    ctx.layout_imperative();

    // Items keep their heights; the container is content-sized.
    assert_eq!(ctx.height(pill).round(), 28.0);
    assert_eq!(ctx.height(column).round(), 54.0);
    assert_eq!(ctx.height(floating).round(), 70.0);
}

#[test]
fn abs_auto_height_still_honours_max_height() {
    let mut ctx = TestCtx::new();
    let (floating, _, _) = build(&mut ctx, &format!("{FLOATING} max-height: 40px;"));
    ctx.layout_imperative();

    assert_eq!(ctx.height(floating).round(), 40.0);
}

#[test]
fn percentage_height_child_of_auto_height_box_is_content_sized() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 375px; height: 600px; position: relative;");
    let wrapper = ctx.create_node("div");
    ctx.set_style(
        wrapper,
        "position: absolute; left: 0; right: 0; bottom: 200px;",
    );
    ctx.append(root, wrapper);
    let child = ctx.create_node("div");
    ctx.set_style(child, "height: 100%;");
    ctx.append(wrapper, child);
    let text = ctx.create_text("text");
    ctx.set_style(text, "font-size: 14px; line-height: 20px;");
    ctx.append(child, text);
    ctx.layout_imperative();

    // A percentage height against an indefinite parent height behaves as auto.
    assert!(ctx.height(child) > 0.0);
    assert_eq!(ctx.height(child), ctx.height(wrapper));
}

#[test]
fn abs_auto_height_is_not_capped_by_remaining_space() {
    // Positive remaining space is not a block-axis constraint either: content taller than
    // the containing block must not be squeezed into it.
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 375px; height: 600px; position: relative;");
    let short = ctx.create_node("div");
    ctx.set_style(short, "width: 375px; height: 20px;");
    ctx.append(root, short);
    let floating = ctx.create_node("div");
    ctx.set_style(
        floating,
        "position: absolute; left: 0; right: 0; top: 0; display: flex; align-items: end;",
    );
    ctx.append(short, floating);
    let column = ctx.create_node("div");
    ctx.set_style(
        column,
        "display: flex; flex-direction: column; flex: 1; margin-bottom: 16px; overflow: hidden;",
    );
    ctx.append(floating, column);
    let pill = ctx.create_node("div");
    ctx.set_style(pill, PILL);
    ctx.append(column, pill);
    let text = ctx.create_text("28");
    ctx.set_style(text, "font-size: 14px; line-height: 18px;");
    ctx.append(pill, text);
    let info = ctx.create_node("div");
    ctx.set_style(info, "height: 18px; margin-top: 8px; align-self: flex-start;");
    ctx.append(column, info);
    ctx.layout_imperative();

    assert_eq!(ctx.height(pill).round(), 28.0);
    assert_eq!(ctx.height(column).round(), 54.0);
    assert_eq!(ctx.height(floating).round(), 70.0);
}

#[test]
fn max_height_defines_wrapped_column_width() {
    // A definite `max-height` bounds the used height, so it decides how many
    // columns a wrapping column flex produces, and therefore its intrinsic width.
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 200px; height: 200px; position: relative;");
    let container = ctx.create_node("div");
    ctx.set_style(
        container,
        "position: absolute; top: 0; left: 0; display: flex; flex-direction: column; \
         flex-wrap: wrap; max-height: 100px;",
    );
    ctx.append(root, container);
    for _ in 0..3 {
        let item = ctx.create_node("div");
        ctx.set_style(item, "width: 50px; height: 60px; flex: none;");
        ctx.append(container, item);
    }
    ctx.layout_imperative();

    // 3 columns of 60px inside a 100px tall box; the box itself stays 100px tall.
    assert_eq!(ctx.width(container).round(), 150.0);
    assert_eq!(ctx.height(container).round(), 100.0);
}

#[test]
fn wrapped_column_without_cap_stays_single_column() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 200px; height: 200px; position: relative;");
    let container = ctx.create_node("div");
    ctx.set_style(
        container,
        "position: absolute; top: 0; left: 0; display: flex; flex-direction: column; flex-wrap: wrap;",
    );
    ctx.append(root, container);
    for _ in 0..3 {
        let item = ctx.create_node("div");
        ctx.set_style(item, "width: 50px; height: 60px; flex: none;");
        ctx.append(container, item);
    }
    ctx.layout_imperative();

    assert_eq!(ctx.width(container).round(), 50.0);
    assert_eq!(ctx.height(container).round(), 180.0);
}

#[test]
fn definite_height_still_shrinks_flex_items() {
    let mut ctx = TestCtx::new();
    let (floating, column, pill) = build(
        &mut ctx,
        &format!("{FLOATING} flex-direction: column; height: 30px;"),
    );
    ctx.layout_imperative();

    assert_eq!(ctx.height(floating).round(), 30.0);
    assert_eq!(ctx.height(column).round(), 14.0);
    assert!(ctx.height(pill) < 28.0);
}
