//! CSS Grid Layout Algorithm Implementation
//!
//! This module implements the CSS Grid Layout Module Level 1 specification.
//! Reference: <https://www.w3.org/TR/css-grid-1/>
//!
//! ## Current Implementation Steps
//!
//! 1. STEP 1: Compute available grid space (§11.1)
//! 2. STEP 2: Resolve gutters/gap (§10.1)
//! 3. STEP 3: Resolve explicit grid (§7.1)
//! 4. STEP 4: Place grid items (§8.5)
//! 5. STEP 5: Track Sizing Algorithm with Iterative Re-resolution (§11.1, §11.3-11.7)
//! 6. STEP 6: Compute item sizes (§11.5)
//! 7. STEP 7: Finalize tracks + Maximize Tracks (§11.5-11.6)
//! 8. STEP 8: Content alignment (§10.5)
//! 9. STEP 9: Item positioning with self-alignment (§10.3-10.4)
//!
//! ## Optimization
//!
//! This implementation separates occupancy tracking from item storage:
//! - Occupancy grid: O(R × C) bytes using 1-byte enum
//! - Items Vec: O(N) items stored separately
//!
//! This achieves:
//! - **Time**: O(N) for item positioning (instead of O(R × C))
//! - **Space**: More efficient for sparse grids
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
mod track;
mod track_size;

use crate::{
    algo::grid::{
        alignment::{
            calculate_align_content_offset, calculate_alignment_offset,
            calculate_justify_content_offset, calculate_justify_offset, resolve_grid_align_self,
            resolve_grid_justify_self,
        },
        grid_item::GridLayoutItem,
        matrix::{GridLayoutMatrix, GridMatrix},
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
    /// 3. Re-resolve columns if min-content contribution changed (§11.1 Step 3)
    /// 4. Re-resolve rows if min-content contribution changed (§11.1 Step 4)
    /// 5. Align tracks via align-content/justify-content
    ///
    /// ## Implementation Steps
    ///
    /// 1. **Available Space**: Calculate container content box
    /// 2. **Gutters** (§10.1): Calculate row-gap and column-gap
    /// 3. **Explicit Grid** (§7.1): Resolve grid-template-rows/columns
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
        // CSS Grid §7.1: https://www.w3.org/TR/css-grid-1/#explicit-grids
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

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 4: Grid Item Placement (Single-Pass with Dynamic Expansion)
        // CSS Grid §8.5: https://www.w3.org/TR/css-grid-1/#auto-placement-algo
        //
        // Grid items are placed according to the auto-placement algorithm.
        // The matrix automatically expands when items are placed beyond
        // the explicit grid boundaries.
        // ═══════════════════════════════════════════════════════════════════════

        let mut grid_matrix = GridMatrix::new(
            row_track_list.len(),    // Explicit row count
            column_track_list.len(), // Explicit column count
            row_track_auto_count,
            column_track_auto_count,
            style.grid_auto_flow(),
        );

        // Single-pass placement with automatic grid expansion
        place_grid_items(&mut grid_matrix, node);

        // After placement, get the actual grid dimensions (may include implicit tracks)
        let actual_row_count = grid_matrix.row_count();
        let actual_column_count = grid_matrix.column_count();

        // Calculate total gap space: (n-1) gaps for n tracks
        // CSS Grid §10.1: Gutters are only placed between tracks, not at edges
        let total_column_gaps = if actual_column_count > 1 {
            column_gap.mul_i32(actual_column_count as i32 - 1)
        } else {
            T::Length::zero()
        };
        let total_row_gaps = if actual_row_count > 1 {
            row_gap.mul_i32(actual_row_count as i32 - 1)
        } else {
            T::Length::zero()
        };

        // Subtract gaps from available space before track sizing
        available_grid_space.width = available_grid_space.width - total_column_gaps;
        available_grid_space.height = available_grid_space.height - total_row_gaps;

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 5: Track Sizing Algorithm (with Iterative Re-resolution)
        // CSS Grid §11.1 Step 2-4: https://www.w3.org/TR/css-grid-1/#algo-grid-sizing
        // CSS Grid §11.3: https://www.w3.org/TR/css-grid-1/#algo-track-sizing
        //
        // This step sizes the grid tracks. The algorithm:
        // 1. Initialize each track's base size and growth limit (§11.4)
        // 2. Resolve intrinsic track sizes (§11.5)
        // 3. Maximize tracks (§11.6)
        // 4. Expand flexible tracks (§11.7)
        //
        // §11.1 Step 3-4: If min-content contribution changes based on row sizes,
        // repeat the column sizing (once only).
        //
        // fr unit (§7.2.4): https://www.w3.org/TR/css-grid-1/#fr-unit
        // Flexible lengths share remaining space proportionally.
        // ═══════════════════════════════════════════════════════════════════════

        // Save original available space for potential re-iteration
        let original_available_width = available_grid_space.width;
        let original_available_height = available_grid_space.height;

        // First pass: size columns then rows
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

        // §11.1 Step 3-4: Check if iterative re-resolution is needed
        // This is needed when items span auto tracks and have content that
        // can reflow (e.g., text wrapping, aspect-ratio items).
        let needs_re_resolution = column_track_auto_count > 0 && row_track_auto_count > 0;

        if needs_re_resolution {
            // Save current track sizes for comparison
            let initial_column_sizes: Vec<_> = grid_matrix
                .items()
                .map(|item| item.fixed_track_inline_size().cloned())
                .collect();

            // Re-apply column sizing with updated constraints
            available_grid_space.width = original_available_width;
            apply_track_size(
                column_track_list.as_slice(),
                GridFlow::Column,
                &mut grid_matrix,
                node,
                requested_inner_size.width,
                &mut available_grid_space.width,
                column_total_fr,
            );

            // Check if column sizes changed significantly
            let column_sizes_changed = grid_matrix
                .items()
                .zip(initial_column_sizes.iter())
                .any(|(item, initial)| item.fixed_track_inline_size() != initial.as_ref());

            if column_sizes_changed {
                // Re-apply row sizing with new column sizes
                available_grid_space.height = original_available_height;
                apply_track_size(
                    row_track_list.as_slice(),
                    GridFlow::Row,
                    &mut grid_matrix,
                    node,
                    requested_inner_size.height,
                    &mut available_grid_space.height,
                    row_total_fr,
                );
            }
        }

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

        let mut each_min_content_size: Vec<Option<Size<T::Length>>> =
            vec![None; grid_matrix.column_count()];

        for grid_item in grid_matrix.items() {
            let row = grid_item.row();
            let column = grid_item.column();
            let child_node = grid_item.node;
            let mut child_layout_node = child_node.layout_node().unit();

            let fixed_track_inline_size = grid_item.fixed_track_inline_size().unwrap().clone();
            let fixed_track_block_size = grid_item.fixed_track_block_size().unwrap().clone();

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
                .normalized_min_max_limit(child_node, track_size, border, padding_border)
                .normalized_size(css_size);

            let size = Normalized(Size::new(
                min_max_limit_css_size.0.width.or(track_size.width),
                min_max_limit_css_size.0.height.or(track_size.height),
            ));
            let min_content_res = child_layout_node.compute_internal(
                env,
                child_node,
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
                child_node,
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
                GridLayoutItem::new(row, column, child_node, child_margin, css_size, track_size);
            grid_layout_item.set_min_content_size(min_content_res.min_content_size.0);
            grid_layout_item.set_computed_size(res.size.0);

            // Update column min-content size
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

            grid_layout_matrix.add_item(grid_layout_item);
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
        // CSS Grid §11.5 + §11.7: https://www.w3.org/TR/css-grid-1/#algo-content
        //
        // Adjust track sizes based on item content:
        // - For auto tracks: size to fit content (using outer/margin-box size)
        // - For fixed tracks: use the specified size
        // - For fr tracks: distribute remaining space (available - fixed - auto)
        // ═══════════════════════════════════════════════════════════════════════
        let (column_result, row_result) = compute_track_sizes(
            &mut grid_layout_matrix,
            should_use_min_content_size,
            &column_track_list,
            &row_track_list,
            available_grid_space,
            column_total_fr,
            row_total_fr,
        );

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 7b: Create GridTracks for subsequent processing
        // CSS Grid §11.6-11.8
        //
        // NOTE: §11.7 (Expand Flexible Tracks) is already done in compute_track_sizes
        // using the iterative algorithm. The fr track sizes in column_result/row_result
        // are already final.
        // ═══════════════════════════════════════════════════════════════════════
        use crate::algo::grid::track::GridTracks;

        let mut column_tracks: GridTracks<T> = GridTracks::from_sizes_with_fr(
            &column_result.sizes,
            &column_result.has_explicit,
            &column_result.fr_values,
        );
        let mut row_tracks: GridTracks<T> = GridTracks::from_sizes_with_fr(
            &row_result.sizes,
            &row_result.has_explicit,
            &row_result.fr_values,
        );

        let has_definite_width = !matches!(style.width(), DefLength::Auto);
        let has_definite_height = !matches!(style.height(), DefLength::Auto);

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 7c: Maximize Tracks (§11.6)
        // CSS Grid §11.6: https://www.w3.org/TR/css-grid-1/#algo-grow-tracks
        //
        // Distribute free space equally to tracks with infinite growth limits
        // (auto tracks that aren't fr).
        // ═══════════════════════════════════════════════════════════════════════
        if has_definite_width {
            if let Some(container_width) = requested_inner_size.0.width.val() {
                let total_column_size = column_tracks.total_base_size();
                let free_space = container_width - total_column_size - total_column_gaps;
                column_tracks.maximize(free_space);
            }
        }

        if has_definite_height {
            if let Some(container_height) = requested_inner_size.0.height.val() {
                let total_row_size = row_tracks.total_base_size();
                let free_space = container_height - total_row_size - total_row_gaps;
                row_tracks.maximize(free_space);
            }
        }

        // ═══════════════════════════════════════════════════════════════════════
        // STEP 7d: Stretch auto Tracks (§11.8)
        // CSS Grid §11.8: https://www.w3.org/TR/css-grid-1/#algo-stretch
        //
        // When align-content/justify-content is `normal`, auto tracks are
        // stretched to fill the container.
        // ═══════════════════════════════════════════════════════════════════════
        use float_pigment_css::typing::{AlignContent, JustifyContent};

        let should_stretch_columns = matches!(style.justify_content(), JustifyContent::Stretch);
        let should_stretch_rows = matches!(
            style.align_content(),
            AlignContent::Normal | AlignContent::Stretch
        );

        if should_stretch_columns && has_definite_width {
            if let Some(container_width) = requested_inner_size.0.width.val() {
                let total_column_size = column_tracks.total_base_size();
                let free_space = container_width - total_column_size - total_column_gaps;
                column_tracks.stretch_auto_tracks(free_space);
            }
        }

        if should_stretch_rows && has_definite_height {
            if let Some(container_height) = requested_inner_size.0.height.val() {
                let total_row_size = row_tracks.total_base_size();
                let free_space = container_height - total_row_size - total_row_gaps;
                row_tracks.stretch_auto_tracks(free_space);
            }
        }

        // Get final track sizes
        let each_inline_size = column_tracks.resolved_sizes();
        let each_block_size = row_tracks.resolved_sizes();

        // Update items with maximized track sizes
        for item in grid_layout_matrix.items_mut() {
            let row = item.row();
            let column = item.column();
            item.track_size.width = OptionNum::some(each_inline_size[column]);
            item.track_size.height = OptionNum::some(each_block_size[row]);
        }

        let total_inline_size: T::Length = each_inline_size
            .iter()
            .fold(T::Length::zero(), |acc, cur| acc + *cur)
            + total_column_gaps;

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

        // Precompute row and column offsets for O(1) lookup during positioning
        // This includes content alignment gaps
        let row_gap_with_alignment = row_gap + block_gap_addition;
        let column_gap_with_alignment = column_gap + inline_gap_addition;
        grid_layout_matrix.set_row_sizes(&each_block_size, row_gap_with_alignment);
        grid_layout_matrix.set_column_sizes(&each_inline_size, column_gap_with_alignment);

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
        for grid_layout_item in grid_layout_matrix.items() {
            let row = grid_layout_item.row();
            let column = grid_layout_item.column();

            let block_offset = block_content_offset + grid_layout_matrix.get_row_offset(row);
            let inline_offset =
                inline_content_offset + grid_layout_matrix.get_column_offset(column);

            let mut layout_node = grid_layout_item.node.layout_node().unit();

            let size = Size::new(
                grid_layout_item
                    .css_size
                    .width
                    .or(grid_layout_item.track_size.width),
                grid_layout_item
                    .css_size
                    .height
                    .or(grid_layout_item.track_size.height),
            );

            let compute_result = layout_node.compute_internal(
                env,
                grid_layout_item.node,
                ComputeRequest {
                    size: Normalized(size),
                    parent_inner_size: Normalized(grid_layout_item.track_size),
                    max_content: Normalized(grid_layout_item.track_size),
                    kind: request.kind,
                    parent_is_block: false,
                    sizing_mode: request.sizing_mode,
                },
            );

            let track_size = Size::new(
                grid_layout_item.track_size.width.val().unwrap(),
                grid_layout_item.track_size.height.val().unwrap(),
            );

            // Calculate alignment for the grid item within its cell
            let child_style = grid_layout_item.node.style();
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
                    + grid_layout_item
                        .margin
                        .cross_axis_start(axis_info.dir, axis_info.cross_dir_rev)
                        .or_zero(),
                inline_offset
                    + justify_offset
                    + grid_layout_item
                        .margin
                        .main_axis_start(axis_info.dir, axis_info.main_dir_rev)
                        .or_zero(),
            );
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

/// Track sizing result containing sizes, explicit flags, and fr values.
struct TrackSizingResult<T: LayoutTreeNode> {
    sizes: Vec<T::Length>,
    has_explicit: Vec<bool>,
    fr_values: Vec<f32>,
}

/// Compute track sizes based on item content.
///
/// CSS Grid §11.5 (Resolve Intrinsic Track Sizes):
/// https://www.w3.org/TR/css-grid-1/#algo-content
///
/// CSS Grid §11.7 (Expand Flexible Tracks):
/// https://www.w3.org/TR/css-grid-1/#algo-flex-tracks
///
/// Phase 1: Collect base sizes for all tracks
/// - For explicit tracks (fixed): use the specified size (at least min-content)
/// - For auto tracks: use the item's outer size (margin box)
/// - For fr tracks: collect min-content as base_size
///
/// Phase 2: Iterative fr algorithm (§11.7)
/// 1. Calculate hypothetical_fr_size = leftover_space / total_flex
/// 2. If any fr track's size < its min-content, freeze it at min-content
/// 3. Repeat until stable
///
/// Returns (column_result, row_result).
fn compute_track_sizes<T: LayoutTreeNode>(
    grid_layout_matrix: &mut GridLayoutMatrix<T>,
    should_use_min_content_size: bool,
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    available_grid_space: OptionSize<T::Length>,
    column_total_fr: f32,
    row_total_fr: f32,
) -> (TrackSizingResult<T>, TrackSizingResult<T>) {
    let row_count = grid_layout_matrix.row_count();
    let column_count = grid_layout_matrix.column_count();

    // Initialize track sizes with default values
    let mut column_sizes: Vec<Option<T::Length>> = vec![None; column_count];
    let mut row_sizes: Vec<Option<T::Length>> = vec![None; row_count];

    // Track whether each column/row has explicit size (not auto, not fr)
    let mut column_has_explicit: Vec<bool> = vec![false; column_count];
    let mut row_has_explicit: Vec<bool> = vec![false; row_count];

    // Track which columns/rows are fr tracks and their fr values
    let mut column_is_fr: Vec<bool> = vec![false; column_count];
    let mut row_is_fr: Vec<bool> = vec![false; row_count];
    let mut column_fr_values: Vec<f32> = vec![0.0; column_count];
    let mut row_fr_values: Vec<f32> = vec![0.0; row_count];

    // Identify fr tracks from track list
    for (i, track) in column_track_list.iter().enumerate() {
        if i < column_count {
            if let LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value)) = track {
                column_is_fr[i] = true;
                column_fr_values[i] = *fr_value;
            }
        }
    }
    for (i, track) in row_track_list.iter().enumerate() {
        if i < row_count {
            if let LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value)) = track {
                row_is_fr[i] = true;
                row_fr_values[i] = *fr_value;
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 1: Collect base sizes (min-content) for ALL tracks
    // CSS Grid §11.5: Resolve Intrinsic Track Sizes
    // https://www.w3.org/TR/css-grid-1/#algo-content
    //
    // For each track, collect the min-content contribution of items.
    // This applies to fixed, auto, AND fr tracks.
    // ═══════════════════════════════════════════════════════════════════════

    // Track min-content base sizes for fr tracks (used in §11.7 iteration)
    // CSS Grid §11.7: The base size for flexible tracks is the item's min-content
    // contribution, which is the larger of:
    // - The item's specified size (if any)
    // - The item's min-content size
    let mut column_min_content: Vec<T::Length> = vec![T::Length::zero(); column_count];
    let mut row_min_content: Vec<T::Length> = vec![T::Length::zero(); row_count];

    for item in grid_layout_matrix.items() {
        let row = item.row();
        let column = item.column();

        // Use min-content for fr track base_size calculation
        // For items with CSS width/height, use that value
        // For items without (auto), use min-content size
        let css_width = item.css_size.width;
        let css_height = item.css_size.height;

        let min_content_width = item.min_content_size().unwrap().width;
        let min_content_height = item.min_content_size().unwrap().height;

        // If item has a CSS width, use it; otherwise use min-content
        let base_width = if css_width.is_some() {
            css_width.val().unwrap().max(min_content_width)
        } else {
            min_content_width
        };
        let outer_width = base_width + item.margin.horizontal();

        let base_height = if css_height.is_some() {
            css_height.val().unwrap().max(min_content_height)
        } else {
            min_content_height
        };
        let outer_height = base_height + item.margin.vertical();

        // Always update base_size for all tracks (including fr)
        column_min_content[column] = column_min_content[column].max(outer_width);
        row_min_content[row] = row_min_content[row].max(outer_height);

        // Handle column sizing for non-fr tracks
        if !column_is_fr[column] {
            if item.track_inline_size().is_some() {
                // Track has explicit size (fixed)
                column_has_explicit[column] = true;
                let track_width = item.track_inline_size().val().unwrap();
                let width = if !should_use_min_content_size {
                    track_width.max(min_content_width)
                } else {
                    min_content_width
                };
                column_sizes[column] =
                    Some(column_sizes[column].map(|s| s.max(width)).unwrap_or(width));
            } else {
                // Auto track - use outer size (margin box)
                let outer_width = if !should_use_min_content_size {
                    let computed = item.computed_size().width + item.margin.horizontal();
                    computed.max(min_content_width)
                } else {
                    min_content_width
                };
                column_sizes[column] = Some(
                    column_sizes[column]
                        .map(|s| s.max(outer_width))
                        .unwrap_or(outer_width),
                );
            }
        }

        // Handle row sizing for non-fr tracks
        if !row_is_fr[row] {
            if item.track_block_size().is_some() {
                // Track has explicit size (fixed)
                row_has_explicit[row] = true;
                let track_height = item.track_block_size().val().unwrap();
                row_sizes[row] = Some(
                    row_sizes[row]
                        .map(|s| s.max(track_height))
                        .unwrap_or(track_height),
                );
            } else {
                // Auto track - use outer size (margin box)
                let outer_height = item.computed_size().height + item.margin.vertical();
                row_sizes[row] = Some(
                    row_sizes[row]
                        .map(|s| s.max(outer_height))
                        .unwrap_or(outer_height),
                );
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 2: Calculate fr track sizes with iterative algorithm (§11.7)
    // CSS Grid §11.7: Expand Flexible Tracks
    // https://www.w3.org/TR/css-grid-1/#algo-flex-tracks
    //
    // The algorithm iteratively calculates fr sizes:
    // 1. hypothetical_fr_size = leftover_space / total_flex
    // 2. If any fr track's size < its min-content, freeze it at min-content
    // 3. Repeat until stable
    // ═══════════════════════════════════════════════════════════════════════
    use float_pigment_css::num_traits::Zero;

    // Calculate total non-fr column size
    let total_non_fr_column_size: T::Length = column_sizes
        .iter()
        .enumerate()
        .filter(|(i, _)| !column_is_fr[*i])
        .fold(T::Length::zero(), |acc, (_, s)| {
            acc + s.unwrap_or(T::Length::zero())
        });

    // Calculate fr column sizes with iterative algorithm
    // Optimization: Use incremental active_flex updates instead of recalculating each iteration
    if column_total_fr > 0.0 {
        if let Some(available_width) = available_grid_space.width.val() {
            let initial_remaining = if available_width > total_non_fr_column_size {
                available_width - total_non_fr_column_size
            } else {
                T::Length::zero()
            };

            // Track which fr tracks are still flexible (not frozen)
            let mut is_flexible: Vec<bool> = column_is_fr.clone();
            let mut remaining_space = initial_remaining;
            // Initialize active_flex once, then update incrementally
            let mut active_flex = column_total_fr;
            let mut iterations = 0;
            const MAX_ITERATIONS: usize = 10;

            loop {
                iterations += 1;
                if iterations > MAX_ITERATIONS || active_flex <= 0.0 {
                    break;
                }

                // Calculate hypothetical fr size
                let hypothetical_fr_size = remaining_space.div_f32(active_flex);

                // Check if any track needs to be frozen
                let mut any_frozen = false;
                for i in 0..column_count {
                    if is_flexible[i] {
                        let hypothetical_size = hypothetical_fr_size.mul_f32(column_fr_values[i]);
                        let min_content = column_min_content[i];

                        if hypothetical_size < min_content {
                            // Freeze this track at its min-content
                            is_flexible[i] = false;
                            column_sizes[i] = Some(min_content);
                            column_has_explicit[i] = true;
                            remaining_space -= min_content;
                            // Incremental update: subtract frozen track's fr value
                            active_flex -= column_fr_values[i];
                            any_frozen = true;
                        }
                    }
                }

                if !any_frozen {
                    // No tracks were frozen, apply final sizes
                    for i in 0..column_count {
                        if is_flexible[i] {
                            let fr_size = hypothetical_fr_size.mul_f32(column_fr_values[i]);
                            column_sizes[i] = Some(fr_size);
                            column_has_explicit[i] = true;
                        }
                    }
                    break;
                }
            }
        }
    }

    // Calculate total non-fr row size
    let total_non_fr_row_size: T::Length = row_sizes
        .iter()
        .enumerate()
        .filter(|(i, _)| !row_is_fr[*i])
        .fold(T::Length::zero(), |acc, (_, s)| {
            acc + s.unwrap_or(T::Length::zero())
        });

    // Calculate fr row sizes with iterative algorithm
    if row_total_fr > 0.0 {
        if let Some(available_height) = available_grid_space.height.val() {
            let initial_remaining = if available_height > total_non_fr_row_size {
                available_height - total_non_fr_row_size
            } else {
                T::Length::zero()
            };

            // Track which fr tracks are still flexible (not frozen)
            let mut is_flexible: Vec<bool> = row_is_fr.clone();
            let mut remaining_space = initial_remaining;
            // Initialize active_flex once, then update incrementally
            let mut active_flex = row_total_fr;
            let mut iterations = 0;
            const MAX_ITERATIONS: usize = 10;

            loop {
                iterations += 1;
                if iterations > MAX_ITERATIONS || active_flex <= 0.0 {
                    break;
                }

                // Calculate hypothetical fr size
                let hypothetical_fr_size = remaining_space.div_f32(active_flex);

                // Check if any track needs to be frozen
                let mut any_frozen = false;
                for i in 0..row_count {
                    if is_flexible[i] {
                        let hypothetical_size = hypothetical_fr_size.mul_f32(row_fr_values[i]);
                        let min_content = row_min_content[i];

                        if hypothetical_size < min_content {
                            // Freeze this track at its min-content
                            is_flexible[i] = false;
                            row_sizes[i] = Some(min_content);
                            row_has_explicit[i] = true;
                            remaining_space -= min_content;
                            // Incremental update: subtract frozen track's fr value
                            active_flex -= row_fr_values[i];
                            any_frozen = true;
                        }
                    }
                }

                if !any_frozen {
                    // No tracks were frozen, apply final sizes
                    for i in 0..row_count {
                        if is_flexible[i] {
                            let fr_size = hypothetical_fr_size.mul_f32(row_fr_values[i]);
                            row_sizes[i] = Some(fr_size);
                            row_has_explicit[i] = true;
                        }
                    }
                    break;
                }
            }
        }
    }

    // Convert Option<Length> to Length, defaulting to zero
    let column_sizes: Vec<T::Length> = column_sizes
        .into_iter()
        .map(|s| s.unwrap_or(T::Length::zero()))
        .collect();
    let row_sizes: Vec<T::Length> = row_sizes
        .into_iter()
        .map(|s| s.unwrap_or(T::Length::zero()))
        .collect();

    // Update items with final track sizes
    for item in grid_layout_matrix.items_mut() {
        let row = item.row();
        let column = item.column();
        item.track_size.width = OptionNum::some(column_sizes[column]);
        item.track_size.height = OptionNum::some(row_sizes[row]);
    }

    (
        TrackSizingResult {
            sizes: column_sizes,
            has_explicit: column_has_explicit,
            fr_values: column_fr_values,
        },
        TrackSizingResult {
            sizes: row_sizes,
            has_explicit: row_has_explicit,
            fr_values: row_fr_values,
        },
    )
}
