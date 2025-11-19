use crate::*;

use float_pigment_css::num_traits::Zero;

/// Computes the total space taken up by gaps in an axis given:
///   - The size of each gap
///   - The number of items (children or flex-lines) between which there are gaps
#[inline(always)]
fn sum_axis_gaps<L: LengthNum>(gap: L, num_items: usize) -> L {
    if num_items <= 1 {
        L::zero()
    } else {
        gap.mul_i32(num_items as i32 - 1)
    }
}

#[inline(always)]
fn resolve_row_gap<T: LayoutTreeNode>(
    style: &T::Style,
    node: &T,
    inner_size: &Normalized<OptionSize<T::Length>>,
) -> T::Length {
    match style.writing_mode() {
        WritingMode::HorizontalTb => style.row_gap().resolve(inner_size.height, node).or_zero(),
        WritingMode::VerticalLr | WritingMode::VerticalRl => {
            style.row_gap().resolve(inner_size.width, node).or_zero()
        }
    }
}

#[inline(always)]
fn resolve_column_gap<T: LayoutTreeNode>(
    style: &T::Style,
    node: &T,
    inner_size: &Normalized<OptionSize<T::Length>>,
) -> T::Length {
    match style.writing_mode() {
        WritingMode::HorizontalTb => style.column_gap().resolve(inner_size.width, node).or_zero(),
        WritingMode::VerticalLr | WritingMode::VerticalRl => style
            .column_gap()
            .resolve(inner_size.height, node)
            .or_zero(),
    }
}

#[inline(always)]
fn main_axis_gap<T: LayoutTreeNode>(
    dir: AxisDirection,
    style: &T::Style,
    node: &T,
    inner_size: &Normalized<OptionSize<T::Length>>,
) -> T::Length {
    match dir {
        AxisDirection::Horizontal => resolve_column_gap(style, node, inner_size),
        AxisDirection::Vertical => resolve_row_gap(style, node, inner_size),
    }
}

#[inline(always)]
fn cross_axis_gap<T: LayoutTreeNode>(
    dir: AxisDirection,
    style: &T::Style,
    node: &T,
    inner_size: &Normalized<OptionSize<T::Length>>,
) -> T::Length {
    match dir {
        AxisDirection::Horizontal => resolve_row_gap(style, node, inner_size),
        AxisDirection::Vertical => resolve_column_gap(style, node, inner_size),
    }
}

pub(crate) fn align_self<T: LayoutTreeNode>(child: &T::Style, parent: &T::Style) -> AlignSelf {
    let s = child.align_self();
    if s == AlignSelf::Auto {
        match parent.align_items() {
            AlignItems::FlexStart => AlignSelf::FlexStart,
            AlignItems::FlexEnd => AlignSelf::FlexEnd,
            AlignItems::Center => AlignSelf::Center,
            AlignItems::Baseline => AlignSelf::Baseline,
            AlignItems::Stretch => AlignSelf::Stretch,
            AlignItems::Normal => AlignSelf::Normal,
            AlignItems::Start => AlignSelf::Start,
            AlignItems::End => AlignSelf::End,
            AlignItems::SelfStart => AlignSelf::Start,
            AlignItems::SelfEnd => AlignSelf::End,
        }
    } else {
        s
    }
}

#[derive(Debug, PartialEq)]
struct FlexItem<T: LayoutTreeNode> {
    child_index: usize,
    pub(crate) order: i32,

    size: OptionSize<T::Length>,
    margin: EdgeOption<T::Length>,
    border: Edge<T::Length>,
    padding_border: Edge<T::Length>,
    min_max_limit: MinMaxLimit<T::Length>,

    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: T::Length,
    inner_flex_basis: T::Length,
    frozen: bool,

    hypothetical_inner_size: Size<T::Length>,
    hypothetical_outer_size: Size<T::Length>,
    target_size: Size<T::Length>,
    outer_target_size: Size<T::Length>,

    final_align_self: AlignSelf,
    first_baseline_ascent: Vector<T::Length>,
    last_baseline_ascent: Vector<T::Length>,
    inner_writing_dir: AxisDirection,
    early_positioning: EarlyPositioning<T::Length>,

    extra_offset_main: T::Length,
    extra_offset_cross: T::Length,
}

#[derive(Debug, PartialEq)]
enum EarlyPositioning<L> {
    AcceptChildCrossSize,
    StretchedCrossSize(L),
    NoPositioning,
}

impl<L> EarlyPositioning<L> {
    fn is_stretched(&self) -> bool {
        matches!(self, Self::StretchedCrossSize(_))
    }
}

#[derive(Debug, PartialEq)]
struct FlexLine<'a, T: LayoutTreeNode> {
    items: &'a mut [FlexItem<T>],
    cross_size: T::Length,
    extra_offset_cross: T::Length,
    first_baseline_ascent: T::Length,
}

pub(crate) trait FlexBox<T: LayoutTreeNode> {
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length>;
}

impl<T: LayoutTreeNode> FlexBox<T> for LayoutUnit<T> {
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length> {
        let style = node.style();
        let flex_direction = style.flex_direction();
        let flex_wrap = style.flex_wrap();
        let dir = match flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => AxisDirection::Horizontal,
            FlexDirection::Column | FlexDirection::ColumnReverse => AxisDirection::Vertical,
        };
        let main_dir_rev = match flex_direction {
            FlexDirection::Row | FlexDirection::Column => AxisReverse::NotReversed,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse => AxisReverse::Reversed,
        };
        let cross_dir_rev = match flex_wrap {
            FlexWrap::NoWrap | FlexWrap::Wrap => AxisReverse::NotReversed,
            FlexWrap::WrapReverse => AxisReverse::Reversed,
        };

        let axis_info = AxisInfo::from_writing_mode(style.writing_mode());

        let collapsed_margin = CollapsedBlockMargin::from_margin(
            margin
                .or_zero()
                .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
            margin
                .or_zero()
                .main_axis_end(axis_info.dir, axis_info.main_dir_rev),
        );
        if let Some(x) = self.is_requested_size_fixed(&request, Some(collapsed_margin)) {
            return x;
        }

        let main_size_request_kind = match dir {
            AxisDirection::Horizontal => ComputeRequestKind::RowSize,
            AxisDirection::Vertical => ComputeRequestKind::ColSize,
        };

        let requested_size = request.size;
        let requested_inner_size = Normalized(OptionSize::new(
            requested_size.width - padding_border.horizontal(),
            requested_size.height - padding_border.vertical(),
        ));
        let self_min_max_limit =
            self.normalized_min_max_limit(node, *request.parent_inner_size, border, padding_border);

        let mut container_size = Size::zero(); // the final node size
        let mut inner_container_size = Size::zero(); // the final node size without padding and border
        let mut self_first_baseline_ascent: Option<Vector<T::Length>> = None;
        let mut self_last_baseline_ascent: Option<Vector<T::Length>> = None;

        // 9.1 Initial Setup

        // 1. Generate anonymous flex items as described in §4 Flex Items.

        let mut flex_items: Vec<FlexItem<T>> =
            generate_anonymous_flex_items(node, style, &requested_inner_size, dir);

        // 9.2. Line Length Determination

        // 2. Determine the available main and cross space for the flex items.
        //    For each dimension, if that dimension of the flex container’s content box
        //    is a definite size, use that; if that dimension of the flex container is
        //    being sized under a min or max-content constraint, the available space in
        //    that dimension is that constraint; otherwise, subtract the flex container’s
        //    margin, border, and padding from the space available to the flex container
        //    in that dimension and use that value. This might result in an infinite value.

        let available_space = {
            Normalized(OptionSize::new(
                requested_size.width.or(request.max_content.width) - padding_border.horizontal(),
                requested_size.height.or(request.max_content.height) - padding_border.vertical(),
            ))
        };
        let inner_max_content = OptionSize::new(
            request.max_content.width - padding_border.horizontal(),
            request.max_content.height - padding_border.vertical(),
        );

        let mut flex_lines: Vec<_>;

        // 3. Determine the flex base size and hypothetical main size of each item:

        for flex_child in &mut flex_items {
            let child_node = node
                .tree_visitor()
                .child_at(flex_child.child_index)
                .unwrap();
            let mut child = child_node.layout_node().unit();
            let child_style = child_node.style();

            // A. If the item has a definite used flex basis, that’s the flex base size.

            let flex_basis = child_style
                .flex_basis()
                .resolve(requested_inner_size.main_size(dir), child_node);

            if flex_basis.is_some() {
                flex_child.flex_basis = flex_basis.or_zero();
                continue;
            };

            // B. If the flex item has an intrinsic aspect ratio,
            //    a used flex basis of content, and a definite cross size,
            //    then the flex base size is calculated from its inner
            //    cross size and the flex item’s intrinsic aspect ratio.

            if let Some(ratio) = child_style.aspect_ratio() {
                let (style_main_size, style_cross_size) = match dir {
                    AxisDirection::Horizontal => (child_style.width(), child_style.height()),
                    AxisDirection::Vertical => (child_style.height(), child_style.width()),
                };
                if ratio > 0.
                    && style_main_size
                        .resolve(requested_inner_size.main_size(dir), child_node)
                        .is_none()
                {
                    let option_cross = if let Some(cross) = style_cross_size
                        .resolve(requested_inner_size.cross_size(dir), child_node)
                        .val()
                    {
                        Some(cross)
                    } else if flex_child.final_align_self == AlignSelf::Stretch {
                        requested_inner_size.cross_size(dir).val()
                    } else {
                        None
                    };
                    if let Some(cross) = option_cross {
                        match dir {
                            AxisDirection::Horizontal => {
                                flex_child.flex_basis = cross.mul_f32(ratio);
                            }
                            AxisDirection::Vertical => {
                                flex_child.flex_basis = cross.div_f32(ratio);
                            }
                        }
                        continue;
                    }
                }
            }

            // C. If the used flex basis is content or depends on its available space,
            //    and the flex container is being sized under a min-content or max-content
            //    constraint (e.g. when performing automatic table layout [CSS21]),
            //    size the item under that constraint. The flex base size is the item’s
            //    resulting main size.

            // D. Otherwise, if the used flex basis is content or depends on its
            //    available space, the available main size is infinite, and the flex item’s
            //    inline axis is parallel to the main axis, lay the item out using the rules
            //    for a box in an orthogonal flow [CSS3-WRITING-MODES]. The flex base size
            //    is the item’s max-content main size.

            // E. Otherwise, size the item into the available space using its used flex basis
            //    in place of its main size, treating a value of content as max-content.
            //    If a cross size is needed to determine the main size (e.g. when the
            //    flex item’s main size is in its block axis) and the flex item’s cross size
            //    is auto and not definite, in this calculation use fit-content as the
            //    flex item’s cross size. The flex base size is the item’s resulting main size.

            let cross_size = if !flex_child.size.cross_size(dir).is_some()
                && flex_child.final_align_self == AlignSelf::Stretch
            {
                available_space.cross_size(dir) - flex_child.margin.cross_axis_sum(dir)
            } else {
                flex_child.size.cross_size(dir)
            };

            // calculate the style-based size and use it as flex basis
            let mut size = flex_child.size;
            size.set_cross_size(dir, cross_size);
            let size = flex_child.min_max_limit.normalized_size(size);
            let mut max_content = OptionSize::new(OptionNum::none(), OptionNum::none());
            max_content.set_cross_size(
                dir,
                cross_size
                    .or(available_space.cross_size(dir) - flex_child.margin.cross_axis_sum(dir)),
            );
            let max_content = Normalized(max_content);
            flex_child.flex_basis = child
                .compute_internal(
                    env,
                    child_node,
                    ComputeRequest {
                        size,
                        parent_inner_size: available_space,
                        max_content,
                        kind: main_size_request_kind.shift_to_all_size(),
                        parent_is_block: false,
                    },
                )
                .size
                .main_size(dir);
        }

        // The hypothetical main size is the item’s flex base size clamped according to its
        // used min and max main sizes (and flooring the content box size at zero).

        for flex_child in &mut flex_items {
            flex_child.inner_flex_basis =
                flex_child.flex_basis - flex_child.padding_border.main_axis_sum(dir);
            let hypothetical_inner_size = flex_child
                .min_max_limit
                .main_size(flex_child.flex_basis, dir);
            flex_child
                .hypothetical_inner_size
                .set_main_size(dir, hypothetical_inner_size);
            flex_child.hypothetical_outer_size.set_main_size(
                dir,
                hypothetical_inner_size + flex_child.margin.main_axis_sum(dir),
            );
        }

        // 9.3. Main Size Determination

        // 5. Collect flex items into flex lines:
        //    - If the flex container is single-line, collect all the flex items into
        //      a single flex line.
        //    - Otherwise, starting from the first uncollected item, collect consecutive
        //      items one by one until the first time that the next collected item would
        //      not fit into the flex container’s inner main size (or until a forced break
        //      is encountered, see §10 Fragmenting Flex Layout). If the very first
        //      uncollected item wouldn’t fit, collect just it into the line.
        //
        //      For this step, the size of a flex item is its outer hypothetical main size. (Note: This can be negative.)
        //      Repeat until all flex items have been collected into flex lines
        //
        //      Note that the "collect as many" line will collect zero-sized flex items onto
        //      the end of the previous line even if the last non-zero item exactly "filled up" the line.

        flex_lines = {
            let mut lines = Vec::with_capacity(1);
            if style.flex_wrap() == FlexWrap::NoWrap {
                lines.push(FlexLine {
                    items: &mut flex_items[..],
                    cross_size: T::Length::zero(),
                    extra_offset_cross: T::Length::zero(),
                    first_baseline_ascent: T::Length::zero(),
                });
            } else {
                let mut flex_items = &mut flex_items[..];
                let main_axis_gap = main_axis_gap::<T>(dir, style, node, &requested_inner_size);
                while !flex_items.is_empty() {
                    let mut line_length = T::Length::zero();
                    let index = flex_items
                        .iter()
                        .enumerate()
                        .find(|&(idx, child)| {
                            let gap_contribution = if idx == 0 {
                                T::Length::zero()
                            } else {
                                main_axis_gap
                            };
                            line_length +=
                                child.hypothetical_outer_size.main_size(dir) + gap_contribution;
                            match available_space.main_size(dir).val() {
                                Some(x) => line_length > x && idx != 0,
                                None => false,
                            }
                        })
                        .map(|(idx, _)| idx)
                        .unwrap_or_else(|| flex_items.len());
                    let (items, rest) = flex_items.split_at_mut(index);
                    lines.push(FlexLine {
                        items,
                        cross_size: T::Length::zero(),
                        extra_offset_cross: T::Length::zero(),
                        first_baseline_ascent: T::Length::zero(),
                    });
                    flex_items = rest;
                }
            }
            lines
        };

        // 6. Resolve the flexible lengths of all the flex items to find their used main size.
        //    See §9.7 Resolving Flexible Lengths.
        //
        // 9.7. Resolving Flexible Lengths

        let multi_flex_line = flex_lines.len() > 1;
        for line in &mut flex_lines {
            let total_main_axis_gap = sum_axis_gaps(
                main_axis_gap::<T>(dir, style, node, &requested_inner_size),
                line.items.len(),
            );
            // 1. Determine the used flex factor. Sum the outer hypothetical main sizes of all
            //    items on the line. If the sum is less than the flex container’s inner main size,
            //    use the flex grow factor for the rest of this algorithm; otherwise, use the
            //    flex shrink factor.

            let used_flex_factor: T::Length = total_main_axis_gap
                + length_sum(
                    line.items
                        .iter()
                        .map(|child| child.hypothetical_outer_size.main_size(dir)),
                );
            let growing = used_flex_factor < requested_inner_size.main_size(dir).or_zero();
            let shrinking = !growing;

            // if the main size is unlimited but the main max-content is limited,
            // and the sum of hypothetical main sizes exceeds the max-content,
            // the request size should be limited to the max-content (but no less than sum of the min-content).
            let shrink_max_content = match requested_inner_size.main_size(dir).is_none() {
                false => None,
                true => inner_max_content
                    .main_size(dir)
                    .val()
                    .filter(|&max_content| used_flex_factor > max_content),
            };
            let target_len = if let Some(max_content) = shrink_max_content {
                let sum_of_min_content = length_sum(line.items.iter().map(|flex_child| {
                    let child_node = node
                        .tree_visitor()
                        .child_at(flex_child.child_index)
                        .unwrap();
                    let mut child = child_node.layout_node().unit();
                    let (_, child_border, child_padding_border) = child.margin_border_padding(
                        child_node,
                        OptionSize::new(OptionNum::none(), OptionNum::none()),
                    );
                    let css_size = child.css_border_box_size(
                        child_node,
                        OptionSize::new(OptionNum::none(), OptionNum::none()),
                        child_border,
                        child_padding_border,
                    );
                    let size = flex_child.min_max_limit.normalized_size(css_size);
                    let mut max_content = OptionSize::new(OptionNum::none(), OptionNum::none());
                    max_content.set_main_size(dir, OptionNum::zero());
                    let max_content = flex_child.min_max_limit.normalized_size(max_content);
                    child
                        .compute_internal(
                            env,
                            child_node,
                            ComputeRequest {
                                size,
                                parent_inner_size: available_space,
                                max_content,
                                kind: main_size_request_kind.shift_to_all_size(),
                                parent_is_block: false,
                            },
                        )
                        .size
                        .main_size(dir)
                }));
                OptionNum::some(sum_of_min_content.max(max_content))
            } else {
                requested_inner_size.main_size(dir)
            };

            // 2. Size inflexible items. Freeze, setting its target main size to its hypothetical main size
            //    - Any item that has a flex factor of zero
            //    - If using the flex grow factor: any item that has a flex base size
            //      greater than its hypothetical main size
            //    - If using the flex shrink factor: any item that has a flex base size
            //      smaller than its hypothetical main size

            for flex_child in line.items.iter_mut() {
                let child_node = node
                    .tree_visitor()
                    .child_at(flex_child.child_index)
                    .unwrap();
                let child_style = child_node.style();
                let hypothetical_inner_main_size =
                    flex_child.hypothetical_inner_size.main_size(dir);
                flex_child
                    .target_size
                    .set_main_size(dir, hypothetical_inner_main_size);

                if (child_style.flex_grow() == 0. && child_style.flex_shrink() == 0.)
                    || (growing && flex_child.flex_basis > hypothetical_inner_main_size)
                    || (shrinking && flex_child.flex_basis < hypothetical_inner_main_size)
                {
                    flex_child.frozen = true;
                }
                if multi_flex_line {
                    if let EarlyPositioning::StretchedCrossSize(_) = flex_child.early_positioning {
                        flex_child.early_positioning = EarlyPositioning::NoPositioning;
                    }
                }
            }

            // 3. Calculate initial free space. Sum the outer sizes of all items on the line,
            //    and subtract this from the flex container’s inner main size. For frozen items,
            //    use their outer target main size; for other items, use their outer flex base size.

            let used_space: T::Length = total_main_axis_gap
                + length_sum(line.items.iter().map(|child| {
                    child.margin.main_axis_sum(dir)
                        + if child.frozen {
                            child.target_size.main_size(dir)
                        } else {
                            child.flex_basis
                        }
                }));
            let initial_free_space = (target_len - used_space).or_zero();
            let mut prev_free_space: Option<T::Length> = None;

            // 4. Loop

            loop {
                // a. Check for flexible items. If all the flex items on the line are frozen,
                //    free space has been distributed; exit this loop.

                if line.items.iter().all(|child| child.frozen) {
                    break;
                }

                // b. Calculate the remaining free space as for initial free space, above.
                //    If the sum of the unfrozen flex items’ flex factors is less than one,
                //    multiply the initial free space by this sum. If the magnitude of this
                //    value is less than the magnitude of the remaining free space, use this
                //    as the remaining free space.

                let used_space: T::Length = total_main_axis_gap
                    + length_sum(line.items.iter().map(|child| {
                        child.margin.main_axis_sum(dir)
                            + if child.frozen {
                                child.target_size.main_size(dir)
                            } else {
                                child.flex_basis
                            }
                    }));

                let mut unfrozen: Vec<&mut FlexItem<T>> = line
                    .items
                    .iter_mut()
                    .filter(|child| !child.frozen)
                    .collect();

                let (sum_flex_grow, sum_flex_shrink): (f32, f32) =
                    unfrozen
                        .iter()
                        .fold((0., 0.), |(flex_grow, flex_shrink), child| {
                            (flex_grow + child.flex_grow, flex_shrink + child.flex_shrink)
                        });
                let free_space = if growing && sum_flex_grow < 1.0 {
                    (initial_free_space.mul_f32(sum_flex_grow) - total_main_axis_gap)
                        .maybe_min(target_len - used_space)
                } else if shrinking && sum_flex_shrink < 1.0 {
                    (initial_free_space.mul_f32(sum_flex_shrink) - total_main_axis_gap)
                        .maybe_max(target_len - used_space)
                } else {
                    (target_len - used_space).or_zero()
                };
                if let Some(prev_free_space) = prev_free_space {
                    if free_space == prev_free_space {
                        break;
                    }
                }
                prev_free_space = Some(free_space);

                // c. Distribute free space proportional to the flex factors.
                //    - If the remaining free space is zero
                //        Do Nothing
                //    - If using the flex grow factor
                //        Find the ratio of the item’s flex grow factor to the sum of the
                //        flex grow factors of all unfrozen items on the line. Set the item’s
                //        target main size to its flex base size plus a fraction of the remaining
                //        free space proportional to the ratio.
                //    - If using the flex shrink factor
                //        For every unfrozen item on the line, multiply its flex shrink factor by
                //        its inner flex base size, and note this as its scaled flex shrink factor.
                //        Find the ratio of the item’s scaled flex shrink factor to the sum of the
                //        scaled flex shrink factors of all unfrozen items on the line. Set the item’s
                //        target main size to its flex base size minus a fraction of the absolute value
                //        of the remaining free space proportional to the ratio. Note this may result
                //        in a negative inner main size; it will be corrected in the next step.
                //    - Otherwise
                //        Do Nothing

                if free_space.is_normal() {
                    if growing && sum_flex_grow > 0.0 {
                        for child in &mut unfrozen {
                            child.target_size.set_main_size(
                                dir,
                                child.flex_basis
                                    + free_space.mul_f32(child.flex_grow / sum_flex_grow),
                            );
                        }
                    } else if shrinking && sum_flex_shrink > 0.0 {
                        let sum_scaled_shrink_factor: T::Length = length_sum(
                            unfrozen
                                .iter()
                                .map(|child| child.inner_flex_basis.mul_f32(child.flex_shrink)),
                        );
                        if sum_scaled_shrink_factor > T::Length::zero() {
                            for child in &mut unfrozen {
                                let scaled_shrink_factor =
                                    child.inner_flex_basis.mul_f32(child.flex_shrink);
                                child.target_size.set_main_size(
                                    dir,
                                    child.flex_basis
                                        + free_space
                                            * (scaled_shrink_factor / sum_scaled_shrink_factor),
                                );
                            }
                        }
                    }
                }

                // d. Fix min/max violations. Clamp each non-frozen item’s target main size by its
                //    used min and max main sizes and floor its content-box size at zero. If the
                //    item’s target main size was made smaller by this, it’s a max violation.
                //    If the item’s target main size was made larger by this, it’s a min violation.

                // e. Freeze over-flexed items. The total violation is the sum of the adjustments
                //    from the previous step ∑(clamped size - unclamped size). If the total violation is:
                //    - Zero
                //        Freeze all items.
                //    - Positive
                //        Freeze all the items with min violations.
                //    - Negative
                //        Freeze all the items with max violations.

                for flex_child in unfrozen.iter_mut() {
                    let cur = flex_child.target_size.main_size(dir);
                    let clamped = flex_child.min_max_limit.main_size(cur, dir);
                    flex_child.target_size.set_main_size(dir, clamped);
                    if clamped != cur {
                        flex_child.frozen = true;
                        prev_free_space = None;
                    }
                }

                // f. Return to the start of this loop.
            }

            for flex_child in line.items.iter_mut() {
                flex_child.outer_target_size.set_main_size(
                    dir,
                    flex_child.target_size.main_size(dir) + flex_child.margin.main_axis_sum(dir),
                );
            }
        }

        // Not part of the spec from what i can see but seems correct
        let container_main_size = self_min_max_limit.main_size(
            requested_size.main_size(dir).val().unwrap_or_else(|| {
                let longest_line = flex_lines.iter().fold(T::Length::zero(), |acc, line| {
                    let length: T::Length = length_sum(
                        line.items
                            .iter()
                            .map(|item| item.outer_target_size.main_size(dir)),
                    );
                    acc.max(length)
                });
                let size = longest_line + padding_border.main_axis_sum(dir);
                match available_space.main_size(dir).val() {
                    Some(val) if flex_lines.len() > 1 && size < val => val,
                    _ => size,
                }
            }),
            dir,
        );
        container_size.set_main_size(dir, container_main_size);
        inner_container_size
            .set_main_size(dir, container_main_size - padding_border.main_axis_sum(dir));

        // TODO consider stop here if we only need main size
        // if (request.kind == ComputeRequestKind::RowSize && dir == AxisDirection::Horizontal)
        //     || (request.kind == ComputeRequestKind::ColSize && dir == AxisDirection::Vertical) {
        //     let size = self.min_max_size_limit(
        //         node,
        //         *request.parent_inner_size,
        //         container_size,
        //         border,
        //         padding_border,
        //     );
        //     let ret = ComputeResult {
        //         size,
        //         first_baseline_ascent: Vector::zero(),
        //         last_baseline_ascent: Vector::zero(),
        //         collapsed_margin,
        //     };
        //     self.cache.write(request, ret);
        //     return ret;
        // }

        // [9.4. Cross Size Determination](https://www.w3.org/TR/css-flexbox-1/#cross-sizing)

        // 7. Determine the hypothetical cross size of each item.
        // - [**Determine the hypothetical cross size of each item**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-item)
        //      by performing layout with the used main size and the available space, treating auto as fit-content.
        let flex_wrap = style.flex_wrap() == FlexWrap::Wrap;
        for line in &mut flex_lines {
            for flex_child in line.items.iter_mut() {
                let child_node = node
                    .tree_visitor()
                    .child_at(flex_child.child_index)
                    .unwrap();
                let mut child = child_node.layout_node().unit();

                let mut size = size_to_option(flex_child.target_size);
                let cross_size = flex_child.min_max_limit.maybe().cross_size(dir, {
                    if !flex_wrap && flex_child.early_positioning.is_stretched() {
                        match flex_child.early_positioning {
                            EarlyPositioning::StretchedCrossSize(x) => OptionNum::some(x),
                            _ => unreachable!(),
                        }
                    } else {
                        let mut cross_size = flex_child.size.cross_size(dir);
                        if cross_size.is_none() {
                            if let Some(ratio) = child_node.style().aspect_ratio() {
                                match dir {
                                    AxisDirection::Horizontal => {
                                        cross_size = resolve_aspect_ratio_height(
                                            ratio,
                                            flex_child.target_size.main_size(dir),
                                        );
                                    }
                                    AxisDirection::Vertical => {
                                        cross_size = resolve_aspect_ratio_width(
                                            ratio,
                                            flex_child.target_size.main_size(dir),
                                        );
                                    }
                                }
                            }
                        }
                        cross_size
                    }
                });
                size.set_cross_size(dir, cross_size);
                let mut parent_size = size_to_option(inner_container_size);
                parent_size.set_cross_size(dir, available_space.cross_size(dir));
                let mut max_content = size;
                max_content.set_cross_size(
                    dir,
                    size.cross_size(dir)
                        .or(inner_max_content.cross_size(dir)
                            - flex_child.margin.cross_axis_sum(dir)),
                );
                let child_res = child.compute_internal(
                    env,
                    child_node,
                    ComputeRequest {
                        size: Normalized(size), // main_size and cross_size is both normalized above
                        parent_inner_size: Normalized(parent_size), // main_size and cross_size is both normalized above
                        max_content: Normalized(max_content), // main_size and cross_size is both normalized above
                        kind: request.kind.shift_to_all_size_with_position(
                            flex_child.early_positioning != EarlyPositioning::NoPositioning,
                        ),
                        parent_is_block: false,
                    },
                );
                flex_child
                    .hypothetical_inner_size
                    .set_cross_size(dir, child_res.size.cross_size(dir));
                flex_child.hypothetical_outer_size.set_cross_size(
                    dir,
                    flex_child.hypothetical_inner_size.cross_size(dir)
                        + flex_child.margin.cross_axis_sum(dir),
                );
                flex_child.first_baseline_ascent = child_res.first_baseline_ascent;
                flex_child.last_baseline_ascent = child_res.last_baseline_ascent;
            }
        }

        // 8. Calculate the cross size of each flex line.
        //
        // - [**Calculate the cross size of each flex line**](https://www.w3.org/TR/css-flexbox-1/#algo-cross-line).
        //
        //    If the flex container is single-line and has a definite cross size, the cross size of the flex line is the flex container’s inner cross size.
        //
        //    Otherwise, for each flex line:
        //
        //    1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose align-self is baseline, and whose cross-axis margins
        //       are both non-auto. Find the largest of the distances between each item’s baseline and its hypothetical outer cross-start edge,
        //       and the largest of the distances between each item’s baseline and its hypothetical outer cross-end edge, and sum these two values.
        //
        //    2. Among all the items not collected by the previous step, find the largest outer hypothetical cross size.
        //
        //    3. The used cross-size of the flex line is the largest of the numbers found in the previous two steps and zero.
        //
        //    If the flex container is single-line, then clamp the line’s cross-size to be within the container’s computed min
        //    and max cross sizes.
        //
        //    Note that if CSS 2.1’s definition of min/max-width/height applied more generally, this behavior would fall out automatically.

        let padding_border_cross = padding_border.cross_axis_sum(dir);
        if flex_lines.len() == 1
            && requested_size.cross_size(dir).is_some()
            && (!flex_wrap
                || matches!(
                    style.align_content(),
                    AlignContent::Stretch | AlignContent::SpaceEvenly | AlignContent::SpaceAround
                ))
        {
            flex_lines[0].cross_size = self_min_max_limit
                .cross_size(requested_size.cross_size(dir).or_zero(), dir)
                - padding_border_cross;
            let max_baseline = flex_lines[0]
                .items
                .iter()
                .map(|child| {
                    if child.final_align_self == AlignSelf::Baseline {
                        child.first_baseline_ascent.cross_axis(dir)
                    } else {
                        T::Length::zero()
                    }
                })
                .fold(T::Length::zero(), |acc, x| acc.max(x));
            flex_lines[0].first_baseline_ascent = max_baseline;
        } else {
            for line in &mut flex_lines {
                //    1. Collect all the flex items whose inline-axis is parallel to the main-axis, whose
                //       align-self is baseline, and whose cross-axis margins are both non-auto. Find the
                //       largest of the distances between each item’s baseline and its hypothetical outer
                //       cross-start edge, and the largest of the distances between each item’s baseline
                //       and its hypothetical outer cross-end edge, and sum these two values.

                //    2. Among all the items not collected by the previous step, find the largest
                //       outer hypothetical cross size.

                //    3. The used cross-size of the flex line is the largest of the numbers found in the
                //       previous two steps and zero.

                let max_baseline: T::Length = line
                    .items
                    .iter()
                    .map(|child| {
                        if child.final_align_self == AlignSelf::Baseline {
                            child.first_baseline_ascent.cross_axis(dir)
                                + child.margin.cross_axis_start(dir, cross_dir_rev).or_zero()
                        } else {
                            T::Length::zero()
                        }
                    })
                    .fold(T::Length::zero(), |acc, x| acc.max(x));
                line.first_baseline_ascent = max_baseline;
                line.cross_size = line
                    .items
                    .iter()
                    .map(|child| {
                            dbg!(&child.first_baseline_ascent, &child.margin);
                        if child.final_align_self == AlignSelf::Baseline {
                            max_baseline - child.first_baseline_ascent.cross_axis(dir)
                                + child.hypothetical_outer_size.cross_size(dir)
                                + child.margin.cross_axis_end(dir, cross_dir_rev).or_zero()
                        } else {
                            child.hypothetical_outer_size.cross_size(dir)
                        }
                    })
                    .fold(T::Length::zero(), |acc, x| acc.max(x));
                dbg!(line.cross_size, max_baseline);
            }

            //    If the flex container is single-line, then clamp the line’s cross-size to be within the container’s computed min
            //    and max cross sizes.
            //
            //    Note that if CSS 2.1’s definition of min/max-width/height applied more generally, this behavior would fall out automatically.
            if !flex_wrap {
                flex_lines[0].cross_size = self_min_max_limit
                    .cross_size(flex_lines[0].cross_size + padding_border_cross, dir)
                    - padding_border_cross;
            }
        }

        // 9. Handle 'align-content: stretch'.
        //  - [**Handle 'align-content: stretch'**](https://www.w3.org/TR/css-flexbox-1/#algo-line-stretch).
        //      If the flex container has a definite cross size, align-content is stretch,
        //      and the sum of the flex lines' cross sizes is less than the flex container’s inner cross size,
        //      increase the cross size of each flex line by equal amounts such that the sum of their cross sizes
        //      exactly equals the flex container’s inner cross size.

        if style.align_content() == AlignContent::Stretch {
            let requested_cross_size = requested_size.cross_size(dir).or_zero();
            let min_inner_cross =
                self_min_max_limit.cross_size(requested_cross_size, dir) - padding_border_cross;
            let total_cross_axis_gap = sum_axis_gaps(
                cross_axis_gap::<T>(dir, style, node, &requested_inner_size),
                flex_lines.len(),
            );
            let line_total_cross: T::Length =
                length_sum(flex_lines.iter().map(|line| line.cross_size)) + total_cross_axis_gap;

            if line_total_cross < min_inner_cross {
                let remaining = min_inner_cross - line_total_cross;
                let addition = remaining.div_i32(flex_lines.len() as i32);
                flex_lines
                    .iter_mut()
                    .for_each(|line| line.cross_size += addition);
            }
        }

        // 15. Determine the flex container’s used cross size:
        //     - If the cross size property is a definite size, use that, clamped by the used
        //       min and max cross sizes of the flex container.
        //     - Otherwise, use the sum of the flex lines' cross sizes, clamped by the used
        //       min and max cross sizes of the flex container.

        let total_cross_size = length_sum(flex_lines.iter().map(|line| line.cross_size));
        container_size.set_cross_size(
            dir,
            self_min_max_limit.cross_size(
                requested_size
                    .cross_size(dir)
                    .unwrap_or(total_cross_size + padding_border_cross),
                dir,
            ),
        );
        inner_container_size
            .set_cross_size(dir, container_size.cross_size(dir) - padding_border_cross);

        let container_size = Normalized(container_size); // container size is both normalized above
        let inner_container_size = Normalized(inner_container_size); // size is both normalized above
        let inner_container_option_size = Normalized(size_to_option(*inner_container_size));

        // stop here if we only need size, no positioning
        if request.kind != ComputeRequestKind::Position {
            let ret = ComputeResult {
                size: container_size,
                first_baseline_ascent: Vector::zero(),
                last_baseline_ascent: Vector::zero(),
                collapsed_margin,
            };
            self.cache.write_all_size(node, &request, ret);
            return ret;
        }

        // 10. Collapse visibility:collapse items. If any flex items have visibility: collapse,
        //     note the cross size of the line they’re in as the item’s strut size, and restart
        //     layout from the beginning.
        //
        //     In this second layout round, when collecting items into lines, treat the collapsed
        //     items as having zero main size. For the rest of the algorithm following that step,
        //     ignore the collapsed items entirely (as if they were display:none) except that after
        //     calculating the cross size of the lines, if any line’s cross size is less than the
        //     largest strut size among all the collapsed items in the line, set its cross size to
        //     that strut size.
        //
        //     Skip this step in the second layout round.

        // IDEA consider support visibility:collapse

        // 11. Determine the used cross size of each flex item. If a flex item has align-self: stretch,
        //     its computed cross size property is auto, and neither of its cross-axis margins are auto,
        //     the used outer cross size is the used cross size of its flex line, clamped according to
        //     the item’s used min and max cross sizes. Otherwise, the used cross size is the item’s
        //     hypothetical cross size.
        //
        //     If the flex item has align-self: stretch, redo layout for its contents, treating this
        //     used size as its definite cross size so that percentage-sized children can be resolved.
        //
        //     Note that this step does not affect the main size of the flex item, even if it has an
        //     intrinsic aspect ratio.

        for line in &mut flex_lines {
            let line_cross_size = line.cross_size;
            for flex_child in line.items.iter_mut() {
                let child_node = node
                    .tree_visitor()
                    .child_at(flex_child.child_index)
                    .unwrap();
                let mut child = child_node.layout_node().unit();

                let child_margin_cross = flex_child.margin.cross_axis_sum(dir);
                match flex_child.early_positioning {
                    EarlyPositioning::NoPositioning => {
                        flex_child.target_size.set_cross_size(
                            dir,
                            flex_child
                                .min_max_limit
                                .cross_size(line_cross_size - child_margin_cross, dir),
                        );
                        let size = size_to_option(flex_child.target_size);
                        child.compute_internal(
                            env,
                            child_node,
                            ComputeRequest {
                                size: Normalized(size), // main_size and cross_size is both normalized above
                                parent_inner_size: inner_container_option_size, // main_size and cross_size is both normalized above
                                max_content: Normalized(size), // main_size and cross_size is both normalized above
                                kind: ComputeRequestKind::Position,
                                parent_is_block: false,
                            },
                        );
                    }
                    _ => {
                        flex_child.target_size.set_cross_size(
                            dir,
                            flex_child.hypothetical_inner_size.cross_size(dir),
                        );
                    }
                }
                flex_child.outer_target_size.set_cross_size(
                    dir,
                    flex_child.target_size.cross_size(dir) + child_margin_cross,
                );
            }
        }

        // 9.5. Main-Axis Alignment

        // 12. Distribute any remaining free space. For each flex line:
        //     1. If the remaining free space is positive and at least one main-axis margin on this
        //        line is auto, distribute the free space equally among these margins. Otherwise,
        //        set all auto margins to zero.
        //     2. Align the items along the main-axis per justify-content.

        for line in &mut flex_lines {
            let total_main_axis_gap = sum_axis_gaps(
                main_axis_gap::<T>(dir, style, node, &requested_inner_size),
                line.items.len(),
            );
            let used_space: T::Length = total_main_axis_gap
                + length_sum(
                    line.items
                        .iter()
                        .map(|child| child.outer_target_size.main_size(dir)),
                );
            let free_space = inner_container_size.main_size(dir) - used_space;
            let mut num_auto_margins = 0;

            for flex_child in line.items.iter_mut() {
                if flex_child
                    .margin
                    .main_axis_start(dir, main_dir_rev)
                    .is_none()
                {
                    num_auto_margins += 1;
                }
                if flex_child.margin.main_axis_end(dir, main_dir_rev).is_none() {
                    num_auto_margins += 1;
                }
            }

            if free_space > T::Length::zero() && num_auto_margins > 0 {
                let margin = OptionNum::some(free_space.div_i32(num_auto_margins));
                for flex_child in line.items.iter_mut() {
                    if flex_child
                        .margin
                        .main_axis_start(dir, main_dir_rev)
                        .is_none()
                    {
                        flex_child
                            .margin
                            .set_main_axis_start(dir, main_dir_rev, margin);
                    }
                    if flex_child.margin.main_axis_end(dir, main_dir_rev).is_none() {
                        flex_child
                            .margin
                            .set_main_axis_end(dir, main_dir_rev, margin);
                    }
                }
            } else {
                let num_items = line.items.len() as i32;
                let is_reversed = main_dir_rev == AxisReverse::Reversed;
                for (index, flex_child) in line.items.iter_mut().enumerate() {
                    let is_first = index == 0;
                    let gap = main_axis_gap::<T>(dir, style, node, &requested_inner_size);
                    flex_child.extra_offset_main = if is_first {
                        match style.justify_content() {
                            JustifyContent::Start
                            | JustifyContent::Baseline
                            | JustifyContent::Stretch => T::Length::zero(),
                            JustifyContent::FlexStart => T::Length::zero(),
                            JustifyContent::Center => free_space.div_i32(2),
                            JustifyContent::FlexEnd => free_space,
                            JustifyContent::End => free_space,
                            JustifyContent::Left => {
                                if is_reversed {
                                    free_space
                                } else {
                                    T::Length::zero()
                                }
                            }
                            JustifyContent::Right => {
                                if is_reversed {
                                    T::Length::zero()
                                } else {
                                    free_space
                                }
                            }
                            JustifyContent::SpaceBetween => T::Length::zero(),
                            JustifyContent::SpaceAround => {
                                if free_space >= T::Length::zero() {
                                    free_space.div_i32(num_items).div_i32(2)
                                } else {
                                    free_space.div_i32(2)
                                }
                            }
                            JustifyContent::SpaceEvenly => {
                                if free_space >= T::Length::zero() {
                                    free_space.div_i32(num_items + 1)
                                } else {
                                    free_space.div_i32(2)
                                }
                            }
                        }
                    } else {
                        let free_space = free_space.max(T::Length::zero());
                        gap + match style.justify_content() {
                            JustifyContent::FlexStart
                            | JustifyContent::Start
                            | JustifyContent::Baseline
                            | JustifyContent::Stretch => T::Length::zero(),
                            JustifyContent::Center => T::Length::zero(),
                            JustifyContent::FlexEnd | JustifyContent::End => T::Length::zero(),
                            JustifyContent::Left => T::Length::zero(),
                            JustifyContent::Right => T::Length::zero(),
                            JustifyContent::SpaceBetween => free_space.div_i32(num_items - 1),
                            JustifyContent::SpaceAround => free_space.div_i32(num_items),
                            JustifyContent::SpaceEvenly => free_space.div_i32(num_items + 1),
                        }
                    }
                }
            }
        }

        // 9.6. Cross-Axis Alignment

        // 13. Resolve cross-axis auto margins. If a flex item has auto cross-axis margins:
        //     - If its outer cross size (treating those auto margins as zero) is less than the
        //       cross size of its flex line, distribute the difference in those sizes equally
        //       to the auto margins.
        //     - Otherwise, if the block-start or inline-start margin (whichever is in the cross axis)
        //       is auto, set it to zero. Set the opposite margin so that the outer cross size of the
        //       item equals the cross size of its flex line.
        let mut max_baseline_margin: T::Length = T::Length::zero();
        for line in &mut flex_lines {
            let line_cross_size = line.cross_size;

            for flex_child in line.items.iter_mut() {
                let free_space = line_cross_size - flex_child.outer_target_size.cross_size(dir);
                let is_cross_both_auto = flex_child
                    .margin
                    .cross_axis_start(dir, cross_dir_rev)
                    .is_none()
                    && flex_child
                        .margin
                        .cross_axis_end(dir, cross_dir_rev)
                        .is_none();
                if is_cross_both_auto {
                    flex_child.margin.set_cross_axis_start(
                        dir,
                        cross_dir_rev,
                        OptionNum::some(free_space.div_i32(2)),
                    );
                    flex_child.margin.set_cross_axis_end(
                        dir,
                        cross_dir_rev,
                        OptionNum::some(free_space.div_i32(2)),
                    );
                } else if flex_child
                    .margin
                    .cross_axis_start(dir, cross_dir_rev)
                    .is_none()
                {
                    flex_child.margin.set_cross_axis_start(
                        dir,
                        cross_dir_rev,
                        OptionNum::some(free_space),
                    );
                } else if flex_child
                    .margin
                    .cross_axis_end(dir, cross_dir_rev)
                    .is_none()
                {
                    flex_child.margin.set_cross_axis_end(
                        dir,
                        cross_dir_rev,
                        OptionNum::some(free_space),
                    );
                } else {
                    flex_child.extra_offset_cross = match flex_child.final_align_self {
                        AlignSelf::FlexStart | AlignSelf::Start | AlignSelf::SelfStart => {
                            T::Length::zero()
                        }
                        AlignSelf::FlexEnd | AlignSelf::End | AlignSelf::SelfEnd => free_space,
                        AlignSelf::Center => free_space.div_i32(2),
                        AlignSelf::Baseline => {
                            max_baseline_margin = max_baseline_margin.max(
                                flex_child.first_baseline_ascent.cross_axis(dir)
                                    + flex_child
                                        .margin
                                        .cross_axis_start(dir, cross_dir_rev)
                                        .or_zero(),
                            );
                            line.first_baseline_ascent
                                - flex_child.first_baseline_ascent.cross_axis(dir)
                        }
                        AlignSelf::Stretch | AlignSelf::Normal => T::Length::zero(),
                        AlignSelf::Auto => unreachable!(),
                    };
                }
            }
        }

        // 16. Align all flex lines per align-content.

        let num_lines = flex_lines.len() as i32;
        let gap = cross_axis_gap::<T>(dir, style, node, &requested_inner_size);
        let total_cross_axis_gap = sum_axis_gaps(gap, flex_lines.len());
        let free_space = (inner_container_size.cross_size(dir) - total_cross_size)
            .max(T::Length::zero())
            + total_cross_axis_gap;
        for (index, line) in flex_lines.iter_mut().enumerate() {
            let is_first = index == 0;
            line.extra_offset_cross = if is_first {
                match style.align_content() {
                    AlignContent::Start | AlignContent::Normal | AlignContent::Baseline => {
                        T::Length::zero()
                    }
                    AlignContent::FlexStart => T::Length::zero(),
                    AlignContent::End => free_space,
                    AlignContent::FlexEnd => free_space,
                    AlignContent::Center => free_space.div_i32(2),
                    AlignContent::Stretch => T::Length::zero(),
                    AlignContent::SpaceBetween => T::Length::zero(),
                    AlignContent::SpaceEvenly => {
                        if free_space >= T::Length::zero() {
                            free_space.div_i32(num_lines + 1)
                        } else {
                            free_space.div_i32(2)
                        }
                    }
                    AlignContent::SpaceAround => {
                        if free_space >= T::Length::zero() {
                            free_space.div_i32(num_lines).div_i32(2)
                        } else {
                            free_space.div_i32(2)
                        }
                    }
                }
            } else {
                gap + match style.align_content() {
                    AlignContent::Start | AlignContent::Normal | AlignContent::Baseline => {
                        T::Length::zero()
                    }
                    AlignContent::FlexStart => T::Length::zero(),
                    AlignContent::End => T::Length::zero(),
                    AlignContent::FlexEnd => T::Length::zero(),
                    AlignContent::Center => T::Length::zero(),
                    AlignContent::Stretch => T::Length::zero(),
                    AlignContent::SpaceBetween => free_space.div_i32(num_lines - 1),
                    AlignContent::SpaceEvenly => free_space.div_i32(num_lines + 1),
                    AlignContent::SpaceAround => free_space.div_i32(num_lines),
                }
            };
        }

        // Do a final layout pass and gather the resulting layouts
        {
            let mut total_offset_cross = padding_border.cross_axis_start(dir, cross_dir_rev);

            for line in flex_lines.iter_mut() {
                let mut total_offset_main = padding_border.main_axis_start(dir, main_dir_rev);
                total_offset_cross += line.extra_offset_cross;

                for flex_child in line.items.iter_mut() {
                    let child_node = node
                        .tree_visitor()
                        .child_at(flex_child.child_index)
                        .unwrap();
                    let mut child = child_node.layout_node().unit();

                    let offset_main = total_offset_main
                        + flex_child.extra_offset_main
                        + flex_child
                            .margin
                            .main_axis_start(dir, main_dir_rev)
                            .or_zero();
                    let offset_cross = match flex_child.final_align_self {
                        AlignSelf::Baseline => {
                            total_offset_cross + max_baseline_margin
                                - flex_child.first_baseline_ascent.cross_axis(dir)
                        }
                        _ => {
                            total_offset_cross
                                + flex_child.extra_offset_cross
                                + flex_child
                                    .margin
                                    .cross_axis_start(dir, cross_dir_rev)
                                    .or_zero()
                        }
                    };
                    let baseline_diff = child.gen_origin(
                        AxisInfo {
                            main_dir_rev,
                            cross_dir_rev,
                            dir,
                        },
                        *container_size,
                        offset_main,
                        offset_cross,
                    );
                    if self_first_baseline_ascent.is_none() {
                        self_first_baseline_ascent =
                            Some(flex_child.first_baseline_ascent + baseline_diff);
                    }
                    self_last_baseline_ascent =
                        Some(flex_child.last_baseline_ascent + baseline_diff);

                    total_offset_main += flex_child.extra_offset_main
                        + flex_child
                            .margin
                            .main_axis_start(dir, main_dir_rev)
                            .or_zero()
                        + child.result.size.main_size(dir)
                        + flex_child.margin.main_axis_end(dir, main_dir_rev).or_zero();
                }

                total_offset_cross += line.cross_size;
            }
        }

        let ret = ComputeResult {
            size: container_size,
            first_baseline_ascent: self_first_baseline_ascent
                .unwrap_or_else(|| container_size.to_vector()),
            last_baseline_ascent: self_last_baseline_ascent
                .unwrap_or_else(|| container_size.to_vector()),
            collapsed_margin,
        };
        compute_special_position_children(
            env,
            node,
            &ret,
            border,
            padding_border,
            AxisInfo {
                dir,
                main_dir_rev,
                cross_dir_rev: AxisReverse::NotReversed,
            },
            true,
        );
        self.result = Rect::new(Point::zero(), ret.size.0);
        self.cache.write_position(node, &request, ret);
        ret
    }
}

/// Generate anonymous flex items.
///
/// 9.1 Initial Setup
///
/// 1. Generate anonymous flex items as described in §4 Flex Items.
///
/// https://www.w3.org/TR/css-flexbox-1/#algo-anon-box
///
#[inline]
fn generate_anonymous_flex_items<T: LayoutTreeNode>(
    node: &T,
    style: &T::Style,
    inner_size: &Normalized<OptionSize<T::Length>>,
    dir: AxisDirection,
) -> Vec<FlexItem<T>> {
    let mut flex_items: Vec<FlexItem<T>> = Vec::with_capacity(node.tree_visitor().children_len());
    node.tree_visitor()
        .for_each_child(|child_node, child_index| {
            if is_independent_positioning(child_node) {
                return;
            }
            let child_layout_unit = child_node.layout_node().unit();
            let child_style = child_node.style();
            let (margin, border, padding_border) =
                child_layout_unit.margin_border_padding(child_node, **inner_size);
            let size = child_layout_unit.css_border_box_size(
                child_node,
                **inner_size,
                border,
                padding_border,
            );
            let final_align_self = align_self::<T>(child_style, style);

            let flex_item = FlexItem {
                child_index,
                order: child_style.order(),
                size,
                margin,
                border,
                padding_border,
                min_max_limit: child_layout_unit.normalized_min_max_limit(
                    child_node,
                    **inner_size,
                    border,
                    padding_border,
                ),
                flex_grow: child_style.flex_grow().max(0.),
                flex_shrink: child_style.flex_shrink().max(0.),
                flex_basis: T::Length::zero(),
                inner_flex_basis: T::Length::zero(),
                frozen: false,
                hypothetical_inner_size: Size::zero(),
                hypothetical_outer_size: Size::zero(),
                target_size: Size::zero(),
                outer_target_size: Size::zero(),
                final_align_self: final_align_self.clone(),
                first_baseline_ascent: Vector::zero(),
                last_baseline_ascent: Vector::zero(),
                inner_writing_dir: {
                    match child_style.writing_mode() {
                        WritingMode::HorizontalTb => AxisDirection::Vertical,
                        WritingMode::VerticalLr | WritingMode::VerticalRl => {
                            AxisDirection::Horizontal
                        }
                    }
                },
                early_positioning: {
                    match final_align_self {
                        AlignSelf::Stretch => match dir {
                            AxisDirection::Horizontal => {
                                if margin.is_top_bottom_either_none() || size.height.is_some() {
                                    EarlyPositioning::AcceptChildCrossSize
                                } else if let Some(x) = inner_size.height.val() {
                                    EarlyPositioning::StretchedCrossSize(x - margin.vertical())
                                } else {
                                    EarlyPositioning::NoPositioning
                                }
                            }
                            AxisDirection::Vertical => {
                                if margin.is_left_right_either_none() || size.width.is_some() {
                                    EarlyPositioning::AcceptChildCrossSize
                                } else if let Some(x) = inner_size.width.val() {
                                    EarlyPositioning::StretchedCrossSize(x - margin.horizontal())
                                } else {
                                    EarlyPositioning::NoPositioning
                                }
                            }
                        },
                        _ => EarlyPositioning::AcceptChildCrossSize,
                    }
                },
                extra_offset_main: T::Length::zero(),
                extra_offset_cross: T::Length::zero(),
            };
            flex_items.push(flex_item);
        });
    flex_items.sort_by(|a, b| a.order.cmp(&b.order));
    flex_items
}

// ratio = width / height
#[inline]
pub(crate) fn resolve_aspect_ratio_height<L: LengthNum>(ratio: f32, width: L) -> OptionNum<L> {
    if ratio > 0. {
        OptionNum::some(width.div_f32(ratio))
    } else {
        OptionNum::none()
    }
}

#[inline]
pub(crate) fn resolve_aspect_ratio_width<L: LengthNum>(ratio: f32, height: L) -> OptionNum<L> {
    if ratio > 0. {
        OptionNum::some(height.mul_f32(ratio))
    } else {
        OptionNum::none()
    }
}
