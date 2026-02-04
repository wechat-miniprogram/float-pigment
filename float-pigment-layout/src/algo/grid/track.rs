//! Grid Track Data Structure
//!
//! CSS Grid Layout Module Level 1 - §11.4 Initialize Track Sizes
//! <https://www.w3.org/TR/css-grid-1/#algo-init>
//!
//! This module defines the `GridTrack` structure which represents a single
//! grid track (row or column) during the track sizing algorithm.

#![allow(dead_code)]

use crate::{DefLength, LayoutTreeNode, OptionNum};
use float_pigment_css::length_num::LengthNum;
use float_pigment_css::num_traits::Zero;
use std::fmt::Debug;

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

    /// Whether this track contains any items.
    /// Used for `auto-fill`/`auto-fit` empty track handling.
    pub is_occupied: bool,
}

impl<T: LayoutTreeNode> Debug for GridTrack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            is_occupied: false,
        }
    }

    /// Create a new track from a single sizing function (min == max).
    pub fn from_single(sizing_function: TrackSizingFunction<T>) -> Self {
        // For non-minmax tracks, we need to duplicate the sizing function
        // We'll extract the values and create new instances
        let (min_fn, max_fn) = match sizing_function {
            TrackSizingFunction::Fixed(ref length) => (
                TrackSizingFunction::Fixed(length.clone()),
                TrackSizingFunction::Fixed(length.clone()),
            ),
            TrackSizingFunction::Flex(fr) => {
                (TrackSizingFunction::Flex(fr), TrackSizingFunction::Flex(fr))
            }
            TrackSizingFunction::Auto => (TrackSizingFunction::Auto, TrackSizingFunction::Auto),
        };
        Self::new(min_fn, max_fn)
    }

    /// Initialize base_size and growth_limit based on sizing functions.
    ///
    /// CSS Grid §11.4: Initialize Track Sizes
    /// <https://www.w3.org/TR/css-grid-1/#algo-init>
    pub fn initialize(
        &mut self,
        available_space: OptionNum<T::Length>,
        resolve_length: impl Fn(
            &DefLength<T::Length, T::LengthCustom>,
            OptionNum<T::Length>,
        ) -> T::Length,
    ) {
        // Initialize base_size from min sizing function
        self.base_size = match &self.min_sizing_function {
            TrackSizingFunction::Fixed(length) => resolve_length(length, available_space),
            TrackSizingFunction::Flex(_) => T::Length::zero(),
            TrackSizingFunction::Auto => T::Length::zero(),
        };

        // Initialize growth_limit from max sizing function
        self.growth_limit = match &self.max_sizing_function {
            TrackSizingFunction::Fixed(length) => {
                let resolved = resolve_length(length, available_space);
                // growth_limit must be >= base_size
                if resolved >= self.base_size {
                    Some(resolved)
                } else {
                    Some(self.base_size)
                }
            }
            TrackSizingFunction::Flex(_) => None, // infinity
            TrackSizingFunction::Auto => None,    // infinity
        };
    }

    /// Get the resolved track size (base_size).
    #[inline(always)]
    pub fn resolved_size(&self) -> T::Length {
        self.base_size
    }

    /// Check if the track has an infinite growth limit.
    #[inline(always)]
    pub fn has_infinite_growth_limit(&self) -> bool {
        self.growth_limit.is_none()
    }

    /// Check if the track is flexible (fr unit).
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        matches!(self.max_sizing_function, TrackSizingFunction::Flex(_))
    }

    /// Get the flex factor if this is a flexible track.
    #[inline(always)]
    pub fn flex_factor(&self) -> Option<f32> {
        match &self.max_sizing_function {
            TrackSizingFunction::Flex(fr) => Some(*fr),
            _ => None,
        }
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
pub(crate) enum TrackSizingFunction<T: LayoutTreeNode> {
    /// Fixed length: `100px`, `50%`, etc.
    Fixed(DefLength<T::Length, T::LengthCustom>),

    /// Flexible length: `1fr`, `2fr`, etc.
    /// CSS Grid §7.2.4: <https://www.w3.org/TR/css-grid-1/#fr-unit>
    Flex(f32),

    /// Auto sizing: `auto`
    /// Represents max-content as max, min-content as min
    Auto,
}

impl<T: LayoutTreeNode> Debug for TrackSizingFunction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(length) => write!(f, "Fixed({:?})", length),
            Self::Flex(fr) => write!(f, "Flex({}fr)", fr),
            Self::Auto => write!(f, "Auto"),
        }
    }
}

/// A collection of grid tracks for one axis (rows or columns).
pub(crate) struct GridTracks<T: LayoutTreeNode> {
    /// The tracks in this axis
    tracks: Vec<GridTrack<T>>,
    /// Count of auto tracks (for distributing remaining space)
    auto_count: usize,
    /// Total flex factor (sum of all fr values)
    total_flex: f32,
}

impl<T: LayoutTreeNode> Debug for GridTracks<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridTracks {{ count: {}, auto_count: {}, total_flex: {} }}",
            self.tracks.len(),
            self.auto_count,
            self.total_flex
        )
    }
}

impl<T: LayoutTreeNode> GridTracks<T> {
    /// Create a new empty track collection.
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            auto_count: 0,
            total_flex: 0.0,
        }
    }

    /// Create tracks from a track list.
    pub fn from_track_list(
        track_count: usize,
        get_sizing_function: impl Fn(usize) -> TrackSizingFunction<T>,
    ) -> Self {
        let mut tracks = Vec::with_capacity(track_count);
        let mut auto_count = 0;
        let mut total_flex = 0.0;

        for i in 0..track_count {
            let sizing_fn = get_sizing_function(i);
            match &sizing_fn {
                TrackSizingFunction::Auto => auto_count += 1,
                TrackSizingFunction::Flex(fr) => total_flex += fr,
                _ => {}
            }
            tracks.push(GridTrack::from_single(sizing_fn));
        }

        Self {
            tracks,
            auto_count,
            total_flex,
        }
    }

    /// Get the number of tracks.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    /// Check if empty.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    /// Get a track by index.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&GridTrack<T>> {
        self.tracks.get(index)
    }

    /// Get a mutable track by index.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut GridTrack<T>> {
        self.tracks.get_mut(index)
    }

    /// Get the count of auto tracks.
    #[inline(always)]
    pub fn auto_count(&self) -> usize {
        self.auto_count
    }

    /// Get the total flex factor.
    #[inline(always)]
    pub fn total_flex(&self) -> f32 {
        self.total_flex
    }

    /// Mark a track as occupied.
    pub fn mark_occupied(&mut self, index: usize) {
        if let Some(track) = self.tracks.get_mut(index) {
            track.is_occupied = true;
        }
    }

    /// Ensure we have at least `count` tracks, adding implicit auto tracks as needed.
    pub fn ensure_count(&mut self, count: usize) {
        while self.tracks.len() < count {
            self.tracks.push(GridTrack::new(
                TrackSizingFunction::Auto,
                TrackSizingFunction::Auto,
            ));
            self.auto_count += 1;
        }
    }

    /// Initialize all track sizes.
    pub fn initialize(
        &mut self,
        available_space: OptionNum<T::Length>,
        resolve_length: impl Fn(
            &DefLength<T::Length, T::LengthCustom>,
            OptionNum<T::Length>,
        ) -> T::Length,
    ) {
        for track in &mut self.tracks {
            track.initialize(available_space, &resolve_length);
        }
    }

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

    /// Iterate over tracks.
    pub fn iter(&self) -> impl Iterator<Item = &GridTrack<T>> {
        self.tracks.iter()
    }

    /// Iterate mutably over tracks.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GridTrack<T>> {
        self.tracks.iter_mut()
    }

    /// Create GridTracks from computed sizes.
    ///
    /// This is used to create GridTracks after the initial track sizing
    /// has been computed, for use in maximize_tracks.
    ///
    /// - `sizes`: The computed base sizes for each track
    /// - `has_explicit`: Whether each track has an explicit (non-auto) size
    pub fn from_sizes(sizes: &[T::Length], has_explicit: &[bool]) -> Self {
        Self::from_sizes_with_fr(sizes, has_explicit, &[])
    }

    /// Create GridTracks from computed sizes with fr track info.
    ///
    /// - `sizes`: The computed base sizes for each track
    /// - `has_explicit`: Whether each track has an explicit (non-auto) size
    /// - `fr_values`: The fr value for each track (0.0 if not an fr track)
    pub fn from_sizes_with_fr(
        sizes: &[T::Length],
        has_explicit: &[bool],
        fr_values: &[f32],
    ) -> Self {
        let track_count = sizes.len();
        let mut tracks = Vec::with_capacity(track_count);
        let mut auto_count = 0;
        let mut total_flex = 0.0;

        for (i, &size) in sizes.iter().enumerate() {
            let fr_value = fr_values.get(i).copied().unwrap_or(0.0);
            let is_auto = !has_explicit.get(i).copied().unwrap_or(false);

            let sizing_fn = if fr_value > 0.0 {
                total_flex += fr_value;
                TrackSizingFunction::Flex(fr_value)
            } else if is_auto {
                auto_count += 1;
                TrackSizingFunction::Auto
            } else {
                TrackSizingFunction::Fixed(DefLength::Points(size))
            };

            let mut track = GridTrack::from_single(sizing_fn);
            track.base_size = size;
            // fr tracks and auto tracks have infinite growth limit
            track.growth_limit = if fr_value > 0.0 || is_auto {
                None
            } else {
                Some(size)
            };
            tracks.push(track);
        }

        Self {
            tracks,
            auto_count,
            total_flex,
        }
    }

    /// Maximize tracks by distributing free space.
    ///
    /// CSS Grid §11.6: Maximize Tracks
    /// <https://www.w3.org/TR/css-grid-1/#algo-grow-tracks>
    ///
    /// If the free space is positive, distribute it equally to the base sizes
    /// of all tracks with infinite growth limits.
    ///
    /// ## Optimization
    ///
    /// Single-pass implementation: collect eligible track indices during counting,
    /// then directly update those tracks without re-filtering.
    pub fn maximize(&mut self, free_space: T::Length) {
        if free_space <= T::Length::zero() {
            return;
        }

        // Single pass: collect indices of tracks with infinite growth limit (non-flex)
        let mut infinite_indices: Vec<usize> = Vec::with_capacity(self.tracks.len());
        for (i, track) in self.tracks.iter().enumerate() {
            if track.has_infinite_growth_limit() && !track.is_flexible() {
                infinite_indices.push(i);
            }
        }

        if infinite_indices.is_empty() {
            return;
        }

        // Distribute free space equally among collected tracks
        let space_per_track = free_space.div_f32(infinite_indices.len() as f32);

        for i in infinite_indices {
            self.tracks[i].base_size += space_per_track;
        }
    }

    /// Expand flexible tracks (fr units) with iterative algorithm.
    ///
    /// CSS Grid §11.7: Expand Flexible Tracks
    /// <https://www.w3.org/TR/css-grid-1/#algo-flex-tracks>
    ///
    /// The algorithm iteratively calculates fr track sizes:
    /// 1. Find the hypothetical fr size = free_space / total_flex
    /// 2. If any fr track's size < its base_size, treat it as inflexible
    /// 3. Repeat until stable
    ///
    /// This ensures fr tracks respect their minimum sizes (base_size).
    pub fn expand_flexible_tracks(&mut self, free_space: T::Length) {
        if free_space <= T::Length::zero() || self.total_flex == 0.0 {
            return;
        }

        // Track which fr tracks are still flexible (not clamped to base_size)
        let mut is_flexible: Vec<bool> = self.tracks.iter().map(|t| t.is_flexible()).collect();

        let mut remaining_space = free_space;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10; // Prevent infinite loops

        loop {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                break;
            }

            // Calculate total flex of still-flexible tracks
            let active_flex: f32 = self
                .tracks
                .iter()
                .enumerate()
                .filter(|(i, _)| is_flexible[*i])
                .map(|(_, t)| t.flex_factor().unwrap_or(0.0))
                .sum();

            if active_flex <= 0.0 {
                break;
            }

            // Calculate hypothetical fr size
            let hypothetical_fr_size = remaining_space.div_f32(active_flex);

            // Check if any track needs to be frozen (hypothetical size < base_size)
            let mut any_frozen = false;

            for (i, track) in self.tracks.iter().enumerate() {
                if is_flexible[i] {
                    if let Some(flex_factor) = track.flex_factor() {
                        let hypothetical_size = hypothetical_fr_size.mul_f32(flex_factor);
                        if hypothetical_size < track.base_size {
                            // Freeze this track at its base_size
                            is_flexible[i] = false;
                            remaining_space -= track.base_size;
                            any_frozen = true;
                        }
                    }
                }
            }

            if !any_frozen {
                // No tracks were frozen, we can apply the final sizes
                for (i, track) in self.tracks.iter_mut().enumerate() {
                    if is_flexible[i] {
                        if let Some(flex_factor) = track.flex_factor() {
                            track.base_size = hypothetical_fr_size.mul_f32(flex_factor);
                        }
                    }
                }
                break;
            }
        }
    }

    /// Stretch auto tracks to fill remaining space.
    ///
    /// CSS Grid §11.8: Stretch auto Tracks
    /// <https://www.w3.org/TR/css-grid-1/#algo-stretch>
    ///
    /// This step only applies when the content-distribution property
    /// (align-content/justify-content) is `normal`.
    ///
    /// It distributes any remaining free space equally among auto tracks,
    /// similar to maximize but specifically for the stretch behavior.
    pub fn stretch_auto_tracks(&mut self, free_space: T::Length) {
        if free_space <= T::Length::zero() {
            return;
        }

        // Count auto tracks (tracks with infinite growth limit that aren't fr)
        let auto_count = self
            .tracks
            .iter()
            .filter(|track| track.is_auto() && track.has_infinite_growth_limit())
            .count();

        if auto_count == 0 {
            return;
        }

        // Distribute free space equally among auto tracks
        let space_per_track = free_space.div_f32(auto_count as f32);

        for track in &mut self.tracks {
            if track.is_auto() && track.has_infinite_growth_limit() {
                track.base_size += space_per_track;
            }
        }
    }

    /// Get the total flex factor of all fr tracks.
    pub fn total_flex_factor(&self) -> f32 {
        self.total_flex
    }

    /// Check if there are any flexible tracks.
    pub fn has_flexible_tracks(&self) -> bool {
        self.total_flex > 0.0
    }
}
