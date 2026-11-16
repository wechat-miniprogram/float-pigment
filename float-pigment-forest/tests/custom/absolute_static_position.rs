//! Incremental static-position tests. Static cases live in cases/custom_css_position.
use crate::*;
use float_pigment_css::typing::{Direction, Position};

#[test]
fn block_auto_offsets_follow_preceding_in_flow_siblings() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 200px; height: 200px; position: relative;");
    let before = ctx.create_node("div");
    ctx.set_style(before, "width: 200px; height: 100px;");
    ctx.append(root, before);
    let first = ctx.create_node("div");
    ctx.set_style(first, "position: absolute; width: 20px; height: 20px;");
    ctx.append(root, first);
    let middle = ctx.create_node("div");
    ctx.set_style(middle, "width: 200px; height: 40px;");
    ctx.append(root, middle);
    let second = ctx.create_node("div");
    ctx.set_style(second, "position: absolute; width: 20px; height: 20px;");
    ctx.append(root, second);
    ctx.layout_imperative();
    assert_eq!((ctx.left(first), ctx.top(first)), (0., 100.));
    assert_eq!((ctx.left(second), ctx.top(second)), (0., 140.));

    // Reuse the built tree: layout_imperative would rebuild it and bypass caches.
    ctx.root = ctx.imperative_root;
    ctx.layout(false);
    assert_eq!((ctx.left(first), ctx.top(first)), (0., 100.));
    assert_eq!((ctx.left(second), ctx.top(second)), (0., 140.));
    unsafe {
        let before_node = &*ctx.imperative_built[before.0];
        before_node.set_height(DefLength::Points(Len::from_f32(80.)));
        ctx.layout(false);
        assert_eq!((ctx.left(first), ctx.top(first)), (0., 80.));
        assert_eq!((ctx.left(second), ctx.top(second)), (0., 120.));
        before_node.set_position(Position::Relative);
        before_node.set_top(DefLength::Points(Len::from_f32(50.)));
        ctx.layout(false);
        assert_eq!(ctx.top(before), 50.);
        assert_eq!((ctx.left(first), ctx.top(first)), (0., 80.));
        assert_eq!((ctx.left(second), ctx.top(second)), (0., 120.));
    }
}

#[test]
fn static_position_uses_content_origin_but_explicit_insets_use_padding_edge() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(
        root,
        "width: 200px; height: 200px; position: relative; padding-top: 10px; \
         padding-left: 12px; border-top: 3px solid; border-left: 3px solid;",
    );
    let before = ctx.create_node("div");
    ctx.set_style(before, "width: 200px; height: 100px;");
    ctx.append(root, before);
    let absolute = ctx.create_node("div");
    ctx.set_style(absolute, "position: absolute; width: 20px; height: 20px;");
    ctx.append(root, absolute);
    ctx.layout_imperative();
    assert_eq!((ctx.left(absolute), ctx.top(absolute)), (15., 113.));

    ctx.root = ctx.imperative_root;
    unsafe {
        let node = &*ctx.imperative_built[absolute.0];
        node.set_top(DefLength::Points(Len::from_f32(0.)));
        ctx.layout(false);
        assert_eq!((ctx.left(absolute), ctx.top(absolute)), (15., 3.));
        node.set_top(DefLength::Auto);
        node.set_left(DefLength::Points(Len::from_f32(0.)));
        ctx.layout(false);
        assert_eq!((ctx.left(absolute), ctx.top(absolute)), (3., 113.));
    }
}

#[test]
fn absolute_margins_do_not_join_the_normal_flow_margin_collapse() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 200px; height: 200px; position: relative;");
    let before = ctx.create_node("div");
    ctx.set_style(before, "width: 200px; height: 100px; margin-bottom: 20px;");
    ctx.append(root, before);
    let absolute = ctx.create_node("div");
    ctx.set_style(
        absolute,
        "position: absolute; width: 20px; height: 20px; margin-top: 30px;",
    );
    ctx.append(root, absolute);
    let after = ctx.create_node("div");
    ctx.set_style(after, "width: 200px; height: 10px; margin-top: 40px;");
    ctx.append(root, after);
    ctx.layout_imperative();
    assert_eq!(ctx.top(absolute), 150.);
    assert_eq!(ctx.top(after), 140.);

    ctx.root = ctx.imperative_root;
    unsafe {
        let absolute_node = &*ctx.imperative_built[absolute.0];
        absolute_node.set_margin_top(DefLength::Points(Len::from_f32(300.)));
        ctx.layout(false);
        assert_eq!(ctx.top(absolute), 420.);
        assert_eq!(ctx.top(after), 140.);
        (*ctx.imperative_built[before.0]).set_margin_bottom(DefLength::Points(Len::from_f32(-10.)));
        absolute_node.set_margin_top(DefLength::Points(Len::from_f32(5.)));
        ctx.layout(false);
        assert_eq!(ctx.top(absolute), 95.);
        assert_eq!(ctx.top(after), 130.);
    }
}

#[test]
fn static_inline_start_respects_rtl_and_absolute_auto_margins_are_zero() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(
        root,
        "width: 200px; height: 200px; position: relative; direction: rtl;",
    );
    let before = ctx.create_node("div");
    ctx.set_style(before, "width: 200px; height: 100px;");
    ctx.append(root, before);
    let absolute = ctx.create_node("div");
    ctx.set_style(absolute, "position: absolute; width: 20px; height: 20px;");
    ctx.append(root, absolute);
    ctx.layout_imperative();
    assert_eq!((ctx.left(absolute), ctx.top(absolute)), (180., 100.));

    ctx.root = ctx.imperative_root;
    unsafe {
        let node = &*ctx.imperative_built[absolute.0];
        node.set_margin_right(DefLength::Points(Len::from_f32(7.)));
        node.set_margin_left(DefLength::Auto);
        ctx.layout(false);
        assert_eq!((ctx.left(absolute), ctx.top(absolute)), (173., 100.));
        (*ctx.imperative_built[root.0]).set_direction(Direction::LTR);
        ctx.layout(false);
        assert_eq!((ctx.left(absolute), ctx.top(absolute)), (0., 100.));
    }
}

#[test]
fn display_none_and_position_changes_refresh_static_anchors() {
    let mut ctx = TestCtx::new();
    let viewport = ctx.create_node("div");
    ctx.set_style(viewport, "width: 200px; height: 200px;");
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 200px; position: relative;");
    ctx.append(viewport, root);
    let before = ctx.create_node("div");
    ctx.set_style(before, "width: 200px; height: 100px;");
    ctx.append(root, before);
    let absolute = ctx.create_node("div");
    ctx.set_style(absolute, "position: absolute; width: 20px; height: 20px;");
    ctx.append(root, absolute);
    ctx.layout_imperative();
    assert_eq!(ctx.top(absolute), 100.);

    ctx.root = ctx.imperative_root;
    unsafe {
        let before_node = &*ctx.imperative_built[before.0];
        let absolute_node = &*ctx.imperative_built[absolute.0];
        before_node.set_display(Display::None);
        ctx.layout(false);
        assert_eq!(ctx.top(absolute), 0.);
        before_node.set_display(Display::Block);
        absolute_node.set_position(Position::Static);
        ctx.layout(false);
        assert_eq!(ctx.height(root), 120.);
        absolute_node.set_position(Position::Absolute);
        ctx.layout(false);
        assert_eq!(ctx.top(absolute), 100.);
        assert_eq!(ctx.height(root), 100.);
    }
}

#[test]
fn fixed_auto_insets_use_static_position_and_explicit_insets_use_the_viewport() {
    let mut ctx = TestCtx::new();
    let root = ctx.create_node("div");
    ctx.set_style(root, "width: 200px; height: 200px; position: relative;");
    let before = ctx.create_node("div");
    ctx.set_style(before, "width: 200px; height: 100px;");
    ctx.append(root, before);
    let fixed = ctx.create_node("div");
    ctx.set_style(fixed, "position: fixed; width: 20px; height: 20px;");
    ctx.append(root, fixed);
    ctx.layout_imperative();
    assert_eq!((ctx.left(fixed), ctx.top(fixed)), (0., 100.));

    ctx.root = ctx.imperative_root;
    unsafe {
        let node = &*ctx.imperative_built[fixed.0];
        node.set_right(DefLength::Points(Len::from_f32(0.)));
        node.set_bottom(DefLength::Points(Len::from_f32(0.)));
        ctx.layout(false);
        // TestCtx uses a 375 x 750 viewport, independent of the 200 x 200 root.
        assert_eq!((ctx.left(fixed), ctx.top(fixed)), (355., 730.));
    }
}
