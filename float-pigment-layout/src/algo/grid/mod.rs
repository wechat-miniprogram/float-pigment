use std::fmt::Debug;

use euclid::Rect;
use float_pigment_css::{length_num::LengthNum, num_traits::Zero, typing::GridAutoFlow};

mod grid_item;

use grid::*;

use crate::{
    algo::grid::grid_item::GridItem, compute_special_position_children, is_display_none,
    is_independent_positioning, AxisInfo, CollapsedBlockMargin, ComputeRequest, ComputeRequestKind,
    ComputeResult, DefLength, Edge, EdgeOption, LayoutGridTemplate, LayoutStyle,
    LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode, LayoutTreeVisitor, LayoutUnit, MinMax,
    Normalized, OptionNum, OptionSize, Point, Size, Vector,
};

pub(crate) trait GridContainer<T: LayoutTreeNode> {
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length>;

    fn grid_template_track_iterator(
        grid_template: &LayoutGridTemplate<T::Length, T::LengthCustom>,
    ) -> Option<impl Iterator<Item = &LayoutTrackListItem<T::Length, T::LengthCustom>>>;

    // fn compute_track_sizes(
    //     track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    //     node: &T,
    // ) -> Vec<OptionNum<T::Length>>;

    fn place_grid_items<'a>(
        grid_matrix: &mut GridMatrix<'a, T>,
        node: &'a T,
        style: &'a T::Style,
        row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
        column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    );
}

impl<T: LayoutTreeNode> GridContainer<T> for LayoutUnit<T> {
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length> {
        // 1. Compute available-grid-space
        // https://www.w3.org/TR/css-grid-1/#available-grid-space

        let style = node.style();

        // TODO handle direction
        let axis_info = AxisInfo::from_writing_mode(style.writing_mode());
        let collapsed_margin = CollapsedBlockMargin::from_margin(
            margin
                .or_zero()
                .main_axis_start(axis_info.dir, axis_info.main_dir_rev),
            margin
                .or_zero()
                .main_axis_end(axis_info.dir, axis_info.main_dir_rev),
        );

        // Short-circuit if the requested size is determined and the ComputeRequestKind is not Position.
        if let Some(size) = self.is_requested_size_fixed(&request, Some(collapsed_margin)) {
            return size;
        }

        let requested_size = request.size;
        let requested_inner_size = Normalized(OptionSize::new(
            requested_size.width - padding_border.horizontal(),
            requested_size.height - padding_border.vertical(),
        ));

        let mut available_grid_space = OptionSize::new(
            requested_size.width.or(request.max_content.width) - padding_border.horizontal(),
            requested_size.height.or(request.max_content.height) - padding_border.vertical(),
        );

        // 2. Resolve the explicit grid

        let grid_template_rows = style.grid_template_rows();
        let grid_template_columns = style.grid_template_columns();

        let row_track_list = Self::grid_template_track_iterator(&grid_template_rows)
            .map(|it| it.collect::<Vec<_>>())
            .unwrap_or(Vec::with_capacity(0));

        let column_track_list = Self::grid_template_track_iterator(&grid_template_columns)
            .map(|it| it.collect::<Vec<_>>())
            .unwrap_or(Vec::with_capacity(0));

        let row_track_auto_count = row_track_list
            .iter()
            .filter(|item| {
                matches!(
                    item,
                    LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(DefLength::Auto))
                )
            })
            .count();
        let column_track_auto_count = column_track_list
            .iter()
            .filter(|item| {
                matches!(
                    item,
                    LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(DefLength::Auto))
                )
            })
            .count();

        // 3.TODO: Optimize Implicit grid estimation
        // This is necessary as part of placement. Doing it early here is a perf optimisation to reduce allocations.

        // 4. Grid item placement
        let mut grid_matrix = GridMatrix::new(
            node.tree_visitor().children_len(),
            row_track_list.len(),
            column_track_list.len(),
        );

        Self::place_grid_items(
            &mut grid_matrix,
            node,
            style,
            row_track_list.as_slice(),
            column_track_list.as_slice(),
        );

        // 5. Compute track preferred size

        let mut preferred_inline_size = T::Length::zero();
        // handle specified track inline size
        column_track_list
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                matches!(
                    item,
                    LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(_))
                )
            })
            .for_each(|(idx, track_item)| {
                let mut current_inline_size = T::Length::zero();
                for row in 0..grid_matrix.row_count {
                    let grid_item = grid_matrix
                        .items
                        .get_mut(row * grid_matrix.column_count + idx);
                    if let Some(grid_item) = grid_item {
                        if let LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(length)) =
                            track_item
                        {
                            current_inline_size =
                                length.resolve(requested_inner_size.width, node).or_zero();
                            grid_item.update_track_inline_size(length.clone());
                        }
                    }
                }
                preferred_inline_size += current_inline_size;
            });
        if available_grid_space.width.is_none() && preferred_inline_size > T::Length::zero() {
            available_grid_space.width = OptionNum::some(preferred_inline_size);
        }

        // handle auto track inline size
        grid_matrix
            .items
            .iter_mut()
            .for_each(|grid_item| match grid_item.track_inline_size {
                DefLength::Auto => {
                    if available_grid_space.width.is_some()
                        && column_track_auto_count > 0
                        && available_grid_space.width.val().unwrap() > preferred_inline_size
                    {
                        grid_item.update_track_inline_size(DefLength::Points(
                            (available_grid_space.width.val().unwrap() - preferred_inline_size)
                                .div_f32(column_track_auto_count as f32),
                        ));
                    } else {
                        grid_item.update_track_inline_size(DefLength::Undefined);
                    }
                }
                DefLength::Percent(percent) => {
                    if available_grid_space.width.is_some() {
                        grid_item.update_track_inline_size(DefLength::Points(
                            available_grid_space.width.val().unwrap().mul_f32(percent),
                        ));
                    } else {
                        grid_item.update_track_inline_size(DefLength::Undefined);
                    }
                }
                _ => {}
            });

        let mut preferred_block_size = T::Length::zero();
        // handle specified track block size
        row_track_list
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                matches!(
                    item,
                    LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(_))
                )
            })
            .for_each(|(idx, track_item)| {
                let mut current_block_size = T::Length::zero();
                for col in 0..grid_matrix.column_count {
                    preferred_block_size = T::Length::zero();
                    let grid_item = grid_matrix.items.get_mut(idx * grid_matrix.row_count + col);
                    if let Some(grid_item) = grid_item {
                        if let LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(length)) =
                            track_item
                        {
                            current_block_size =
                                length.resolve(requested_inner_size.height, node).or_zero();
                            grid_item.update_track_block_size(length.clone());
                        }
                    }
                }
                preferred_block_size += current_block_size
            });
        if available_grid_space.height.is_none() && preferred_block_size > T::Length::zero() {
            available_grid_space.height = OptionNum::some(preferred_block_size);
        }

        // handle auto track block size
        grid_matrix
            .items
            .iter_mut()
            .for_each(|grid_item| match grid_item.track_block_size {
                DefLength::Auto => {
                    if available_grid_space.height.is_some()
                        && row_track_auto_count > 0
                        && available_grid_space.height.val().unwrap() > preferred_block_size
                    {
                        grid_item.update_track_block_size(DefLength::Points(
                            (available_grid_space.height.val().unwrap() - preferred_block_size)
                                .div_f32(row_track_auto_count as f32),
                        ));
                    } else {
                        grid_item.update_track_block_size(DefLength::Undefined);
                    }
                }
                DefLength::Percent(percent) => {
                    if available_grid_space.height.is_some() {
                        grid_item.update_track_block_size(DefLength::Points(
                            available_grid_space.height.val().unwrap().mul_f32(percent),
                        ));
                    } else {
                        grid_item.update_track_block_size(DefLength::Undefined);
                    }
                }
                _ => {}
            });

        let mut layout_grid_matrix: LayoutGridMatrix<T> =
            LayoutGridMatrix::new(grid_matrix.row_count, grid_matrix.column_count);

        // 6. Compute track size
        for row in 0..grid_matrix.row_count {
            for column in 0..grid_matrix.column_count {
                let grid_item = grid_matrix
                    .items
                    .get(row * grid_matrix.column_count + column);
                if let Some(grid_item) = grid_item {
                    let mut child = grid_item.node.layout_node().unit();
                    let child_node = grid_item.node;
                    let track_size = Normalized(Size::new(
                        grid_item.track_inline_size.resolve(OptionNum::none(), node),
                        grid_item.track_block_size.resolve(OptionNum::none(), node),
                    ));
                    let (child_margin, child_border, child_padding_border) = child
                        .margin_border_padding(
                            child_node,
                            OptionSize::new(OptionNum::none(), OptionNum::none()),
                        );
                    let css_size = child.css_border_box_size(
                        child_node,
                        OptionSize::new(OptionNum::none(), OptionNum::none()),
                        child_border,
                        child_padding_border,
                    );
                    let mut size = child
                        .normalized_min_max_limit(child_node, track_size.0, border, padding_border)
                        .normalized_size(css_size);
                    if size.0.width.is_none() && track_size.0.width.is_some() {
                        size.0.width = track_size.0.width;
                    }
                    if size.0.height.is_none() && track_size.0.height.is_some() {
                        size.0.height = track_size.0.height;
                    }

                    let res = child.compute_internal(
                        env,
                        grid_item.node,
                        ComputeRequest {
                            size,
                            parent_inner_size: track_size,
                            max_content: track_size,
                            kind: request.kind,
                            parent_is_block: false,
                        },
                    );
                    layout_grid_matrix.items[(row, column)] = Some(GridMatrixItem {
                        node: child_node,
                        margin: child_margin,
                        css_size,
                        result: res,
                        track_size: track_size.0,
                    });
                }
            }
        }
        let mut total_inline_size = T::Length::zero();
        let mut total_block_size = T::Length::zero();

        for col_index in 0..layout_grid_matrix.column_count {
            let inline_size: <T as LayoutTreeNode>::Length;
            if let Some(Some(grid_matrix_item)) =
                layout_grid_matrix.items.iter_col(col_index).find(|item| {
                    if let Some(item) = item.as_ref() {
                        return item.track_size.width.is_some();
                    }
                    false
                })
            {
                inline_size = grid_matrix_item.track_size.width.val().unwrap();
            } else {
                inline_size = layout_grid_matrix.items.iter_col(col_index).fold(
                    T::Length::zero(),
                    |acc, cur| {
                        if let Some(cur) = cur.as_ref() {
                            return acc.max(cur.result.size.width);
                        }
                        acc
                    },
                );
            }
            layout_grid_matrix
                .items
                .iter_col_mut(col_index)
                .for_each(|item| {
                    if item.is_some() {
                        let grid_matrix_item = item.as_mut().unwrap();
                        grid_matrix_item.track_size.width = OptionNum::some(inline_size);
                        if grid_matrix_item.css_size.width.is_none() {
                            grid_matrix_item.node.layout_node().unit().result.size.width =
                                inline_size;
                        }
                    }
                });

            total_inline_size += inline_size;
        }

        for row_index in 0..layout_grid_matrix.row_count {
            let block_size;
            if let Some(Some(grid_matrix_item)) =
                layout_grid_matrix.items.iter_row(row_index).find(|item| {
                    if let Some(item) = item.as_ref() {
                        return item.track_size.height.is_some();
                    }
                    false
                })
            {
                block_size = grid_matrix_item.track_size.height.val().unwrap();
            } else {
                block_size = layout_grid_matrix.items.iter_row(row_index).fold(
                    T::Length::zero(),
                    |acc, cur| {
                        if let Some(cur) = cur.as_ref() {
                            return acc.max(cur.result.size.height);
                        }
                        acc
                    },
                );
            }
            layout_grid_matrix
                .items
                .iter_row_mut(row_index)
                .for_each(|item| {
                    if item.is_some() {
                        let grid_matrix_item = item.as_mut().unwrap();
                        grid_matrix_item.track_size.height = OptionNum::some(block_size);

                        if grid_matrix_item.css_size.height.is_none() {
                            grid_matrix_item
                                .node
                                .layout_node()
                                .unit()
                                .result
                                .size
                                .height = block_size;
                        }
                    }
                });
            total_block_size += block_size;
        }

        let mut block_offset = T::Length::zero();
        for row_index in 0..layout_grid_matrix.row_count {
            let mut current_block_size = T::Length::zero();
            let mut inline_offset = T::Length::zero();
            for column_index in 0..layout_grid_matrix.column_count {
                if let Some(grid_matrix_item) =
                    layout_grid_matrix.items[(row_index, column_index)].as_mut()
                {
                    let mut layout_node = grid_matrix_item.node.layout_node().unit();
                    layout_node.gen_origin(
                        axis_info,
                        Size::new(
                            grid_matrix_item.track_size.width.val().unwrap(),
                            grid_matrix_item.track_size.height.val().unwrap(),
                        ),
                        inline_offset,
                        block_offset,
                    );
                    layout_node.result.origin = Point::new(
                        inline_offset
                            + grid_matrix_item
                                .margin
                                .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                                .or_zero(),
                        block_offset
                            + grid_matrix_item
                                .margin
                                .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                                .or_zero(),
                    );
                    inline_offset += grid_matrix_item.track_size.width.val().unwrap();
                    current_block_size = grid_matrix_item.track_size.height.val().unwrap();
                }
            }
            block_offset += current_block_size;
        }

        let size = Size::new(
            requested_size
                .width
                .unwrap_or(total_inline_size + padding_border.horizontal()),
            requested_size
                .height
                .unwrap_or(total_block_size + padding_border.vertical()),
        );
        let ret = ComputeResult {
            size: Normalized(size),
            first_baseline_ascent: Vector::zero(),
            last_baseline_ascent: Vector::zero(),
            collapsed_margin: CollapsedBlockMargin::zero(),
        };
        if request.kind != ComputeRequestKind::Position {
            self.cache.write_all_size(node, &request, ret);
        } else {
            compute_special_position_children(
                env,
                node,
                &ret,
                border,
                padding_border,
                AxisInfo {
                    dir: axis_info.dir,
                    main_dir_rev: axis_info.main_dir_rev,
                    cross_dir_rev: axis_info.cross_dir_rev,
                },
                true,
            );
            self.result = Rect::new(Point::zero(), ret.size.0);
            self.cache.write_position(node, &request, ret);
        }
        ret
    }

    fn grid_template_track_iterator(
        grid_template: &LayoutGridTemplate<T::Length, T::LengthCustom>,
    ) -> Option<impl Iterator<Item = &LayoutTrackListItem<T::Length, T::LengthCustom>>> {
        match grid_template {
            LayoutGridTemplate::TrackList(track_list) => Some(
                track_list
                    .iter()
                    .filter(|&item| matches!(item, LayoutTrackListItem::TrackSize(_))),
            ),
            _ => None,
        }
    }

    fn place_grid_items<'a>(
        grid_matrix: &mut GridMatrix<'a, T>,
        node: &'a T,
        style: &'a T::Style,
        row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
        column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    ) {
        let mut row_num = row_track_list.len().max(1);
        let mut column_num = column_track_list.len().max(1);

        let mut cur_row = 0;
        let mut cur_column = 0;

        let children_iter = node
            .tree_visitor()
            .children_iter()
            .enumerate()
            .filter(|(_, node)| {
                !is_independent_positioning(*node) && !is_display_none::<T>(node.style())
            });

        // TODO: Implement grid auto flow dense
        children_iter.for_each(|(origin_idx, child)| match style.grid_auto_flow() {
            GridAutoFlow::Row | GridAutoFlow::RowDense => {
                if cur_column >= column_num {
                    cur_column = 0;
                    cur_row += 1;
                    if cur_row >= row_num {
                        row_num = cur_row + 1;
                    }
                }
                let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
                grid_matrix.items.push(grid_item);
                cur_column += 1;
            }
            GridAutoFlow::Column | GridAutoFlow::ColumnDense => {
                if cur_row >= row_num {
                    cur_row = 0;
                    cur_column += 1;
                    if cur_column >= column_num {
                        column_num = cur_column + 1;
                    }
                }
                let grid_item = GridItem::new(child, origin_idx, cur_row, cur_column);
                grid_matrix.items.push(grid_item);
                cur_row += 1;
            }
        });
        grid_matrix.update_row_count(row_num.max(row_track_list.len().max(1)));
        grid_matrix.update_column_count(column_num.max(column_track_list.len().max(1)));
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct GridMatrix<'a, T: LayoutTreeNode> {
    items: Vec<GridItem<'a, T>>,
    row_count: usize,
    column_count: usize,
}

impl<'a, T: LayoutTreeNode> GridMatrix<'a, T> {
    fn new(items_count: usize, row_count: usize, column_count: usize) -> Self {
        Self {
            items: Vec::with_capacity(items_count),
            row_count,
            column_count,
        }
    }

    fn update_row_count(&mut self, row_count: usize) {
        self.row_count = row_count;
    }

    fn update_column_count(&mut self, column_count: usize) {
        self.column_count = column_count;
    }
}

impl<'a, T: LayoutTreeNode> Debug for GridMatrix<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = write!(
            f,
            "GridMatrix {{ grid_items: {:?} row_count: {}, column_count: {} }}",
            self.items, self.row_count, self.column_count
        );
        r
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct LayoutGridMatrix<'a, T: LayoutTreeNode> {
    items: Grid<Option<GridMatrixItem<'a, T>>>,
    row_count: usize,
    column_count: usize,
}

impl<'a, T: LayoutTreeNode> LayoutGridMatrix<'a, T> {
    fn new(row_count: usize, column_count: usize) -> Self {
        Self {
            items: Grid::new(row_count, column_count),
            row_count,
            column_count,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]

pub(crate) struct GridMatrixItem<'a, T: LayoutTreeNode> {
    node: &'a T,
    margin: EdgeOption<T::Length>,
    css_size: Size<OptionNum<T::Length>>,
    result: ComputeResult<T::Length>,
    track_size: Size<OptionNum<T::Length>>,
}
