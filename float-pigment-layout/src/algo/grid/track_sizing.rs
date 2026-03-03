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
        .fold(L::zero(), |acc, t| acc + t.base_size.unwrap_or(L::zero()));

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
                tracks[i].base_size = Some(tracks[i].min_content);
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
                tracks[i].base_size = Some(fr_size);
                tracks[i].has_explicit = true;
            }
            break;
        }
    }
}

/// A single track's sizing result after the track sizing algorithm.
///
/// Combines the computed size with metadata needed by subsequent phases
/// (§11.6 Maximize Tracks, §11.8 Stretch auto Tracks).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TrackSizingResult<L: LengthNum> {
    /// The track's final computed base size (§11.4–§11.7).
    pub size: L,
    /// The flex factor if this is an fr track (§11.7); 0.0 otherwise.
    pub fr_value: f32,
    /// Whether this track has an `auto` sizing function (§7.2).
    pub is_auto: bool,
    /// Whether this track has a `min-content` sizing function (§7.2).
    pub is_min_content: bool,
    /// Whether this track has a `max-content` sizing function (§7.2).
    pub is_max_content: bool,
    /// The track's growth limit; `None` represents infinity (§11.4).
    pub growth_limit: Option<L>,
}

/// The type of track sizing function for intrinsic sizing (§11.5).
///
/// CSS Grid §7.2: Track Sizing Functions
/// <https://www.w3.org/TR/css-grid-1/#track-sizing>
///
/// This determines how the track participates in intrinsic sizing:
/// - Fixed: size is predetermined, no intrinsic sizing needed
/// - Auto: min = min-content, max = max-content (§11.5)
/// - MinContent: min = min-content, max = min-content (§11.5)
/// - MaxContent: min = min-content, max = max-content (§11.5)
/// - Fr: handled separately in §11.7
#[derive(Clone, Copy, Debug, PartialEq)]
enum IntrinsicTrackType {
    Fixed,
    Auto,
    MinContent,
    MaxContent,
    Fr,
}

struct TrackInfo<L: LengthNum> {
    /// The track's computed base size (§11.4).
    base_size: Option<L>,
    /// The track's growth limit (§11.4).
    /// `None` represents infinity.
    growth_limit: Option<L>,
    has_explicit: bool,
    is_fr: bool,
    fr_value: f32,
    min_content: L,
    /// The type of track for intrinsic sizing purposes (§11.5).
    track_type: IntrinsicTrackType,
}

impl<L: LengthNum + Copy + Default> Default for TrackInfo<L> {
    fn default() -> Self {
        Self {
            base_size: None,
            growth_limit: None,
            has_explicit: false,
            is_fr: false,
            fr_value: 0.0,
            min_content: L::default(),
            track_type: IntrinsicTrackType::Auto,
        }
    }
}

/// Determine IntrinsicTrackType from a LayoutTrackSize.
///
/// CSS Grid §7.2: Track Sizing Functions
/// <https://www.w3.org/TR/css-grid-1/#track-sizing>
fn classify_track_type<L: LengthNum, C: PartialEq + Clone>(
    track_size: &LayoutTrackSize<L, C>,
) -> IntrinsicTrackType {
    match track_size {
        LayoutTrackSize::Length(DefLength::Points(_) | DefLength::Percent(_)) => {
            IntrinsicTrackType::Fixed
        }
        LayoutTrackSize::Length(_) => IntrinsicTrackType::Auto,
        LayoutTrackSize::Fr(_) => IntrinsicTrackType::Fr,
        LayoutTrackSize::MinContent => IntrinsicTrackType::MinContent,
        LayoutTrackSize::MaxContent => IntrinsicTrackType::MaxContent,
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ComputedTrackSizes<T: LayoutTreeNode> {
    pub rows: Vec<TrackSizingResult<T::Length>>,
    pub columns: Vec<TrackSizingResult<T::Length>>,
}

/// Compute track sizes based on item content.
///
/// Implements the core parts of the Track Sizing Algorithm:
/// - CSS Grid §11.5 (Resolve Intrinsic Track Sizes):
///   <https://www.w3.org/TR/css-grid-1/#algo-content>
/// - CSS Grid §11.7 (Expand Flexible Tracks):
///   <https://www.w3.org/TR/css-grid-1/#algo-flex-tracks>
///
/// Phase 1: Resolve intrinsic track sizes (§11.5)
///   Step 2 - For tracks with intrinsic min sizing function (auto, min-content,
///   max-content): set base_size = item's min-content contribution
///   Step 4 - For tracks with intrinsic max sizing function:
///   set growth_limit = item's max-content contribution
///   (auto/max-content → max-content; min-content → min-content)
///
///   For fixed tracks: use the specified size for both base_size and growth_limit
///   For fr tracks: collect min-content as freeze threshold
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
    column_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    row_track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    available_grid_space: OptionSize<T::Length>,
    column_total_fr: f32,
    row_total_fr: f32,
    grid_auto_columns: &LayoutGridAuto<T::Length, T::LengthCustom>,
    grid_auto_rows: &LayoutGridAuto<T::Length, T::LengthCustom>,
) -> ComputedTrackSizes<T> {
    let row_count = grid_layout_matrix.row_count();
    let column_count = grid_layout_matrix.column_count();

    let explicit_column_count = column_track_list.len();
    let explicit_row_count = row_track_list.len();

    // Initialize column track info
    // CSS Grid §7.6: Implicit tracks use grid-auto-columns
    let mut columns: Vec<TrackInfo<T::Length>> = (0..column_count)
        .map(|i| {
            let mut info = TrackInfo {
                base_size: None,
                growth_limit: None,
                has_explicit: false,
                is_fr: false,
                fr_value: 0.0,
                min_content: T::Length::zero(),
                track_type: IntrinsicTrackType::Auto,
            };
            if let Some(LayoutTrackListItem::TrackSize(track_size)) = column_track_list.get(i) {
                info.track_type = classify_track_type(track_size);
                if let LayoutTrackSize::Fr(fr_value) = track_size {
                    info.is_fr = true;
                    info.fr_value = *fr_value;
                }
            } else if i >= explicit_column_count {
                // Implicit track - use grid-auto-columns (§7.6)
                let implicit_index = i - explicit_column_count;
                let implicit_track_size = grid_auto_columns.get(implicit_index);
                info.track_type = classify_track_type(&implicit_track_size);
                if let LayoutTrackSize::Fr(fr_value) = implicit_track_size {
                    info.is_fr = true;
                    info.fr_value = fr_value;
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
                base_size: None,
                growth_limit: None,
                has_explicit: false,
                is_fr: false,
                fr_value: 0.0,
                min_content: T::Length::zero(),
                track_type: IntrinsicTrackType::Auto,
            };
            if let Some(LayoutTrackListItem::TrackSize(track_size)) = row_track_list.get(i) {
                info.track_type = classify_track_type(track_size);
                if let LayoutTrackSize::Fr(fr_value) = track_size {
                    info.is_fr = true;
                    info.fr_value = *fr_value;
                }
            } else if i >= explicit_row_count {
                // Implicit track - use grid-auto-rows (§7.6)
                let implicit_index = i - explicit_row_count;
                let implicit_track_size = grid_auto_rows.get(implicit_index);
                info.track_type = classify_track_type(&implicit_track_size);
                if let LayoutTrackSize::Fr(fr_value) = implicit_track_size {
                    info.is_fr = true;
                    info.fr_value = fr_value;
                }
            }
            info
        })
        .collect();

    // ═══════════════════════════════════════════════════════════════════════
    // Phase 1: Resolve intrinsic track sizes (§11.5)
    // CSS Grid §11.5: Resolve Intrinsic Track Sizes
    // https://www.w3.org/TR/css-grid-1/#algo-content
    //
    // §11.5 Step 2: For tracks with an intrinsic min track sizing function,
    //   increase base_size to the item's min-content contribution.
    //
    // §11.5 Step 4: For tracks with an intrinsic max track sizing function,
    //   increase growth_limit to the item's max-content contribution.
    //   - auto / max-content → growth_limit = max-content contribution
    //   - min-content → growth_limit = min-content contribution
    //
    // For fixed tracks: use the explicit track size (§11.4)
    // For fr tracks: collect min-content for freeze threshold (§11.7)
    // ═══════════════════════════════════════════════════════════════════════

    for item in grid_layout_matrix.items() {
        let row = item.row();
        let column = item.column();

        let css_width = item.css_size.width;
        let css_height = item.css_size.height;
        let min_content_size = item.min_content_size().copied().unwrap_or(Size::zero());

        // §11.5 Step 2: min-content contribution for base_size
        // If item has a CSS width, use it; otherwise use min-content
        let min_content_width = if css_width.is_some() {
            css_width.val().unwrap().max(min_content_size.width)
        } else {
            min_content_size.width
        };
        let outer_min_content_width = min_content_width + item.margin.horizontal();

        let min_content_height = if css_height.is_some() {
            css_height.val().unwrap().max(min_content_size.height)
        } else {
            min_content_size.height
        };
        let outer_min_content_height = min_content_height + item.margin.vertical();

        // §11.5 Step 4: max-content contribution for growth_limit
        //
        // For columns: use max_content_size (unconstrained layout) to get the
        // true max-content width. This is the size the item would take with
        // infinite available space.
        //
        // For rows: use computed_size (constrained by resolved column width).
        // Per §11.5, row contributions are computed using the item's resolved
        // column size, so the constrained layout result is correct.
        let max_content_size = item.max_content_size().copied();
        let max_content_width = if css_width.is_some() {
            css_width.val().unwrap()
        } else {
            max_content_size.map_or(item.computed_size().width, |s| s.width)
        };
        let outer_max_content_width = max_content_width + item.margin.horizontal();

        let max_content_height = if css_height.is_some() {
            css_height.val().unwrap()
        } else {
            item.computed_size().height
        };
        let outer_max_content_height = max_content_height + item.margin.vertical();

        // Intrinsic sizing invariants from CSS Sizing:
        // min-content contribution must not exceed max-content contribution.
        // Clamp defensively because engine-internal min-content measurement can be
        // over-constrained in some contexts.
        let effective_min_content_width = outer_min_content_width.min(outer_max_content_width);
        let effective_min_content_height = outer_min_content_height.min(outer_max_content_height);

        // Always update min_content for all tracks (used as fr freeze threshold §11.7)
        columns[column].min_content = columns[column].min_content.max(outer_min_content_width);
        rows[row].min_content = rows[row].min_content.max(outer_min_content_height);

        // Handle column sizing for non-fr tracks
        if !columns[column].is_fr {
            match columns[column].track_type {
                IntrinsicTrackType::Fixed => {
                    // Fixed track: use the specified size (§11.4)
                    let track_size = if column < explicit_column_count {
                        item.track_inline_size()
                    } else {
                        let implicit_index = column - explicit_column_count;
                        match grid_auto_columns.get(implicit_index) {
                            LayoutTrackSize::Length(def_len) => {
                                def_len.resolve(available_grid_space.width, item.node)
                            }
                            _ => OptionNum::none(),
                        }
                    };

                    if track_size.is_some() {
                        columns[column].has_explicit = true;
                        let track_width = track_size.val().unwrap();
                        columns[column].base_size = Some(
                            columns[column]
                                .base_size
                                .map(|s| s.max(track_width))
                                .unwrap_or(track_width),
                        );
                        // Fixed track: growth_limit = base_size (§11.4)
                        columns[column].growth_limit = columns[column].base_size;
                    }
                }
                IntrinsicTrackType::Auto => {
                    // §11.5 Step 2: auto min track sizing function
                    // → increase base_size to min-content contribution.
                    columns[column].base_size = Some(
                        columns[column]
                            .base_size
                            .map(|s| s.max(effective_min_content_width))
                            .unwrap_or(effective_min_content_width),
                    );
                    // §11.4: auto max → growth_limit initialized to infinity (None).
                    // §11.5 Step 4: increase(infinity, max-content) = infinity.
                    // The "increase" operation is max(current, new); since
                    // infinity > any finite value, it stays infinity.
                    // This allows §11.6 Maximize to distribute free space
                    // into auto tracks without freezing them.
                    columns[column].growth_limit = None;
                }
                IntrinsicTrackType::MinContent => {
                    // §11.5 Step 2: base_size = min-content contribution
                    columns[column].base_size = Some(
                        columns[column]
                            .base_size
                            .map(|s| s.max(effective_min_content_width))
                            .unwrap_or(effective_min_content_width),
                    );
                    // §11.5 Step 4: min-content max → growth_limit = min-content
                    columns[column].growth_limit = Some(
                        columns[column]
                            .growth_limit
                            .map(|s| s.max(effective_min_content_width))
                            .unwrap_or(effective_min_content_width),
                    );
                }
                IntrinsicTrackType::MaxContent => {
                    // §11.5 Step 2: base_size = min-content contribution
                    columns[column].base_size = Some(
                        columns[column]
                            .base_size
                            .map(|s| s.max(effective_min_content_width))
                            .unwrap_or(effective_min_content_width),
                    );
                    // §11.5 Step 4: max-content max → growth_limit = max-content
                    columns[column].growth_limit = Some(
                        columns[column]
                            .growth_limit
                            .map(|s| s.max(outer_max_content_width))
                            .unwrap_or(outer_max_content_width),
                    );
                }
                IntrinsicTrackType::Fr => {
                    // Fr tracks handled by resolve_fr_track_sizes
                }
            }
        }

        // Handle row sizing for non-fr tracks
        if !rows[row].is_fr {
            match rows[row].track_type {
                IntrinsicTrackType::Fixed => {
                    // Fixed track: use the specified size (§11.4)
                    let track_size = if row < explicit_row_count {
                        item.track_block_size()
                    } else {
                        let implicit_index = row - explicit_row_count;
                        match grid_auto_rows.get(implicit_index) {
                            LayoutTrackSize::Length(def_len) => {
                                def_len.resolve(available_grid_space.height, item.node)
                            }
                            _ => OptionNum::none(),
                        }
                    };

                    if track_size.is_some() {
                        rows[row].has_explicit = true;
                        let track_height = track_size.val().unwrap();
                        rows[row].base_size = Some(
                            rows[row]
                                .base_size
                                .map(|s| s.max(track_height))
                                .unwrap_or(track_height),
                        );
                        // Fixed track: growth_limit = base_size (§11.4)
                        rows[row].growth_limit = rows[row].base_size;
                    }
                }
                IntrinsicTrackType::Auto => {
                    // §11.5 Step 2: auto min track sizing function
                    // → increase base_size to min-content contribution.
                    rows[row].base_size = Some(
                        rows[row]
                            .base_size
                            .map(|s| s.max(effective_min_content_height))
                            .unwrap_or(effective_min_content_height),
                    );
                    // §11.4: auto max → growth_limit initialized to infinity (None).
                    // §11.5 Step 4: increase(infinity, max-content) = infinity.
                    // See column Auto comment above for rationale.
                    rows[row].growth_limit = None;
                }
                IntrinsicTrackType::MinContent => {
                    // §11.5 Step 2: base_size = min-content contribution
                    rows[row].base_size = Some(
                        rows[row]
                            .base_size
                            .map(|s| s.max(effective_min_content_height))
                            .unwrap_or(effective_min_content_height),
                    );
                    // §11.5 Step 4: min-content max → growth_limit = min-content
                    rows[row].growth_limit = Some(
                        rows[row]
                            .growth_limit
                            .map(|s| s.max(effective_min_content_height))
                            .unwrap_or(effective_min_content_height),
                    );
                }
                IntrinsicTrackType::MaxContent => {
                    // §11.5 Step 2: base_size = min-content contribution
                    rows[row].base_size = Some(
                        rows[row]
                            .base_size
                            .map(|s| s.max(effective_min_content_height))
                            .unwrap_or(effective_min_content_height),
                    );
                    // §11.5 Step 4: max-content max → growth_limit = max-content
                    rows[row].growth_limit = Some(
                        rows[row]
                            .growth_limit
                            .map(|s| s.max(outer_max_content_height))
                            .unwrap_or(outer_max_content_height),
                    );
                }
                IntrinsicTrackType::Fr => {
                    // Fr tracks handled by resolve_fr_track_sizes
                }
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
        .map(|c| c.base_size.unwrap_or(T::Length::zero()))
        .collect();
    let row_sizes: Vec<T::Length> = rows
        .iter()
        .map(|r| r.base_size.unwrap_or(T::Length::zero()))
        .collect();

    // Update items with final track sizes
    for item in grid_layout_matrix.items_mut() {
        let row = item.row();
        let column = item.column();
        item.track_size.width = OptionNum::some(column_sizes[column]);
        item.track_size.height = OptionNum::some(row_sizes[row]);
    }

    let column_items: Vec<TrackSizingResult<T::Length>> = columns
        .iter()
        .zip(column_sizes.iter())
        .map(|(c, &size)| TrackSizingResult {
            size,
            fr_value: c.fr_value,
            is_auto: c.track_type == IntrinsicTrackType::Auto,
            is_min_content: c.track_type == IntrinsicTrackType::MinContent,
            is_max_content: c.track_type == IntrinsicTrackType::MaxContent,
            growth_limit: c.growth_limit,
        })
        .collect();
    let row_items: Vec<TrackSizingResult<T::Length>> = rows
        .iter()
        .zip(row_sizes.iter())
        .map(|(r, &size)| TrackSizingResult {
            size,
            fr_value: r.fr_value,
            is_auto: r.track_type == IntrinsicTrackType::Auto,
            is_min_content: r.track_type == IntrinsicTrackType::MinContent,
            is_max_content: r.track_type == IntrinsicTrackType::MaxContent,
            growth_limit: r.growth_limit,
        })
        .collect();

    ComputedTrackSizes {
        columns: column_items,
        rows: row_items,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a non-fr TrackInfo with a fixed base_size.
    fn fixed_track(base: f32) -> TrackInfo<f32> {
        TrackInfo {
            base_size: Some(base),
            growth_limit: Some(base),
            has_explicit: true,
            is_fr: false,
            fr_value: 0.0,
            min_content: 0.0,
            track_type: IntrinsicTrackType::Fixed,
        }
    }

    /// Helper: create an fr TrackInfo.
    fn fr_track(fr: f32, min_content: f32) -> TrackInfo<f32> {
        TrackInfo {
            base_size: None,
            growth_limit: None,
            has_explicit: false,
            is_fr: true,
            fr_value: fr,
            min_content,
            track_type: IntrinsicTrackType::Fr,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // §11.7: Expand Flexible Tracks (resolve_fr_track_sizes)
    // <https://www.w3.org/TR/css-grid-1/#algo-flex-tracks>
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn fr_single_track_takes_all_remaining_space() {
        // CSS Grid §11.7: A single 1fr track should consume all remaining
        // space after non-flexible tracks are subtracted.
        //
        // Layout: grid-template-columns: 100px 1fr
        // Available: 400px → remaining = 400 - 100 = 300px
        let mut tracks = vec![fixed_track(100.0), fr_track(1.0, 0.0)];
        resolve_fr_track_sizes(&mut tracks, 1.0, OptionNum::some(400.0));
        assert_eq!(tracks[0].base_size, Some(100.0));
        assert_eq!(tracks[1].base_size, Some(300.0));
    }

    #[test]
    fn fr_multiple_tracks_distribute_proportionally() {
        // CSS Grid §11.7: Multiple fr tracks share remaining space
        // proportionally to their flex factor.
        //
        // Layout: grid-template-columns: 1fr 2fr 1fr
        // Available: 400px → each fr unit = 400 / 4 = 100px
        let mut tracks = vec![fr_track(1.0, 0.0), fr_track(2.0, 0.0), fr_track(1.0, 0.0)];
        resolve_fr_track_sizes(&mut tracks, 4.0, OptionNum::some(400.0));
        assert_eq!(tracks[0].base_size, Some(100.0));
        assert_eq!(tracks[1].base_size, Some(200.0));
        assert_eq!(tracks[2].base_size, Some(100.0));
    }

    #[test]
    fn fr_with_fixed_tracks_subtracts_fixed_first() {
        // CSS Grid §11.7: Fixed tracks are subtracted from available space
        // before distributing to fr tracks.
        //
        // Layout: grid-template-columns: 50px 1fr 50px 2fr
        // Available: 350px → remaining = 350 - 50 - 50 = 250px
        // fr unit = 250 / 3 = 83.333...
        let mut tracks = vec![
            fixed_track(50.0),
            fr_track(1.0, 0.0),
            fixed_track(50.0),
            fr_track(2.0, 0.0),
        ];
        resolve_fr_track_sizes(&mut tracks, 3.0, OptionNum::some(350.0));
        assert_eq!(tracks[0].base_size, Some(50.0)); // fixed
        let fr_unit = 250.0 / 3.0;
        assert!((tracks[1].base_size.unwrap() - fr_unit).abs() < 0.01);
        assert_eq!(tracks[2].base_size, Some(50.0)); // fixed
        assert!((tracks[3].base_size.unwrap() - fr_unit * 2.0).abs() < 0.01);
    }

    #[test]
    fn fr_freeze_at_min_content() {
        // CSS Grid §11.7 iterative freeze: If a hypothetical fr size would
        // make a track smaller than its min-content, that track is frozen at
        // min-content and the remaining space is redistributed.
        //
        // Layout: grid-template-columns: 1fr 1fr
        // Available: 100px, track[0].min_content = 80px
        // Round 1: hypothetical = 100/2 = 50 < 80 → freeze track[0] at 80
        // Round 2: remaining = 100 - 80 = 20 → track[1] = 20
        let mut tracks = vec![fr_track(1.0, 80.0), fr_track(1.0, 0.0)];
        resolve_fr_track_sizes(&mut tracks, 2.0, OptionNum::some(100.0));
        assert_eq!(tracks[0].base_size, Some(80.0));
        assert_eq!(tracks[1].base_size, Some(20.0));
    }

    #[test]
    fn fr_all_frozen_at_min_content() {
        // CSS Grid §11.7: When all fr tracks are frozen because their
        // hypothetical size < min-content, each gets its min-content.
        //
        // Layout: grid-template-columns: 1fr 1fr
        // Available: 100px, min-content = 60 each
        // Round 1: hypothetical = 100/2 = 50 < 60 → both frozen
        let mut tracks = vec![fr_track(1.0, 60.0), fr_track(1.0, 60.0)];
        resolve_fr_track_sizes(&mut tracks, 2.0, OptionNum::some(100.0));
        assert_eq!(tracks[0].base_size, Some(60.0));
        assert_eq!(tracks[1].base_size, Some(60.0));
    }

    #[test]
    fn fr_no_available_space_returns_zero() {
        // CSS Grid §11.7: When available space is zero, fr tracks get zero.
        let mut tracks = vec![fr_track(1.0, 0.0), fr_track(2.0, 0.0)];
        resolve_fr_track_sizes(&mut tracks, 3.0, OptionNum::some(0.0));
        assert_eq!(tracks[0].base_size, Some(0.0));
        assert_eq!(tracks[1].base_size, Some(0.0));
    }

    #[test]
    fn fr_indefinite_container_skipped() {
        // CSS Grid §11.7: When the available space is indefinite,
        // fr tracks cannot be resolved and are left untouched.
        let mut tracks = vec![fr_track(1.0, 0.0)];
        resolve_fr_track_sizes(&mut tracks, 1.0, OptionNum::none());
        assert_eq!(tracks[0].base_size, None); // unchanged
    }

    #[test]
    fn fr_zero_total_fr_is_noop() {
        // Edge case: total_fr = 0 should be a no-op.
        let mut tracks = vec![fixed_track(100.0)];
        resolve_fr_track_sizes(&mut tracks, 0.0, OptionNum::some(400.0));
        assert_eq!(tracks[0].base_size, Some(100.0)); // unchanged
    }

    #[test]
    fn fr_fixed_consumes_all_space_fr_gets_zero() {
        // CSS Grid §11.7: When non-fr tracks consume all available space,
        // remaining = 0, fr tracks get zero.
        //
        // Layout: grid-template-columns: 300px 1fr
        // Available: 300px → remaining = 0
        let mut tracks = vec![fixed_track(300.0), fr_track(1.0, 0.0)];
        resolve_fr_track_sizes(&mut tracks, 1.0, OptionNum::some(300.0));
        assert_eq!(tracks[0].base_size, Some(300.0));
        assert_eq!(tracks[1].base_size, Some(0.0));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // IntrinsicTrackType classification
    // <https://www.w3.org/TR/css-grid-1/#track-sizing>
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn classify_track_type_fixed_points() {
        // §7.2: Fixed track sizing function (e.g. 100px)
        let ts: LayoutTrackSize<f32, i32> = LayoutTrackSize::Length(DefLength::Points(100.0));
        assert_eq!(
            classify_track_type::<f32, i32>(&ts),
            IntrinsicTrackType::Fixed
        );
    }

    #[test]
    fn classify_track_type_fixed_percent() {
        // §7.2: Percentage is also a fixed sizing function
        let ts: LayoutTrackSize<f32, i32> = LayoutTrackSize::Length(DefLength::Percent(0.5));
        assert_eq!(
            classify_track_type::<f32, i32>(&ts),
            IntrinsicTrackType::Fixed
        );
    }

    #[test]
    fn classify_track_type_auto() {
        // §7.2: Auto maps to min-content/max-content intrinsic sizing
        let ts: LayoutTrackSize<f32, i32> = LayoutTrackSize::Length(DefLength::Auto);
        assert_eq!(
            classify_track_type::<f32, i32>(&ts),
            IntrinsicTrackType::Auto
        );
    }

    #[test]
    fn classify_track_type_fr() {
        // §7.2: Flexible sizing function (fr unit)
        let ts: LayoutTrackSize<f32, i32> = LayoutTrackSize::Fr(1.0);
        assert_eq!(classify_track_type::<f32, i32>(&ts), IntrinsicTrackType::Fr);
    }

    #[test]
    fn classify_track_type_min_content() {
        // §7.2: min-content intrinsic sizing function
        let ts: LayoutTrackSize<f32, i32> = LayoutTrackSize::MinContent;
        assert_eq!(
            classify_track_type::<f32, i32>(&ts),
            IntrinsicTrackType::MinContent
        );
    }

    #[test]
    fn classify_track_type_max_content() {
        // §7.2: max-content intrinsic sizing function
        let ts: LayoutTrackSize<f32, i32> = LayoutTrackSize::MaxContent;
        assert_eq!(
            classify_track_type::<f32, i32>(&ts),
            IntrinsicTrackType::MaxContent
        );
    }
}
