use crate::*;
use float_pigment_css::num_traits::{bounds::Bounded, Zero};

#[allow(clippy::type_complexity)]
pub(crate) struct LayoutUnit<T: LayoutTreeNode> {
    pub(crate) cache: LayoutComputeCache<T::Length>,
    pub(crate) result: Rect<T::Length>,
    pub(crate) result_padding_rect: Rect<T::Length>,
    pub(crate) result_content_rect: Rect<T::Length>,
    pub(crate) computed_style: ComputedStyle<T::Length>,
    pub(crate) layout_algorithm: LayoutAlgorithm,
}

impl<T: LayoutTreeNode> LayoutUnit<T> {
    pub(crate) fn new() -> Self {
        Self {
            cache: LayoutComputeCache::new(),
            result: Rect::zero(),
            result_padding_rect: Rect::zero(),
            result_content_rect: Rect::zero(),
            computed_style: ComputedStyle::default(),
            layout_algorithm: LayoutAlgorithm::None,
        }
    }

    pub(crate) fn mark_dirty(&mut self, node_tree_visitor: &T::TreeVisitor) -> bool {
        if !self.mark_self_dirty() {
            return false;
        }
        let mut cur = node_tree_visitor;
        while let Some(parent) = cur.parent() {
            parent.tree_visitor().dirty_marked();
            if !parent.layout_node().unit().mark_self_dirty() {
                break;
            }
            cur = parent.tree_visitor();
        }
        true
    }

    fn mark_self_dirty(&mut self) -> bool {
        self.cache.clear()
    }

    #[inline]
    pub(crate) fn result(&self) -> Rect<T::Length> {
        self.result
    }

    #[inline]
    pub(crate) fn result_padding_rect(&self) -> Rect<T::Length> {
        self.result_padding_rect
    }

    #[inline]
    pub(crate) fn result_content_rect(&self) -> Rect<T::Length> {
        self.result_content_rect
    }

    #[inline]
    pub(crate) fn compute_with_containing_size(
        &mut self,
        env: &mut T::Env,
        node: &T,
        available_size: OptionSize<T::Length>,
        containing_size: OptionSize<T::Length>,
    ) {
        let (margin, border, padding_border) = self.margin_border_padding(node, containing_size);
        let min_max_limit =
            self.normalized_min_max_limit(node, containing_size, border, padding_border);
        let mut css_size = self.css_border_box_size(node, containing_size, border, padding_border);
        css_size.width = css_size.width.or({
            match node.style().display() {
                Display::InlineBlock | Display::InlineFlex => OptionNum::none(),
                _ => available_size.width - margin.left - margin.right,
            }
        });
        css_size.height = css_size.height.or({
            match node.style().display() {
                Display::InlineBlock | Display::InlineFlex => OptionNum::none(),
                _ => available_size.height - margin.top - margin.bottom,
            }
        });
        let size = min_max_limit.normalized_size(css_size);
        let req = ComputeRequest {
            size,
            parent_inner_size: size,
            max_content: size,
            kind: ComputeRequestKind::Position,
            parent_is_block: node.style().display() == Display::Block
                || node.style().display() == Display::InlineBlock
                || node.style().display() == Display::Inline,
            sizing_mode: SizingMode::Normal,
        };
        let result = self.compute_internal(env, node, req);
        self.result = Rect::new(
            Point::new(margin.left.or_zero(), result.collapsed_margin.start.solve()),
            result.size.0,
        );

        // FIXME
        self.computed_style.margin.top = result.collapsed_margin.start.solve();
        self.computed_style.margin.left = margin.left.or_zero();
        self.computed_style.margin.right = margin.right.or_zero();
        self.computed_style.margin.bottom = result.collapsed_margin.end.solve();
    }

    #[inline]
    pub(crate) fn compute(&mut self, env: &mut T::Env, node: &T, size: OptionSize<T::Length>) {
        let size = Normalized(size);
        let req = ComputeRequest {
            size,
            parent_inner_size: size,
            max_content: size,
            kind: ComputeRequestKind::Position,
            parent_is_block: false,
            sizing_mode: SizingMode::Normal,
        };
        let result = self.compute_internal(env, node, req);
        self.result = Rect::new(Point::zero(), result.size.0);
    }

    #[inline]
    pub(crate) fn computed_style(&self) -> ComputedStyle<T::Length> {
        self.computed_style
    }

    pub(crate) fn clear_display_none_result(&mut self, node: &T) {
        // it is required to mark cache dirty here (although the cache is not used)
        self.cache.touch(node);
        self.cache.clear_position_cache();
        self.result = Rect::zero();
        self.result_padding_rect = Rect::zero();
        self.result_content_rect = Rect::zero();
        self.layout_algorithm = LayoutAlgorithm::None;
        node.tree_visitor().for_each_child(|child_node, _| {
            child_node
                .layout_node()
                .unit()
                .clear_display_none_result(child_node);
        });
    }

    pub(crate) fn compute_internal(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
    ) -> ComputeResult<T::Length> {
        let style = node.style();
        let mut layout_algorithm = match style.display() {
            Display::None => LayoutAlgorithm::None,
            Display::Block
            | Display::InlineBlock
            | Display::Inline => LayoutAlgorithm::Block,
            Display::Flex | Display::InlineFlex => LayoutAlgorithm::Flex,
            Display::Grid | Display::InlineGrid => LayoutAlgorithm::Grid,
            Display::FlowRoot => todo!(),
        };

        let ret = if let Some(r) = self.cache.read(node, &request) {
            // if cached, use the cache value
            // info!("!!! {:p} cache req {:?}", self, request);
            r
        } else {
            // do request
            // info!("!!! {:p} req {:?}", self, request);
            let (margin, border, padding_border) =
                self.margin_border_padding(node, *request.parent_inner_size);
            if let Some(ret) = self.compute_measure_block_if_exists(
                env,
                node,
                request.clone(),
                margin,
                border,
                padding_border,
            ) {
                layout_algorithm = LayoutAlgorithm::BlockMeasure;
                ret
            } else {
                match layout_algorithm {
                    LayoutAlgorithm::None => {
                        self.clear_display_none_result(node);
                        ComputeResult {
                            size: Normalized(Size::zero()),
                            min_content_size: Normalized(Size::zero()),
                            first_baseline_ascent: Vector::zero(),
                            last_baseline_ascent: Vector::zero(),
                            collapsed_margin: CollapsedBlockMargin::zero(),
                        }
                    }
                    LayoutAlgorithm::Block => {
                        algo::flow::Flow::compute(
                            self,
                            env,
                            node,
                            request.clone(),
                            margin,
                            border,
                            padding_border,
                        )
                    }
                    LayoutAlgorithm::Flex => algo::flex_box::FlexBox::compute(
                        self,
                        env,
                        node,
                        request.clone(),
                        margin,
                        border,
                        padding_border,
                    ),
                    LayoutAlgorithm::Grid => algo::grid::GridContainer::compute(
                        self,
                        env,
                        node,
                        request.clone(),
                        margin,
                        border,
                        padding_border,
                    ),
                    _ => unreachable!(),
                }
            }
        };

        if request.kind == ComputeRequestKind::Position {
            self.save_all_results(node, env, *request.parent_inner_size, layout_algorithm);
        }
        // info!("!!! {:p} res {:?} {:?}", self, request, ret);
        ret
    }

    pub(crate) fn save_all_results(
        &mut self,
        node: &T,
        env: &mut T::Env,
        parent_inner_size: OptionSize<T::Length>,
        layout_algorithm: LayoutAlgorithm,
    ) {
        let (margin, border, padding_border) = self.margin_border_padding(node, parent_inner_size);
        self.save_border_padding_result(border, padding_border);
        self.save_computed_style(margin, border, padding_border - border);
        self.layout_algorithm = layout_algorithm;
        node.size_updated(env, self.result.size, &self.computed_style);
    }

    pub(crate) fn update_result_layout_algorithm(&mut self, f: impl FnOnce(LayoutAlgorithm) -> LayoutAlgorithm) {
        self.layout_algorithm = f(self.layout_algorithm);
    }

    pub(crate) fn save_computed_style(
        &mut self,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding: Edge<T::Length>,
    ) {
        self.computed_style = ComputedStyle {
            margin: Edge {
                left: margin.left.or_zero(),
                right: margin.right.or_zero(),
                top: margin.top.or_zero(),
                bottom: margin.bottom.or_zero(),
            },
            border: Edge {
                left: border.left,
                right: border.right,
                top: border.top,
                bottom: border.bottom,
            },
            padding: Edge {
                left: padding.left,
                right: padding.right,
                top: padding.top,
                bottom: padding.bottom,
            },
        }
    }

    pub(crate) fn save_border_padding_result(
        &mut self,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) {
        self.result_padding_rect = Rect::new(
            Point::new(border.left, border.top),
            Size::new(
                self.result.size.width - border.left - border.right,
                self.result.size.height - border.top - border.bottom,
            ),
        );
        self.result_content_rect = Rect::new(
            Point::new(padding_border.left, padding_border.top),
            Size::new(
                self.result.size.width - padding_border.left - padding_border.right,
                self.result.size.height - padding_border.top - padding_border.bottom,
            ),
        );
        // info!("!!! {:p} rect padding {:?} content {:?}", self, self.result_padding_rect, self.result_content_rect);
    }

    pub(crate) fn is_requested_size_fixed(
        &mut self,
        request: &ComputeRequest<T::Length>,
        collapsed_margin: Option<CollapsedBlockMargin<T::Length>>,
    ) -> Option<ComputeResult<T::Length>> {
        let collapsed_margin = if let Some(x) = collapsed_margin {
            x
        } else if request.parent_is_block {
            return None;
        } else {
            CollapsedBlockMargin::zero()
        };
        // return if requested size is specified
        match request.kind {
            ComputeRequestKind::AllSize => {
                if let Some(width) = request.size.width.val() {
                    if let Some(height) = request.size.height.val() {
                        let size = Size::new(width, height);
                        return Some(ComputeResult {
                            size: Normalized(size),
                            min_content_size: Normalized(size),
                            first_baseline_ascent: size.to_vector(),
                            last_baseline_ascent: size.to_vector(),
                            collapsed_margin,
                        });
                    }
                }
            }
            ComputeRequestKind::RowSize => {
                if let Some(width) = request.size.width.val() {
                    let size = Size::new(width, T::Length::zero());
                    return Some(ComputeResult {
                        size: Normalized(size),
                        min_content_size: Normalized(size),
                        first_baseline_ascent: size.to_vector(),
                        last_baseline_ascent: size.to_vector(),
                        collapsed_margin,
                    });
                }
            }
            ComputeRequestKind::ColSize => {
                if let Some(height) = request.size.height.val() {
                    let size = Size::new(T::Length::zero(), height);
                    return Some(ComputeResult {
                        size: Normalized(size),
                        min_content_size: Normalized(size),
                        first_baseline_ascent: size.to_vector(),
                        last_baseline_ascent: size.to_vector(),
                        collapsed_margin,
                    });
                }
            }
            _ => {}
        }
        None
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn margin_border_padding(
        &self,
        node: &T,
        parent_inner_size: OptionSize<T::Length>,
    ) -> (EdgeOption<T::Length>, Edge<T::Length>, Edge<T::Length>) {
        let style = node.style();
        let length_ratio_base = match style.writing_mode() {
            WritingMode::HorizontalTb => parent_inner_size.width,
            WritingMode::VerticalLr | WritingMode::VerticalRl => parent_inner_size.height,
        };
        let margin = EdgeOption {
            left: style
                .margin_left()
                .resolve_with_auto(length_ratio_base, node),
            right: style
                .margin_right()
                .resolve_with_auto(length_ratio_base, node),
            top: style
                .margin_top()
                .resolve_with_auto(length_ratio_base, node),
            bottom: style
                .margin_bottom()
                .resolve_with_auto(length_ratio_base, node),
        };
        let border = Edge {
            left: style
                .border_left()
                .resolve(length_ratio_base, node)
                .or_zero(),
            right: style
                .border_right()
                .resolve(length_ratio_base, node)
                .or_zero(),
            top: style
                .border_top()
                .resolve(length_ratio_base, node)
                .or_zero(),
            bottom: style
                .border_bottom()
                .resolve(length_ratio_base, node)
                .or_zero(),
        };
        let padding = Edge {
            left: style
                .padding_left()
                .resolve(length_ratio_base, node)
                .or_zero(),
            right: style
                .padding_right()
                .resolve(length_ratio_base, node)
                .or_zero(),
            top: style
                .padding_top()
                .resolve(length_ratio_base, node)
                .or_zero(),
            bottom: style
                .padding_bottom()
                .resolve(length_ratio_base, node)
                .or_zero(),
        };
        let padding_border = Edge {
            left: padding.left + border.left,
            right: padding.right + border.right,
            top: padding.top + border.top,
            bottom: padding.bottom + border.bottom,
        };
        (margin, border, padding_border)
    }

    #[inline]
    pub(crate) fn min_max_size_limit(
        &self,
        node: &T,
        parent_inner_size: OptionSize<T::Length>,
        size: Size<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> Normalized<Size<T::Length>> {
        let ret = self.min_max_option_size_limit(
            node,
            parent_inner_size,
            size_to_option(size),
            border,
            padding_border,
        );
        Normalized(Size::new(ret.width.or_zero(), ret.height.or_zero()))
    }

    #[inline]
    pub(crate) fn min_max_option_size_limit(
        &self,
        node: &T,
        parent_inner_size: OptionSize<T::Length>,
        size: OptionSize<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> Normalized<OptionSize<T::Length>> {
        let min_max_limit =
            self.normalized_min_max_limit(node, parent_inner_size, border, padding_border);
        min_max_limit.normalized_size(size)
    }
    #[inline]
    pub(crate) fn min_max_size(
        node: &T,
        parent_size: OptionSize<T::Length>,
    ) -> MinMaxSize<T::Length> {
        let style = node.style();
        let min_width = style.min_width().resolve(parent_size.width, node);
        let max_width = style.max_width().resolve(parent_size.width, node);
        let min_height = style.min_height().resolve(parent_size.height, node);
        let max_height = style.max_height().resolve(parent_size.height, node);
        MinMaxSize {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    #[inline]
    pub(crate) fn normalized_min_max_limit(
        &self,
        node: &T,
        parent_size: OptionSize<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> MinMaxLimit<T::Length> {
        let style = node.style();
        let MinMaxSize {
            min_width,
            max_width,
            min_height,
            max_height,
        } = Self::min_max_size(node, parent_size);
        match style.box_sizing() {
            BoxSizing::BorderBox => {
                let min_width = padding_border.horizontal().maybe_max(min_width);
                let min_height = padding_border.vertical().maybe_max(min_height);
                MinMaxLimit {
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                }
            }
            BoxSizing::PaddingBox => {
                let min_width = border.horizontal().maybe_max(min_width)
                    + padding_border.horizontal()
                    - border.horizontal();
                let max_width = max_width + padding_border.horizontal() - border.horizontal();
                let min_height = border.vertical().maybe_max(min_height)
                    + padding_border.vertical()
                    - border.vertical();
                let max_height = max_height + padding_border.vertical() - border.vertical();
                MinMaxLimit {
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                }
            }
            BoxSizing::ContentBox => {
                let min_width =
                    T::Length::zero().maybe_max(min_width) + padding_border.horizontal();
                let max_width = max_width + padding_border.horizontal();
                let min_height =
                    T::Length::zero().maybe_max(min_height) + padding_border.vertical();
                let max_height = max_height + padding_border.vertical();
                MinMaxLimit {
                    min_width,
                    max_width,
                    min_height,
                    max_height,
                }
            }
        }
    }

    #[inline]
    pub(crate) fn css_border_box_size(
        &self,
        node: &T,
        parent_inner_size: OptionSize<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> OptionSize<T::Length> {
        let style = node.style();
        let size = OptionSize::new(
            style.width().resolve(parent_inner_size.width, node),
            style.height().resolve(parent_inner_size.height, node),
        );
        match style.box_sizing() {
            BoxSizing::BorderBox => size,
            BoxSizing::PaddingBox => OptionSize::new(
                size.width + padding_border.horizontal() - border.horizontal(),
                size.height + padding_border.vertical() - border.vertical(),
            ),
            BoxSizing::ContentBox => OptionSize::new(
                size.width + padding_border.horizontal(),
                size.height + padding_border.vertical(),
            ),
        }
    }

    #[inline]
    pub(crate) fn compute_measure_block_if_exists(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> Option<ComputeResult<T::Length>> {
        // if the node has measure, accept the measure result
        if node.should_measure(env) {
            let req_size = match request.sizing_mode {
                SizingMode::Normal => OptionSize::new(
                    request.size.width - padding_border.horizontal(),
                    request.size.height - padding_border.vertical(),
                ),
                SizingMode::MinContent => OptionSize::new(OptionNum::zero(), OptionNum::none()),
                SizingMode::MaxContent => OptionSize::new(OptionNum::none(), OptionNum::none()),
            };
            let max_content = request.max_content;
            let min_max_limit = self.normalized_min_max_limit(
                node,
                *request.parent_inner_size,
                border,
                padding_border,
            );
            let r = node.measure_block_size(
                env,
                req_size,
                Size::new(
                    min_max_limit.min_width - padding_border.horizontal(),
                    min_max_limit.min_height - padding_border.vertical(),
                ),
                Size::new(
                    (min_max_limit.max_width - padding_border.horizontal())
                        .unwrap_or(T::Length::max_value()),
                    (min_max_limit.max_height - padding_border.vertical())
                        .unwrap_or(T::Length::max_value()),
                ),
                OptionSize::new(
                    max_content.width - padding_border.horizontal(),
                    max_content.height - padding_border.vertical(),
                ),
                request.kind == ComputeRequestKind::Position,
                request.sizing_mode,
            );
            let size = match request.sizing_mode {
                SizingMode::Normal => Normalized(Size::new(
                    r.size.width + padding_border.horizontal(),
                    r.size.height + padding_border.vertical(),
                )), // original r.size is normalized
                SizingMode::MinContent | SizingMode::MaxContent => {
                    Normalized(Size::new(r.size.width, r.size.height))
                }
            };
            let first_baseline_ascent =
                r.first_baseline_ascent + Vector::new(padding_border.left, padding_border.top);
            let last_baseline_ascent =
                r.last_baseline_ascent + Vector::new(padding_border.left, padding_border.top);
            let axis_info = AxisInfo::from_writing_mode(node.style().writing_mode());
            let ret = ComputeResult {
                size,
                min_content_size: size,
                first_baseline_ascent,
                last_baseline_ascent,
                collapsed_margin: CollapsedBlockMargin::from_margin(
                    margin
                        .or_zero()
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
                    margin
                        .or_zero()
                        .main_axis_end(axis_info.dir, axis_info.main_dir_rev),
                ),
            };
            if request.kind == ComputeRequestKind::Position {
                self.result = Rect::new(Point::zero(), *size);
                self.cache.write_position(node, &request, ret);
            } else {
                self.cache.write_all_size(node, &request, ret);
            }
            Some(ret)
        } else {
            None
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_measure_inline_unit_if_exists(
        &mut self,
        env: &mut T::Env,
        node: &T,
        parent_inner_size: OptionSize<T::Length>,
        max_content: OptionSize<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
        sizing_mode: SizingMode,
    ) -> Option<T::InlineUnit> {
        // if the node has measure, accept the measure result
        if node.should_measure(env) {
            let css_box_size =
                self.css_border_box_size(node, parent_inner_size, border, padding_border);
            let req_size = match sizing_mode {
                SizingMode::Normal => OptionSize::new(
                    css_box_size.width - padding_border.horizontal(),
                    css_box_size.height - padding_border.vertical(),
                ),
                SizingMode::MinContent => OptionSize::new(OptionNum::zero(), OptionNum::none()),
                SizingMode::MaxContent => OptionSize::new(OptionNum::none(), OptionNum::none()),
            };
            let min_max_limit =
                self.normalized_min_max_limit(node, parent_inner_size, border, padding_border);
            let r = node.measure_inline_unit(
                env,
                req_size,
                Size::new(
                    min_max_limit.min_width - padding_border.horizontal(),
                    min_max_limit.min_height - padding_border.vertical(),
                ),
                Size::new(
                    (min_max_limit.max_width - padding_border.horizontal())
                        .unwrap_or(T::Length::max_value()),
                    (min_max_limit.max_height - padding_border.vertical())
                        .unwrap_or(T::Length::max_value()),
                ),
                OptionSize::new(
                    max_content.width - padding_border.horizontal(),
                    max_content.height - padding_border.vertical(),
                ),
                sizing_mode,
            );

            let size = match sizing_mode {
                SizingMode::Normal => Size::new(
                    r.size.width + padding_border.horizontal(),
                    r.size.height + padding_border.vertical(),
                ), // original r.size is normalized
                SizingMode::MinContent | SizingMode::MaxContent => {
                    Size::new(r.size.width, r.size.height)
                }
            };
            let first_baseline_ascent =
                r.first_baseline_ascent + Vector::new(padding_border.left, padding_border.top);
            let last_baseline_ascent =
                r.last_baseline_ascent + Vector::new(padding_border.left, padding_border.top);
            let ret = T::InlineUnit::new(
                env,
                node,
                MeasureResult {
                    size,
                    first_baseline_ascent,
                    last_baseline_ascent,
                },
            );
            Some(ret)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn gen_origin(
        &mut self,
        axis_info: AxisInfo,
        parent_size: Size<T::Length>,
        offset_main: T::Length,
        offset_cross: T::Length,
    ) -> Vector<T::Length> {
        let (width, height, width_rev, height_rev) = match axis_info.dir {
            AxisDirection::Horizontal => (
                offset_main,
                offset_cross,
                axis_info.main_dir_rev,
                axis_info.cross_dir_rev,
            ),
            AxisDirection::Vertical => (
                offset_cross,
                offset_main,
                axis_info.cross_dir_rev,
                axis_info.main_dir_rev,
            ),
        };
        let width = match width_rev {
            AxisReverse::NotReversed => width,
            AxisReverse::Reversed => parent_size.width - width - self.result.size.width,
        };
        let height = match height_rev {
            AxisReverse::NotReversed => height,
            AxisReverse::Reversed => parent_size.height - height - self.result.size.height,
        };
        self.result.origin = Point::new(width, height);

        // info!("!!! {:p} pos {:?}", self, self.result);
        self.result.origin.to_vector()
    }
}

#[allow(missing_docs)]
/// SizingMode is used to determine the sizing mode of the node.
#[derive(Clone, PartialEq, Copy, Hash, Eq, Debug)]
pub enum SizingMode {
    Normal,
    MinContent,
    MaxContent,
}

#[derive(Clone, PartialEq)]
pub(crate) struct ComputeRequest<L: LengthNum> {
    pub(crate) size: Normalized<OptionSize<L>>, // the expected size, margin excluded, but css width, height, and min max size considered
    pub(crate) parent_inner_size: Normalized<OptionSize<L>>, // parent size without its padding and border, none represents auto
    pub(crate) max_content: Normalized<OptionSize<L>>, // the max-content constraint, an extra max-size limit for text content, with self padding and border added
    pub(crate) kind: ComputeRequestKind,
    pub(crate) parent_is_block: bool,
    pub(crate) sizing_mode: SizingMode,
}

impl<L: LengthNum> fmt::Debug for ComputeRequest<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ComputeRequest<{:?},{:?}>({:?}x{:?}, parent {:?}x{:?}, max {:?}x{:?}, parent_is_block {:?})",
            self.kind,
            self.sizing_mode,
            self.size.width,
            self.size.height,
            self.parent_inner_size.width,
            self.parent_inner_size.height,
            self.max_content.width,
            self.max_content.height,
            self.parent_is_block,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ComputeResult<L: LengthNum> {
    pub(crate) size: Normalized<Size<L>>, // only valid on corresponding size which the request includes
    pub(crate) min_content_size: Normalized<Size<L>>, // only valid on corresponding size which the request includes
    pub(crate) first_baseline_ascent: Vector<L>,      // only valid on position request
    pub(crate) last_baseline_ascent: Vector<L>,       // only valid on position request
    pub(crate) collapsed_margin: CollapsedBlockMargin<L>, // only valid on corresponding size which the request includes and collapsed_margin set
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) enum ComputeRequestKind {
    RowSize,
    ColSize,
    AllSize,
    Position,
}

impl ComputeRequestKind {
    pub(crate) fn shift_to_all_size(&self) -> Self {
        match self {
            Self::RowSize => Self::AllSize,
            Self::ColSize => Self::AllSize,
            Self::AllSize => Self::AllSize,
            Self::Position => Self::Position,
        }
    }

    pub(crate) fn shift_to_all_size_with_position(&self, with_position: bool) -> Self {
        match self {
            Self::RowSize => Self::AllSize,
            Self::ColSize => Self::AllSize,
            Self::AllSize => Self::AllSize,
            Self::Position => {
                if with_position {
                    Self::Position
                } else {
                    Self::AllSize
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CollapsedBlockMargin<L: LengthNum> {
    pub(crate) collapsed_through: bool,
    pub(crate) start: CollapsedMargin<L>,
    pub(crate) end: CollapsedMargin<L>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CollapsedMargin<L: LengthNum> {
    positive: L,
    negative: L,
}

impl<L: LengthNum> CollapsedBlockMargin<L> {
    pub(crate) fn from_margin(margin_start: L, margin_end: L) -> Self {
        Self {
            collapsed_through: false,
            start: CollapsedMargin::new(margin_start),
            end: CollapsedMargin::new(margin_end),
        }
    }
    pub(crate) fn from_collapsed_margin(
        margin_start: CollapsedMargin<L>,
        margin_end: CollapsedMargin<L>,
    ) -> Self {
        Self {
            collapsed_through: false,
            start: margin_start,
            end: margin_end,
        }
    }
    pub(crate) fn zero() -> Self {
        Self {
            collapsed_through: false,
            start: CollapsedMargin::zero(),
            end: CollapsedMargin::zero(),
        }
    }
}

impl<L: LengthNum> CollapsedMargin<L> {
    pub(crate) fn zero() -> Self {
        Self {
            positive: L::zero(),
            negative: L::zero(),
        }
    }
    pub(crate) fn new(margin: L) -> Self {
        Self {
            positive: margin.max(L::zero()),
            negative: margin.min(L::zero()),
        }
    }

    pub(crate) fn adjoin(&self, other: &Self) -> Self {
        Self {
            positive: self.positive.max(other.positive),
            negative: self.negative.min(other.negative),
        }
    }

    pub(crate) fn adjoin_assign(&mut self, other: &Self) {
        *self = self.adjoin(other);
    }

    pub(crate) fn solve(&self) -> L {
        self.positive + self.negative
    }
}

#[inline(always)]
pub(crate) fn is_display_none<T: LayoutTreeNode>(style: &T::Style) -> bool {
    style.display() == Display::None
}
