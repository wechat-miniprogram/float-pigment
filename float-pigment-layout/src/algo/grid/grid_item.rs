//! Grid Item Structures
//!
//! CSS Grid Layout Module Level 1 - §6 Grid Items
//! <https://www.w3.org/TR/css-grid-1/#grid-items>
//!
//! This module defines the data structures for grid items during the
//! placement and layout phases.

use std::fmt::Debug;

use crate::{
    algo::grid::track_size::TrackSize, DefLength, EdgeOption, LayoutTreeNode, OptionNum, Size,
};

/// Grid item with computed layout information.
///
/// CSS Grid §6.2: Grid Item Sizing
/// <https://www.w3.org/TR/css-grid-1/#grid-item-sizing>
///
/// This structure stores the final layout information for a grid item
/// after the track sizing algorithm has been applied, including:
/// - The item's position in the grid (row, column)
/// - The item's margin box
/// - The CSS-specified size
/// - The track size (cell size in the grid)
/// - Min-content and computed sizes for intrinsic sizing
#[derive(Clone, PartialEq)]
pub(crate) struct GridLayoutItem<'a, T: LayoutTreeNode> {
    /// The item's row index in the grid (0-based)
    pub(crate) row: usize,
    /// The item's column index in the grid (0-based)
    pub(crate) column: usize,
    /// Reference to the DOM node
    pub(crate) node: &'a T,
    /// The item's margin (top, right, bottom, left)
    pub(crate) margin: EdgeOption<T::Length>,
    /// The CSS-specified width/height (may be auto)
    pub(crate) css_size: Size<OptionNum<T::Length>>,
    /// The track size (grid cell size allocated to this item)
    pub(crate) track_size: Size<OptionNum<T::Length>>,
    /// The item's min-content size (used for intrinsic track sizing)
    pub(crate) min_content_size: Option<Size<T::Length>>,
    /// The item's final computed size
    pub(crate) computed_size: Size<T::Length>,
}

impl<'a, T: LayoutTreeNode> GridLayoutItem<'a, T> {
    pub(crate) fn new(
        row: usize,
        column: usize,
        node: &'a T,
        margin: EdgeOption<T::Length>,
        css_size: Size<OptionNum<T::Length>>,
        track_size: Size<OptionNum<T::Length>>,
    ) -> Self {
        Self {
            row,
            column,
            node,
            margin,
            css_size,
            track_size,
            min_content_size: None,
            computed_size: Size::zero(),
        }
    }

    #[inline(always)]
    pub(crate) fn row(&self) -> usize {
        self.row
    }

    #[inline(always)]
    pub(crate) fn column(&self) -> usize {
        self.column
    }

    pub(crate) fn track_inline_size(&self) -> OptionNum<T::Length> {
        self.track_size.width.clone()
    }

    pub(crate) fn track_block_size(&self) -> OptionNum<T::Length> {
        self.track_size.height.clone()
    }

    pub(crate) fn min_content_size(&self) -> Option<&Size<T::Length>> {
        self.min_content_size.as_ref()
    }

    pub(crate) fn set_min_content_size(&mut self, min_content_size: Size<T::Length>) {
        self.min_content_size = Some(min_content_size);
    }

    pub(crate) fn computed_size(&self) -> Size<T::Length> {
        self.computed_size
    }

    pub(crate) fn set_computed_size(&mut self, computed_size: Size<T::Length>) {
        self.computed_size = computed_size;
    }
}

/// Grid item during the placement phase.
///
/// CSS Grid §8: Grid Item Placement
/// <https://www.w3.org/TR/css-grid-1/#placement>
///
/// This structure represents a grid item during the auto-placement phase,
/// before track sizes have been fully resolved. It stores:
/// - The item's position in the grid (row, column)
/// - The track sizing function for its row/column
#[derive(Clone, PartialEq)]
pub(crate) struct GridItem<'a, T: LayoutTreeNode> {
    /// The item's row index in the grid (0-based)
    row: usize,
    /// The item's column index in the grid (0-based)
    column: usize,
    /// Original DOM index for maintaining source order
    origin_index: usize,
    /// Reference to the DOM node
    pub(crate) node: &'a T,
    /// Track sizing function for the block (row) axis
    pub(crate) track_block_size: TrackSize<T>,
    /// Track sizing function for the inline (column) axis
    pub(crate) track_inline_size: TrackSize<T>,
}

impl<'a, T: LayoutTreeNode> Debug for GridItem<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridItem {{ row: {}, column: {}, origin_index: {}, track_block_size: {:?}, track_inline_size: {:?} }}",
            self.row, self.column, self.origin_index, self.track_block_size, self.track_inline_size
        )
    }
}

impl<'a, T: LayoutTreeNode> GridItem<'a, T> {
    pub fn new(node: &'a T, origin_index: usize, row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            node,
            origin_index,
            track_block_size: TrackSize::Original(DefLength::Auto),
            track_inline_size: TrackSize::Original(DefLength::Auto),
        }
    }

    /// Get the item's row index.
    #[inline(always)]
    pub(crate) fn row(&self) -> usize {
        self.row
    }

    /// Get the item's column index.
    #[inline(always)]
    pub(crate) fn column(&self) -> usize {
        self.column
    }

    pub(crate) fn update_track_block_size(&mut self, track_block_size: TrackSize<T>) {
        self.track_block_size = track_block_size;
    }

    pub(crate) fn update_track_inline_size(&mut self, track_inline_size: TrackSize<T>) {
        self.track_inline_size = track_inline_size;
    }

    pub(crate) fn fixed_track_block_size(&self) -> Option<&OptionNum<T::Length>> {
        match &self.track_block_size {
            TrackSize::Fixed(size) => Some(size),
            _ => None,
        }
    }

    pub(crate) fn fixed_track_inline_size(&self) -> Option<&OptionNum<T::Length>> {
        match &self.track_inline_size {
            TrackSize::Fixed(size) => Some(size),
            _ => None,
        }
    }
}
