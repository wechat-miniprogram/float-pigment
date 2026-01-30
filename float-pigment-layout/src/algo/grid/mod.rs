//! CSS Grid Layout Algorithm Implementation
//!
//! This module implements the CSS Grid Layout Module Level 1 specification.
//! Reference: <https://www.w3.org/TR/css-grid-1/>
//!
//! ## W3C Grid Sizing Algorithm (§11.1)
//! <https://www.w3.org/TR/css-grid-1/#algo-grid-sizing>
//!
//! The W3C specification defines Grid Sizing Algorithm with these steps:
//!
//! 1. **First**: Use track sizing algorithm to resolve grid **column** sizes
//! 2. **Next**: Use track sizing algorithm to resolve grid **row** sizes
//! 3. **Then**: If min-content contribution changed based on row sizes,
//!    **re-resolve columns** (⚠️ NOT IMPLEMENTED)
//! 4. **Next**: If min-content contribution changed based on column sizes,
//!    **re-resolve rows** (⚠️ NOT IMPLEMENTED)
//! 5. **Finally**: Align tracks via align-content/justify-content
//!
//! ## Current Implementation Steps
//!
//! This implementation follows a simplified approach:
//!
//! 1. Resolve explicit grid (§7.1)
//! 2. Calculate gutters/gap (§10.1)
//! 3. Place grid items (§8.5)
//! 4. Size columns, then rows (§11.3) - single pass, no re-resolution
//! 5. Compute item sizes and finalize tracks
//! 6. Apply content alignment (§10.5)
//! 7. Position items with self-alignment (§10.3-10.4)
//!
//! ## Known Limitations
//!
//! - **No iterative re-resolution**: Steps 3-4 of W3C algorithm not implemented.
//!   This may cause incorrect sizing when item sizes depend on both axes.
//!
//! ## Related Specifications
//! - CSS Grid Layout Module Level 1: <https://www.w3.org/TR/css-grid-1/>
//! - CSS Box Alignment Module Level 3: <https://www.w3.org/TR/css-align-3/>

use euclid::Rect;
use float_pigment_css::length_num::LengthNum;
use float_pigment_css::num_traits::Zero;

mod alignment;
mod grid_item;
mod matrix;
mod placement;
mod track_size;

use crate::{
    algo::grid::{
        alignment::{
            calculate_align_content_offset, calculate_alignment_offset,
            calculate_justify_content_offset, calculate_justify_offset, resolve_grid_align_self,
            resolve_grid_justify_self,
        },
        grid_item::GridLayoutItem,
        matrix::{estimate_track_count, GridLayoutMatrix, GridMatrix, MatrixCell},
        placement::place_grid_items,
        track_size::apply_track_size,
    },
    compute_special_position_children, AxisInfo, CollapsedBlockMargin, ComputeRequest,
    ComputeRequestKind, ComputeResult, DefLength, Edge, EdgeOption, LayoutGridTemplate,
    LayoutStyle, LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode, LayoutUnit, MinMax,
    Normalized, OptionNum, OptionSize, Point, Size, SizingMode, Vector,
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
    /// Main Grid layout computation function.
    ///
    /// Implements the Grid Sizing Algorithm from CSS Grid Layout Module Level 1.
    /// Reference: <https://www.w3.org/TR/css-grid-1/#algo-grid-sizing>
    ///
    /// ## W3C §11.1 Grid Sizing Algorithm Steps
    ///
    /// The specification defines these steps:
    /// 1. Use track sizing algorithm for columns
    /// 2. Use track sizing algorithm for rows
    /// 3. Re-resolve columns if min-content contribution changed (⚠️ NOT IMPLEMENTED)
    /// 4. Re-resolve rows if min-content contribution changed (⚠️ NOT IMPLEMENTED)
    /// 5. Align tracks via align-content/justify-content
    ///
    /// ## Implementation Steps
    ///
    /// 1. **Available Space**: Calculate container content box
    /// 2. **Gutters** (§10.1): Calculate row-gap and column-gap
    /// 3. **Explicit Grid** (§7.2): Parse grid-template-rows/columns
    /// 4. **Placement** (§8.5): Place items using auto-placement
    /// 5. **Track Sizing** (§11.3): Size columns, then rows (single pass)
    /// 6. **Item Sizing**: Compute each item's size
    /// 7. **Finalize Tracks**: Adjust for auto tracks
    /// 8. **Content Alignment** (§10.5): Apply align/justify-content
    /// 9. **Item Positioning** (§10.3-10.4): Apply align/justify-self
    fn compute(
        &mut self,
        env: &mut T::Env,
        node: &T,
        request: ComputeRequest<T::Length>,
        margin: EdgeOption<T::Length>,
        border: Edge<T::Length>,
        padding_border: Edge<T::Length>,
    ) -> ComputeResult<T::Length> {
        // ═══════════════════════════════════════════════════════════════════════
        // STEP 1: Compute Available Grid Space
        // CSS Grid §11.1: https://www.w3.org/TR/css-grid-1/#algo-grid-sizing
        //
        // The available grid space is the space in which the grid tracks are
        // sized, determined by the grid container's content box.
        // ═══════════════════════════════════════════════════════════════════════

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

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 2: Resolve Gutters (Gap)
        // CSS Grid §10.1: https://www.w3.org/TR/css-grid-1/#gutters
        //
        // The row-gap and column-gap properties define the size of the gutters
        // between grid rows and columns respectively.
        // ═══════════════════════════════════════════════════════════════════════
        let column_gap = style
            .column_gap()
            .resolve(available_grid_space.width, node)
            .or_zero();
        let row_gap = style
            .row_gap()
            .resolve(available_grid_space.height, node)
            .or_zero();

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 3: Resolve the Explicit Grid
        // CSS Grid §7.2: https://www.w3.org/TR/css-grid-1/#explicit-grids
        //
        // The grid-template-columns and grid-template-rows properties define
        // the line names and track sizing functions of the explicit grid.
        // ═══════════════════════════════════════════════════════════════════════
        let grid_template_rows = style.grid_template_rows();
        let grid_template_columns = style.grid_template_columns();

        let InitializedTrackListInfo {
            list: row_track_list,
            auto_count: row_track_auto_count,
            total_fr: row_total_fr,
        } = initialize_track_list::<T>(&grid_template_rows);
        let InitializedTrackListInfo {
            list: column_track_list,
            auto_count: column_track_auto_count,
            total_fr: column_total_fr,
        } = initialize_track_list::<T>(&grid_template_columns);

        let (estimated_row_count, estimated_column_count) = estimate_track_count(
            node,
            style,
            row_track_list.as_slice(),
            column_track_list.as_slice(),
        );

        // Calculate total gap space: (n-1) gaps for n tracks
        // CSS Grid §10.1: Gutters are only placed between tracks, not at edges
        let total_column_gaps = if estimated_column_count > 1 {
            column_gap.mul_i32(estimated_column_count as i32 - 1)
        } else {
            T::Length::zero()
        };
        let total_row_gaps = if estimated_row_count > 1 {
            row_gap.mul_i32(estimated_row_count as i32 - 1)
        } else {
            T::Length::zero()
        };

        // Subtract gaps from available space before track sizing
        available_grid_space.width = available_grid_space.width - total_column_gaps;
        available_grid_space.height = available_grid_space.height - total_row_gaps;

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 4: Grid Item Placement
        // CSS Grid §8.5: https://www.w3.org/TR/css-grid-1/#auto-placement-algo
        //
        // Grid items are placed according to the auto-placement algorithm,
        // which fills the grid in row-major or column-major order based on
        // the grid-auto-flow property.
        // ═══════════════════════════════════════════════════════════════════════

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

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 5: Track Sizing Algorithm
        // CSS Grid §11.3: https://www.w3.org/TR/css-grid-1/#algo-track-sizing
        //
        // This step sizes the grid tracks. The algorithm:
        // 1. Initialize each track's base size and growth limit (§11.4)
        // 2. Resolve intrinsic track sizes (§11.5)
        // 3. Maximize tracks (§11.6)
        // 4. Expand flexible tracks (§11.7)
        //
        // fr unit (§7.2.4): https://www.w3.org/TR/css-grid-1/#fr-unit
        // Flexible lengths share remaining space proportionally.
        // ═══════════════════════════════════════════════════════════════════════
        apply_track_size(
            column_track_list.as_slice(),
            GridFlow::Column,
            &mut grid_matrix,
            node,
            requested_inner_size.width,
            &mut available_grid_space.width,
            column_total_fr,
        );

        apply_track_size(
            row_track_list.as_slice(),
            GridFlow::Row,
            &mut grid_matrix,
            node,
            requested_inner_size.height,
            &mut available_grid_space.height,
            row_total_fr,
        );

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 6: Compute Item Sizes
        // CSS Grid §11.5: https://www.w3.org/TR/css-grid-1/#algo-content
        //
        // Compute the min-content and max-content sizes of each grid item.
        // These values are used for intrinsic track sizing.
        //
        // The min-content/max-content contribution of a grid item is its
        // outer size (including margins).
        // Reference: https://www.w3.org/TR/css-grid-1/#min-size-contribution
        // ═══════════════════════════════════════════════════════════════════════

        let mut grid_layout_matrix =
            GridLayoutMatrix::new(grid_matrix.row_count(), grid_matrix.column_count());

        let mut each_min_content_size: Vec<Option<_>> =
            Vec::with_capacity(grid_matrix.column_auto_count());
        each_min_content_size.fill(None);
        for row in 0..grid_matrix.row_count() {
            for column in 0..grid_matrix.column_count() {
                let grid_item = grid_matrix.get_item_mut(row, column);
                if let Some(grid_item) = grid_item {
                    if !grid_item.is_unoccupied() {
                        let grid_item = grid_item.get_auto_placed_unchecked();
                        let child_node = grid_item.node;
                        let mut child_layout_node = grid_item.node.layout_node().unit();

                        let fixed_track_inline_size =
                            grid_item.fixed_track_inline_size().unwrap().clone();
                        let fixed_track_block_size =
                            grid_item.fixed_track_block_size().unwrap().clone();

                        let track_size = Size::new(fixed_track_inline_size, fixed_track_block_size);

                        let (child_margin, child_border, child_padding_border) =
                            child_layout_node.margin_border_padding(child_node, track_size);
                        let css_size = child_layout_node.css_border_box_size(
                            child_node,
                            track_size,
                            child_border,
                            child_padding_border,
                        );
                        let min_max_limit_css_size = child_layout_node
                            .normalized_min_max_limit(
                                child_node,
                                track_size,
                                border,
                                padding_border,
                            )
                            .normalized_size(css_size);

                        let size = Normalized(Size::new(
                            min_max_limit_css_size.0.width.or(track_size.width),
                            min_max_limit_css_size.0.height.or(track_size.height),
                        ));
                        let min_content_res = child_layout_node.compute_internal(
                            env,
                            grid_item.node,
                            ComputeRequest {
                                size: Normalized(track_size),
                                parent_inner_size: Normalized(track_size),
                                max_content: Normalized(track_size),
                                kind: ComputeRequestKind::AllSize,
                                parent_is_block: false,
                                sizing_mode: SizingMode::MinContent,
                            },
                        );

                        let res = child_layout_node.compute_internal(
                            env,
                            grid_item.node,
                            ComputeRequest {
                                size,
                                parent_inner_size: Normalized(track_size),
                                max_content: Normalized(track_size),
                                kind: ComputeRequestKind::AllSize,
                                parent_is_block: false,
                                sizing_mode: request.sizing_mode,
                            },
                        );

                        let mut grid_layout_item =
                            GridLayoutItem::new(child_node, child_margin, css_size, track_size);
                        grid_layout_item.set_min_content_size(min_content_res.min_content_size.0);
                        grid_layout_item.set_computed_size(res.size.0);
                        if let Some(min_content_size) = each_min_content_size.get_mut(column) {
                            if min_content_size.is_none() {
                                min_content_size.replace(min_content_res.min_content_size.0);
                            } else {
                                min_content_size.replace(
                                    min_content_size
                                        .as_ref()
                                        .unwrap()
                                        .max(min_content_res.min_content_size.0),
                                );
                            }
                        }
                        grid_layout_matrix.update_item(
                            row,
                            column,
                            MatrixCell::AutoPlaced(grid_layout_item),
                        );
                    }
                }
            }
        }

        let total_min_content_size =
            each_min_content_size
                .into_iter()
                .fold(T::Length::zero(), |acc, cur| {
                    if let Some(min_content_size) = cur {
                        return acc + min_content_size.width;
                    }
                    acc
                });

        let should_use_min_content_size = if let Some(request_width) = requested_size.width.val() {
            total_min_content_size > request_width
        } else {
            false
        };

        drop(grid_matrix);

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 7: Finalize Track Sizes
        // CSS Grid §11.6-11.7: https://www.w3.org/TR/css-grid-1/#algo-grow-tracks
        //
        // Adjust track sizes to their final values:
        // - For auto tracks: size to fit content (using outer/margin-box size)
        // - For fixed tracks: use the specified size
        // - For fr tracks: share remaining space proportionally (§11.7)
        // ═══════════════════════════════════════════════════════════════════════
        let each_inline_size =
            adjust_each_inline_size(&mut grid_layout_matrix, should_use_min_content_size);
        let total_inline_size: T::Length = each_inline_size
            .iter()
            .fold(T::Length::zero(), |acc, cur| acc + *cur)
            + total_column_gaps;

        let each_block_size = adjust_each_block_size(&mut grid_layout_matrix);
        let total_block_size: T::Length = each_block_size
            .iter()
            .fold(T::Length::zero(), |acc, cur| acc + *cur)
            + total_row_gaps;

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 8: Content Alignment (align-content / justify-content)
        // CSS Grid §10.5: https://www.w3.org/TR/css-grid-1/#grid-align
        //
        // Distribute remaining space among tracks within the container:
        // - align-content: Aligns tracks in the block axis (vertical)
        // - justify-content: Aligns tracks in the inline axis (horizontal)
        //
        // Values like space-between, space-around, space-evenly distribute
        // extra space between and around the tracks.
        // ═══════════════════════════════════════════════════════════════════════
        let container_content_width = requested_size.width.unwrap_or(total_inline_size);
        let container_content_height = requested_size.height.unwrap_or(total_block_size);

        let (block_content_offset, block_gap_addition) = calculate_align_content_offset(
            style.align_content(),
            total_block_size,
            container_content_height,
            grid_layout_matrix.row_count(),
        );

        let (inline_content_offset, inline_gap_addition) = calculate_justify_content_offset(
            style.justify_content(),
            total_inline_size,
            container_content_width,
            grid_layout_matrix.column_count(),
        );

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 9: Item Positioning and Self-Alignment
        // CSS Grid §10.3-10.4: https://www.w3.org/TR/css-grid-1/#grid-align
        //
        // Position each grid item within its grid area:
        // - align-self (§10.4): Aligns item in the block axis within its cell
        // - justify-self (§10.3): Aligns item in the inline axis within its cell
        //
        // Items are positioned at: base_offset + alignment_offset + margin
        // ═══════════════════════════════════════════════════════════════════════
        let mut block_offset = block_content_offset;
        for row_index in 0..grid_layout_matrix.row_count() {
            // Add row gap before non-first rows (original gap + content alignment gap)
            if row_index > 0 {
                block_offset = block_offset + row_gap + block_gap_addition;
            }

            let mut current_block_size = T::Length::zero();
            let mut inline_offset = inline_content_offset;
            let mut is_first_column_in_row = true;
            for column_index in 0..grid_layout_matrix.column_count() {
                if let Some(grid_matrix_item) =
                    grid_layout_matrix.inner.get_mut(row_index, column_index)
                {
                    if grid_matrix_item.is_unoccupied() {
                        continue;
                    }

                    // Add column gap before non-first items (original gap + content alignment gap)
                    if !is_first_column_in_row {
                        inline_offset = inline_offset + column_gap + inline_gap_addition;
                    }
                    is_first_column_in_row = false;

                    let grid_matrix_item = grid_matrix_item.get_auto_placed_unchecked();
                    let mut layout_node = grid_matrix_item.node.layout_node().unit();

                    let size = Size::new(
                        grid_matrix_item
                            .css_size
                            .width
                            .or(grid_matrix_item.track_size.width),
                        grid_matrix_item
                            .css_size
                            .height
                            .or(grid_matrix_item.track_size.height),
                    );

                    let compute_result = layout_node.compute_internal(
                        env,
                        grid_matrix_item.node,
                        ComputeRequest {
                            size: Normalized(size),
                            parent_inner_size: Normalized(grid_matrix_item.track_size),
                            max_content: Normalized(grid_matrix_item.track_size),
                            kind: request.kind,
                            parent_is_block: false,
                            sizing_mode: request.sizing_mode,
                        },
                    );
                    let track_size = Size::new(
                        grid_matrix_item.track_size.width.val().unwrap(),
                        grid_matrix_item.track_size.height.val().unwrap(),
                    );

                    // Calculate alignment for the grid item within its cell
                    let child_style = grid_matrix_item.node.style();
                    let align_self = resolve_grid_align_self::<T>(child_style, style);
                    let justify_self = resolve_grid_justify_self::<T>(child_style, style);

                    // Get the actual item size (computed size)
                    let item_size = compute_result.size.0;

                    // Calculate alignment offset in block axis (vertical)
                    let align_offset =
                        calculate_alignment_offset(align_self, item_size.height, track_size.height);

                    // Calculate justify offset in inline axis (horizontal)
                    let justify_offset =
                        calculate_justify_offset(justify_self, item_size.width, track_size.width);

                    layout_node.gen_origin(
                        axis_info,
                        track_size,
                        block_offset
                            + align_offset
                            + grid_matrix_item
                                .margin
                                .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                                .or_zero(),
                        inline_offset
                            + justify_offset
                            + grid_matrix_item
                                .margin
                                .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                                .or_zero(),
                    );
                    inline_offset += track_size.width;
                    current_block_size = track_size.height;
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
            min_content_size: Normalized(size),
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

pub(crate) struct InitializedTrackListInfo<'a, T: LayoutTreeNode> {
    list: Vec<&'a LayoutTrackListItem<T::Length, T::LengthCustom>>,
    auto_count: usize,
    total_fr: f32,
}

fn initialize_track_list<'a, T: LayoutTreeNode>(
    grid_template_rows: &'a LayoutGridTemplate<T::Length, T::LengthCustom>,
) -> InitializedTrackListInfo<'a, T> {
    let mut track_auto_count = 0;
    let mut total_fr: f32 = 0.0;
    let track_list = grid_template_track_iterator::<T>(grid_template_rows, |item| {
        match item {
            LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(DefLength::Auto)) => {
                track_auto_count += 1;
            }
            LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value)) => {
                total_fr += fr_value;
            }
            _ => {}
        }
        matches!(item, LayoutTrackListItem::TrackSize(_))
    })
    .map(|it| it.collect::<Vec<_>>())
    .unwrap_or(Vec::with_capacity(0));
    InitializedTrackListInfo {
        list: track_list,
        auto_count: track_auto_count,
        total_fr,
    }
}

/// Finalize column (inline) track sizes.
///
/// CSS Grid §11.5-11.6: Track Sizing Algorithm
/// <https://www.w3.org/TR/css-grid-1/#algo-track-sizing>
///
/// For each column:
/// - **Explicit tracks**: Use the specified size (e.g., `100px`, `50%`)
/// - **Auto tracks**: Size based on item outer size (margin-box)
///
/// CSS Grid - Grid Item Contributions:
/// <https://www.w3.org/TR/css-grid-1/#min-size-contribution>
/// > "The min-content/max-content contribution of a grid item is its outer size"
///
/// This means auto tracks must consider the item's margin when sizing.
pub(crate) fn adjust_each_inline_size<T: LayoutTreeNode>(
    grid_layout_matrix: &mut GridLayoutMatrix<T>,
    should_use_min_content_size: bool,
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
            // Track has explicit size - use it directly without adding margin
            // Margin only affects item position within the track, not the track size
            inline_size = if !should_use_min_content_size {
                item.get_auto_placed_unchecked()
                    .track_inline_size()
                    .val()
                    .unwrap()
                    .max(
                        grid_layout_matrix
                            .inner
                            .iter_col(col_index)
                            .filter(|item| !item.is_unoccupied())
                            .fold(T::Length::zero(), |acc, cur| {
                                acc.max(
                                    cur.get_auto_placed_unchecked()
                                        .min_content_size()
                                        .unwrap()
                                        .width,
                                )
                            }),
                    )
            } else {
                grid_layout_matrix
                    .inner
                    .iter_col(col_index)
                    .filter(|item| !item.is_unoccupied())
                    .fold(T::Length::zero(), |acc, cur| {
                        acc.max(
                            cur.get_auto_placed_unchecked()
                                .min_content_size()
                                .unwrap()
                                .width,
                        )
                    })
            };
        } else {
            // For auto tracks, use the outer size (margin box) per W3C spec:
            // https://www.w3.org/TR/css-grid-1/#algo-track-sizing
            // "The min-content/max-content contribution of a grid item is its outer size"
            inline_size = if !should_use_min_content_size {
                grid_layout_matrix
                    .inner
                    .iter_col(col_index)
                    .filter(|item| !item.is_unoccupied())
                    .fold(T::Length::zero(), |acc, cur| {
                        let item = cur.get_auto_placed_unchecked();
                        // outer size = border-box width + margin-left + margin-right
                        let outer_width = item.computed_size().width + item.margin.horizontal();
                        let min_content_outer_width =
                            item.min_content_size().unwrap().width + item.margin.horizontal();
                        acc.max(outer_width.max(min_content_outer_width))
                    })
            } else {
                grid_layout_matrix
                    .inner
                    .iter_col(col_index)
                    .filter(|item| !item.is_unoccupied())
                    .fold(T::Length::zero(), |acc, cur| {
                        let item = cur.get_auto_placed_unchecked();
                        // outer size = min-content width + margin-left + margin-right
                        acc.max(item.min_content_size().unwrap().width + item.margin.horizontal())
                    })
            }
        }

        grid_layout_matrix
            .inner
            .iter_col_mut(col_index)
            .filter(|item| !item.is_unoccupied())
            .for_each(|item| {
                let item: &mut GridLayoutItem<'_, T> = item.get_auto_placed_mut_unchecked();
                item.track_size.width = OptionNum::some(inline_size);
                // if item.css_size.width.is_none() {
                //     item.node.layout_node().unit().result.size.width = inline_size;
                // }
            });

        each_inline_size.push(inline_size);
    }

    each_inline_size
}

/// Finalize row (block) track sizes.
///
/// CSS Grid §11.5-11.6: Track Sizing Algorithm
/// <https://www.w3.org/TR/css-grid-1/#algo-track-sizing>
///
/// For each row:
/// - **Explicit tracks**: Use the specified size (e.g., `50px`, `30%`)
/// - **Auto tracks**: Size based on item outer size (margin-box)
///
/// CSS Grid - Grid Item Contributions:
/// <https://www.w3.org/TR/css-grid-1/#min-size-contribution>
/// > "The min-content/max-content contribution of a grid item is its outer size"
///
/// For auto rows, the row height = max(all items' margin-box heights in that row)
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
            // For auto tracks, use the outer size (margin box) per W3C spec:
            // https://www.w3.org/TR/css-grid-1/#algo-track-sizing
            // "The min-content/max-content contribution of a grid item is its outer size"
            block_size = grid_layout_matrix
                .inner
                .iter_row(row_index)
                .filter(|item| !item.is_unoccupied())
                .fold(T::Length::zero(), |acc, cur| {
                    let item = cur.get_auto_placed_unchecked();
                    // outer size = border-box height + margin-top + margin-bottom
                    acc.max(item.computed_size().height + item.margin.vertical())
                });
        }
        grid_layout_matrix
            .inner
            .iter_row_mut(row_index)
            .filter(|item| !item.is_unoccupied())
            .for_each(|item| {
                let item = item.get_auto_placed_mut_unchecked();
                item.track_size.height = OptionNum::some(block_size);
                // if item.css_size.height.is_none() {
                //     item.node.layout_node().unit().result.size.height = block_size;
                // }
            });
        each_block_size.push(block_size)
    }
    each_block_size
}
