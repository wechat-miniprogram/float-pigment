//! CSS 2.1 10.3.7 / 10.6.4 and CSS Positioned Layout 3 static positions.
use float_pigment_css::typing::{
    AlignItemsType, Direction, DisplayType, JustifyContentType, PositionType, TextAlignType,
    WritingModeType,
};
use float_pigment_forest::{ffi::*, Node, StyleSetter};

struct Tree(Vec<NodePtr>);

impl Tree {
    unsafe fn new() -> Self {
        let mut tree = Self(Vec::new());
        let root = tree.box_node(None, 200., 200.);
        NodeStyleSetPosition(root, PositionType::Relative);
        tree
    }

    fn root(&self) -> NodePtr {
        self.0[0]
    }

    unsafe fn box_node(&mut self, parent: Option<NodePtr>, width: f32, height: f32) -> NodePtr {
        let node = NodeNew();
        NodeStyleSetDisplay(node, DisplayType::Block);
        NodeStyleSetWidth(node, width);
        NodeStyleSetHeight(node, height);
        if let Some(parent) = parent {
            NodeAppendChild(parent, node);
        }
        self.0.push(node);
        node
    }

    unsafe fn absolute(&mut self, parent: NodePtr) -> NodePtr {
        let node = self.box_node(Some(parent), 20., 20.);
        NodeStyleSetPosition(node, PositionType::Absolute);
        node
    }

    unsafe fn layout(&self) {
        NodeCalculateLayout(self.root(), 200., f32::NAN, 400., 800.);
    }

    unsafe fn position(node: NodePtr) -> (f32, f32) {
        (NodeLayoutGetLeft(node), NodeLayoutGetTop(node))
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        // Freeing each parent first detaches its surviving children.
        for &node in &self.0 {
            unsafe { NodeFree(node) }
        }
    }
}

#[test]
fn block_auto_offsets_follow_preceding_in_flow_siblings() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        let before = tree.box_node(Some(root), 200., 100.);
        let first = tree.absolute(root);
        tree.box_node(Some(root), 200., 40.);
        let second = tree.absolute(root);
        tree.layout();
        assert_eq!(Tree::position(first), (0., 100.));
        assert_eq!(Tree::position(second), (0., 140.));

        // Both cached layout and later in-flow changes must retain each anchor.
        tree.layout();
        assert_eq!(Tree::position(second), (0., 140.));
        NodeStyleSetHeight(before, 80.);
        tree.layout();
        assert_eq!(Tree::position(first), (0., 80.));
        assert_eq!(Tree::position(second), (0., 120.));
        NodeStyleSetPosition(before, PositionType::Relative);
        NodeStyleSetTop(before, 50.);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(before), 50.);
        assert_eq!(Tree::position(first), (0., 80.));
        assert_eq!(Tree::position(second), (0., 120.));
    }
}

#[test]
fn static_position_uses_content_origin_but_explicit_insets_use_padding_edge() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetPaddingTop(root, 10.);
        NodeStyleSetPaddingLeft(root, 12.);
        NodeStyleSetBorderTop(root, 3.);
        NodeStyleSetBorderLeft(root, 3.);
        tree.box_node(Some(root), 200., 100.);
        let absolute = tree.absolute(root);
        tree.layout();
        assert_eq!(Tree::position(absolute), (15., 113.));

        NodeStyleSetTop(absolute, 0.);
        tree.layout();
        assert_eq!(Tree::position(absolute), (15., 3.));
        NodeStyleSetTopAuto(absolute);
        NodeStyleSetLeft(absolute, 0.);
        tree.layout();
        assert_eq!(Tree::position(absolute), (3., 113.));
    }
}

#[test]
fn absolute_margins_do_not_join_the_normal_flow_margin_collapse() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        let before = tree.box_node(Some(root), 200., 100.);
        NodeStyleSetMarginBottom(before, 20.);
        let absolute = tree.absolute(root);
        NodeStyleSetMarginTop(absolute, 30.);
        let after = tree.box_node(Some(root), 200., 10.);
        NodeStyleSetMarginTop(after, 40.);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 150.);
        assert_eq!(NodeLayoutGetTop(after), 140.);

        NodeStyleSetMarginTop(absolute, 300.);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 420.);
        assert_eq!(NodeLayoutGetTop(after), 140.);
        NodeStyleSetMarginBottom(before, -10.);
        NodeStyleSetMarginTop(absolute, 5.);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 95.);
        assert_eq!(NodeLayoutGetTop(after), 130.);
    }
}

#[test]
fn empty_collapsing_blocks_do_not_make_the_anchor_consume_margins_twice() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        let empty = tree.box_node(Some(root), 200., 0.);
        NodeStyleSetMarginTop(empty, 30.);
        NodeStyleSetMarginBottom(empty, 10.);
        let absolute = tree.absolute(root);
        let after = tree.box_node(Some(root), 200., 10.);
        NodeStyleSetMarginTop(after, 40.);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 30.);
        assert_eq!(NodeLayoutGetTop(after), 40.);
    }
}

#[test]
fn absolute_boxes_do_not_add_to_auto_container_size() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetHeightAuto(root);
        tree.box_node(Some(root), 200., 100.);
        let absolute = tree.absolute(root);
        NodeStyleSetHeight(absolute, 500.);
        tree.layout();
        assert_eq!(NodeLayoutGetHeight(root), 100.);
        assert_eq!(NodeLayoutGetTop(absolute), 100.);
    }
}

#[test]
fn static_inline_start_respects_rtl_and_absolute_auto_margins_are_zero() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        (&*(root as *const Node)).set_direction(Direction::RTL);
        tree.box_node(Some(root), 200., 100.);
        let absolute = tree.absolute(root);
        tree.layout();
        assert_eq!(Tree::position(absolute), (180., 100.));

        NodeStyleSetMarginRight(absolute, 7.);
        NodeStyleSetMarginLeftAuto(absolute);
        tree.layout();
        assert_eq!(Tree::position(absolute), (173., 100.));
        (&*(root as *const Node)).set_direction(Direction::LTR);
        tree.layout();
        assert_eq!(Tree::position(absolute), (0., 100.));
    }
}

#[test]
fn static_block_start_follows_vertical_writing_modes() {
    unsafe {
        for (mode, expected_left) in [
            (WritingModeType::VerticalRl, 80.),
            (WritingModeType::VerticalLr, 100.),
        ] {
            let mut tree = Tree::new();
            let root = tree.root();
            NodeStyleSetWritingMode(root, mode);
            tree.box_node(Some(root), 100., 200.);
            let absolute = tree.absolute(root);
            tree.layout();
            assert_eq!(Tree::position(absolute), (expected_left, 0.));
        }
    }
}

#[test]
fn block_static_position_after_inline_content_does_not_split_the_actual_line() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetHeightAuto(root);
        let before = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(before, DisplayType::InlineBlock);
        let absolute = tree.absolute(root);
        let after = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(after, DisplayType::InlineBlock);
        tree.layout();
        assert_eq!(Tree::position(absolute), (0., 20.));
        assert_eq!(Tree::position(after), (50., 0.));
        assert_eq!(NodeLayoutGetHeight(root), 20.);
    }
}

#[test]
fn inline_static_position_uses_the_inline_cursor() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        let before = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(before, DisplayType::InlineBlock);
        let absolute = tree.absolute(root);
        NodeStyleSetDisplay(absolute, DisplayType::Inline);
        let after = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(after, DisplayType::InlineBlock);
        tree.layout();
        assert_eq!(Tree::position(absolute), (50., 0.));
        assert_eq!(Tree::position(after), (50., 0.));
    }
}

#[test]
fn wrapped_inline_prefix_determines_the_block_static_position() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetWidth(root, 100.);
        NodeStyleSetHeightAuto(root);
        for _ in 0..2 {
            let before = tree.box_node(Some(root), 80., 20.);
            NodeStyleSetDisplay(before, DisplayType::InlineBlock);
        }
        let absolute = tree.absolute(root);
        let after = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(after, DisplayType::InlineBlock);
        tree.layout();
        assert_eq!(Tree::position(absolute), (0., 40.));
        assert_eq!(Tree::position(after), (0., 40.));
        assert_eq!(NodeLayoutGetHeight(root), 60.);
    }
}

#[test]
fn flex_static_alignment_is_preserved() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetDisplay(root, DisplayType::Flex);
        NodeStyleSetJustifyContent(root, JustifyContentType::Center);
        NodeStyleSetAlignItems(root, AlignItemsType::Center);
        let absolute = tree.absolute(root);
        tree.layout();
        assert_eq!(Tree::position(absolute), (90., 90.));
    }
}

#[test]
fn inline_cursor_uses_the_line_box_and_text_alignment() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetTextAlign(root, TextAlignType::Center);
        let before = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(before, DisplayType::InlineBlock);
        let inline = tree.absolute(root);
        NodeStyleSetDisplay(inline, DisplayType::Inline);
        let block = tree.absolute(root);
        let after = tree.box_node(Some(root), 50., 40.);
        NodeStyleSetDisplay(after, DisplayType::InlineBlock);
        tree.layout();
        assert_eq!(Tree::position(inline), (100., 0.));
        assert_eq!(Tree::position(block), (0., 40.));
        assert_eq!(Tree::position(after), (100., 0.));
    }
}

#[test]
fn inline_wrapper_keeps_formatting_context_axes_for_static_anchors() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        tree.box_node(Some(root), 200., 30.);
        let wrapper = tree.box_node(Some(root), 50., 20.);
        NodeStyleSetDisplay(wrapper, DisplayType::Inline);
        NodeStyleSetPosition(wrapper, PositionType::Relative);
        (&*(wrapper as *const Node)).set_direction(Direction::RTL);
        let before = tree.box_node(Some(wrapper), 50., 20.);
        NodeStyleSetDisplay(before, DisplayType::InlineBlock);
        let absolute = tree.absolute(wrapper);
        NodeStyleSetDisplay(absolute, DisplayType::Inline);
        tree.layout();
        assert_eq!(Tree::position(wrapper), (0., 30.));
        assert_eq!(Tree::position(absolute), (50., 0.));
    }
}

#[test]
fn display_none_and_position_changes_refresh_static_anchors() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        NodeStyleSetHeightAuto(root);
        let before = tree.box_node(Some(root), 200., 100.);
        let absolute = tree.absolute(root);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 100.);
        NodeStyleSetDisplay(before, DisplayType::None);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 0.);
        NodeStyleSetDisplay(before, DisplayType::Block);
        NodeStyleSetPosition(absolute, PositionType::Static);
        tree.layout();
        assert_eq!(NodeLayoutGetHeight(root), 120.);
        NodeStyleSetPosition(absolute, PositionType::Absolute);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(absolute), 100.);
        assert_eq!(NodeLayoutGetHeight(root), 100.);
    }
}

#[test]
fn static_anchor_observes_collapsed_end_margin_without_changing_parent_height() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        let wrapper = tree.box_node(Some(root), 200., 0.);
        NodeStyleSetPosition(wrapper, PositionType::Relative);
        NodeStyleSetHeightAuto(wrapper);
        let before = tree.box_node(Some(wrapper), 200., 100.);
        NodeStyleSetMarginTop(before, 10.);
        NodeStyleSetMarginBottom(before, 20.);
        let absolute = tree.absolute(wrapper);
        let after = tree.box_node(Some(root), 200., 10.);
        tree.layout();
        assert_eq!(NodeLayoutGetTop(wrapper), 10.);
        assert_eq!(NodeLayoutGetHeight(wrapper), 100.);
        assert_eq!(NodeLayoutGetTop(absolute), 120.);
        assert_eq!(NodeLayoutGetTop(after), 130.);
    }
}

#[test]
fn fixed_auto_insets_use_static_position_and_explicit_insets_use_the_viewport() {
    unsafe {
        let mut tree = Tree::new();
        let root = tree.root();
        tree.box_node(Some(root), 200., 100.);
        let fixed = tree.absolute(root);
        NodeStyleSetPosition(fixed, PositionType::Fixed);
        tree.layout();
        assert_eq!(Tree::position(fixed), (0., 100.));
        NodeStyleSetRight(fixed, 0.);
        NodeStyleSetBottom(fixed, 0.);
        tree.layout();
        assert_eq!(Tree::position(fixed), (380., 780.));
    }
}
