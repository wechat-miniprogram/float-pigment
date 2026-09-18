//! Abstract inline/block axis coordinates.
//!
//! Line breaking and justification are performed in these coordinates,
//! and mapped to physical coordinates at the end according to
//! `writing-mode` and `text-orientation`.

use float_pigment_css::length_num::LengthNum;

use crate::geometry::{Point, Size, Vector};
use crate::model::TextOrientation;
use float_pigment_css::typing::WritingMode;

/// A point in abstract axis coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct AxialPoint<L: LengthNum> {
    /// The offset along the baseline.
    pub(crate) inline: L,
    /// The offset perpendicular to the baseline, positive in the block direction.
    pub(crate) block: L,
}

/// A size in abstract axis coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct AxialSize<L: LengthNum> {
    /// The size along the baseline.
    pub(crate) inline: L,
    /// The size perpendicular to the baseline.
    pub(crate) block: L,
}

/// The mapping between abstract axis coordinates and physical coordinates.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AxisMapping {
    pub(crate) writing_mode: WritingMode,
    pub(crate) text_orientation: TextOrientation,
}

#[allow(unused)]
impl AxisMapping {
    pub(crate) fn to_physical_point<L: LengthNum>(&self, p: AxialPoint<L>) -> Point<L> {
        todo!()
    }

    pub(crate) fn to_physical_size<L: LengthNum>(&self, s: AxialSize<L>) -> Size<L> {
        todo!()
    }

    pub(crate) fn to_physical_vector<L: LengthNum>(&self, v: AxialPoint<L>) -> Vector<L> {
        todo!()
    }
}
