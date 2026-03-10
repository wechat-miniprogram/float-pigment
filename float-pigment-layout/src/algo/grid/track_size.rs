//! Track Sizing Implementation
//!
//! CSS Grid Layout Module Level 1 - §11.3 Track Sizing Algorithm
//! <https://www.w3.org/TR/css-grid-1/#algo-track-sizing>

use crate::{
    algo::grid::{GridFlow, GridMatrix},
    DefLength, LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode, OptionNum,
};
use core::fmt::Debug;
use float_pigment_css::{length_num::LengthNum, num_traits::Zero};

/// Represents the sizing state of a track during the sizing algorithm.
///
/// CSS Grid §11.4: Initialize Track Sizes
/// <https://www.w3.org/TR/css-grid-1/#algo-init>
#[derive(Clone, PartialEq)]
pub(crate) enum TrackSize<T: LayoutTreeNode> {
    /// Original track sizing function (auto, length, percentage)
    Original(DefLength<T::Length, T::LengthCustom>),
    /// Flexible length (fr unit) with its fr value
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
/// This function implements the initial track sizing in two phases:
///
/// ## Phase 1: Initialize Track Sizes (§11.4)
/// - Process fixed-length tracks (`100px`, `50%`)
/// - Mark flexible (`fr`) and `auto` tracks for later processing
///
/// ## Phase 2: Resolve Track Sizes (§11.5-11.7)
/// - **Auto tracks**: size determined later by content (§11.5)
/// - **Fr tracks**: deferred to `compute_track_sizes` (§11.7 iterative algorithm)
/// - **Fixed tracks**: Use the resolved value
pub(crate) fn apply_track_size<'a, T: LayoutTreeNode>(
    track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    flow: GridFlow,
    grid_matrix: &mut GridMatrix<'a, T>,
    parent_node: &'a T,
    current_flow_parent_size: OptionNum<T::Length>,
    available_grid_space: &mut OptionNum<T::Length>,
) {
    let mut total_specified_track_size = T::Length::zero();

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
                // MinContent/MaxContent tracks: initial base size = 0.
                // Their actual sizes are resolved by content-based sizing
                // in compute_track_sizes (§11.5), not here.
                _ => {}
            }
        }
    }


    // ═══════════════════════════════════════════════════════════════════════
    // Phase 2: Resolve track sizes
    // CSS Grid §11.5-11.7
    //
    // - Auto tracks (§11.5): Resolve based on content contribution
    // - Fr tracks (§11.7): Deferred to compute_track_sizes iterative algorithm
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
                        if let Some(available_width) = available_grid_space.val() {
                            TrackSize::Fixed(OptionNum::some(available_width.mul_f32(*percent)))
                        } else {
                            TrackSize::Fixed(OptionNum::none())
                        }
                    }
                    _ => TrackSize::Fixed(OptionNum::none()),
                }
            }
            TrackSize::Fr(_) => {
                // Fr tracks: deferred to compute_track_sizes (§11.7 iterative algorithm)
                // which handles min-content freeze correctly.
                TrackSize::Fixed(OptionNum::none())
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
