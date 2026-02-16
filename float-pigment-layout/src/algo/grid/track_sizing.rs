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

/// Resolve fr track sizes using the iterative algorithm from CSS Grid §11.7.
///
/// 1. hypothetical_fr_size = remaining_space / active_flex
/// 2. If any fr track's size < its min-content, freeze it at min-content
/// 3. Repeat until stable
fn resolve_fr_track_sizes<L: LengthNum + Copy>(
    tracks: &mut [TrackInfo<L>],
    total_fr: f32,
    available_space: OptionNum<L>,
) {
    if total_fr <= 0.0 {
        return;
    }
    let available = match available_space.val() {
        Some(v) => v,
        None => return,
    };

    let total_non_fr_size: L = tracks
        .iter()
        .filter(|t| !t.is_fr)
        .fold(L::zero(), |acc, t| acc + t.size.unwrap_or(L::zero()));

    let initial_remaining = if available > total_non_fr_size {
        available - total_non_fr_size
    } else {
        L::zero()
    };

    let mut remaining_space = initial_remaining;
    let mut active_flex = total_fr;
    let mut flexible_indices: Vec<usize> = (0..tracks.len()).filter(|&i| tracks[i].is_fr).collect();
    if flexible_indices.is_empty() {
        return;
    }
    loop {
        if active_flex <= 0.0 {
            break;
        }
        let hypothetical_fr_size = remaining_space.div_f32(active_flex);

        let mut any_frozen = false;
        flexible_indices.retain(|&i| {
            let hypothetical_size = hypothetical_fr_size.mul_f32(tracks[i].fr_value);
            if hypothetical_size < tracks[i].min_content {
                tracks[i].size = Some(tracks[i].min_content);
                tracks[i].has_explicit = true;
                remaining_space -= tracks[i].min_content;
                active_flex -= tracks[i].fr_value;
                any_frozen = true;
                false
            } else {
                true
            }
        });

        if !any_frozen {
            // All tracks are stable, apply final sizes
            for &i in &flexible_indices {
                let fr_size = hypothetical_fr_size.mul_f32(tracks[i].fr_value);
                tracks[i].size = Some(fr_size);
                tracks[i].has_explicit = true;
            }
            break;
        }
    }
}

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
/// CSS Grid §11: Track Sizing Algorithm
/// <https://www.w3.org/TR/css-grid-1/#algo-track-sizing>
///
/// Phase 1: Collect base sizes for all tracks
/// - For explicit tracks (fixed): use the specified size (at least min-content)
/// - For implicit tracks: use grid-auto-rows/columns (§7.6)
/// - For auto tracks: use the item's outer size (margin box)
/// - For fr tracks: collect min-content as base_size
///
/// Phase 2: Iterative fr algorithm (§11.7)
/// 1. Calculate hypothetical_fr_size = leftover_space / total_flex
/// 2. If any fr track's size < its min-content, freeze it at min-content
/// 3. Repeat until stable
///
/// Returns (column_result, row_result).
#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_track_sizes<T: LayoutTreeNode>(
    grid_layout_matrix: &mut GridLayoutMatrix<T>,
    should_use_min_content_size: bool,
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    available_grid_space: OptionSize<T::Length>,
    column_total_fr: f32,
    row_total_fr: f32,
    grid_auto_columns: &LayoutGridAuto<T::Length, T::LengthCustom>,
    grid_auto_rows: &LayoutGridAuto<T::Length, T::LengthCustom>,
) -> (TrackSizingResult<T>, TrackSizingResult<T>) {
    let row_count = grid_layout_matrix.row_count();
    let column_count = grid_layout_matrix.column_count();

    let explicit_column_count = column_track_list.len();
    let explicit_row_count = row_track_list.len();

    // Initialize column track info
    // CSS Grid §7.6: Implicit tracks use grid-auto-columns
    let mut columns: Vec<TrackInfo<T::Length>> = (0..column_count)
        .map(|i| {
            let mut info = TrackInfo {
                size: None,
                has_explicit: false,
                is_fr: false,
                fr_value: 0.0,
                min_content: T::Length::zero(),
            };
            // Check explicit grid first
            if let Some(LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value))) =
                column_track_list.get(i)
            {
                info.is_fr = true;
                info.fr_value = *fr_value;
            } else if i >= explicit_column_count {
                // Implicit track - use grid-auto-columns (§7.6)
                let implicit_index = i - explicit_column_count;
                match grid_auto_columns.get(implicit_index) {
                    LayoutTrackSize::Fr(fr_value) => {
                        info.is_fr = true;
                        info.fr_value = *fr_value;
                    }
                    _ => {}
                }
            }
            info
        })
        .collect();

    // Initialize row track info
    // CSS Grid §7.6: Implicit tracks use grid-auto-rows
    let mut rows: Vec<TrackInfo<T::Length>> = (0..row_count)
        .map(|i| {
            let mut info = TrackInfo {
                size: None,
                has_explicit: false,
                is_fr: false,
                fr_value: 0.0,
                min_content: T::Length::zero(),
            };
            // Check explicit grid first
            if let Some(LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value))) =
                row_track_list.get(i)
            {
                info.is_fr = true;
                info.fr_value = *fr_value;
            } else if i >= explicit_row_count {
                // Implicit track - use grid-auto-rows (§7.6)
                let implicit_index = i - explicit_row_count;
                match grid_auto_rows.get(implicit_index) {
                    LayoutTrackSize::Fr(fr_value) => {
                        info.is_fr = true;
                        info.fr_value = *fr_value;
                    }
                    _ => {}
                }
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
            // Determine track size source: explicit grid or grid-auto-columns
            let track_size = if column < explicit_column_count {
                // Explicit track
                item.track_inline_size()
            } else {
                // Implicit track - use grid-auto-columns (§7.6)
                let implicit_index = column - explicit_column_count;
                match grid_auto_columns.get(implicit_index) {
                    LayoutTrackSize::Length(def_len) => {
                        def_len.resolve(available_grid_space.width, item.node)
                    }
                    LayoutTrackSize::MinContent | LayoutTrackSize::MaxContent => OptionNum::none(),
                    LayoutTrackSize::Fr(_) => OptionNum::none(), // Handled above
                }
            };

            if track_size.is_some() {
                // Track has explicit size (fixed)
                columns[column].has_explicit = true;
                let track_width = track_size.val().unwrap();
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
            // Determine track size source: explicit grid or grid-auto-rows
            let track_size = if row < explicit_row_count {
                // Explicit track
                item.track_block_size()
            } else {
                // Implicit track - use grid-auto-rows (§7.6)
                let implicit_index = row - explicit_row_count;
                match grid_auto_rows.get(implicit_index) {
                    LayoutTrackSize::Length(def_len) => {
                        def_len.resolve(available_grid_space.height, item.node)
                    }
                    LayoutTrackSize::MinContent | LayoutTrackSize::MaxContent => OptionNum::none(),
                    LayoutTrackSize::Fr(_) => OptionNum::none(), // Handled above
                }
            };

            if track_size.is_some() {
                // Track has explicit size (fixed)
                rows[row].has_explicit = true;
                let track_height = track_size.val().unwrap();
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

    resolve_fr_track_sizes(&mut columns, column_total_fr, available_grid_space.width);
    resolve_fr_track_sizes(&mut rows, row_total_fr, available_grid_space.height);

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
