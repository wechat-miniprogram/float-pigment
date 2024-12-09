use float_pigment_css::num_traits::Zero;

use crate::*;

/// Returns if the node has a special position, e.g. `display: none` or `position: absolute`.
#[inline]
pub fn is_independent_positioning<T: LayoutTreeNode>(node: &T) -> bool {
    let style = node.style();
    is_display_none::<T>(style) || is_out_of_flow::<T>(style)
}

#[inline(always)]
pub(crate) fn is_out_of_flow<T: LayoutTreeNode>(style: &T::Style) -> bool {
    match style.position() {
        Position::Absolute | Position::Fixed | Position::Sticky => true,
        Position::Static | Position::Relative => false,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_special_position_children<T: LayoutTreeNode>(
    env: &mut T::Env,
    node: &T,
    node_result: &ComputeResult<T::Length>,
    border: Edge<T::Length>,
    padding_border: Edge<T::Length>,
    axis_info: AxisInfo,
    accept_flex_props: bool,
) {
    let style: &<T as LayoutTreeNode>::Style = node.style();
    let node_size = node_result.size.0;
    let node_inner_size = Normalized(Size::new(
        node_size.width - border.horizontal(),
        node_size.height - border.vertical(),
    ));
    let node_inner_option_size = Normalized(OptionSize::new(
        OptionNum::some(node_size.width - border.horizontal()),
        OptionNum::some(node_size.height - border.vertical()),
    ));

    // TODO relative to non-static
    let mut f = move |child_node: &T| {
        let child_style = child_node.style();
        if child_style.display() == Display::None {
            let mut child = child_node.layout_node().unit();
            child.clear_display_none_result(child_node);
            return;
        }
        compute_special_position(
            env,
            child_node,
            ParentInfo {
                compute_result: node_result,
                style,
                inner_size: &node_inner_size,
                inner_option_size: &node_inner_option_size,
                border: &border,
                padding_border: &padding_border,
            },
            axis_info,
            accept_flex_props,
            child_style.position(),
        );
    };
    node.tree_visitor()
        .for_each_child(|child_node, _| f(child_node));
}

pub(crate) struct ParentInfo<'a, T: LayoutTreeNode> {
    pub(crate) compute_result: &'a ComputeResult<T::Length>,
    pub(crate) style: &'a T::Style,
    pub(crate) inner_size: &'a Normalized<Size<T::Length>>,
    pub(crate) inner_option_size: &'a Normalized<OptionSize<T::Length>>,
    pub(crate) border: &'a Edge<T::Length>,
    pub(crate) padding_border: &'a Edge<T::Length>,
}

#[inline]
pub(crate) fn compute_special_position<T: LayoutTreeNode>(
    env: &mut T::Env,
    node: &T,
    parent: ParentInfo<T>,
    axis_info: AxisInfo,
    accept_flex_props: bool,
    position: Position,
) {
    if position == Position::Static {
        return;
    }
    let mut layout_unit = node.layout_node().unit();
    let style = node.style();

    let container_size = if position == Position::Fixed {
        Size::new(env.screen_width(), env.screen_height())
    } else {
        **parent.inner_size
    };
    let container_option_size = if position == Position::Fixed {
        Normalized(OptionSize::new(
            OptionNum::some(container_size.width),
            OptionNum::some(container_size.height),
        ))
    } else {
        *parent.inner_option_size
    };

    let left = style.left().resolve_num(container_size.width, node);
    let right = style.right().resolve_num(container_size.width, node);
    let top = style.top().resolve_num(container_size.height, node);
    let bottom = style.bottom().resolve_num(container_size.height, node);

    if position == Position::Relative {
        if let Some(left) = left.val() {
            layout_unit.result.origin.x += left;
        } else if let Some(right) = right.val() {
            layout_unit.result.origin.x -= right;
        }
        if let Some(top) = top.val() {
            layout_unit.result.origin.y += top;
        } else if let Some(bottom) = bottom.val() {
            layout_unit.result.origin.y -= bottom;
        }
        return;
    }

    let (margin, border, padding_border) =
        layout_unit.margin_border_padding(node, *container_option_size);
    let css_size =
        layout_unit.css_border_box_size(node, *container_option_size, border, padding_border);
    let width = css_size.width.or(if left.is_some() && right.is_some() {
        OptionNum::some(container_size.width)
            - left
            - right
            - margin.left.or_zero()
            - margin.right.or_zero()
    } else {
        OptionNum::none()
    });
    let height = css_size.height.or(if top.is_some() && bottom.is_some() {
        OptionNum::some(container_size.height)
            - top
            - bottom
            - margin.top.or_zero()
            - margin.bottom.or_zero()
    } else {
        OptionNum::none()
    });
    let min_max_limit =
        layout_unit.normalized_min_max_limit(node, *container_option_size, border, padding_border);
    let size = min_max_limit.normalized_size(OptionSize::new(width, height));
    let max_content = min_max_limit.normalized_size(OptionSize::new(
        OptionNum::some(size.width.unwrap_or(
            container_size.width
                - left.or_zero()
                - right.or_zero()
                - margin.left.or_zero()
                - margin.right.or_zero(),
        )),
        OptionNum::some(size.height.unwrap_or(
            container_size.height
                - top.or_zero()
                - bottom.or_zero()
                - margin.top.or_zero()
                - margin.bottom.or_zero(),
        )),
    ));

    let result = if size.width.is_none() || size.height.is_none() {
        let auto_size = layout_unit
            .compute_internal(
                env,
                node,
                ComputeRequest {
                    size,
                    parent_inner_size: container_option_size,
                    max_content,
                    kind: ComputeRequestKind::AllSize,
                    parent_is_block: false,
                },
            )
            .size;
        layout_unit.compute_internal(
            env,
            node,
            ComputeRequest {
                size: Normalized(OptionSize::new(
                    OptionNum::some(auto_size.width),
                    OptionNum::some(auto_size.height),
                )),
                parent_inner_size: container_option_size,
                max_content,
                kind: ComputeRequestKind::Position,
                parent_is_block: false,
            },
        )
    } else {
        layout_unit.compute_internal(
            env,
            node,
            ComputeRequest {
                size,
                parent_inner_size: container_option_size,
                max_content,
                kind: ComputeRequestKind::Position,
                parent_is_block: false,
            },
        )
    };
    let free_space = container_size - result.size.0;
    let parent_free_space = **parent.inner_size - result.size.0;
    let (main_start, main_end, cross_start, cross_end) =
        match (axis_info.dir, axis_info.main_dir_rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => (left, right, top, bottom),
            (AxisDirection::Horizontal, AxisReverse::Reversed) => (right, left, top, bottom),
            (AxisDirection::Vertical, AxisReverse::NotReversed) => (top, bottom, left, right),
            (AxisDirection::Vertical, AxisReverse::Reversed) => (top, bottom, right, left),
        };

    let offset_main = if main_start.val().is_some()
        && main_end.val().is_some()
        && margin
            .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
            .val()
            .is_none()
        && margin
            .main_axis_end(axis_info.dir, axis_info.main_dir_rev)
            .val()
            .is_none()
    {
        let main_start = main_start.val().unwrap_or(T::Length::zero());
        let main_end = main_end.val().unwrap_or(T::Length::zero());
        let free_space_main_size = container_size.main_size(axis_info.dir);

        if position == Position::Fixed {
            main_start + (free_space_main_size - main_end - main_start).div_i32(2)
                - result.size.0.main_size(axis_info.dir).div_i32(2)
        } else {
            main_start + (free_space_main_size - main_end - main_start).div_i32(2)
                - parent
                    .border
                    .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                - result.size.0.main_size(axis_info.dir).div_i32(2)
        }
    } else {
        let offset_main = if let Some(x) = main_start.val() {
            if position == Position::Fixed {
                x
            } else {
                parent
                    .border
                    .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                    + x
            }
        } else if let Some(x) = main_end.val() {
            if position == Position::Fixed {
                free_space.main_size(axis_info.dir) - x
            } else {
                parent
                    .border
                    .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                    + parent_free_space.main_size(axis_info.dir)
                    - x
            }
        } else if accept_flex_props {
            match parent.style.justify_content() {
                JustifyContent::SpaceBetween
                | JustifyContent::FlexStart
                | JustifyContent::Start
                | JustifyContent::Stretch
                | JustifyContent::Baseline => parent
                    .padding_border
                    .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
                JustifyContent::FlexEnd | JustifyContent::End => {
                    parent
                        .border
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + parent_free_space.main_size(axis_info.dir)
                        - (parent
                            .padding_border
                            .main_axis_end(axis_info.dir, axis_info.main_dir_rev)
                            - parent
                                .border
                                .main_axis_end(axis_info.dir, axis_info.main_dir_rev))
                }
                JustifyContent::Left => match axis_info.main_dir_rev {
                    AxisReverse::NotReversed => parent
                        .padding_border
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
                    AxisReverse::Reversed => {
                        parent
                            .border
                            .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                            + parent_free_space.main_size(axis_info.dir)
                            - (parent
                                .padding_border
                                .main_axis_end(axis_info.dir, axis_info.main_dir_rev)
                                - parent
                                    .border
                                    .main_axis_end(axis_info.dir, axis_info.main_dir_rev))
                    }
                },
                JustifyContent::Right => match axis_info.main_dir_rev {
                    AxisReverse::NotReversed => {
                        parent
                            .border
                            .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                            + parent_free_space.main_size(axis_info.dir)
                            - (parent
                                .padding_border
                                .main_axis_end(axis_info.dir, axis_info.main_dir_rev)
                                - parent
                                    .border
                                    .main_axis_end(axis_info.dir, axis_info.main_dir_rev))
                    }
                    AxisReverse::Reversed => parent
                        .padding_border
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
                },
                JustifyContent::SpaceEvenly
                | JustifyContent::SpaceAround
                | JustifyContent::Center => {
                    let parent_padding = *parent.padding_border - *parent.border;
                    let free_content_main_size = parent_free_space.main_size(axis_info.dir)
                        - parent_padding.main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        - parent_padding.main_axis_end(axis_info.dir, axis_info.main_dir_rev);
                    parent
                        .border
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + parent_padding.main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + free_content_main_size.div_i32(2)
                }
            }
        } else {
            parent
                .padding_border
                .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
        };
        let offset_main = if let Some(x) = margin
            .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
            .val()
        {
            offset_main + x
        } else if let Some(x) = margin
            .main_axis_end(axis_info.dir, axis_info.main_dir_rev)
            .val()
        {
            offset_main - x
        } else {
            offset_main
        };
        offset_main
    };

    let offset_cross = if cross_end.val().is_some()
        && cross_end.val().is_some()
        && margin
            .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
            .val()
            .is_none()
        && margin
            .cross_axis_end(axis_info.dir, axis_info.main_dir_rev)
            .val()
            .is_none()
    {
        let cross_start = cross_start.val().unwrap_or(T::Length::zero());
        let cross_end = cross_end.val().unwrap_or(T::Length::zero());
        let free_space_cross_size = container_size.cross_size(axis_info.dir);
        cross_start + (free_space_cross_size - cross_end - cross_start).div_i32(2)
            - parent
                .border
                .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
            - (result.size.0.cross_size(axis_info.dir).div_i32(2))
    } else {
        let offset_cross = if let Some(x) = cross_start.val() {
            if position == Position::Fixed {
                x
            } else {
                x + parent
                    .border
                    .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
            }
        } else if let Some(x) = cross_end.val() {
            if position == Position::Fixed {
                free_space.cross_size(axis_info.dir) - x
            } else {
                parent
                    .border
                    .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
                    + parent_free_space.cross_size(axis_info.dir)
                    - x
            }
        } else if accept_flex_props {
            match algo::flex_box::align_self::<T>(style, parent.style) {
                AlignSelf::FlexStart
                | AlignSelf::Start
                | AlignSelf::SelfStart
                | AlignSelf::Normal => parent
                    .padding_border
                    .cross_axis_start(axis_info.dir, axis_info.main_dir_rev),
                AlignSelf::FlexEnd | AlignSelf::End | AlignSelf::SelfEnd => {
                    parent
                        .border
                        .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + parent_free_space.cross_size(axis_info.dir)
                        - (parent
                            .padding_border
                            .cross_axis_end(axis_info.dir, axis_info.main_dir_rev)
                            - parent
                                .border
                                .cross_axis_end(axis_info.dir, axis_info.main_dir_rev))
                }
                AlignSelf::Center => {
                    let parent_padding = *parent.padding_border - *parent.border;
                    let free_content_cross_size = parent_free_space.cross_size(axis_info.dir)
                        - parent_padding.cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        - parent_padding.cross_axis_end(axis_info.dir, axis_info.main_dir_rev);
                    parent
                        .border
                        .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + parent_padding.cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        + free_content_cross_size.div_i32(2)
                }
                AlignSelf::Baseline => {
                    parent
                        .compute_result
                        .first_baseline_ascent
                        .cross_axis(axis_info.dir)
                        - result.first_baseline_ascent.cross_axis(axis_info.dir)
                }
                AlignSelf::Auto | AlignSelf::Stretch => parent
                    .padding_border
                    .cross_axis_start(axis_info.dir, axis_info.main_dir_rev),
            }
        } else {
            parent
                .padding_border
                .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
        };
        let offset_cross = if let Some(x) = margin
            .cross_axis_start(axis_info.dir, axis_info.main_dir_rev)
            .val()
        {
            offset_cross + x
        } else if let Some(x) = margin
            .cross_axis_end(axis_info.dir, axis_info.main_dir_rev)
            .val()
        {
            offset_cross - x
        } else {
            offset_cross
        };
        offset_cross
    };

    layout_unit.gen_origin(axis_info, **parent.inner_size, offset_main, offset_cross);
}

pub(crate) fn compute_position_relative<T: LayoutTreeNode>(
    node: &T,
    parent_inner_size: Normalized<Size<T::Length>>,
) {
    let mut layout_unit = node.layout_node().unit();
    let style = node.style();
    // apply relative offset
    let left = style.left().resolve_num(parent_inner_size.width, node);
    let right = style.right().resolve_num(parent_inner_size.width, node);
    let top = style.top().resolve_num(parent_inner_size.height, node);
    let bottom = style.bottom().resolve_num(parent_inner_size.height, node);
    if let Some(left) = left.val() {
        layout_unit.result.origin.x += left;
    } else if let Some(right) = right.val() {
        layout_unit.result.origin.x -= right;
    }
    if let Some(top) = top.val() {
        layout_unit.result.origin.y += top;
    } else if let Some(bottom) = bottom.val() {
        layout_unit.result.origin.y -= bottom;
    }
}
