//! Track Sizing Implementation
//!
//! CSS Grid Layout Module Level 1 - §11.3 Track Sizing Algorithm
//! <https://www.w3.org/TR/css-grid-1/#algo-track-sizing>

use crate::{
    algo::grid::{GridFlow, GridMatrix},
    DefLength, LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode, OptionNum,
};
use float_pigment_css::{length_num::LengthNum, num_traits::Zero};
use core::fmt::Debug;

/// Represents the sizing state of a track during the sizing algorithm.
///
/// CSS Grid §11.4: Initialize Track Sizes
/// <https://www.w3.org/TR/css-grid-1/#algo-init>
#[derive(Clone, PartialEq)]
pub(crate) enum TrackSize<T: LayoutTreeNode> {
    /// Original track sizing function (auto, length, percentage)
    Original(DefLength<T::Length, T::LengthCustom>),
    /// Flexible length (fr unit) with its fr value
    /// CSS Grid §7.2.4: https://www.w3.org/TR/css-grid-1/#fr-unit
    Fr(f32),
    /// Resolved fixed size after track sizing algorithm
    Fixed(OptionNum<T::Length>),
}

impl<T: LayoutTreeNode> Debug for TrackSize<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Original(length) => write!(f, "Original({:?})", length),
            Self::Fr(fr) => write!(f, "Fr({:?})", fr),
            Self::Fixed(length) => write!(f, "Fixed({:?})", length),
        }
    }
}

/// Apply track sizing to the grid (initial pass).
///
/// CSS Grid §11.3: Track Sizing Algorithm
/// <https://www.w3.org/TR/css-grid-1/#algo-track-sizing>
///
/// This function implements the initial track sizing in three phases:
///
/// ## Phase 1: Initialize Track Sizes (§11.4)
/// - Process fixed-length tracks (`100px`, `50%`)
/// - Mark flexible (`fr`) and `auto` tracks for later processing
///
/// ## Phase 2: Calculate Fr Unit Size (§11.7)
/// - When no auto tracks: calculate `size_per_fr = remaining_space / total_fr`
/// - When auto and fr mixed: defer fr calculation to STEP 7 (after auto tracks are sized)
///
/// ## Phase 3: Resolve Track Sizes (§11.5-11.7)
/// - **Auto tracks**: size determined later by content (§11.5)
/// - **Fr tracks (no auto)**: `track_size = fr_value * size_per_fr`
/// - **Fr tracks (with auto)**: deferred to `compute_track_sizes`
/// - **Fixed tracks**: Use the resolved value
pub(crate) fn apply_track_size<'a, T: LayoutTreeNode>(
    track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    flow: GridFlow,
    grid_matrix: &mut GridMatrix<'a, T>,
    parent_node: &'a T,
    current_flow_parent_size: OptionNum<T::Length>,
    available_grid_space: &mut OptionNum<T::Length>,
    total_fr: f32,
) {
    let mut total_specified_track_size = T::Length::zero();
    let auto_count = match flow {
        GridFlow::Row => grid_matrix.row_auto_count(),
        GridFlow::Column => grid_matrix.column_auto_count(),
    };

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 1: Initialize Track Sizes
    // CSS Grid §11.4: https://www.w3.org/TR/css-grid-1/#algo-init
    //
    // For each track, initialize its base size and growth limit based on
    // its track sizing function.
    // ═══════════════════════════════════════════════════════════════════════

    // Calculate total specified track size from the track list
    // Include both fixed lengths AND auto tracks (auto = 0 initially, will be sized later)
    for track_item in track_list.iter() {
        if let LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(length)) = track_item {
            let current_size = length
                .resolve(current_flow_parent_size, parent_node)
                .or_zero();
            total_specified_track_size += current_size;
        }
        // Note: auto tracks have base size = 0 initially, content-based sizing happens in §11.5
    }

    // Apply track sizes to items
    for item in grid_matrix.items_mut() {
        let track_idx = match flow {
            GridFlow::Row => item.row(),
            GridFlow::Column => item.column(),
        };

        if track_idx < track_list.len() {
            match track_list[track_idx] {
                LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(length)) => match flow {
                    GridFlow::Row => {
                        item.update_track_block_size(TrackSize::Original(length.clone()))
                    }
                    GridFlow::Column => {
                        item.update_track_inline_size(TrackSize::Original(length.clone()))
                    }
                },
                LayoutTrackListItem::TrackSize(LayoutTrackSize::Fr(fr_value)) => match flow {
                    GridFlow::Row => item.update_track_block_size(TrackSize::Fr(*fr_value)),
                    GridFlow::Column => item.update_track_inline_size(TrackSize::Fr(*fr_value)),
                },
                _ => {}
            }
        }
    }

    if available_grid_space.is_none() && total_specified_track_size > T::Length::zero() {
        *available_grid_space = OptionNum::some(total_specified_track_size);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 2: Calculate the size per fr unit
    // CSS Grid §11.7: Expand Flexible Tracks
    // https://www.w3.org/TR/css-grid-1/#algo-flex-tracks
    //
    // The fr unit represents a fraction of the leftover space.
    // size_per_fr = (available_space - fixed_tracks_size - auto_tracks_size) / total_fr
    //
    // NOTE: When auto and fr are mixed, fr calculation is deferred to STEP 7
    // because auto tracks need content-based sizing first (§11.5).
    // ═══════════════════════════════════════════════════════════════════════

    // If there are auto tracks and fr tracks, defer fr calculation
    // The fr size will be recalculated in compute_track_sizes after auto tracks are sized
    let has_mixed_auto_and_fr = auto_count > 0 && total_fr > 0.0;

    let remaining_space = if let Some(available) = available_grid_space.val() {
        if available > total_specified_track_size {
            available - total_specified_track_size
        } else {
            T::Length::zero()
        }
    } else {
        T::Length::zero()
    };

    let size_per_fr = if total_fr > 0.0 && !has_mixed_auto_and_fr {
        // Only calculate fr size if there are no auto tracks
        remaining_space.div_f32(total_fr)
    } else {
        // When auto and fr are mixed, fr tracks get 0 here
        // They will be recalculated in STEP 7 after auto tracks are sized
        T::Length::zero()
    };

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 3: Resolve all track sizes
    // CSS Grid §11.5-11.7
    //
    // - Auto tracks (§11.5): Resolve based on content contribution
    // - Fr tracks (§11.7): size = fr_value × size_per_fr
    // - Percentage tracks: size = percentage × container_size
    // - Fixed tracks: Already resolved
    // ═══════════════════════════════════════════════════════════════════════
    for item in grid_matrix.items_mut() {
        let track_size_ref = match flow {
            GridFlow::Row => &item.track_block_size,
            GridFlow::Column => &item.track_inline_size,
        };

        let fixed_track_size = match track_size_ref {
            TrackSize::Original(def_length) => {
                match def_length {
                    DefLength::Auto => {
                        // Auto tracks: base size determined by content (§11.5)
                        // Do NOT pre-allocate space here. Content-based sizing happens
                        // in compute_track_sizes, and maximize_tracks (§11.6) distributes
                        // free space afterward.
                        TrackSize::Fixed(OptionNum::none())
                    }
                    DefLength::Points(points) => TrackSize::Fixed(OptionNum::some(points.clone())),
                    DefLength::Percent(percent) => {
                        let track_size = if let Some(available_width) = available_grid_space.val() {
                            TrackSize::Fixed(OptionNum::some(available_width.mul_f32(*percent)))
                        } else {
                            TrackSize::Fixed(OptionNum::none())
                        };
                        track_size
                    }
                    _ => TrackSize::Fixed(OptionNum::none()),
                }
            }
            TrackSize::Fr(fr_value) => {
                // Fr tracks: size = fr_value * size_per_fr
                let fr_size = size_per_fr.mul_f32(*fr_value);
                TrackSize::Fixed(OptionNum::some(fr_size))
            }
            TrackSize::Fixed(_) => {
                // Already fixed, skip
                continue;
            }
        };

        match flow {
            GridFlow::Row => item.update_track_block_size(fixed_track_size),
            GridFlow::Column => item.update_track_inline_size(fixed_track_size),
        }
    }
}
