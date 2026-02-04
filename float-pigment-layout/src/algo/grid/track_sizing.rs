//! Track Sizing Algorithm Implementation
//!
//! CSS Grid §11.5 (Resolve Intrinsic Track Sizes):
//! https://www.w3.org/TR/css-grid-1/#algo-content
//!
//! CSS Grid §11.7 (Expand Flexible Tracks):
//! https://www.w3.org/TR/css-grid-1/#algo-flex-tracks

use crate::*;
use float_pigment_css::length_num::LengthNum;
use float_pigment_css::num_traits::Zero;

use super::matrix::GridLayoutMatrix;

/// Track sizing result containing sizes, explicit flags, and fr values.
pub(crate) struct TrackSizingResult<T: LayoutTreeNode> {
    pub sizes: Vec<T::Length>,
    pub has_explicit: Vec<bool>,
    pub fr_values: Vec<f32>,
}

struct TrackInfo<L: LengthNum> {
    size: Option<L>,
    has_explicit: bool,
    is_fr: bool,
    fr_value: f32,
    min_content: L,
}

impl<L: LengthNum + Copy + Default> Default for TrackInfo<L> {
    fn default() -> Self {
        Self {
            size: None,
            has_explicit: false,
            is_fr: false,
            fr_value: 0.0,
            min_content: L::default(),
        }
    }
}

/// Compute track sizes based on item content.
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
pub(crate) fn compute_track_sizes<T: LayoutTreeNode>(
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

    let mut columns: Vec<TrackInfo<T::Length>> = (0..column_count)
        .map(|i| {
            let mut info = TrackInfo {
                size: None,
                has_explicit: false,
                is_fr: false,
                fr_value: 0.0,
                min_content: T::Length::zero(),
            };
            if let Some(LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value))) =
                column_track_list.get(i)
            {
                info.is_fr = true;
                info.fr_value = *fr_value;
            }
            info
        })
        .collect();

    let mut rows: Vec<TrackInfo<T::Length>> = (0..row_count)
        .map(|i| {
            let mut info = TrackInfo {
                size: None,
                has_explicit: false,
                is_fr: false,
                fr_value: 0.0,
                min_content: T::Length::zero(),
            };
            if let Some(LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value))) =
                row_track_list.get(i)
            {
                info.is_fr = true;
                info.fr_value = *fr_value;
            }
            info
        })
        .collect();

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 1: Collect base sizes (min-content) for ALL tracks
    // CSS Grid §11.5: Resolve Intrinsic Track Sizes
    // https://www.w3.org/TR/css-grid-1/#algo-content
    //
    // For each track, collect the min-content contribution of items.
    // This applies to fixed, auto, AND fr tracks.
    // ═══════════════════════════════════════════════════════════════════════

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
        columns[column].min_content = columns[column].min_content.max(outer_width);
        rows[row].min_content = rows[row].min_content.max(outer_height);

        // Handle column sizing for non-fr tracks
        if !columns[column].is_fr {
            if item.track_inline_size().is_some() {
                // Track has explicit size (fixed)
                columns[column].has_explicit = true;
                let track_width = item.track_inline_size().val().unwrap();
                let width = if !should_use_min_content_size {
                    track_width.max(min_content_width)
                } else {
                    min_content_width
                };
                columns[column].size =
                    Some(columns[column].size.map(|s| s.max(width)).unwrap_or(width));
            } else {
                // Auto track - use outer size (margin box)
                let outer_width = if !should_use_min_content_size {
                    let computed = item.computed_size().width + item.margin.horizontal();
                    computed.max(min_content_width)
                } else {
                    min_content_width
                };
                columns[column].size = Some(
                    columns[column]
                        .size
                        .map(|s| s.max(outer_width))
                        .unwrap_or(outer_width),
                );
            }
        }

        // Handle row sizing for non-fr tracks
        if !rows[row].is_fr {
            if item.track_block_size().is_some() {
                // Track has explicit size (fixed)
                rows[row].has_explicit = true;
                let track_height = item.track_block_size().val().unwrap();
                rows[row].size = Some(
                    rows[row]
                        .size
                        .map(|s| s.max(track_height))
                        .unwrap_or(track_height),
                );
            } else {
                // Auto track - use outer size (margin box)
                let outer_height = item.computed_size().height + item.margin.vertical();
                rows[row].size = Some(
                    rows[row]
                        .size
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

    // Calculate total non-fr column size
    let total_non_fr_column_size: T::Length = columns
        .iter()
        .filter(|c| !c.is_fr)
        .fold(T::Length::zero(), |acc, c| {
            acc + c.size.unwrap_or(T::Length::zero())
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
            let mut is_flexible: Vec<bool> = columns.iter().map(|c| c.is_fr).collect();
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
                        let hypothetical_size = hypothetical_fr_size.mul_f32(columns[i].fr_value);
                        let min_content = columns[i].min_content;

                        if hypothetical_size < min_content {
                            // Freeze this track at its min-content
                            is_flexible[i] = false;
                            columns[i].size = Some(min_content);
                            columns[i].has_explicit = true;
                            remaining_space -= min_content;
                            // Incremental update: subtract frozen track's fr value
                            active_flex -= columns[i].fr_value;
                            any_frozen = true;
                        }
                    }
                }

                if !any_frozen {
                    // No tracks were frozen, apply final sizes
                    for i in 0..column_count {
                        if is_flexible[i] {
                            let fr_size = hypothetical_fr_size.mul_f32(columns[i].fr_value);
                            columns[i].size = Some(fr_size);
                            columns[i].has_explicit = true;
                        }
                    }
                    break;
                }
            }
        }
    }

    // Calculate total non-fr row size
    let total_non_fr_row_size: T::Length = rows
        .iter()
        .filter(|r| !r.is_fr)
        .fold(T::Length::zero(), |acc, r| {
            acc + r.size.unwrap_or(T::Length::zero())
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
            let mut is_flexible: Vec<bool> = rows.iter().map(|r| r.is_fr).collect();
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
                        let hypothetical_size = hypothetical_fr_size.mul_f32(rows[i].fr_value);
                        let min_content = rows[i].min_content;

                        if hypothetical_size < min_content {
                            // Freeze this track at its min-content
                            is_flexible[i] = false;
                            rows[i].size = Some(min_content);
                            rows[i].has_explicit = true;
                            remaining_space -= min_content;
                            // Incremental update: subtract frozen track's fr value
                            active_flex -= rows[i].fr_value;
                            any_frozen = true;
                        }
                    }
                }

                if !any_frozen {
                    // No tracks were frozen, apply final sizes
                    for i in 0..row_count {
                        if is_flexible[i] {
                            let fr_size = hypothetical_fr_size.mul_f32(rows[i].fr_value);
                            rows[i].size = Some(fr_size);
                            rows[i].has_explicit = true;
                        }
                    }
                    break;
                }
            }
        }
    }

    // Extract final sizes, defaulting to zero
    let column_sizes: Vec<T::Length> = columns
        .iter()
        .map(|c| c.size.unwrap_or(T::Length::zero()))
        .collect();
    let row_sizes: Vec<T::Length> = rows
        .iter()
        .map(|r| r.size.unwrap_or(T::Length::zero()))
        .collect();

    // Update items with final track sizes
    for item in grid_layout_matrix.items_mut() {
        let row = item.row();
        let column = item.column();
        item.track_size.width = OptionNum::some(column_sizes[column]);
        item.track_size.height = OptionNum::some(row_sizes[row]);
    }

    // Extract result data from TrackInfo
    let column_has_explicit: Vec<bool> = columns.iter().map(|c| c.has_explicit).collect();
    let row_has_explicit: Vec<bool> = rows.iter().map(|r| r.has_explicit).collect();
    let column_fr_values: Vec<f32> = columns.iter().map(|c| c.fr_value).collect();
    let row_fr_values: Vec<f32> = rows.iter().map(|r| r.fr_value).collect();

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
