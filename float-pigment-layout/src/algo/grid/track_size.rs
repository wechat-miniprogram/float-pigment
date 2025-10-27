use crate::{
    algo::grid::{GridFlow, GridMatrix},
    DefLength, LayoutTrackListItem, LayoutTrackSize, LayoutTreeNode, OptionNum,
};
use float_pigment_css::{length_num::LengthNum, num_traits::Zero};
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub(crate) enum TrackSize<T: LayoutTreeNode> {
    Original(DefLength<T::Length, T::LengthCustom>),
    Fixed(OptionNum<T::Length>),
}

impl<T: LayoutTreeNode> Debug for TrackSize<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Original(length) => write!(f, "Original({:?})", length),
            Self::Fixed(length) => write!(f, "Fixed({:?})", length),
        }
    }
}

pub(crate) fn apply_track_size<'a, T: LayoutTreeNode>(
    track_list: &[&LayoutTrackListItem<T::Length, T::LengthCustom>],
    flow: GridFlow,
    grid_matrix: &mut GridMatrix<'a, T>,
    parent_node: &'a T,
    current_flow_parent_size: OptionNum<T::Length>,
    available_grid_space: &mut OptionNum<T::Length>,
) {
    let mut total_specified_track_size = T::Length::zero();
    let (secondary_count, auto_count) = match flow {
        GridFlow::Row => (grid_matrix.column_count(), grid_matrix.row_auto_count()),
        GridFlow::Column => (grid_matrix.row_count(), grid_matrix.column_auto_count()),
    };

    // handle specified track size
    track_list
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            matches!(
                item,
                LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(_))
            )
        })
        .for_each(|(idx, track_item)| {
            let mut current_size = T::Length::zero();
            for index in 0..secondary_count {
                let grid_item = match flow {
                    GridFlow::Row => grid_matrix.get_item_mut(idx, index),
                    GridFlow::Column => grid_matrix.get_item_mut(index, idx),
                };
                if let Some(grid_item) = grid_item {
                    if !grid_item.is_unoccupied() {
                        if let LayoutTrackListItem::TrackSize(LayoutTrackSize::Length(length)) =
                            track_item
                        {
                            current_size = length
                                .resolve(current_flow_parent_size, parent_node)
                                .or_zero();
                            match flow {
                                GridFlow::Row => grid_item
                                    .get_auto_placed_mut_unchecked()
                                    .update_track_block_size(TrackSize::Original(length.clone())),
                                GridFlow::Column => grid_item
                                    .get_auto_placed_mut_unchecked()
                                    .update_track_inline_size(TrackSize::Original(length.clone())),
                            }
                        }
                    }
                }
            }
            total_specified_track_size += current_size
        });

    if available_grid_space.is_none() && total_specified_track_size > T::Length::zero() {
        *available_grid_space = OptionNum::some(total_specified_track_size);
    }

    // handle auto track size
    grid_matrix.iter_mut().for_each(|grid_item| {
        if grid_item.is_unoccupied() {
            return;
        }
        let grid_item = grid_item.get_auto_placed_mut_unchecked();
        if let TrackSize::Original(track_size) = match flow {
            GridFlow::Row => &grid_item.track_block_size,
            GridFlow::Column => &grid_item.track_inline_size,
        } {
            let fixed_track_size = match track_size {
                DefLength::Auto => {
                    let track_size = if available_grid_space.is_some()
                        && auto_count > 0
                        && available_grid_space.val().unwrap() > total_specified_track_size
                    {
                        TrackSize::Fixed(OptionNum::some(
                            (available_grid_space.val().unwrap() - total_specified_track_size)
                                .div_f32(auto_count as f32),
                        ))
                    } else {
                        TrackSize::Fixed(OptionNum::none())
                    };
                    track_size
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
                _ => {
                    unreachable!()
                }
            };
            match flow {
                GridFlow::Row => grid_item.update_track_block_size(fixed_track_size),
                GridFlow::Column => grid_item.update_track_inline_size(fixed_track_size),
            }
        }
    });
}
