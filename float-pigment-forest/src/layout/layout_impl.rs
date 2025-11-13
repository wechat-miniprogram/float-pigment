use float_pigment_css::length_num::*;
use float_pigment_css::typing::{GridAutoFlow, TextAlign};
use float_pigment_css::{
    num_traits::Zero,
    typing::{
        AlignContent, AlignItems, AlignSelf, BoxSizing, Direction, Display, FlexDirection,
        FlexWrap, JustifyContent, Position, WritingMode,
    },
};
use float_pigment_layout::{
    DefLength, EdgeOption, InlineMeasure, InlineUnit, InlineUnitMetadata, LayoutNode, LayoutStyle,
    LayoutTreeNode, LayoutTreeVisitor, MeasureResult, OptionNum, OptionSize, Point, Size,
    SizingMode, Vector,
};

use crate::{convert_node_ref_to_ptr, LayoutGridTemplate, Length};
use crate::{
    env::Env,
    node::{ChildOperation, Node},
    Len, MeasureMode, NodeType,
};

fn is_specified(x: Len) -> bool {
    x != Len::MAX
}

impl LayoutTreeNode for Node {
    type Length = Len;
    type LengthCustom = i32;
    type TreeVisitor = Node;
    type Style = Node;
    type InlineUnit = LayoutInlineUnit;
    type InlineMeasure = LayoutInlineMeasure;
    type Env = Env;

    #[inline]
    fn layout_node(&self) -> &LayoutNode<Self> {
        &self.layout_node
    }

    #[inline]
    fn tree_visitor(&self) -> &Self::TreeVisitor {
        self
    }

    #[inline]
    fn style(&self) -> &Self::Style {
        self
    }

    #[inline]
    fn resolve_custom_length(
        &self,
        custom: &Self::LengthCustom,
        owner: Self::Length,
    ) -> Self::Length {
        if let Some(func) = self.resolve_calc() {
            return func(*custom, owner);
        };
        Len::zero()
    }

    #[inline]
    fn should_measure(&self, _env: &mut Self::Env) -> bool {
        self.has_measure_func()
    }

    #[inline]
    fn measure_block_size(
        &self,
        _env: &mut Self::Env,
        req_size: OptionSize<Self::Length>,
        min: Size<Self::Length>,
        max: Size<Self::Length>,
        max_content: OptionSize<Self::Length>,
        _update_position: bool,
        _sizing_mode: SizingMode,
    ) -> MeasureResult<Self::Length> {
        let width = req_size.width.val();
        let height = req_size.height.val();
        let mut size = Size::new(Len::zero(), Len::zero());
        let mut skip_measure = false;
        if let (Some(width), Some(height)) = (width, height) {
            if self.style().width() != DefLength::Auto
                && self.style().height() != DefLength::Auto
                && is_specified(width)
                && is_specified(height)
            {
                size = Size::new(width, height);
                skip_measure = true
            }
        } else if let Some(ratio) = self.style().aspect_ratio() {
            if let Some(height) = height {
                if is_specified(height) {
                    size = Size::new(height.mul_f32(ratio), height);
                    skip_measure = true;
                }
            } else if let Some(width) = width {
                if is_specified(width) {
                    size = Size::new(width, width.div_f32(ratio));
                    skip_measure = true;
                }
            }
        }
        if !skip_measure {
            if let Some(func) = unsafe { self.measure_func() } {
                let mut width_measure_mode = MeasureMode::AtMost;
                let mut height_measure_mode = MeasureMode::AtMost;
                let (min_width, max_width) = if let Some(req_size_width) = req_size.width.val() {
                    let min_width = req_size_width;
                    let max_width = req_size_width;
                    width_measure_mode = MeasureMode::Exactly;
                    (min_width, max_width)
                } else {
                    let min_width = if !is_specified(min.width) {
                        Len::zero()
                    } else {
                        min.width
                    };
                    let max_width = max.width;
                    (min_width, max_width)
                };
                let (min_height, max_height) = if let Some(req_size_height) = req_size.height.val()
                {
                    let min_height = req_size_height;
                    let max_height = req_size_height;
                    height_measure_mode = MeasureMode::Exactly;
                    (min_height, max_height)
                } else {
                    let min_height = if !is_specified(min.height) {
                        Len::zero()
                    } else {
                        min.height
                    };
                    let max_height = max.height;
                    (min_height, max_height)
                };
                let mut size_from_cache = false;
                if self.node_type() == NodeType::Text {
                    if let Some(cache) = unsafe { self.measure_cache() }.as_mut() {
                        if let Some(size_cache) = cache.get(&(
                            OptionSize::new(
                                OptionNum::some(min_width).to_hashable(),
                                OptionNum::some(min_height).to_hashable(),
                            ),
                            OptionSize::new(
                                OptionNum::some(max_width).to_hashable(),
                                OptionNum::some(max_height).to_hashable(),
                            ),
                            OptionSize::new(
                                max_content.width.to_hashable(),
                                max_content.height.to_hashable(),
                            ),
                        )) {
                            size = *size_cache;
                            size_from_cache = true;
                        }
                    }
                }
                if !size_from_cache {
                    let measure_size = func(
                        convert_node_ref_to_ptr(self),
                        max_width,
                        width_measure_mode,
                        max_height,
                        height_measure_mode,
                        min_width,
                        min_height,
                        max_content.width.unwrap_or(max_width),
                        max_content.height.unwrap_or(max_height),
                    );
                    let width = if is_specified(measure_size.width) {
                        measure_size.width
                    } else {
                        Len::zero()
                    };
                    let height = if is_specified(measure_size.height) {
                        measure_size.height
                    } else {
                        Len::zero()
                    };
                    let measure_size = Size::new(width, height);
                    size = Size::new(
                        measure_size.width.clamp(min.width, max.width),
                        measure_size.height.clamp(min.height, max.height),
                    );
                    if self.node_type() == NodeType::Text {
                        if let Some(cache) = unsafe { self.measure_cache() }.as_mut() {
                            cache.put(
                                (
                                    OptionSize::new(
                                        OptionNum::some(min_width),
                                        OptionNum::some(min_height),
                                    ),
                                    OptionSize::new(
                                        OptionNum::some(max_width),
                                        OptionNum::some(max_height),
                                    ),
                                    OptionSize::new(
                                        max_content.width.to_hashable(),
                                        max_content.height.to_hashable(),
                                    ),
                                ),
                                size,
                            );
                        }
                    }
                }
            };
        }
        let mut baseline = size.to_vector();
        let mut baseline_from_cache = false;
        if self.node_type() == NodeType::Text {
            if let Some(cache) = unsafe { self.baseline_cache() }.as_mut() {
                if let Some(baseline_cache) = cache.get(&Size::new(size.width, size.height)) {
                    baseline_from_cache = true;
                    baseline = Vector::new(Len::zero(), *baseline_cache);
                }
            }
        }
        if !baseline_from_cache {
            if let Some(func) = unsafe { self.baseline_func() } {
                let ret = func(convert_node_ref_to_ptr(self), size.width, size.height);
                baseline = Vector::new(Len::zero(), ret);
                if let Some(cache) = unsafe { self.baseline_cache() }.as_mut() {
                    cache.put(Size::new(size.width, size.height), ret);
                }
            }
        }
        MeasureResult {
            size,
            first_baseline_ascent: baseline,
            last_baseline_ascent: baseline,
        }
    }

    #[inline]
    fn measure_inline_unit(
        &self,
        env: &mut Self::Env,
        req_size: OptionSize<Self::Length>,
        min: Size<Self::Length>,
        max: Size<Self::Length>,
        max_content: OptionSize<Self::Length>,
        sizing_mode: SizingMode,
    ) -> MeasureResult<Self::Length> {
        self.measure_block_size(env, req_size, min, max, max_content, false, sizing_mode)
    }
}

impl LayoutTreeVisitor<Node> for Node {
    #[inline]
    fn parent(&self) -> Option<&Node> {
        unsafe { Node::parent(self) }
    }

    #[inline]
    fn for_each_child<'a, 'b: 'a, F>(&'b self, f: F)
    where
        F: FnMut(&'a Node, usize),
        Node: 'a,
    {
        unsafe { self.for_each_child_node(f) }
    }

    #[inline]
    fn children_len(&self) -> usize {
        Node::children_len(self)
    }

    #[inline]
    fn child_at(&self, index: usize) -> Option<&Node> {
        unsafe { self.get_child_at(index) }
    }

    #[inline]
    fn children_iter<'a, 'b: 'a>(&'b self) -> impl Iterator<Item = &'a Node>
    where
        Node: 'a,
    {
        unsafe { self.children().into_iter() }
    }
}
#[derive(Debug, Clone)]
pub struct LayoutInlineUnit {
    offset: Point<Len>,
    size: Size<Len>,
    first_baseline_ascent: Vector<Len>,
    last_baseline_ascent: Vector<Len>,
}

impl LayoutInlineUnit {
    fn to_tuple(&self) -> (Point<Len>, MeasureResult<Len>) {
        (
            self.offset,
            MeasureResult {
                size: self.size,
                first_baseline_ascent: self.first_baseline_ascent,
                last_baseline_ascent: self.last_baseline_ascent,
            },
        )
    }
}

impl InlineUnit<Node> for LayoutInlineUnit {
    type Env = Env;
    fn new(_env: &mut Env, _node: &Node, res: MeasureResult<Len>) -> Self {
        Self {
            offset: Point::zero(),
            size: res.size,
            first_baseline_ascent: res.first_baseline_ascent,
            last_baseline_ascent: res.last_baseline_ascent,
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    inline_units: Vec<LayoutInlineUnit>,
    total_inline_size: Len,
    total_block_size: Len,
    block_start: Len,
    inline_offset: Len,
    first_baseline_ascent: Vector<Len>,
    // text_align: TextAlign,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            inline_units: vec![],
            total_inline_size: Len::zero(),
            total_block_size: Len::zero(),
            block_start: Len::zero(),
            inline_offset: Len::zero(),
            first_baseline_ascent: Vector::default(),
            // text_align: TextAlign::Start,
        }
    }
}

impl Line {
    fn is_empty(&self) -> bool {
        self.inline_units.is_empty()
    }

    fn collect_inline_unit(&mut self, mut inline_unit: LayoutInlineUnit, margin: EdgeOption<Len>) {
        inline_unit.offset.x += self.total_inline_size + margin.left.or_zero();
        inline_unit.offset.y += self.block_start + margin.top.or_zero();
        self.total_inline_size += inline_unit.size.width + margin.horizontal();
        self.total_block_size = self
            .total_block_size
            .max(inline_unit.size.height + margin.vertical());
        self.first_baseline_ascent = self
            .first_baseline_ascent
            .max(inline_unit.first_baseline_ascent);
        self.inline_units.push(inline_unit);
    }

    fn adjust_inline_offset(&mut self) {
        self.inline_units.iter_mut().for_each(|inline_unit| {
            inline_unit.offset.x += self.inline_offset;
        })
    }

    fn adjust_block_offset(&mut self) {
        // TODO affected by vertical-align
        self.inline_units.iter_mut().for_each(|inline_unit| {
            let max_baseline = self.first_baseline_ascent.y;
            let self_baseline = inline_unit.first_baseline_ascent.y;
            inline_unit.offset.y += max_baseline - self_baseline;
        })
    }

    fn set_inline_offset(&mut self, inline_offset: Len) {
        self.inline_offset = inline_offset
    }
}

pub struct LayoutInlineMeasure {}
impl InlineMeasure<Node> for LayoutInlineMeasure {
    type InlineUnit = LayoutInlineUnit;
    type Env = Env;

    fn block_size(
        _env: &mut Env,
        block_node: &Node,
        inline_nodes: Vec<InlineUnitMetadata<Node>>,
        req_size: OptionSize<Len>,
        _max_content_with_max_size: OptionSize<Len>,
        _update_position: bool,
        sizing_mode: SizingMode,
    ) -> (Size<Len>, Vec<(Point<Len>, MeasureResult<Len>)>) {
        let suggested_width = req_size.width;
        let suggested_height = req_size.height;
        if inline_nodes.is_empty() {
            return (
                Size::new(
                    suggested_width.unwrap_or(Len::zero()),
                    suggested_height.unwrap_or(Len::zero()),
                ),
                Vec::with_capacity(0),
            );
        }
        let mut lines: Vec<Line> = vec![];
        let mut prev_line_block_start = Len::zero();
        let mut current_line = Line::default();
        if let Some(suggested_width) = suggested_width.val() {
            inline_nodes
                .into_iter()
                .for_each(|InlineUnitMetadata { unit, margin }| {
                    if (current_line.total_inline_size + unit.size.width + margin.horizontal()
                        > suggested_width)
                        && !current_line.is_empty()
                    {
                        prev_line_block_start += current_line.total_block_size;
                        lines.push(current_line.clone());
                        current_line = Line::default();
                        current_line.block_start = prev_line_block_start;
                    }
                    current_line.collect_inline_unit(unit, margin);
                });
        } else {
            inline_nodes
                .into_iter()
                .for_each(|InlineUnitMetadata { unit, margin }| {
                    current_line.collect_inline_unit(unit, margin);
                });
        }
        if !current_line.is_empty() {
            lines.push(current_line.clone());
        }
        let (mut block_width, mut block_height) = (Len::zero(), Len::zero());
        lines.iter_mut().for_each(|line| {
            block_width = block_width.max(line.total_inline_size);
            block_height += line.total_block_size;
            line.adjust_block_offset();
        });
        let block_size = match sizing_mode {
            SizingMode::Normal => Size::new(
                suggested_width.unwrap_or(block_width),
                suggested_height.unwrap_or(block_height),
            ),
            SizingMode::MinContent => Size::new(block_width, block_height),
            SizingMode::MaxContent => Size::new(block_width, block_height),
        };
        if let Some(suggested_width) = suggested_width.val() {
            if suggested_width > block_width {
                let text_align = block_node.style().text_align();
                match text_align {
                    TextAlign::Start | TextAlign::Left => {}
                    TextAlign::End | TextAlign::Right => lines.iter_mut().for_each(|line| {
                        let inline_offset = suggested_width - line.total_inline_size;
                        line.set_inline_offset(inline_offset);
                        line.adjust_inline_offset()
                    }),
                    TextAlign::Center => lines.iter_mut().for_each(|line| {
                        let inline_offset = (suggested_width - line.total_inline_size).div_f32(2.);
                        line.set_inline_offset(inline_offset);
                        line.adjust_inline_offset()
                    }),
                    TextAlign::Justify => {}
                    TextAlign::JustifyAll => {}
                    TextAlign::MatchParent => {}
                }
            }
        }
        let detailed_position = lines
            .into_iter()
            .flat_map(|line| {
                line.inline_units
                    .into_iter()
                    .map(|inline_unit| inline_unit.to_tuple())
            })
            .collect();
        (block_size, detailed_position)
    }
}

impl LayoutStyle<Len> for Node {
    #[inline]
    fn display(&self) -> Display {
        self.style_manager().display()
    }

    #[inline]
    fn position(&self) -> Position {
        self.style_manager().position()
    }

    #[inline]
    fn box_sizing(&self) -> BoxSizing {
        self.style_manager().box_sizing()
    }

    #[inline]
    fn direction(&self) -> Direction {
        self.style_manager().direction()
    }

    #[inline]
    fn writing_mode(&self) -> WritingMode {
        self.style_manager().writing_mode()
    }

    #[inline]
    fn flex_direction(&self) -> FlexDirection {
        self.style_manager().flex_direction()
    }

    #[inline]
    fn flex_wrap(&self) -> FlexWrap {
        self.style_manager().flex_wrap()
    }

    #[inline]
    fn flex_grow(&self) -> f32 {
        self.style_manager().flex_grow()
    }

    #[inline]
    fn flex_shrink(&self) -> f32 {
        self.style_manager().flex_shrink()
    }

    #[inline]
    fn align_items(&self) -> AlignItems {
        self.style_manager().align_items()
    }

    #[inline]
    fn align_self(&self) -> AlignSelf {
        self.style_manager().align_self()
    }

    #[inline]
    fn align_content(&self) -> AlignContent {
        self.style_manager().align_content()
    }

    #[inline]
    fn justify_content(&self) -> JustifyContent {
        self.style_manager().justify_content()
    }

    #[inline]
    fn left(&self) -> Length {
        self.style_manager().left()
    }

    #[inline]
    fn right(&self) -> Length {
        self.style_manager().right()
    }

    #[inline]
    fn top(&self) -> Length {
        self.style_manager().top()
    }

    #[inline]
    fn bottom(&self) -> Length {
        self.style_manager().bottom()
    }

    #[inline]
    fn border_left(&self) -> Length {
        self.style_manager().border_left()
    }

    #[inline]
    fn border_right(&self) -> Length {
        self.style_manager().border_right()
    }

    #[inline]
    fn border_top(&self) -> Length {
        self.style_manager().border_top()
    }

    #[inline]
    fn border_bottom(&self) -> Length {
        self.style_manager().border_bottom()
    }

    #[inline]
    fn margin_left(&self) -> Length {
        self.style_manager().margin_left()
    }

    #[inline]
    fn margin_right(&self) -> Length {
        self.style_manager().margin_right()
    }

    #[inline]
    fn margin_top(&self) -> Length {
        self.style_manager().margin_top()
    }

    #[inline]
    fn margin_bottom(&self) -> Length {
        self.style_manager().margin_bottom()
    }

    #[inline]
    fn padding_left(&self) -> Length {
        self.style_manager().padding_left()
    }

    #[inline]
    fn padding_right(&self) -> Length {
        self.style_manager().padding_right()
    }

    #[inline]
    fn padding_top(&self) -> Length {
        self.style_manager().padding_top()
    }

    #[inline]
    fn padding_bottom(&self) -> Length {
        self.style_manager().padding_bottom()
    }

    #[inline]
    fn flex_basis(&self) -> Length {
        self.style_manager().flex_basis()
    }

    #[inline]
    fn width(&self) -> Length {
        self.style_manager().width()
    }

    #[inline]
    fn height(&self) -> Length {
        self.style_manager().height()
    }

    #[inline]
    fn min_width(&self) -> Length {
        self.style_manager().min_width()
    }

    #[inline]
    fn min_height(&self) -> Length {
        self.style_manager().min_height()
    }

    #[inline]
    fn max_width(&self) -> Length {
        self.style_manager().max_width()
    }

    #[inline]
    fn max_height(&self) -> Length {
        self.style_manager().max_height()
    }

    #[inline]
    fn aspect_ratio(&self) -> Option<f32> {
        self.style_manager().aspect_ratio()
    }

    #[inline]
    fn order(&self) -> i32 {
        self.style_manager().order()
    }

    #[inline]
    fn text_align(&self) -> TextAlign {
        self.style_manager().text_align()
    }

    #[inline]
    fn row_gap(&self) -> Length {
        self.style_manager().row_gap()
    }

    #[inline]
    fn column_gap(&self) -> Length {
        self.style_manager().column_gap()
    }
    #[inline]
    fn grid_template_rows(&self) -> LayoutGridTemplate {
        self.style_manager().grid_template_rows()
    }

    #[inline]
    fn grid_template_columns(&self) -> LayoutGridTemplate {
        self.style_manager().grid_template_columns()
    }

    #[inline]
    fn grid_auto_flow(&self) -> GridAutoFlow {
        self.style_manager().grid_auto_flow()
    }
}
