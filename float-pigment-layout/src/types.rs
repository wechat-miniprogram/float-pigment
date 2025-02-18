use core::{fmt::Display, ops::Deref};

use euclid::UnknownUnit;
use style::typing::WritingMode;

use crate::LayoutTreeNode;
pub(crate) use style::length_num::{length_sum, LengthNum};

/// Position with size.
pub type Rect<L> = euclid::Rect<L, UnknownUnit>;

/// Size.
pub type Size<L> = euclid::Size2D<L, UnknownUnit>;

/// Size, but each value can be `None` for undetermined.
pub type OptionSize<L> = euclid::Size2D<OptionNum<L>, UnknownUnit>;

/// 2D Vector.
pub type Vector<L> = euclid::Vector2D<L, euclid::UnknownUnit>;

/// Position.
pub type Point<L> = euclid::Point2D<L, euclid::UnknownUnit>;

pub(crate) trait PointGetter<T> {
    fn main_axis(&self, dir: AxisDirection) -> T;
    fn cross_axis(&self, dir: AxisDirection) -> T;
}

impl<L: LengthNum> PointGetter<L> for Point<L> {
    fn main_axis(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.x,
            AxisDirection::Vertical => self.y,
        }
    }
    fn cross_axis(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.y,
            AxisDirection::Vertical => self.x,
        }
    }
}

pub(crate) trait SizeGetter<T: Copy> {
    fn main_size(&self, dir: AxisDirection) -> T;
    fn cross_size(&self, dir: AxisDirection) -> T;
}

pub(crate) trait SizeSetter<T> {
    fn set_main_size(&mut self, dir: AxisDirection, value: T);
    fn set_cross_size(&mut self, dir: AxisDirection, value: T);
}

impl<T: Copy> SizeGetter<T> for euclid::Size2D<T, UnknownUnit> {
    #[inline]
    fn main_size(&self, dir: AxisDirection) -> T {
        match dir {
            AxisDirection::Horizontal => self.width,
            AxisDirection::Vertical => self.height,
        }
    }

    #[inline]
    fn cross_size(&self, dir: AxisDirection) -> T {
        match dir {
            AxisDirection::Horizontal => self.height,
            AxisDirection::Vertical => self.width,
        }
    }
}

impl<T> SizeSetter<T> for euclid::Size2D<T, UnknownUnit> {
    #[inline]
    fn set_main_size(&mut self, dir: AxisDirection, value: T) {
        match dir {
            AxisDirection::Horizontal => self.width = value,
            AxisDirection::Vertical => self.height = value,
        }
    }

    #[inline]
    fn set_cross_size(&mut self, dir: AxisDirection, value: T) {
        match dir {
            AxisDirection::Horizontal => self.height = value,
            AxisDirection::Vertical => self.width = value,
        }
    }
}

pub(crate) trait SizeProxy<T> {
    fn new_with_dir(dir: AxisDirection, main_size: T, cross_size: T) -> Self;
}

impl<T> SizeProxy<T> for euclid::Size2D<T, UnknownUnit> {
    #[inline(always)]
    fn new_with_dir(dir: AxisDirection, main_size: T, cross_size: T) -> Self {
        let (width, height) = match dir {
            AxisDirection::Horizontal => (main_size, cross_size),
            AxisDirection::Vertical => (cross_size, main_size),
        };
        Self::new(width, height)
    }
}

pub(crate) trait OrZero<L: LengthNum>: Sized {
    fn or_zero(&self) -> Size<L>;
}

impl<L: LengthNum> OrZero<L> for OptionSize<L> {
    #[inline(always)]
    fn or_zero(&self) -> Size<L> {
        Size::new(self.width.or_zero(), self.height.or_zero())
    }
}

pub(crate) trait VectorGetter<T> {
    #[allow(unused)]
    fn main_axis(&self, dir: AxisDirection) -> T;
    fn cross_axis(&self, dir: AxisDirection) -> T;
}

impl<L: LengthNum> VectorGetter<L> for Vector<L> {
    #[inline]
    fn main_axis(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.x,
            AxisDirection::Vertical => self.y,
        }
    }
    #[inline]
    fn cross_axis(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.y,
            AxisDirection::Vertical => self.x,
        }
    }
}

pub(crate) trait VectorSetter<T> {
    fn set_main_axis(&mut self, dir: AxisDirection, value: T);
    fn set_cross_axis(&mut self, dir: AxisDirection, value: T);
}

impl<L: LengthNum> VectorSetter<L> for Vector<L> {
    #[inline(always)]
    fn set_main_axis(&mut self, dir: AxisDirection, value: L) {
        match dir {
            AxisDirection::Horizontal => self.x = value,
            AxisDirection::Vertical => self.y = value,
        }
    }
    #[inline(always)]
    fn set_cross_axis(&mut self, dir: AxisDirection, value: L) {
        match dir {
            AxisDirection::Horizontal => self.y = value,
            AxisDirection::Vertical => self.x = value,
        }
    }
}

pub(crate) trait VectorProxy<T> {
    fn new_with_dir(dir: AxisDirection, main_axis: T, cross_axis: T) -> Self;
}

impl<L: LengthNum> VectorProxy<L> for Vector<L> {
    fn new_with_dir(dir: AxisDirection, main_axis: L, cross_axis: L) -> Self {
        let (x, y) = match dir {
            AxisDirection::Horizontal => (main_axis, cross_axis),
            AxisDirection::Vertical => (cross_axis, main_axis),
        };
        Self::new(x, y)
    }
}

/// A length type that can be undefined or auto.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefLength<L: LengthNum, T: PartialEq = i32> {
    /// The length is undetermined.
    Undefined,

    /// The length is auto.
    Auto,

    /// A fixed value.
    Points(L),

    /// A ratio.
    Percent(f32),

    /// Custom length value.
    ///
    /// Will be resolved by `LayoutTreeNode::resolve_custom_length`.
    Custom(T),
}

impl<L: LengthNum, T: PartialEq + Display> Display for DefLength<L, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Undefined => write!(f, "Undefined"),
            Self::Auto => write!(f, "Auto"),
            Self::Points(x) => write!(f, "Points({})", L::to_f32(*x)),
            Self::Percent(x) => write!(f, "Percent({})", *x),
            Self::Custom(x) => write!(f, "Custom({})", *x),
        }
    }
}

impl<L: LengthNum, T: PartialEq> Default for DefLength<L, T> {
    fn default() -> Self {
        Self::Undefined
    }
}

impl<L: LengthNum, T: PartialEq> DefLength<L, T> {
    pub(crate) fn resolve<N: LayoutTreeNode<Length = L, LengthCustom = T>>(
        &self,
        parent: OptionNum<L>,
        node: &N,
    ) -> OptionNum<L> {
        match self {
            Self::Undefined => OptionNum::none(),
            Self::Auto => OptionNum::none(),
            Self::Points(x) => OptionNum::some(*x),
            Self::Percent(x) => parent * *x,
            Self::Custom(x) => {
                OptionNum::some(node.resolve_custom_length(x, parent.unwrap_or(L::zero())))
            }
        }
    }

    pub(crate) fn resolve_with_auto<N: LayoutTreeNode<Length = L, LengthCustom = T>>(
        &self,
        parent: OptionNum<L>,
        node: &N,
    ) -> OptionNum<L> {
        match self {
            Self::Undefined => OptionNum::some(L::zero()),
            Self::Auto => OptionNum::none(),
            Self::Points(x) => OptionNum::some(*x),
            Self::Percent(x) => parent * *x,
            Self::Custom(x) => {
                OptionNum::some(node.resolve_custom_length(x, parent.unwrap_or(L::zero())))
            }
        }
    }

    pub(crate) fn resolve_num<N: LayoutTreeNode<Length = L, LengthCustom = T>>(
        &self,
        parent: L,
        node: &N,
    ) -> OptionNum<L> {
        match self {
            Self::Undefined => OptionNum::none(),
            Self::Auto => OptionNum::none(),
            Self::Points(x) => OptionNum::some(*x),
            Self::Percent(x) => OptionNum::some(parent.mul_f32(*x)),
            Self::Custom(x) => OptionNum::some(node.resolve_custom_length(x, parent)),
        }
    }
}

/// A number or undetermined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptionNum<L>(Option<L>);

impl<L: LengthNum> OptionNum<L> {
    /// Convert to a hashable value.
    pub fn to_hashable(&self) -> OptionNum<L::Hashable> {
        match self.0 {
            None => OptionNum(None),
            Some(x) => OptionNum(Some(x.to_hashable())),
        }
    }

    /// New undetermined value.
    #[inline]
    pub fn none() -> Self {
        Self(None)
    }

    /// New number.
    #[inline]
    pub fn some(v: L) -> Self {
        Self(Some(v))
    }

    /// New zero value.
    #[inline]
    pub fn zero() -> Self {
        Self(Some(L::zero()))
    }

    /// Return `true` for undetermined value.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Return `true` for number.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Convert to a number with `Option`.
    #[inline]
    pub fn val(&self) -> Option<L> {
        self.0
    }

    /// Unwrap to a number, default to `rhs`.
    #[inline]
    pub fn unwrap_or(self, rhs: L) -> L {
        self.0.unwrap_or(rhs)
    }

    /// If `self` is undetermined, return `rhs`.
    #[inline]
    pub fn or(self, rhs: Self) -> Self {
        Self(self.0.or(rhs.0))
    }

    /// Unwrap to a number, default to zero.
    #[inline]
    pub fn or_zero(self) -> L {
        self.0.unwrap_or_else(L::zero)
    }

    /// Map the number.
    #[inline]
    pub fn map(self, f: impl FnOnce(L) -> L) -> Self {
        Self(self.0.map(f))
    }
}

impl<L: LengthNum> core::ops::Add<L> for OptionNum<L> {
    type Output = Self;

    fn add(self, rhs: L) -> Self {
        self.map(|x| x + rhs)
    }
}

impl<L: LengthNum> core::ops::Sub<L> for OptionNum<L> {
    type Output = Self;

    fn sub(self, rhs: L) -> Self {
        self.map(|x| x - rhs)
    }
}

impl<L: LengthNum> core::ops::Mul<f32> for OptionNum<L> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        self.map(|x| x.mul_f32(rhs))
    }
}

impl<L: LengthNum> core::ops::Mul<i32> for OptionNum<L> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        self.map(|x| x.mul_i32(rhs))
    }
}

impl<L: LengthNum> core::ops::Add<Self> for OptionNum<L> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match rhs.val() {
            Some(x) => self + x,
            None => self,
        }
    }
}

impl<L: LengthNum> core::ops::Sub<Self> for OptionNum<L> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match rhs.val() {
            Some(x) => self - x,
            None => self,
        }
    }
}

pub(crate) trait MinMax {
    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;
}

impl<L: LengthNum> MinMax for L {
    fn min(self, rhs: Self) -> Self {
        if self < rhs {
            self
        } else {
            rhs
        }
    }

    fn max(self, rhs: Self) -> Self {
        if self > rhs {
            self
        } else {
            rhs
        }
    }
}

pub(crate) trait MaybeMinMax<In, Out> {
    fn maybe_min(self, rhs: In) -> Out;
    fn maybe_max(self, rhs: In) -> Out;
}

impl<L: LengthNum> MaybeMinMax<OptionNum<L>, L> for L {
    fn maybe_min(self, rhs: OptionNum<L>) -> L {
        match rhs.val() {
            None => self,
            Some(x) => self.min(x),
        }
    }

    fn maybe_max(self, rhs: OptionNum<L>) -> L {
        match rhs.val() {
            None => self,
            Some(x) => self.max(x),
        }
    }
}

impl<L: LengthNum> MaybeMinMax<Self, Self> for OptionNum<L> {
    fn maybe_min(self, rhs: Self) -> Self {
        match self.val() {
            None => OptionNum::none(),
            Some(x) => match rhs.val() {
                None => self,
                Some(y) => OptionNum::some(x.min(y)),
            },
        }
    }

    fn maybe_max(self, rhs: Self) -> Self {
        match self.val() {
            None => OptionNum::none(),
            Some(x) => match rhs.val() {
                None => self,
                Some(y) => OptionNum::some(x.max(y)),
            },
        }
    }
}

impl<L: LengthNum> MaybeMinMax<L, Self> for OptionNum<L> {
    fn maybe_min(self, rhs: L) -> OptionNum<L> {
        match self.val() {
            None => OptionNum::none(),
            Some(x) => OptionNum::some(x.min(rhs)),
        }
    }

    fn maybe_max(self, rhs: L) -> OptionNum<L> {
        match self.val() {
            None => OptionNum::none(),
            Some(x) => OptionNum::some(x.max(rhs)),
        }
    }
}

impl<L: LengthNum> MaybeMinMax<OptionSize<L>, Size<L>> for Size<L> {
    fn maybe_min(self, rhs: OptionSize<L>) -> Size<L> {
        let width = match rhs.width.val() {
            None => self.width,
            Some(x) => self.width.min(x),
        };
        let height = match rhs.height.val() {
            None => self.height,
            Some(x) => self.height.min(x),
        };
        Size::new(width, height)
    }

    fn maybe_max(self, rhs: OptionSize<L>) -> Size<L> {
        let width = match rhs.width.val() {
            None => self.width,
            Some(x) => self.width.max(x),
        };
        let height = match rhs.height.val() {
            None => self.height,
            Some(x) => self.height.max(x),
        };
        Size::new(width, height)
    }
}

/// Four edge lengths.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge<L: LengthNum> {
    pub left: L,
    pub right: L,
    pub top: L,
    pub bottom: L,
}

impl<L: LengthNum> Edge<L> {
    /// New zero-length edges.
    #[inline(always)]
    pub fn zero() -> Self {
        Self {
            left: L::zero(),
            right: L::zero(),
            top: L::zero(),
            bottom: L::zero(),
        }
    }

    /// Get the sum of horizontal lengths.
    #[inline(always)]
    pub fn horizontal(&self) -> L {
        self.left + self.right
    }

    /// Get the sum of vertical lengths.
    #[inline(always)]
    pub fn vertical(&self) -> L {
        self.top + self.bottom
    }

    #[inline(always)]
    pub(crate) fn main_axis_sum(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Vertical => self.vertical(),
            AxisDirection::Horizontal => self.horizontal(),
        }
    }

    #[inline(always)]
    pub(crate) fn cross_axis_sum(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Vertical => self.horizontal(),
            AxisDirection::Horizontal => self.vertical(),
        }
    }

    #[inline(always)]
    pub(crate) fn main_axis_start(&self, dir: AxisDirection, rev: AxisReverse) -> L {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.left,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.right,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.top,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.bottom,
        }
    }

    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn main_axis_end(&self, dir: AxisDirection, rev: AxisReverse) -> L {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.right,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.left,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.bottom,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.top,
        }
    }

    #[inline(always)]
    pub(crate) fn cross_axis_start(&self, dir: AxisDirection, rev: AxisReverse) -> L {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.top,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.bottom,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.left,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.right,
        }
    }

    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn cross_axis_end(&self, dir: AxisDirection, rev: AxisReverse) -> L {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.bottom,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.top,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.right,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.left,
        }
    }
}

impl<L: LengthNum> core::ops::Add<Edge<L>> for Edge<L> {
    type Output = Self;

    fn add(self, rhs: Edge<L>) -> Self {
        Self {
            left: self.left + rhs.left,
            right: self.right + rhs.right,
            top: self.top + rhs.top,
            bottom: self.bottom + rhs.bottom,
        }
    }
}

impl<L: LengthNum> core::ops::Sub<Edge<L>> for Edge<L> {
    type Output = Self;
    fn sub(self, rhs: Edge<L>) -> Self {
        Self {
            left: self.left - rhs.left,
            right: self.right - rhs.right,
            top: self.top - rhs.top,
            bottom: self.bottom - rhs.bottom,
        }
    }
}

/// Four edge lengths, each edge can be undetermined.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeOption<L: LengthNum> {
    pub left: OptionNum<L>,
    pub right: OptionNum<L>,
    pub top: OptionNum<L>,
    pub bottom: OptionNum<L>,
}

impl<L: LengthNum> EdgeOption<L> {
    /// Unwrap to `Edge`, default to zero.
    #[inline(always)]
    pub fn or_zero(&self) -> Edge<L> {
        Edge {
            left: self.left.or_zero(),
            right: self.right.or_zero(),
            top: self.top.or_zero(),
            bottom: self.bottom.or_zero(),
        }
    }

    /// Get the sum of horizontal lengths, default to zero.
    #[inline(always)]
    pub fn horizontal(&self) -> L {
        self.left.or_zero() + self.right.or_zero()
    }

    /// Get the sum of vertical lengths, default to zero.
    #[inline(always)]
    pub fn vertical(&self) -> L {
        self.top.or_zero() + self.bottom.or_zero()
    }

    pub(crate) fn is_left_right_either_none(&self) -> bool {
        self.left.is_none() || self.right.is_none()
    }

    pub(crate) fn is_top_bottom_either_none(&self) -> bool {
        self.top.is_none() || self.bottom.is_none()
    }

    #[inline(always)]
    pub(crate) fn main_axis_sum(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.horizontal(),
            AxisDirection::Vertical => self.vertical(),
        }
    }

    #[inline(always)]
    pub(crate) fn cross_axis_sum(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.vertical(),
            AxisDirection::Vertical => self.horizontal(),
        }
    }

    #[inline(always)]
    pub(crate) fn main_axis_start(&self, dir: AxisDirection, rev: AxisReverse) -> OptionNum<L> {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.left,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.right,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.top,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.bottom,
        }
    }

    #[inline(always)]
    pub(crate) fn set_main_axis_start(
        &mut self,
        dir: AxisDirection,
        rev: AxisReverse,
        value: OptionNum<L>,
    ) {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.left = value,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.right = value,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.top = value,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.bottom = value,
        }
    }

    #[inline(always)]
    pub(crate) fn main_axis_end(&self, dir: AxisDirection, rev: AxisReverse) -> OptionNum<L> {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.right,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.left,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.bottom,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.top,
        }
    }

    #[inline(always)]
    pub(crate) fn set_main_axis_end(
        &mut self,
        dir: AxisDirection,
        rev: AxisReverse,
        value: OptionNum<L>,
    ) {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.right = value,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.left = value,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.bottom = value,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.top = value,
        }
    }

    #[inline(always)]
    pub(crate) fn cross_axis_start(&self, dir: AxisDirection, rev: AxisReverse) -> OptionNum<L> {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.top,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.bottom,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.left,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.right,
        }
    }

    #[inline(always)]
    pub(crate) fn set_cross_axis_start(
        &mut self,
        dir: AxisDirection,
        rev: AxisReverse,
        value: OptionNum<L>,
    ) {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.top = value,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.bottom = value,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.left = value,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.right = value,
        }
    }

    #[inline(always)]
    pub(crate) fn cross_axis_end(&self, dir: AxisDirection, rev: AxisReverse) -> OptionNum<L> {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.bottom,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.top,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.right,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.left,
        }
    }

    #[inline(always)]
    pub(crate) fn set_cross_axis_end(
        &mut self,
        dir: AxisDirection,
        rev: AxisReverse,
        value: OptionNum<L>,
    ) {
        match (dir, rev) {
            (AxisDirection::Horizontal, AxisReverse::NotReversed) => self.bottom = value,
            (AxisDirection::Horizontal, AxisReverse::Reversed) => self.top = value,
            (AxisDirection::Vertical, AxisReverse::NotReversed) => self.right = value,
            (AxisDirection::Vertical, AxisReverse::Reversed) => self.left = value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum AxisDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) enum AxisReverse {
    NotReversed,
    Reversed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AxisInfo {
    pub(crate) dir: AxisDirection,
    pub(crate) main_dir_rev: AxisReverse,
    pub(crate) cross_dir_rev: AxisReverse,
}

impl AxisInfo {
    pub(crate) fn from_writing_mode(writing_mode: WritingMode) -> Self {
        let (dir, main_dir_rev, cross_dir_rev) = match writing_mode {
            WritingMode::HorizontalTb => (
                AxisDirection::Vertical,
                AxisReverse::NotReversed,
                AxisReverse::NotReversed,
            ),
            WritingMode::VerticalLr => (
                AxisDirection::Horizontal,
                AxisReverse::NotReversed,
                AxisReverse::NotReversed,
            ),
            WritingMode::VerticalRl => (
                AxisDirection::Horizontal,
                AxisReverse::Reversed,
                AxisReverse::NotReversed,
            ),
        };
        Self {
            dir,
            main_dir_rev,
            cross_dir_rev,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MinMaxSize<L: LengthNum> {
    pub(crate) min_width: OptionNum<L>,
    pub(crate) max_width: OptionNum<L>,
    pub(crate) min_height: OptionNum<L>,
    pub(crate) max_height: OptionNum<L>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MinMaxLimit<L: LengthNum> {
    pub(crate) min_width: L,
    pub(crate) max_width: OptionNum<L>,
    pub(crate) min_height: L,
    pub(crate) max_height: OptionNum<L>,
}

impl<L: LengthNum> MinMaxLimit<L> {
    pub(crate) fn normalized_size(&self, v: OptionSize<L>) -> Normalized<OptionSize<L>> {
        Normalized(OptionSize::new(
            v.width
                .map(|x| x.maybe_min(self.max_width).max(self.min_width)),
            v.height
                .map(|x| x.maybe_min(self.max_height).max(self.min_height)),
        ))
    }

    pub(crate) fn width(&self, x: L) -> L {
        x.maybe_min(self.max_width).max(self.min_width)
    }

    pub(crate) fn height(&self, x: L) -> L {
        x.maybe_min(self.max_height).max(self.min_height)
    }

    pub(crate) fn maybe(&self) -> MinMaxLimitMaybe<L> {
        MinMaxLimitMaybe(self)
    }

    #[inline(always)]
    pub(crate) fn main_size(&self, x: L, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.width(x),
            AxisDirection::Vertical => self.height(x),
        }
    }

    #[inline(always)]
    pub(crate) fn cross_size(&self, x: L, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.height(x),
            AxisDirection::Vertical => self.width(x),
        }
    }

    #[inline(always)]
    pub(crate) fn min_main_size(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.min_width,
            AxisDirection::Vertical => self.min_height,
        }
    }

    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn max_main_size(&self, dir: AxisDirection) -> OptionNum<L> {
        match dir {
            AxisDirection::Horizontal => self.max_width,
            AxisDirection::Vertical => self.max_height,
        }
    }

    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn min_cross_size(&self, dir: AxisDirection) -> L {
        match dir {
            AxisDirection::Horizontal => self.min_height,
            AxisDirection::Vertical => self.min_width,
        }
    }

    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn max_cross_size(&self, dir: AxisDirection) -> OptionNum<L> {
        match dir {
            AxisDirection::Horizontal => self.max_height,
            AxisDirection::Vertical => self.max_width,
        }
    }
}

pub(crate) struct MinMaxLimitMaybe<'a, L: LengthNum>(&'a MinMaxLimit<L>);

impl<'a, L: LengthNum> MinMaxLimitMaybe<'a, L> {
    pub(crate) fn width(&self, v: OptionNum<L>) -> OptionNum<L> {
        v.map(|x| x.maybe_min(self.0.max_width).max(self.0.min_width))
    }

    pub(crate) fn height(&self, v: OptionNum<L>) -> OptionNum<L> {
        v.map(|x| x.maybe_min(self.0.max_height).max(self.0.min_height))
    }

    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn main_size(&self, dir: AxisDirection, v: OptionNum<L>) -> OptionNum<L> {
        match dir {
            AxisDirection::Horizontal => self.width(v),
            AxisDirection::Vertical => self.height(v),
        }
    }

    #[inline(always)]
    pub(crate) fn cross_size(&self, dir: AxisDirection, v: OptionNum<L>) -> OptionNum<L> {
        match dir {
            AxisDirection::Horizontal => self.height(v),
            AxisDirection::Vertical => self.width(v),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub(crate) struct Normalized<T>(pub(crate) T);

impl<T> Deref for Normalized<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[inline(always)]
pub(crate) fn size_to_option<L: LengthNum>(size: Size<L>) -> OptionSize<L> {
    OptionSize::new(OptionNum::some(size.width), OptionNum::some(size.height))
}

#[cfg(test)]
mod test {
    use crate::{AxisDirection, Normalized, OptionSize, Size, SizeGetter, SizeSetter};

    #[test]
    fn option_size_get_main_size() {
        let os = OptionSize::new(crate::OptionNum::some(10.), crate::OptionNum::some(20.));
        let size = os.main_size(AxisDirection::Horizontal);
        // let size = main_size(target, dir)
        assert_eq!(size.val(), Some(10.));
        let size = os.main_size(AxisDirection::Vertical);
        assert_eq!(size.val(), Some(20.));
    }

    #[test]
    fn option_size_get_cross_size() {
        let os = OptionSize::new(crate::OptionNum::some(10.), crate::OptionNum::some(20.));
        let size = os.cross_size(AxisDirection::Horizontal);
        assert_eq!(size.val(), Some(20.));
        let size = os.cross_size(AxisDirection::Vertical);
        assert_eq!(size.val(), Some(10.));
    }

    #[test]
    fn normalized_option_size_get_size() {
        let os = OptionSize::new(crate::OptionNum::some(10.), crate::OptionNum::some(20.));
        let normalized = Normalized(os);
        let size = normalized.main_size(AxisDirection::Horizontal);
        assert_eq!(size.val(), Some(10.));
        let size = normalized.main_size(AxisDirection::Vertical);
        assert_eq!(size.val(), Some(20.));
    }

    #[test]
    fn size_get_size() {
        let s = Size::new(10.0_f32, 20.0_f32);
        let size = s.main_size(AxisDirection::Horizontal);
        assert_eq!(size, 10.);
        let size = s.main_size(AxisDirection::Vertical);
        assert_eq!(size, 20.);
    }

    #[test]
    fn size_set_size() {
        let mut s = Size::new(10.0_f32, 20.0_f32);
        s.set_main_size(AxisDirection::Horizontal, 20.);
        assert_eq!(s.main_size(AxisDirection::Horizontal), 20.);
        s.set_cross_size(AxisDirection::Horizontal, 30.);
        assert_eq!(s.cross_size(AxisDirection::Horizontal), 30.);
    }
}
