use euclid::UnknownUnit;

/// Position.
pub type Point<L> = euclid::Point2D<L, UnknownUnit>;

/// Size.
pub type Size<L> = euclid::Size2D<L, UnknownUnit>;

/// Position with size.
pub type Rect<L> = euclid::Rect<L, UnknownUnit>;

/// 2D vector.
pub type Vector<L> = euclid::Vector2D<L, UnknownUnit>;
