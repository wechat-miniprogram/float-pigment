#[cfg(test)]
mod custom;
#[cfg(test)]
mod wpt;
use euclid::num::Zero;
pub use float_pigment_css::length_num::LengthNum;
use float_pigment_css::{
    parser::parse_inline_style,
    property::{NodeProperties, Property, PropertyValueWithGlobal},
    sheet::PropertyMeta,
    typing::{AspectRatio, Display, Gap, GridTemplate, TrackListItem, TrackSize},
};
pub use float_pigment_forest::Len;
use float_pigment_forest::{layout::LayoutPosition, node::Length, *};
use float_pigment_layout::{DefLength, LayoutGridTemplate, LayoutTrackListItem, LayoutTrackSize};
use float_pigment_mlp::{
    context::{Context, Parse},
    node::{attribute::Attribute, NodeType},
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

#[derive(Default, Debug)]
pub struct PartialLayoutPosition {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub top: Option<f32>,
    pub left: Option<f32>,
}

impl PartialEq<LayoutPosition> for PartialLayoutPosition {
    fn eq(&self, other: &LayoutPosition) -> bool {
        if let Some(width) = self.width {
            if width != other.width.to_f32().round() {
                return false;
            }
        }
        if let Some(height) = self.height {
            if height != other.height.to_f32().round() {
                return false;
            }
        }
        if let Some(top) = self.top {
            if top != other.top.to_f32().round() {
                return false;
            }
        }
        if let Some(left) = self.left {
            if left != other.left.to_f32().round() {
                return false;
            }
        }
        true
    }
}
type NodeId = usize;
type PaintPos = FxHashMap<*const Node, (LayoutPosition, Color)>;

#[derive(Debug)]
pub enum Color {
    Rgba(u8, u8, u8, u8),
}

impl Color {
    fn rgba_to_tuple(&self) -> (u8, u8, u8, u8) {
        match *self {
            Self::Rgba(r, g, b, a) => (r, g, b, a),
        }
    }
}

#[derive(Debug)]
pub struct TestCtx {
    pub root: Option<NodePtr>,
    pub layout_pos: FxHashMap<*const Node, LayoutPosition>,
    pub expect_layout_pos: FxHashMap<*const Node, PartialLayoutPosition>,
    pub paint_pos: PaintPos,
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

#[inline(always)]
fn is_measure_text_slot(tag: &str) -> bool {
    tag == "text-slot"
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

impl TestCtx {
    pub fn new() -> Self {
        Self {
            root: None,
            layout_pos: FxHashMap::default(),
            expect_layout_pos: FxHashMap::default(),
            paint_pos: FxHashMap::default(),
        }
    }
    pub fn gen_node_id() -> NodeId {
        NODE_ID.with(|x| {
            x.replace(x.get() + 1);
            x.get()
        })
    }
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &str) -> Option<Self> {
        let mut ctx = Self::new();
        let mut parse_ctx = Context::create(None);
        parse_ctx.parse(input);
        if let Some(tree) = parse_ctx.tree() {
            if let Some(root) = tree.root() {
                unsafe {
                    let root_node: *mut Node = ctx.create_node_recursive(root, None);
                    ctx.root = Some(root_node);
                    return Some(ctx);
                }
            }
        }
        None
    }
    pub fn update_layout_pos_recursive(&mut self, node: &Node, parent_offset: Option<(Len, Len)>) {
        let position = node.layout_position();
        self.layout_pos.insert(node as *const Node, position);
        if let Some(offset) = parent_offset {
            self.paint_pos.insert(
                node as *const Node,
                (
                    LayoutPosition {
                        width: position.width,
                        height: position.height,
                        left: offset.0 + position.left,
                        top: offset.1 + position.top,
                    },
                    Color::Rgba(rand::random(), rand::random(), rand::random(), 1),
                ),
            );
        }

        unsafe {
            node.for_each_child_node(|node, _| {
                self.update_layout_pos_recursive(
                    node,
                    Some((
                        parent_offset.unwrap_or((Len::zero(), Len::zero())).0 + position.left,
                        parent_offset.unwrap_or((Len::zero(), Len::zero())).1 + position.top,
                    )),
                )
            });
        }
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
                self.update_layout_pos_recursive(&*root, None);

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
    #[inline]
    pub fn assert(&mut self) {
        self.expect_layout_pos.iter().for_each(|(id, expect_pos)| {
            if let Some(layout_pos) = self.layout_pos.get(id) {
                // println!("layout_pos {:?}, expect_pos {:?}", layout_pos, expect_pos);
                assert_eq!(expect_pos, layout_pos);
            }
        });
    }
    #[cfg(target_os = "macos")]
    pub fn render(&self) {
        use piston_window::{
            clear, EventLoop, EventSettings, Graphics, PistonWindow, Rectangle, WindowSettings,
        };
        let mut window_settings = WindowSettings::new("test", [375, 750]);
        window_settings.set_resizable(false);
        window_settings.set_exit_on_esc(false);
        let mut window: PistonWindow = window_settings.samples(4).build().unwrap();
        let mut event_settings = EventSettings::new();
        event_settings.set_lazy(true);
        window.set_event_settings(event_settings);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([1., 1., 1., 1.], g);
                g.clear_stencil(0);
                self.paint_pos.iter().for_each(|(_id, (rect, color))| {
                    let rgba = color.rgba_to_tuple();
                    Rectangle::new([rgba.0 as f32, rgba.1 as f32, rgba.2 as f32, rgba.3 as f32])
                        .draw(
                            [
                                rect.left.to_f32() as f64,
                                rect.top.to_f32() as f64,
                                rect.width.to_f32() as f64,
                                rect.height.to_f32() as f64,
                            ],
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                });
            });
        }
    }
    /// # Safety
    ///
    pub unsafe fn create_node_recursive(
        &mut self,
        cur: &NodeType,
        parent_node_props: Option<&NodeProperties>,
    ) -> NodePtr {
        match cur {
            NodeType::Text(t) => {
                let node = Node::new_ptr();
                let mut text_info_builder = TextInfoBuilder::new().with_text_len(t.text().len());
                if let Some(parent_node_props) = parent_node_props {
                    let font_size = parent_node_props.font_size();
                    if let float_pigment_css::typing::Length::Px(x) = font_size {
                        text_info_builder.set_font_size(x);
                    }
                }
                prepare_measure_node(node, text_info_builder.build());
                node
            }
            NodeType::Element(e) => {
                let node = Node::new_ptr();
                if !is_block_tag(e.tag()) {
                    (*node).set_display(float_pigment_css::typing::Display::Inline);
                }

                let mut node_props = NodeProperties::new(parent_node_props);
                if let Some(style) = e.attributes().get("style") {
                    unsafe {
                        TestCtx::set_style(&*node, &style, &mut node_props, parent_node_props);
                    }
                }
                self.set_expect_layout_pos(node, e.attributes());

                if is_measure_text_slot(e.tag()) {
                    let text_len = e
                        .attributes()
                        .get("len")
                        .map(|v| v.parse::<usize>().unwrap())
                        .unwrap_or(0);
                    let font_size = e
                        .attributes()
                        .get("fontSize")
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap_or(16.);
                    let text_info = TextInfoBuilder::new()
                        .with_text_len(text_len)
                        .with_font_size(font_size)
                        .build();
                    prepare_measure_node(node, text_info);
                }

                e.children_mut().iter().for_each(|item| {
                    let child = self.create_node_recursive(item, Some(&node_props));
                    (*node).append_child(child);
                });
                node
            }
            NodeType::Fragment(e) => {
                let node = Node::new_ptr();
                e.children_mut().iter().for_each(|item| {
                    let child = self.create_node_recursive(item, None);
                    (*node).append_child(child);
                });
                node
            }
        }
    }

    pub fn set_expect_layout_pos(&mut self, node_ptr: *const Node, attrs: &Attribute) {
        let mut pos = PartialLayoutPosition::default();
        if let Some(v) = attrs.get("expect_width") {
            pos.width = Some(v.parse::<f32>().unwrap())
        }
        if let Some(v) = attrs.get("expect_height") {
            pos.height = Some(v.parse::<f32>().unwrap())
        }
        if let Some(v) = attrs.get("expect_top") {
            pos.top = Some(v.parse::<f32>().unwrap())
        }
        if let Some(v) = attrs.get("expect_left") {
            pos.left = Some(v.parse::<f32>().unwrap())
        }
        self.expect_layout_pos.insert(node_ptr, pos);
    }

    // style
    pub unsafe fn set_style(
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
                _ => {}
            }
        });

        warnings.iter().for_each(|w| {
            println!("{w:?}");
        });
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

#[macro_export]
macro_rules! assert_xml {
    ($xml: expr) => {{
        let mut ctx = TestCtx::from_str($xml).unwrap();
        ctx.layout(false);
        ctx.assert();
    }};
    ($xml: expr, $dump_style: expr) => {{
        let mut ctx = TestCtx::from_str($xml).unwrap();
        ctx.layout($dump_style);
        ctx.assert();
    }};
}

#[macro_export]
#[cfg(target_os = "macos")]
macro_rules! render_xml {
    ($xml: expr) => {{
        let mut ctx = TestCtx::from_str($xml).unwrap();
        ctx.layout(false);
        ctx.render();
    }};
}

#[test]
fn test_from_str() {
    let input = r#"
    <div id="1" style="width: 100px; height: 100px;">
      <div id="2" style="display: inline">hello</div>
    </div>
  "#;
    let ctx = TestCtx::from_str(input).unwrap();
    if let Some(root) = ctx.root {
        unsafe {
            let root = &*root;
            root.layout_with_containing_size(
                OptionSize::new(
                    OptionNum::some(Len::from_f32(375.)),
                    OptionNum::some(Len::from_f32(750.)),
                ),
                Size::new(Len::from_f32(375.), Len::from_f32(750.)),
                OptionSize::new(
                    OptionNum::some(Len::from_f32(375.)),
                    OptionNum::some(Len::from_f32(750.)),
                ),
            );
            println!(
                "{}",
                (*root).dump_to_html(
                    DumpOptions {
                        recursive: true,
                        layout: true,
                        style: DumpStyleMode::None,
                    },
                    1,
                )
            );
            assert_eq!(root.children_len(), 1);
            let children = root.children();
            let child = children.first().unwrap();
            assert_eq!(child.children_len(), 1);
        }
    }
}
