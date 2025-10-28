use std::fmt::Debug;

use crate::{
    algo::grid::track_size::TrackSize, DefLength, EdgeOption, LayoutTreeNode, OptionNum, Size,
};

#[derive(Clone, PartialEq)]
pub(crate) struct GridLayoutItem<'a, T: LayoutTreeNode> {
    pub(crate) node: &'a T,
    pub(crate) margin: EdgeOption<T::Length>,
    pub(crate) css_size: Size<OptionNum<T::Length>>,
    pub(crate) track_size: Size<OptionNum<T::Length>>,
}

impl<'a, T: LayoutTreeNode> GridLayoutItem<'a, T> {
    pub(crate) fn new(
        node: &'a T,
        margin: EdgeOption<T::Length>,
        css_size: Size<OptionNum<T::Length>>,
        track_size: Size<OptionNum<T::Length>>,
    ) -> Self {
        Self {
            node,
            margin,
            css_size,
            track_size,
        }
    }

    pub(crate) fn track_inline_size(&self) -> OptionNum<T::Length> {
        self.track_size.width.clone()
    }

    pub(crate) fn track_block_size(&self) -> OptionNum<T::Length> {
        self.track_size.height.clone()
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct GridItem<'a, T: LayoutTreeNode> {
    row: usize,
    column: usize,
    origin_index: usize,
    pub(crate) node: &'a T,
    pub(crate) track_block_size: TrackSize<T>,
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
