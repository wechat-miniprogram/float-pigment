use euclid::Rect;
use float_pigment_css::num_traits::Zero;

mod grid_item;
mod placement;
mod track_size;

mod matrix;

use crate::{
    AxisInfo, CollapsedBlockMargin, ComputeRequest, ComputeRequestKind, ComputeResult, DefLength, Edge, EdgeOption, LayoutAlgorithm, LayoutGridTemplate, LayoutStyle, LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode, LayoutUnit, MinMax, Normalized, OptionNum, OptionSize, Point, Size, Vector, algo::grid::{
        grid_item::GridLayoutItem,
        matrix::{GridLayoutMatrix, GridMatrix, MatrixCell, estimate_track_count},
        placement::place_grid_items,
        track_size::apply_track_size,
    }, compute_special_position_children
};

#[derive(Clone, PartialEq)]
pub(crate) enum GridFlow {
    Row,
    Column,
}

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

        let (row_track_list, row_track_auto_count) =
            initialize_track_list::<T>(&grid_template_rows);
        let (column_track_list, column_track_auto_count) =
            initialize_track_list::<T>(&grid_template_columns);

        let (estimated_row_count, estimated_column_count) = estimate_track_count(
            node,
            style,
            row_track_list.as_slice(),
            column_track_list.as_slice(),
        );

        // 3. Grid item placement

        let mut grid_matrix = GridMatrix::new(
            estimated_row_count,
            estimated_column_count,
            row_track_auto_count,
            column_track_auto_count,
            style.grid_auto_flow(),
        );

        place_grid_items(
            &mut grid_matrix,
            node,
            style,
            row_track_list.as_slice(),
            column_track_list.as_slice(),
        );

        // 4. Apply track size to grid item
        apply_track_size(
            column_track_list.as_slice(),
            GridFlow::Column,
            &mut grid_matrix,
            node,
            requested_inner_size.width,
            &mut available_grid_space.width,
        );

        apply_track_size(
            row_track_list.as_slice(),
            GridFlow::Row,
            &mut grid_matrix,
            node,
            requested_inner_size.height,
            &mut available_grid_space.height,
        );

        // 5. Compute track size for each grid item

        let mut grid_layout_matrix =
            GridLayoutMatrix::new(grid_matrix.row_count(), grid_matrix.column_count());

        for row in 0..grid_matrix.row_count() {
            for column in 0..grid_matrix.column_count() {
                let grid_item = grid_matrix.get_item_mut(row, column);
                if let Some(grid_item) = grid_item {
                    if !grid_item.is_unoccupied() {
                        let grid_item = grid_item.get_auto_placed_unchecked();
                        let mut child = grid_item.node.layout_node().unit();
                        let child_node = grid_item.node;

                        let fixed_track_inline_size =
                            grid_item.fixed_track_inline_size().unwrap().clone();
                        let fixed_track_block_size =
                            grid_item.fixed_track_block_size().unwrap().clone();

                        let track_size = Size::new(fixed_track_inline_size, fixed_track_block_size);

                        let (child_margin, child_border, child_padding_border) =
                            child.margin_border_padding(child_node, track_size);
                        let css_size = child.css_border_box_size(
                            child_node,
                            track_size,
                            child_border,
                            child_padding_border,
                        );
                        let mut size = child
                            .normalized_min_max_limit(
                                child_node,
                                track_size,
                                border,
                                padding_border,
                            )
                            .normalized_size(css_size);
                        if size.0.width.is_none() && track_size.width.is_some() {
                            size.0.width = track_size.width;
                        }
                        if size.0.height.is_none() && track_size.height.is_some() {
                            size.0.height = track_size.height;
                        }

                        child.compute_internal(
                            env,
                            grid_item.node,
                            ComputeRequest {
                                size,
                                parent_inner_size: Normalized(track_size),
                                max_content: Normalized(track_size),
                                kind: request.kind,
                                parent_is_block: false,
                            },
                        );

                        let grid_layout_item =
                            GridLayoutItem::new(child_node, child_margin, css_size, track_size);
                        grid_layout_matrix.update_item(
                            row,
                            column,
                            MatrixCell::AutoPlaced(grid_layout_item),
                        );
                    }
                }
            }
        }

        drop(grid_matrix);

        // adjust each size
        let each_inline_size = adjust_each_inline_size(&mut grid_layout_matrix);
        let total_inline_size: T::Length = each_inline_size
            .into_iter()
            .fold(T::Length::zero(), |acc, cur| acc + cur);

        let each_block_size = adjust_each_block_size(&mut grid_layout_matrix);
        let total_block_size: T::Length = each_block_size
            .into_iter()
            .fold(T::Length::zero(), |acc, cur| acc + cur);

        let mut block_offset = T::Length::zero();
        for row_index in 0..grid_layout_matrix.row_count() {
            let mut current_block_size = T::Length::zero();
            let mut inline_offset = T::Length::zero();
            for column_index in 0..grid_layout_matrix.column_count() {
                if let Some(grid_matrix_item) =
                    grid_layout_matrix.inner.get_mut(row_index, column_index)
                {
                    if grid_matrix_item.is_unoccupied() {
                        continue;
                    }
                    let grid_matrix_item = grid_matrix_item.get_auto_placed_unchecked();
                    let mut layout_node = grid_matrix_item.node.layout_node().unit();

                    layout_node.gen_origin(
                        axis_info,
                        Size::new(
                            grid_matrix_item.track_size.width.val().unwrap(),
                            grid_matrix_item.track_size.height.val().unwrap(),
                        ),
                        block_offset
                            + grid_matrix_item
                                .margin
                                .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                                .or_zero(),
                        inline_offset
                            + grid_matrix_item
                                .margin
                                .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                                .or_zero(),
                    );
                    layout_node.save_all_results(
                        grid_matrix_item.node,
                        env,
                        Size::new(
                            grid_matrix_item.track_size.width,
                            grid_matrix_item.track_size.height,
                        ),
                        LayoutAlgorithm::Block, // FIXME here should be a compute_internal
                    );
                    drop(layout_node);

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
}

fn grid_template_track_iterator<T: LayoutTreeNode>(
    grid_template: &LayoutGridTemplate<T::Length, T::LengthCustom>,
    mut filter: impl FnMut(&LayoutTrackListItem<T::Length, T::LengthCustom>) -> bool,
) -> Option<impl Iterator<Item = &LayoutTrackListItem<T::Length, T::LengthCustom>>> {
    match grid_template {
        LayoutGridTemplate::TrackList(track_list) => {
            Some(track_list.iter().filter(move |item| filter(item)))
        }
        _ => None,
    }
}

fn initialize_track_list<T: LayoutTreeNode>(
    grid_template_rows: &LayoutGridTemplate<T::Length, T::LengthCustom>,
) -> (Vec<&LayoutTrackListItem<T::Length, T::LengthCustom>>, usize) {
    let mut track_auto_count = 0;
    let track_list = grid_template_track_iterator::<T>(grid_template_rows, |item| {
        if matches!(
            item,
            LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(DefLength::Auto))
        ) {
            track_auto_count += 1;
        }
        matches!(item, LayoutTrackListItem::TrackSize(_))
    })
    .map(|it| it.collect::<Vec<_>>())
    .unwrap_or(Vec::with_capacity(0));
    (track_list, track_auto_count)
}

pub(crate) fn adjust_each_inline_size<T: LayoutTreeNode>(
    grid_layout_matrix: &mut GridLayoutMatrix<T>,
) -> Vec<T::Length> {
    let mut each_inline_size = Vec::with_capacity(grid_layout_matrix.column_count());

    let column_count = grid_layout_matrix.column_count();

    for col_index in 0..column_count {
        let inline_size: <T as LayoutTreeNode>::Length;

        if let Some(item) = grid_layout_matrix
            .inner
            .iter_col(col_index)
            .filter(|item| !item.is_unoccupied())
            .find(|item| {
                item.get_auto_placed_unchecked()
                    .track_inline_size()
                    .is_some()
            })
        {
            inline_size = item
                .get_auto_placed_unchecked()
                .track_inline_size()
                .val()
                .unwrap();
        } else {
            inline_size = grid_layout_matrix
                .inner
                .iter_col(col_index)
                .filter(|item| !item.is_unoccupied())
                .fold(T::Length::zero(), |acc, cur| {
                    acc.max(
                        cur.get_auto_placed_unchecked()
                            .node
                            .layout_node()
                            .unit()
                            .result()
                            .size
                            .width,
                    )
                });
        }

        grid_layout_matrix
            .inner
            .iter_col_mut(col_index)
            .filter(|item| !item.is_unoccupied())
            .for_each(|item| {
                let item = item.get_auto_placed_mut_unchecked();
                item.track_size.width = OptionNum::some(inline_size);
                if item.css_size.width.is_none() {
                    item.node.layout_node().unit().result.size.width = inline_size;
                }
            });

        each_inline_size.push(inline_size);
    }

    each_inline_size
}

pub(crate) fn adjust_each_block_size<T: LayoutTreeNode>(
    grid_layout_matrix: &mut GridLayoutMatrix<T>,
) -> Vec<T::Length> {
    let mut each_block_size = Vec::with_capacity(grid_layout_matrix.row_count());

    let row_count = grid_layout_matrix.row_count();

    for row_index in 0..row_count {
        let block_size;

        if let Some(item) = grid_layout_matrix
            .inner
            .iter_row(row_index)
            .filter(|item| !item.is_unoccupied())
            .find(|item| {
                item.get_auto_placed_unchecked()
                    .track_block_size()
                    .is_some()
            })
        {
            block_size = item
                .get_auto_placed_unchecked()
                .track_block_size()
                .val()
                .unwrap();
        } else {
            block_size = grid_layout_matrix
                .inner
                .iter_row(row_index)
                .filter(|item| !item.is_unoccupied())
                .fold(T::Length::zero(), |acc, cur| {
                    acc.max(
                        cur.get_auto_placed_unchecked()
                            .node
                            .layout_node()
                            .unit()
                            .result()
                            .size
                            .height,
                    )
                });
        }
        grid_layout_matrix
            .inner
            .iter_row_mut(row_index)
            .filter(|item| !item.is_unoccupied())
            .for_each(|item| {
                let item = item.get_auto_placed_mut_unchecked();
                item.track_size.height = OptionNum::some(block_size);
                if item.css_size.height.is_none() {
                    item.node.layout_node().unit().result.size.height = block_size;
                }
            });
        each_block_size.push(block_size)
    }
    each_block_size
}
