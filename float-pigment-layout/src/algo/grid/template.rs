//! Grid Template Initialization
//!
//! CSS Grid §7.1: https://www.w3.org/TR/css-grid-1/#explicit-grids
//!
//! Utilities for parsing and initializing grid template track lists.

use alloc::vec::Vec;

use crate::{LayoutGridTemplate, LayoutTrackListItem, LayoutTreeNode};

/// Initialize a track list from grid-template-rows/columns.
///
/// Filters the track list to extract only track size items.
pub(crate) fn initialize_track_list<'a, T: LayoutTreeNode>(
    grid_template: &'a LayoutGridTemplate<T::Length, T::LengthCustom>,
) -> Vec<&'a LayoutTrackListItem<T::Length, T::LengthCustom>> {
    match grid_template {
        LayoutGridTemplate::TrackList(track_list) => track_list
            .iter()
            .filter(|item| matches!(item, LayoutTrackListItem::TrackSize(_)))
            .collect(),
        _ => Vec::with_capacity(0),
    }
}
