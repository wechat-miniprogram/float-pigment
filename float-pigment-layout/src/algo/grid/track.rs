//! Grid Track Data Structure
//!
//! CSS Grid Layout Module Level 1 - §11.4 Initialize Track Sizes
//! <https://www.w3.org/TR/css-grid-1/#algo-init>
//!
//! This module defines the `GridTrack` structure which represents a single
//! grid track (row or column) during the track sizing algorithm.

use crate::{algo::grid::track_sizing::TrackInfo, DefLength, LayoutTreeNode};
use alloc::vec::Vec;
use core::fmt::Debug;
use float_pigment_css::length_num::LengthNum;
use float_pigment_css::num_traits::Zero;

use super::track_sizing::IntrinsicTrackType;

/// A single grid track (row or column) with its sizing information.
///
/// CSS Grid §11.4: Initialize Track Sizes
/// <https://www.w3.org/TR/css-grid-1/#algo-init>
///
/// Each track has two main sizing values:
/// - `base_size`: The track's current computed size (initialized from min sizing function)
/// - `growth_limit`: The maximum size this track can grow to (initialized from max sizing function)
///
/// For simple track definitions like `100px` or `1fr`, both values come from the same function.
/// For `minmax(min, max)`, they come from different functions.
pub(crate) struct GridTrack<T: LayoutTreeNode> {
    /// The track's minimum sizing function (determines initial base_size)
    pub min_sizing_function: TrackSizingFunction<T>,

    /// The track's maximum sizing function (determines growth_limit)
    pub max_sizing_function: TrackSizingFunction<T>,

    /// The track's current computed size.
    /// CSS Grid §11.4: "base size"
    ///
    /// Initialized based on the min sizing function:
    /// - Fixed length: the resolved length
    /// - Flexible (fr): 0
    /// - Auto/intrinsic: 0 (resolved later based on content)
    pub base_size: T::Length,

    /// The maximum size this track can grow to.
    /// CSS Grid §11.4: "growth limit"
    ///
    /// Initialized based on the max sizing function:
    /// - Fixed length: the resolved length
    /// - Flexible (fr): infinity (None)
    /// - Auto: infinity (None)
    /// - min-content/max-content: infinity (None), clamped later
    ///
    /// `None` represents infinity.
    pub growth_limit: Option<T::Length>,
}

impl<T: LayoutTreeNode> Debug for GridTrack<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GridTrack {{ min: {:?}, max: {:?}, base_size: {:?}, growth_limit: {:?} }}",
            self.min_sizing_function, self.max_sizing_function, self.base_size, self.growth_limit
        )
    }
}

impl<T: LayoutTreeNode> GridTrack<T> {
    /// Create a new track with the given sizing function.
    ///
    /// For simple track definitions (not minmax), both min and max functions are the same.
    pub fn new(
        min_sizing_function: TrackSizingFunction<T>,
        max_sizing_function: TrackSizingFunction<T>,
    ) -> Self {
        Self {
            min_sizing_function,
            max_sizing_function,
            base_size: T::Length::zero(),
            growth_limit: None,
        }
    }

    /// Create a new track from a single sizing function (min == max).
    pub fn from_single(sizing_function: TrackSizingFunction<T>) -> Self {
        let max_fn = match &sizing_function {
            TrackSizingFunction::Fixed(length) => TrackSizingFunction::Fixed(length.clone()),
            TrackSizingFunction::Flex(fr) => TrackSizingFunction::Flex(*fr),
            TrackSizingFunction::Auto => TrackSizingFunction::Auto,
            TrackSizingFunction::Intrinsic => TrackSizingFunction::Intrinsic,
        };
        Self::new(sizing_function, max_fn)
    }

    /// Get the resolved track size (base_size).
    #[inline(always)]
    pub fn resolved_size(&self) -> T::Length {
        self.base_size
    }

    /// Check if the track is flexible (fr unit).
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        matches!(self.max_sizing_function, TrackSizingFunction::Flex(_))
    }

    /// Check if the track is auto-sized.
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        matches!(self.max_sizing_function, TrackSizingFunction::Auto)
    }
}

/// Track sizing function type.
///
/// CSS Grid §7.2: Track Sizing Functions
/// <https://www.w3.org/TR/css-grid-1/#track-sizing>
///
/// This enum represents the max track sizing function after §11.5 processing.
/// It determines how the track participates in §11.6 Maximize and §11.8 Stretch:
/// - `Fixed`: does not grow in §11.6 (finite growth_limit), excluded from §11.8
/// - `Auto`: grows in §11.6 (infinite growth_limit, never freezes), participates in §11.8
/// - `Intrinsic`: grows in §11.6 (finite growth_limit, freezes at limit), excluded from §11.8
/// - `Flex`: excluded from §11.6 (fr tracks already sized in §11.7), excluded from §11.8
pub(crate) enum TrackSizingFunction<T: LayoutTreeNode> {
    /// Fixed length: `100px`, `50%`, etc.
    Fixed(DefLength<T::Length, T::LengthCustom>),

    /// Flexible length: `1fr`, `2fr`, etc.
    Flex(f32),

    /// Auto sizing: `auto`
    Auto,

    /// Intrinsic sizing: `min-content` or `max-content`
    Intrinsic,
}

impl<T: LayoutTreeNode> Debug for TrackSizingFunction<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fixed(length) => write!(f, "Fixed({:?})", length),
            Self::Flex(fr) => write!(f, "Flex({}fr)", fr),
            Self::Auto => write!(f, "Auto"),
            Self::Intrinsic => write!(f, "Intrinsic"),
        }
    }
}

/// A collection of grid tracks for one axis (rows or columns).
pub(crate) struct GridTracks<T: LayoutTreeNode> {
    /// The tracks in this axis
    tracks: Vec<GridTrack<T>>,
    /// Count of auto tracks (for distributing remaining space)
    auto_count: usize,
}

impl<T: LayoutTreeNode> Debug for GridTracks<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GridTracks {{ count: {}, auto_count: {} }}",
            self.tracks.len(),
            self.auto_count
        )
    }
}

impl<T: LayoutTreeNode> GridTracks<T> {
    /// Get the total base size of all tracks.
    pub fn total_base_size(&self) -> T::Length {
        self.tracks
            .iter()
            .fold(T::Length::zero(), |acc, track| acc + track.base_size)
    }

    /// Get resolved sizes as a vector.
    pub fn resolved_sizes(&self) -> Vec<T::Length> {
        self.tracks.iter().map(|t| t.resolved_size()).collect()
    }

    /// Create GridTracks from resolved `TrackInfo` entries.
    pub fn from_track_info(track_info: &[TrackInfo<T::Length>]) -> Self {
        let mut tracks = Vec::with_capacity(track_info.len());
        let mut auto_count = 0;

        for item in track_info {
            let sizing_fn = match item.track_type {
                IntrinsicTrackType::Fr => TrackSizingFunction::Flex(item.fr_value),
                IntrinsicTrackType::Auto => {
                    auto_count += 1;
                    TrackSizingFunction::Auto
                }
                IntrinsicTrackType::MinContent | IntrinsicTrackType::MaxContent => {
                    TrackSizingFunction::Intrinsic
                }
                IntrinsicTrackType::Fixed => TrackSizingFunction::Fixed(DefLength::Points(
                    item.base_size.unwrap_or(T::Length::zero()),
                )),
            };

            let mut track = GridTrack::from_single(sizing_fn);
            track.base_size = item.base_size.unwrap_or(T::Length::zero());
            track.growth_limit = if item.track_type == IntrinsicTrackType::Fr {
                None
            } else {
                item.growth_limit
            };
            tracks.push(track);
        }

        Self { tracks, auto_count }
    }

    /// Maximize tracks by distributing free space.
    ///
    /// CSS Grid §11.6: Maximize Tracks
    /// <https://www.w3.org/TR/css-grid-1/#algo-grow-tracks>
    ///
    /// If the free space is positive, distribute it equally to the base sizes
    /// of all tracks, freezing tracks as they reach their growth limits.
    ///
    /// This is an iterative algorithm:
    /// 1. Distribute free space equally among all unfrozen non-flex tracks
    /// 2. If any track's base_size would exceed its growth_limit, freeze it
    ///    at the growth_limit and reclaim the excess
    /// 3. Repeat until all free space is consumed or all tracks are frozen
    ///
    /// Tracks with infinite growth limits (None) never freeze.
    pub fn maximize(&mut self, free_space: T::Length) {
        if free_space <= T::Length::zero() {
            return;
        }

        // Collect indices of all non-flex tracks (candidates for space distribution)
        let mut unfrozen: Vec<usize> = Vec::with_capacity(self.tracks.len());
        for (i, track) in self.tracks.iter().enumerate() {
            if !track.is_flexible() {
                unfrozen.push(i);
            }
        }

        if unfrozen.is_empty() {
            return;
        }

        let mut remaining = free_space;

        // Iterative distribution with freeze-on-limit (§11.6)
        loop {
            if unfrozen.is_empty() || remaining <= T::Length::zero() {
                break;
            }

            let share = remaining.div_f32(unfrozen.len() as f32);
            let mut any_frozen = false;
            let mut space_used = T::Length::zero();

            unfrozen.retain(|&i| {
                let track = &mut self.tracks[i];
                let new_base = track.base_size + share;

                if let Some(limit) = track.growth_limit {
                    if new_base >= limit {
                        // Freeze at growth_limit; only consume the delta
                        let delta = limit - track.base_size;
                        if delta > T::Length::zero() {
                            space_used += delta;
                            track.base_size = limit;
                        }
                        any_frozen = true;
                        return false; // remove from unfrozen
                    }
                }

                // Not frozen yet; tentatively accept the share
                space_used += share;
                track.base_size = new_base;
                true // keep in unfrozen
            });

            remaining -= space_used;

            // If no track was frozen this round, all space has been distributed
            if !any_frozen {
                break;
            }
        }
    }

    /// Stretch auto tracks to fill remaining space.
    ///
    /// CSS Grid §11.8: Stretch auto Tracks
    /// <https://www.w3.org/TR/css-grid-1/#algo-stretch>
    ///
    /// This step applies when the content-distribution property
    /// (align-content/justify-content) is `normal` or `stretch`.
    ///
    /// If the free space is positive, distribute it equally to the
    /// base sizes of all auto tracks. Fr tracks are excluded because
    /// they already consumed their share of free space in §11.7.
    pub fn stretch_auto_tracks(&mut self, free_space: T::Length) {
        if free_space <= T::Length::zero() {
            return;
        }

        if self.auto_count == 0 {
            return;
        }

        // Distribute free space equally among auto tracks
        let space_per_track = free_space.div_f32(self.auto_count as f32);

        for track in &mut self.tracks {
            if track.is_auto() {
                track.base_size += space_per_track;
            }
        }
    }
}
