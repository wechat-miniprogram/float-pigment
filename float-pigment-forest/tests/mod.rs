#[cfg(test)]
mod custom;
use euclid::num::Zero;
pub use float_pigment_css::length_num::LengthNum;
use float_pigment_css::{
    parser::parse_inline_style,
    property::{NodeProperties, Property, PropertyValueWithGlobal},
    sheet::PropertyMeta,
    typing::{AspectRatio, Display, Gap, GridAuto, GridTemplate, TrackListItem, TrackSize},
};
pub use float_pigment_forest::Len;
use float_pigment_forest::{node::Length, *};
use float_pigment_layout::{
    DefLength, LayoutGridAuto, LayoutGridTemplate, LayoutTrackListItem, LayoutTrackSize,
    LayoutTreeNode,
};

use rustc_hash::FxHashMap;
use std::cell::Cell;

thread_local! {
static NODE_ID: Cell<usize> = const { Cell::new(1000) };
}

pub(crate) fn def_length(length: float_pigment_css::typing::Length) -> Length {
    match length {
        float_pigment_css::typing::Length::Auto => DefLength::Auto,
        float_pigment_css::typing::Length::Undefined => DefLength::Undefined,
        float_pigment_css::typing::Length::Px(x) => DefLength::Points(Len::from_f32(x)),
        float_pigment_css::typing::Length::Ratio(x) => DefLength::Percent(x),
        _ => todo!(),
    }
}

type NodeId = usize;

#[derive(Debug)]
pub struct TestCtx {
    pub root: Option<NodePtr>,
    /// 命令式 codegen 录制的构建计划（Task 2: record-only，Task 3 在 layout 时消费）
    build_nodes: Vec<BuildNode>,
    /// Task 3: build_dfs 构建出的真实 Node 指针（按 build_nodes 索引）
    imperative_built: Vec<*mut Node>,
    /// Task 3: 命令式树根指针
    imperative_root: Option<*mut Node>,
}

impl Default for TestCtx {
    fn default() -> Self {
        Self::new()
    }
}

#[inline(always)]
fn is_block_tag(tag: &str) -> bool {
    tag == "div" || tag == "view"
}

#[derive(Debug, Default, Clone)]
struct TextInfo {
    font_size: f32,
    // raw_text: String
    text_len: usize,
}

impl TextInfo {
    fn measure(
        &self,
        min_width: Len,
        min_height: Len,
        max_width: Len,
        max_height: Len,
        max_content_width: Len,
        max_content_height: Len,
    ) -> Size<Len> {
        let text_len = self.text_len;
        if text_len == 0 {
            println!(
                "text_info: {self:?}, width: {min_width:?} ~ {max_width:?}, height: {min_height:?} ~ {max_height:?}, max_content_width: {max_content_width:?}, max_content_height: {max_content_height:?}, measured_width: 0, measured_height: 0",
            );
            return Size::new(Len::zero(), Len::zero());
        }
        let text_width = self.font_size * text_len as f32;
        let max_w = max_width.min(max_content_width);
        let max_h = max_height.min(max_content_height);
        let measured_width;
        let measured_height;
        if text_width <= max_w {
            // single line
            measured_width = Len::from_f32(text_width);
            measured_height = Len::from_f32(self.font_size);
        } else {
            // multi line
            let mut row_count = (max_w.to_f32() / self.font_size).floor();
            if row_count < 1. {
                row_count = 1.;
            }
            let col_count = (text_len as f32 / row_count).ceil();
            measured_width = Len::from_f32((row_count * self.font_size) as i32 as f32);
            measured_height = Len::from_f32((col_count * self.font_size) as i32 as f32);
        }
        println!(
            "text_info: {self:?}, width: {min_width:?} ~ {max_width:?}, height: {min_height:?} ~ {max_height:?}, max_content_width: {max_content_width:?}, max_content_height: {max_content_height:?}, measured_width: {measured_width:?}, measured_height: {measured_height:?}",
        );
        Size::new(measured_width, measured_height.min(max_h))
    }
}

struct TextInfoBuilder(TextInfo);

impl TextInfoBuilder {
    fn new() -> Self {
        Self(TextInfo {
            font_size: 16.,
            // raw_text: String::new(),
            text_len: 0,
        })
    }
    fn with_text_len(mut self, text_len: usize) -> Self {
        self.0.text_len = text_len;
        self
    }
    fn with_font_size(mut self, font_size: f32) -> Self {
        self.0.font_size = font_size;
        self
    }
    fn set_font_size(&mut self, font_size: f32) {
        self.0.font_size = font_size;
    }
    fn build(self) -> TextInfo {
        self.0
    }
}

#[inline(always)]
fn convert_font_size_to_px(font_size: float_pigment_css::typing::Length) -> f32 {
    match font_size {
        float_pigment_css::typing::Length::Px(x) => x,
        _ => 16.,
    }
}

#[inline(always)]
fn prepare_measure_node(node: *mut Node, text_info: TextInfo) {
    let node = unsafe { &mut *node };
    unsafe {
        node.set_display(Display::Inline);
        node.set_node_type(float_pigment_forest::NodeType::Text);
    }
    node.set_baseline_func(Some(Box::new(|_, _, _| Len::from_f32(16.))));
    node.set_measure_func(Some(Box::new(
        move |_,
              max_width,
              _,
              max_height,
              _,
              min_width,
              min_height,
              max_content_width,
              max_content_height| {
            text_info.measure(
                min_width,
                min_height,
                max_width,
                max_height,
                max_content_width,
                max_content_height,
            )
        },
    )));
}

/// 测试中节点的安全句柄（ctx 内部 Vec 索引）
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeHandle(pub usize);

/// 构建计划：录制命令式调用，layout 时按拓扑构建真实 Node 树
#[allow(dead_code)]
#[derive(Debug)]
enum BuildNode {
    Element {
        tag: String,
        style: Option<String>,
        parent: Option<NodeHandle>,
        /// If Some, the element is a measure-text slot (mirrors the legacy
        /// `is_measure_text_slot` branch of `create_node_recursive`):
        /// `build_imperative_tree` will attach a TextInfo measure func with
        /// the given text length and font size.
        measure_text: Option<(usize, f32)>,
    },
    Text {
        text: String,
        parent: Option<NodeHandle>,
    },
}

impl BuildNode {
    fn set_parent(&mut self, p: NodeHandle) {
        match self {
            BuildNode::Element { parent, .. } => *parent = Some(p),
            BuildNode::Text { parent, .. } => *parent = Some(p),
        }
    }
}

impl TestCtx {
    pub fn new() -> Self {
        Self {
            root: None,
            build_nodes: Vec::new(),
            imperative_built: Vec::new(),
            imperative_root: None,
        }
    }

    // ===== 高层命令式 API（命令式 codegen 用）=====

    pub fn create_node(&mut self, tag: &str) -> NodeHandle {
        let h = NodeHandle(self.build_nodes.len());
        self.build_nodes.push(BuildNode::Element {
            tag: tag.to_string(),
            style: None,
            parent: None,
            measure_text: None,
        });
        h
    }

    pub fn create_text(&mut self, text: &str) -> NodeHandle {
        let h = NodeHandle(self.build_nodes.len());
        self.build_nodes.push(BuildNode::Text {
            text: text.to_string(),
            parent: None,
        });
        h
    }

    pub fn set_style(&mut self, n: NodeHandle, style: &str) {
        if let BuildNode::Element { style: s, .. } = &mut self.build_nodes[n.0] {
            // 非空才记录，空 style 省略（与 spec §7 一致）
            if !style.is_empty() {
                *s = Some(style.to_string());
            }
        }
    }

    /// Mark an element as a measure-text slot (mirrors the legacy
    /// `is_measure_text_slot` / `prepare_measure_node` branch of
    /// `create_node_recursive`). During `layout_imperative`, build_dfs will
    /// attach a TextInfo measure func to the node so the layout engine can
    /// compute intrinsic sizes from the synthetic text length.
    pub fn set_measure_text(&mut self, n: NodeHandle, text_len: usize, font_size: f32) {
        if let BuildNode::Element { measure_text, .. } = &mut self.build_nodes[n.0] {
            *measure_text = Some((text_len, font_size));
        }
    }

    pub fn append(&mut self, parent: NodeHandle, child: NodeHandle) {
        if let Some(node) = self.build_nodes.get_mut(child.0) {
            node.set_parent(parent);
        }
    }

    /// 命令式 API 的 layout 前置：从构建计划递归构建真实 Node 树。
    fn build_imperative_tree(&mut self) -> Option<*mut Node> {
        let root_idx = self
            .build_nodes
            .iter()
            .position(|n| matches!(n, BuildNode::Element { parent: None, .. }))?;
        let mut children_map: FxHashMap<NodeHandle, Vec<NodeHandle>> = FxHashMap::default();
        for (i, n) in self.build_nodes.iter().enumerate() {
            let parent = match n {
                BuildNode::Element { parent, .. } | BuildNode::Text { parent, .. } => *parent,
            };
            if let Some(p) = parent {
                children_map.entry(p).or_default().push(NodeHandle(i));
            }
        }
        self.imperative_built = vec![std::ptr::null_mut(); self.build_nodes.len()];
        let root = unsafe {
            build_dfs(
                NodeHandle(root_idx),
                None,
                &self.build_nodes,
                &children_map,
                &mut self.imperative_built,
            )
        };
        Some(root)
    }

    /// 命令式 API 的 layout：从构建计划构建树 → layout
    pub fn layout_imperative(&mut self) {
        if let Some(root) = self.build_imperative_tree() {
            unsafe {
                (*root).layout(
                    OptionSize::new(
                        OptionNum::some(Len::from_f32(375.)),
                        OptionNum::some(Len::from_f32(750.)),
                    ),
                    Size::new(Len::from_f32(375.), Len::from_f32(750.)),
                );
                self.imperative_root = Some(root);
            }
        }
    }

    // ===== 命令式 getters（layout_imperative 之后读节点的尺寸/位置/margin）=====

    pub fn width(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_position()
                .width
                .to_f32()
        }
    }
    pub fn height(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_position()
                .height
                .to_f32()
        }
    }
    pub fn left(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_position()
                .left
                .to_f32()
        }
    }
    pub fn top(&self, n: NodeHandle) -> f32 {
        unsafe { (*self.imperative_built[n.0]).layout_position().top.to_f32() }
    }
    pub fn margin_top(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_node()
                .computed_style()
                .margin
                .top
                .to_f32()
        }
    }
    pub fn margin_right(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_node()
                .computed_style()
                .margin
                .right
                .to_f32()
        }
    }
    pub fn margin_bottom(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_node()
                .computed_style()
                .margin
                .bottom
                .to_f32()
        }
    }
    pub fn margin_left(&self, n: NodeHandle) -> f32 {
        unsafe {
            (*self.imperative_built[n.0])
                .layout_node()
                .computed_style()
                .margin
                .left
                .to_f32()
        }
    }
    pub fn gen_node_id() -> NodeId {
        NODE_ID.with(|x| {
            x.replace(x.get() + 1);
            x.get()
        })
    }
    #[inline]
    pub fn layout(&mut self, dump_style: bool) {
        if let Some(root) = self.root {
            unsafe {
                (*root).layout(
                    OptionSize::new(
                        OptionNum::some(Len::from_f32(375.)),
                        OptionNum::some(Len::from_f32(750.)),
                    ),
                    Size::new(Len::from_f32(375.), Len::from_f32(750.)),
                );

                println!(
                    "{}",
                    (*root).dump_to_html(
                        DumpOptions {
                            recursive: true,
                            layout: true,
                            style: if dump_style {
                                DumpStyleMode::Mutation
                            } else {
                                DumpStyleMode::None
                            },
                        },
                        1,
                    )
                );
            }
        }
    }
    // style
    pub unsafe fn apply_inline_style(
        node: &Node,
        style: &str,
        node_props: &mut NodeProperties,
        parent_node_props: Option<&NodeProperties>,
    ) {
        let (props, warnings) = parse_inline_style(
            style,
            float_pigment_css::parser::StyleParsingDebugMode::None,
        );
        let mut font_size_p = float_pigment_css::typing::Length::Px(16.);
        for pm in props.iter() {
            match pm {
                PropertyMeta::Normal { property: p } | PropertyMeta::Important { property: p } => {
                    if let Property::FontSize(x) = p {
                        font_size_p = x
                            .to_inner(
                                parent_node_props.map(|p| p.font_size()).as_ref(),
                                float_pigment_css::typing::Length::Px(16.),
                                true,
                            )
                            .unwrap();
                    }
                }
                PropertyMeta::DebugGroup {
                    properties,
                    disabled,
                    ..
                } => {
                    if !disabled {
                        for p in &**properties {
                            if let Property::FontSize(x) = p {
                                font_size_p = x
                                    .clone()
                                    .to_inner(
                                        parent_node_props.map(|p| p.font_size()).as_ref(),
                                        float_pigment_css::typing::Length::Px(16.),
                                        true,
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }
            };
        }
        props.iter().for_each(|p| {
            p.merge_to_node_properties(
                node_props,
                parent_node_props,
                convert_font_size_to_px(font_size_p.clone()),
            );
            match p.get_property_name().to_string().as_str() {
                "display" => node.set_display(node_props.display()),
                "box-sizing" => node.set_box_sizing(node_props.box_sizing()),
                "direction" => node.set_direction(node_props.direction()),
                "writing-mode" => node.set_writing_mode(node_props.writing_mode()),
                "position" => node.set_position(node_props.position()),
                "left" => node.set_left(def_length(node_props.left())),
                "top" => node.set_top(def_length(node_props.top())),
                "right" => node.set_right(def_length(node_props.right())),
                "bottom" => node.set_bottom(def_length(node_props.bottom())),
                "overflow-x" => node.set_overflow_x(node_props.overflow_x()),
                "overflow-y" => node.set_overflow_y(node_props.overflow_y()),
                "width" => node.set_width(def_length(node_props.width())),
                "height" => node.set_height(def_length(node_props.height())),
                "min-width" => node.set_min_width(def_length(node_props.min_width())),
                "min-height" => node.set_min_height(def_length(node_props.min_height())),
                "max-width" => node.set_max_width(def_length(node_props.max_width())),
                "max-height" => node.set_max_height(def_length(node_props.max_height())),
                "margin-left" => node.set_margin_left(def_length(node_props.margin_left())),
                "margin-top" => node.set_margin_top(def_length(node_props.margin_top())),
                "margin-right" => node.set_margin_right(def_length(node_props.margin_right())),
                "margin-bottom" => node.set_margin_bottom(def_length(node_props.margin_bottom())),
                "padding-left" => node.set_padding_left(def_length(node_props.padding_left())),
                "padding-right" => node.set_padding_right(def_length(node_props.padding_right())),
                "padding-top" => node.set_padding_top(def_length(node_props.padding_top())),
                "padding-bottom" => {
                    node.set_padding_bottom(def_length(node_props.padding_bottom()))
                }
                "border-left-width" => {
                    node.set_border_left(def_length(node_props.border_left_width()))
                }
                "border-right-width" => {
                    node.set_border_right(def_length(node_props.border_right_width()))
                }
                "border-top-width" => {
                    node.set_border_top(def_length(node_props.border_top_width()))
                }
                "border-bottom-width" => {
                    node.set_border_bottom(def_length(node_props.border_bottom_width()))
                }
                "flex-grow" => node.set_flex_grow(node_props.flex_grow().to_f32()),
                "flex-basis" => node.set_flex_basis(def_length(node_props.flex_basis())),
                "flex-shrink" => node.set_flex_shrink(node_props.flex_shrink().to_f32()),
                "flex-direction" => node.set_flex_direction(node_props.flex_direction()),
                "flex-wrap" => node.set_flex_wrap(node_props.flex_wrap()),
                "justify-content" => node.set_justify_content(node_props.justify_content()),
                "align-content" => node.set_align_content(node_props.align_content()),
                "align-items" => node.set_align_items(node_props.align_items()),
                "align-self" => node.set_align_self(node_props.align_self()),
                "justify-items" => node.set_justify_items(node_props.justify_items()),
                "justify-self" => node.set_justify_self(node_props.justify_self()),
                "aspect-ratio" => match node_props.aspect_ratio() {
                    AspectRatio::Auto => node.set_aspect_ratio(None),
                    AspectRatio::Ratio(x, y) => {
                        node.set_aspect_ratio(Some(x.to_f32() / y.to_f32()))
                    }
                },
                "order" => node.set_order(node_props.order().to_i32()),
                "text-align" => node.set_text_align(node_props.text_align()),
                "gap" => {
                    node.set_row_gap({
                        match node_props.row_gap() {
                            Gap::Length(l) => def_length(l),
                            Gap::Normal => Length::Undefined,
                        }
                    });
                    node.set_column_gap({
                        match node_props.column_gap() {
                            Gap::Length(l) => def_length(l),
                            Gap::Normal => Length::Undefined,
                        }
                    });
                }
                "column-gap" => {
                    node.set_column_gap({
                        match node_props.column_gap() {
                            Gap::Length(l) => def_length(l),
                            Gap::Normal => Length::Undefined,
                        }
                    });
                }
                "row-gap" => {
                    node.set_row_gap({
                        match node_props.row_gap() {
                            Gap::Length(l) => def_length(l),
                            Gap::Normal => Length::Undefined,
                        }
                    });
                }
                "grid-template-rows" => {
                    node.set_grid_template_rows({
                        convert_grid_template(node_props.grid_template_rows())
                    });
                }
                "grid-template-columns" => {
                    node.set_grid_template_columns({
                        convert_grid_template(node_props.grid_template_columns())
                    });
                }
                "grid-auto-flow" => {
                    node.set_grid_auto_flow(node_props.grid_auto_flow());
                }
                "grid-auto-rows" => {
                    node.set_grid_auto_rows(convert_grid_auto(node_props.grid_auto_rows()));
                }
                "grid-auto-columns" => {
                    node.set_grid_auto_columns(convert_grid_auto(node_props.grid_auto_columns()));
                }
                _ => {}
            }
        });

        warnings.iter().for_each(|w| {
            println!("{w:?}");
        });
    }
}

/// DFS 构建真实 Node 树。自由函数避开 &self/&mut self 冲突。
unsafe fn build_dfs(
    handle: NodeHandle,
    parent_props: Option<&NodeProperties>,
    nodes: &[BuildNode],
    children_map: &FxHashMap<NodeHandle, Vec<NodeHandle>>,
    built: &mut [*mut Node],
) -> *mut Node {
    match &nodes[handle.0] {
        BuildNode::Element {
            tag,
            style,
            measure_text,
            ..
        } => {
            let node = Node::new_ptr();
            if !is_block_tag(tag) {
                (*node).set_display(float_pigment_css::typing::Display::Inline);
            }
            let mut node_props = NodeProperties::new(parent_props);
            if let Some(s) = style {
                TestCtx::apply_inline_style(&*node, s, &mut node_props, parent_props);
            }
            // Measure-text slot: convert the node into a Text measure node
            // (mirrors `prepare_measure_node` in the legacy
            // `create_node_recursive`). Done after style so the final node
            // type/display/measure/baseline funcs match the legacy path.
            if let Some((text_len, font_size)) = measure_text {
                let text_info = TextInfoBuilder::new()
                    .with_text_len(*text_len)
                    .with_font_size(*font_size)
                    .build();
                prepare_measure_node(node, text_info);
            }
            built[handle.0] = node;
            if let Some(children) = children_map.get(&handle) {
                for ch in children {
                    let child_node = build_dfs(*ch, Some(&node_props), nodes, children_map, built);
                    (*node).append_child(child_node);
                }
            }
            node
        }
        BuildNode::Text { text, .. } => {
            let node = Node::new_ptr();
            let mut tib = TextInfoBuilder::new().with_text_len(text.len());
            if let Some(pp) = parent_props {
                if let float_pigment_css::typing::Length::Px(x) = pp.font_size() {
                    tib.set_font_size(x);
                }
            }
            prepare_measure_node(node, tib.build());
            built[handle.0] = node;
            node
        }
    }
}

fn convert_grid_template(grid_template: GridTemplate) -> LayoutGridTemplate<Len> {
    match grid_template {
        GridTemplate::None => LayoutGridTemplate::None,
        GridTemplate::TrackList(x) => LayoutGridTemplate::TrackList({
            x.into_iter()
                .map(|x| match x {
                    TrackListItem::LineNames(line_names) => LayoutTrackListItem::LineNames(
                        line_names.into_iter().map(|x| x.to_string()).collect(),
                    ),
                    TrackListItem::TrackSize(track_size) => {
                        LayoutTrackListItem::TrackSize(match track_size {
                            TrackSize::MinContent => LayoutTrackSize::MinContent,
                            TrackSize::MaxContent => LayoutTrackSize::MaxContent,
                            TrackSize::Fr(x) => LayoutTrackSize::Fr(x),
                            TrackSize::Length(x) => LayoutTrackSize::Length(def_length(x)),
                        })
                    }
                })
                .collect()
        }),
    }
}

fn convert_grid_auto(grid_auto: GridAuto) -> LayoutGridAuto<Len> {
    match grid_auto {
        GridAuto::List(list) => LayoutGridAuto(
            list.into_iter()
                .map(|track_size| match track_size {
                    TrackSize::MinContent => LayoutTrackSize::MinContent,
                    TrackSize::MaxContent => LayoutTrackSize::MaxContent,
                    TrackSize::Fr(x) => LayoutTrackSize::Fr(x),
                    TrackSize::Length(x) => LayoutTrackSize::Length(def_length(x)),
                })
                .collect(),
        ),
    }
}

#[cfg(use_out_dir)]
include!(concat!(env!("OUT_DIR"), "/generated/all.rs"));
#[cfg(not(use_out_dir))]
mod generated;
