use std::fmt::Debug;

use crate::{DefLength, LayoutTreeNode};

#[derive(Clone, PartialEq)]
pub(crate) struct GridItem<'a, T: LayoutTreeNode> {
    pub row: usize,
    pub column: usize,
    pub node: &'a T,
    pub idx: usize,
    pub track_block_size: DefLength<T::Length, T::LengthCustom>,
    pub track_inline_size: DefLength<T::Length, T::LengthCustom>,
}

impl<'a, T: LayoutTreeNode> Debug for GridItem<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridItem {{ row: {}, column: {}, idx: {}, track_block_size: {:?}, track_inline_size: {:?} }}",
            self.row, self.column, self.idx, self.track_block_size, self.track_inline_size
        )
    }
}

impl<'a, T: LayoutTreeNode> GridItem<'a, T> {
    pub fn new(node: &'a T, idx: usize, row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            node,
            idx,
            track_block_size: DefLength::Auto,
            track_inline_size: DefLength::Auto,
        }
    }
    pub(crate) fn update_track_block_size(
        &mut self,
        track_block_size: DefLength<T::Length, T::LengthCustom>,
    ) {
        self.track_block_size = track_block_size;
    }

    pub(crate) fn update_track_inline_size(
        &mut self,
        track_inline_size: DefLength<T::Length, T::LengthCustom>,
    ) {
        self.track_inline_size = track_inline_size;
    }
}
