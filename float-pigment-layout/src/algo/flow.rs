use crate::*;

use float_pigment_css::num_traits::Zero;

enum BlockOrInlineSeries<'a, T: LayoutTreeNode> {
    Block(&'a T),
    InlineSeries {
        end_nodes: Vec<&'a T>,
        middle_nodes: Vec<&'a T>,
    },
}

/// No content separating parent and descendants
/// If there is no border, padding, inline part, block formatting context created,
/// or clearance to separate the margin-top of a block from the margin-top of one or more of its descendant blocks,
/// then those margins collapse. The collapsed margin ends up outside the parent.
#[inline]
pub(crate) fn is_margin_start_collapsible<L: LengthNum>(
    parent_is_block: bool,
    padding_border_start: L,
) -> bool {
    if !parent_is_block || !padding_border_start.is_zero() {
        return false;
    }
    true
}

/// No content separating parent and descendants
/// or no border, padding, inline content, height,
/// or min-height to separate the margin-bottom of a block from the margin-bottom of one or more of its descendant blocks,
/// then those margins collapse. The collapsed margin ends up outside the parent.
#[inline]
pub(crate) fn is_margin_end_collapsible<L: LengthNum>(
    axis_info: AxisInfo,
    parent_is_block: bool,
    padding_border: Edge<L>,
    min_main_size: L,
    max_main_size: OptionNum<L>,
    total_inner_main_size: L,
) -> bool {
    if !parent_is_block
        || !padding_border
            .main_axis_end(axis_info.dir, axis_info.main_dir_rev)
            .is_zero()
        || !min_main_size.is_zero()
        || !total_inner_main_size.is_zero()
    {
        let total_main_size = total_inner_main_size + padding_border.main_axis_sum(axis_info.dir);
        let max_main_size = if let Some(max) = max_main_size.val() {
            max
        } else {
            L::max_value()
        };
        if min_main_size <= total_main_size && total_main_size <= max_main_size {
            return true;
        }

        return false;
    }
    true
}

/// Empty Block
/// If there is no border, padding, inline content, height,
/// or min-height to separate a block's margin-top from its margin-bottom,
/// then its top and bottom margins collapse.
#[inline]
pub(crate) fn is_empty_block<L: LengthNum>(
    padding_border_main_axis: L,
    min_main_size: L,
    node_inner_main_size: L,
    main_size: OptionNum<L>,
) -> bool {
    if !padding_border_main_axis.is_zero()
        || !min_main_size.is_zero()
        || !node_inner_main_size.is_zero()
        || (main_size.is_some() && !main_size.val().unwrap().is_zero())
    {
        return false;
    }
    true
}

fn for_each_block_or_inline_series<T: LayoutTreeNode>(
    env: &mut T::Env,
    node: &T,
    mut f: impl FnMut(&mut T::Env, BlockOrInlineSeries<T>),
) {
    let mut end_nodes = vec![];
    let mut middle_nodes = vec![];
    fn search_inline_rec<'a, T: LayoutTreeNode>(
        env: &mut T::Env,
        node: &'a T,
        f: &mut impl FnMut(&mut T::Env, BlockOrInlineSeries<T>),
        end_nodes: &mut Vec<&'a T>,
        middle_nodes: &mut Vec<&'a T>,
    ) {
        node.tree_visitor().for_each_child(|child_node, _| {
            let child_style = child_node.style();
            if is_display_none::<T>(child_style) || is_out_of_flow::<T>(child_style) {
                return;
            }
            match child_style.display() {
                Display::Inline | Display::Grid => {
                    if child_node.should_measure(env) {
                        end_nodes.push(child_node);
                    } else {
                        search_inline_rec(env, child_node, f, end_nodes, middle_nodes);
                        middle_nodes.push(child_node);
                    }
                }
                Display::InlineBlock | Display::InlineFlex => {
                    end_nodes.push(child_node);
                }
                Display::Block | Display::Flex => {
                    if !end_nodes.is_empty() {
                        f(
                            env,
                            BlockOrInlineSeries::InlineSeries {
                                end_nodes: core::mem::take(end_nodes),
                                middle_nodes: core::mem::take(middle_nodes),
                            },
                        );
                    }
                    f(env, BlockOrInlineSeries::Block(child_node));
                }
                Display::None => unreachable!(),
                Display::FlowRoot => todo!(),
            }
        });
    }
    search_inline_rec(env, node, &mut f, &mut end_nodes, &mut middle_nodes);
    f(
        env,
        BlockOrInlineSeries::InlineSeries {
            end_nodes,
            middle_nodes,
        },
    );
}

pub(crate) struct BlockOrInlineSeriesComputeResult<L: LengthNum> {
    pub(crate) size: Size<L>,
    pub(crate) first_baseline_ascent_option: Option<Vector<L>>,
    pub(crate) last_baseline_ascent_option: Option<Vector<L>>,
    pub(crate) collapsed_margin: CollapsedBlockMargin<L>,
}

pub(crate) trait Flow<T: LayoutTreeNode> {
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length>;

    #[allow(clippy::too_many_arguments)]
    fn compute_block_or_inline_series(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        axis_info: AxisInfo,
        node_inner_size: OptionSize<T::Length>,
        min_max_limit: MinMaxLimit<T::Length>,
        padding_border: Edge<T::Length>,
        margin: EdgeOption<T::Length>,
    ) -> BlockOrInlineSeriesComputeResult<T::Length>;
}

impl<T: LayoutTreeNode> Flow<T> for LayoutUnit<T> {
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length> {
        // Short-circuit if requestKind is AllSize and size is specified
        if let Some(x) = self.is_requested_size_fixed(&request, None) {
            return x;
        }
        let style = node.style();
        let axis_info = AxisInfo::from_writing_mode(style.writing_mode());
        let node_size = request.size.0;
        let mut node_inner_size = OptionSize::new(
            node_size.width - padding_border.horizontal(),
            node_size.height - padding_border.vertical(),
        );
        let min_max_limit =
            self.normalized_min_max_limit(node, *request.parent_inner_size, border, padding_border);
        let mut max_content = OptionSize::new_with_dir(
            axis_info.dir,
            min_max_limit
                .maybe()
                .main_size(axis_info.dir, OptionNum::none()),
            min_max_limit.maybe().cross_size(
                axis_info.dir,
                node_inner_size
                    .cross_size(axis_info.dir)
                    .or(request.max_content.cross_size(axis_info.dir)),
            ),
        );
        // if cross size is infinite, should get the max cross size first
        if node_inner_size.cross_size(axis_info.dir).is_none() {
            let content_cross_size = self
                .compute_block_or_inline_series(
                    env,
                    node,
                    ComputeRequest {
                        kind: ComputeRequestKind::AllSize,
                        max_content: Normalized(max_content),
                        ..request
                    },
                    axis_info,
                    node_inner_size,
                    min_max_limit,
                    padding_border,
                    margin,
                )
                .size
                .cross_size(axis_info.dir);
            node_inner_size.set_cross_size(axis_info.dir, OptionNum::some(content_cross_size));
            max_content.set_cross_size(axis_info.dir, OptionNum::some(content_cross_size));
        }
        let compute_res = self.compute_block_or_inline_series(
            env,
            node,
            ComputeRequest {
                max_content: Normalized(max_content),
                ..request
            },
            axis_info,
            node_inner_size,
            min_max_limit,
            padding_border,
            margin,
        );
        let mut total_main_size = padding_border
            .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
            + compute_res.size.main_size(axis_info.dir);

        total_main_size += padding_border.main_axis_end(axis_info.dir, axis_info.main_dir_rev);
        let size = Size::new_with_dir(
            axis_info.dir,
            node_size
                .main_size(axis_info.dir)
                .unwrap_or(total_main_size),
            node_size.cross_size(axis_info.dir).unwrap_or(
                compute_res.size.cross_size(axis_info.dir)
                    + padding_border.cross_axis_sum(axis_info.dir),
            ),
        );
        let size = self.min_max_size_limit(
            node,
            *request.parent_inner_size,
            size,
            border,
            padding_border,
        );

        let first_baseline_ascent = Vector::new_with_dir(
            axis_info.dir,
            compute_res
                .first_baseline_ascent_option
                .unwrap_or_else(|| size.to_vector())
                .main_axis(axis_info.dir),
            T::Length::zero(),
        );
        let last_baseline_ascent = Vector::new_with_dir(
            axis_info.dir,
            compute_res
                .last_baseline_ascent_option
                .unwrap_or_else(|| size.to_vector())
                .main_axis(axis_info.dir),
            T::Length::zero(),
        );

        // TODO requires another adjust progress if main axis is reversed?

        let ret = ComputeResult {
            size,
            first_baseline_ascent,
            last_baseline_ascent,
            collapsed_margin: compute_res.collapsed_margin,
        };

        if request.kind == ComputeRequestKind::Position {
            compute_special_position_children(
                env,
                node,
                &ret,
                border,
                padding_border,
                axis_info.dir,
                axis_info.main_dir_rev,
                AxisReverse::NotReversed,
                false,
            );
            self.result = Rect::new(Point::zero(), ret.size.0);
            self.cache.write_position(node, &request, ret);
        } else {
            self.cache.write_all_size(node, &request, ret);
        }

        ret
    }

    fn compute_block_or_inline_series(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        axis_info: AxisInfo,
        node_inner_size: OptionSize<T::Length>,
        min_max_limit: MinMaxLimit<T::Length>,
        padding_border: Edge<T::Length>,
        margin: EdgeOption<T::Length>,
    ) -> BlockOrInlineSeriesComputeResult<T::Length> {
        let mut total_main_size = T::Length::zero();
        let mut max_cross_size = T::Length::zero();
        let cross_offset = padding_border.cross_axis_start(axis_info.dir, axis_info.cross_dir_rev);
        let mut first_baseline_ascent_option = None;
        let mut last_baseline_ascent_option = None;

        let parent_margin_start_collapsible = is_margin_start_collapsible(
            request.parent_is_block,
            padding_border.main_axis_start(axis_info.dir, axis_info.main_dir_rev),
        );

        let mut parent_collapsed_margin_start = CollapsedMargin::new(
            margin
                .or_zero()
                .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
        );
        let mut prev_sibling_collapsed_margin: Option<(CollapsedMargin<T::Length>, bool)> = None;

        for_each_block_or_inline_series(env, node, |env, block_or_inline_series| {
            match block_or_inline_series {
                // for child block
                BlockOrInlineSeries::Block(child_node) => {
                    let mut child = child_node.layout_node().unit();

                    let (child_margin, child_border, child_padding_border) =
                        child.margin_border_padding(child_node, node_inner_size);
                    let min_max_limit = child.normalized_min_max_limit(
                        child_node,
                        node_inner_size,
                        child_border,
                        child_padding_border,
                    );
                    let mut css_size = child.css_border_box_size(
                        child_node,
                        node_inner_size,
                        child_border,
                        child_padding_border,
                    );
                    css_size.set_cross_size(
                        axis_info.dir,
                        css_size
                            .cross_size(axis_info.dir)
                            .or(node_inner_size.cross_size(axis_info.dir)
                                - child_margin.cross_axis_sum(axis_info.dir)),
                    );
                    let size = min_max_limit.normalized_size(css_size);
                    let mut max_content = OptionSize::new(OptionNum::none(), OptionNum::none());
                    max_content.set_cross_size(
                        axis_info.dir,
                        size.cross_size(axis_info.dir)
                            .or(max_content.cross_size(axis_info.dir)
                                - child_margin.cross_axis_sum(axis_info.dir)),
                    );
                    let max_content = min_max_limit.normalized_size(max_content);
                    let child_res = child.compute_internal(
                        env,
                        child_node,
                        ComputeRequest {
                            size,
                            parent_inner_size: Normalized(node_inner_size),
                            max_content,
                            kind: request.kind.shift_to_all_size(),
                            parent_is_block: true,
                        },
                    );
                    let mut main_offset = padding_border
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + total_main_size;
                    // margin collapse between sibling
                    if let Some((prev_sibling_margin, prev_sibling_collapsed_through)) =
                        prev_sibling_collapsed_margin
                    {
                        // ┌───────┐
                        // │       │       ┌───────┐
                        // │       │       │       │
                        // └───────┘       │       │
                        //  AAAAAAA        └───────┘
                        //  AAAAAAA         ABABABA
                        //          ──────► ABABABA
                        //  BBBBBBB         BBBBBBB
                        //  BBBBBBB        ┌───────┐
                        //  BBBBBBB        │       │
                        // ┌───────┐       │       │
                        // │       │       └───────┘
                        // │       │
                        // └───────┘
                        if prev_sibling_collapsed_through {
                            main_offset -= prev_sibling_margin.solve();
                        }
                        main_offset += prev_sibling_margin
                            .adjoin(&child_res.collapsed_margin.start)
                            .solve();
                        let mut current_collapsed_margin =
                            prev_sibling_margin.adjoin(&child_res.collapsed_margin.start);
                        if child_res.collapsed_margin.collapsed_through {
                            current_collapsed_margin.adjoin_assign(&child_res.collapsed_margin.end);
                        }
                        if prev_sibling_collapsed_through {
                            total_main_size -= prev_sibling_margin.solve();
                        }
                        total_main_size += current_collapsed_margin.solve();
                        if child_res.collapsed_margin.collapsed_through {
                            prev_sibling_collapsed_margin.replace((current_collapsed_margin, true));
                        } else {
                            prev_sibling_collapsed_margin
                                .replace((child_res.collapsed_margin.end, false));
                        }
                    } else {
                        // margin collapse between parent and the first child
                        if parent_margin_start_collapsible {
                            // AAAAAAAAAAAAA         BBBBBBBBBBBBB
                            // ┌───────────┐         ABABABABABABA
                            // │  BBBBBBB  │         ┌───────────┐
                            // │  BBBBBBB  │         │ ┌───────┐ │
                            // │ ┌───────┐ │ ──────► │ │       │ │
                            // │ │       │ │         │ └───────┘ │
                            // │ └───────┘ │         └───────────┘
                            // └───────────┘
                            let mut current_collapsed_margin = child_res.collapsed_margin.start;
                            if child_res.collapsed_margin.collapsed_through {
                                current_collapsed_margin
                                    .adjoin_assign(&child_res.collapsed_margin.end);
                            }
                            parent_collapsed_margin_start.adjoin_assign(&current_collapsed_margin);
                            if !child_res.collapsed_margin.collapsed_through {
                                prev_sibling_collapsed_margin
                                    .replace((child_res.collapsed_margin.end, false));
                            }
                        } else {
                            let mut current_collapsed_margin = child_res.collapsed_margin.start;
                            main_offset += current_collapsed_margin.solve();
                            if child_res.collapsed_margin.collapsed_through {
                                current_collapsed_margin
                                    .adjoin_assign(&child_res.collapsed_margin.end);
                                prev_sibling_collapsed_margin
                                    .replace((current_collapsed_margin, true));
                            } else {
                                prev_sibling_collapsed_margin
                                    .replace((child_res.collapsed_margin.end, false));
                            }
                            total_main_size += current_collapsed_margin.solve();
                        }
                    }

                    total_main_size += child_res.size.main_size(axis_info.dir);
                    max_cross_size = max_cross_size.max(
                        child_res.size.cross_size(axis_info.dir)
                            + child_margin.cross_axis_sum(axis_info.dir),
                    );
                    if request.kind == ComputeRequestKind::Position {
                        let cross_offset: <T as LayoutTreeNode>::Length = padding_border
                            .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                            + if child_margin
                                .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                                .is_none()
                                && child_margin
                                    .cross_axis_end(axis_info.dir, axis_info.cross_dir_rev)
                                    .is_none()
                            {
                                (node_inner_size.cross_size(axis_info.dir)
                                    - child_res.size.cross_size(axis_info.dir))
                                .or_zero()
                                .div_i32(2)
                            } else {
                                child_margin
                                    .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                                    .or_zero()
                            };
                        let baseline_diff = child.gen_origin(
                            axis_info.dir,
                            axis_info.main_dir_rev,
                            AxisReverse::NotReversed,
                            node_inner_size.or_zero(),
                            main_offset,
                            cross_offset,
                        );
                        if first_baseline_ascent_option.is_none() {
                            first_baseline_ascent_option =
                                Some(child_res.first_baseline_ascent + baseline_diff);
                        }
                        last_baseline_ascent_option =
                            Some(child_res.last_baseline_ascent + baseline_diff);
                    }
                }

                // for a series of child inlines, group them and layout together
                BlockOrInlineSeries::InlineSeries {
                    end_nodes,
                    middle_nodes,
                } => {
                    if !end_nodes.is_empty() {
                        let inline_units = end_nodes
                            .iter()
                            .map(|child_node| {
                                let mut child = child_node.layout_node().unit();
                                let (child_margin, child_border, child_padding_border) =
                                    child.margin_border_padding(child_node, node_inner_size);
                                let unit = if let Some(unit) = child
                                    .get_measure_inline_unit_if_exists(
                                        env,
                                        child_node,
                                        node_inner_size,
                                        *request.max_content,
                                        child_border,
                                        child_padding_border,
                                    ) {
                                    // for measure-able node, use the measure result
                                    unit
                                } else {
                                    // for inline-blocks, layout as block
                                    let min_max_limit = child.normalized_min_max_limit(
                                        child_node,
                                        node_inner_size,
                                        child_border,
                                        child_padding_border,
                                    );
                                    let css_size = child.css_border_box_size(
                                        child_node,
                                        node_inner_size,
                                        child_border,
                                        child_padding_border,
                                    );
                                    let size = min_max_limit.normalized_size(css_size);
                                    let mut max_content =
                                        OptionSize::new(OptionNum::none(), OptionNum::none());
                                    max_content.set_cross_size(
                                        axis_info.dir,
                                        css_size
                                            .cross_size(axis_info.dir)
                                            .or(max_content.cross_size(axis_info.dir)
                                                - child_margin.cross_axis_sum(axis_info.dir)),
                                    );
                                    let max_content = min_max_limit.normalized_size(max_content);
                                    let child_res = child.compute_internal(
                                        env,
                                        child_node,
                                        ComputeRequest {
                                            size,
                                            parent_inner_size: Normalized(node_inner_size),
                                            max_content,
                                            kind: request.kind.shift_to_all_size(),
                                            parent_is_block: true,
                                        },
                                    );
                                    T::InlineUnit::inline_block(
                                        env,
                                        child_node,
                                        *child_res.size,
                                        child_res.last_baseline_ascent.main_axis(axis_info.dir),
                                    )
                                };
                                (unit, child_margin, child_padding_border)
                            })
                            .collect();
                        let (block_size, positions) = T::InlineMeasure::block_size(
                            env,
                            node,
                            inline_units,
                            Size::new_with_dir(
                                axis_info.dir,
                                OptionNum::none(),
                                node_inner_size.cross_size(axis_info.dir),
                            ),
                            *request.max_content,
                            request.kind == ComputeRequestKind::Position,
                        );

                        if let Some((prev_collapsed_margin, prev_collapsed_through)) =
                            prev_sibling_collapsed_margin
                        {
                            if !prev_collapsed_through {
                                total_main_size += prev_collapsed_margin.solve();
                            }
                        }
                        let main_offset = padding_border
                            .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                            + total_main_size;
                        prev_sibling_collapsed_margin.replace((CollapsedMargin::zero(), false));

                        total_main_size += block_size.main_size(axis_info.dir);
                        max_cross_size = max_cross_size.max(block_size.cross_size(axis_info.dir));
                        if request.kind == ComputeRequestKind::Position {
                            let mut baseline_diff = Vector::zero();
                            baseline_diff.set_main_axis(axis_info.dir, main_offset);
                            baseline_diff.set_cross_axis(axis_info.dir, cross_offset);

                            for ((child_origin, child_res), child_node) in
                                positions.into_iter().zip(end_nodes)
                            {
                                let baseline_diff = baseline_diff + child_origin.to_vector();
                                let mut child = child_node.layout_node().unit();
                                child.result = Rect::new(Point::zero(), child_res.size);
                                child.cache.touch(child_node);
                                child.gen_origin(
                                    axis_info.dir,
                                    axis_info.main_dir_rev,
                                    AxisReverse::NotReversed,
                                    node_inner_size.or_zero(),
                                    main_offset + child_origin.main_axis(axis_info.dir),
                                    cross_offset + child_origin.cross_axis(axis_info.dir),
                                );
                                if first_baseline_ascent_option.is_none() {
                                    first_baseline_ascent_option =
                                        Some(child_res.first_baseline_ascent + baseline_diff);
                                }
                                last_baseline_ascent_option =
                                    Some(child_res.last_baseline_ascent + baseline_diff);
                                child.save_all_results(child_node, env, node_inner_size);
                            }
                        }
                    }
                    if request.kind == ComputeRequestKind::Position {
                        for middle_node in middle_nodes {
                            let mut merged_rect = Rect::zero();
                            middle_node.tree_visitor().for_each_child(|child_node, _| {
                                let child = child_node.layout_node().unit();
                                if is_independent_positioning(child_node) {
                                    return;
                                }
                                merged_rect = merged_rect.union(&child.result);
                            });
                            let mut middle_node_layout_unit = middle_node.layout_node().unit();
                            middle_node_layout_unit.result = merged_rect;
                            middle_node_layout_unit.cache.touch(middle_node);
                            let child_diff = merged_rect.origin.to_vector();
                            middle_node.tree_visitor().for_each_child(|child_node, _| {
                                let mut child = child_node.layout_node().unit();
                                child.result.origin -= child_diff;
                            });
                            middle_node_layout_unit.save_all_results(
                                middle_node,
                                env,
                                node_inner_size,
                            );
                            let (_, border, padding_border) = middle_node_layout_unit
                                .margin_border_padding(middle_node, node_inner_size);
                            compute_special_position_children(
                                env,
                                middle_node,
                                &ComputeResult {
                                    size: Normalized(merged_rect.size),
                                    collapsed_margin: CollapsedBlockMargin::zero(),
                                    first_baseline_ascent: first_baseline_ascent_option
                                        .unwrap_or_else(|| merged_rect.size.to_vector()),
                                    last_baseline_ascent: last_baseline_ascent_option
                                        .unwrap_or_else(|| merged_rect.size.to_vector()),
                                },
                                border,
                                padding_border,
                                axis_info.dir,
                                axis_info.main_dir_rev,
                                axis_info.cross_dir_rev,
                                false,
                            )
                        }
                    }
                }
            }
        });

        let parent_margin_end_collapsible = is_margin_end_collapsible(
            axis_info,
            request.parent_is_block,
            padding_border,
            min_max_limit.min_main_size(axis_info.dir),
            min_max_limit.max_main_size(axis_info.dir),
            total_main_size,
        );

        let mut parent_collapsed_margin_end = CollapsedMargin::new(
            margin
                .or_zero()
                .main_axis_end(axis_info.dir, axis_info.main_dir_rev),
        );

        if let Some((prev_collapsed_margin, prev_collapsed_through)) = prev_sibling_collapsed_margin
        {
            if parent_margin_end_collapsible {
                if prev_collapsed_through {
                    total_main_size -= prev_collapsed_margin.solve();
                }
                parent_collapsed_margin_end.adjoin_assign(&prev_collapsed_margin);
            } else if !prev_collapsed_through {
                total_main_size += prev_collapsed_margin.solve();
            }
        }
        let mut collapsed_margin = CollapsedBlockMargin::from_collapsed_margin(
            parent_collapsed_margin_start,
            parent_collapsed_margin_end,
        );
        if is_empty_block(
            padding_border.main_axis_sum(axis_info.dir),
            min_max_limit.min_main_size(axis_info.dir),
            total_main_size,
            request.size.main_size(axis_info.dir),
        ) {
            collapsed_margin.collapsed_through = true;
        }
        BlockOrInlineSeriesComputeResult {
            size: Size::new_with_dir(axis_info.dir, total_main_size, max_cross_size),
            first_baseline_ascent_option,
            last_baseline_ascent_option,
            collapsed_margin,
        }
    }
}
