//! Grid Template Initialization
//!
//! CSS Grid §7.1: https://www.w3.org/TR/css-grid-1/#explicit-grids
//!
//! Utilities for parsing and initializing grid template track lists.

use alloc::vec::Vec;

use crate::{DefLength, LayoutGridTemplate, LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode};

/// Information about an initialized track list.
pub(crate) struct InitializedTrackListInfo<'a, T: LayoutTreeNode> {
    pub list: Vec<&'a LayoutTrackListItem<T::Length, T::LengthCustom>>,
    pub auto_count: usize,
    pub total_fr: f32,
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

/// Initialize a track list from grid-template-rows/columns.
///
/// Parses the track list and extracts:
/// - The list of track size items
/// - Count of auto tracks
/// - Total fr value for flexible tracks
pub(crate) fn initialize_track_list<'a, T: LayoutTreeNode>(
    grid_template: &'a LayoutGridTemplate<T::Length, T::LengthCustom>,
) -> InitializedTrackListInfo<'a, T> {
    let mut track_auto_count = 0;
    let mut total_fr: f32 = 0.0;
    let track_list = grid_template_track_iterator::<T>(grid_template, |item| {
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
