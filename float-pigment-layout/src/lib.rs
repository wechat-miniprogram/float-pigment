//! Layout engine with common CSS block and flexbox support.
//!
//! Note: it is not a full web-compatible layout engine because it supports a subset of web layout algorithms.
//!
//! Supported layout strategies:
//!
//! * `display: block`
//! * `display: flex`
//! * `display: none`
//! * `position: absolute`
//! * with an optional external *text layout engine*:
//!   * `display: inline`
//!   * `display: inline-block`
//!   * `display: inline-flex`
//!
//! ### Basic Usages
//!
//! This crate does not construct node trees itself. You should firstly:
//!
//! * write a node tree struct that implements `LayoutTreeNode`, each node should owns a `LayoutNode` inside it;
//! * or use the *float-pigment-forest* crate (the `float_pigment_forest::node::Node` is a good implementation).
//!
//! Each tree node has a corresponding `LayoutNode`.
//! Calls the `Layoutnode::update` or `Layoutnode::update_with_containing_size` of tree **root** node every time you need a new layout result.
//! (The all results in the tree will be updated when `Layoutnode::update*` is called on the tree root.)
//! Then you can read any result in any node with `LayoutNode::result*` and `LayoutNode::computed_style`.
//!
//! When any property of any node has been updated, calls the `LayoutNode::mark_dirty`.
//! The next `Layoutnode::update*` call on the tree root will carefully read the new properties and update the results.
//!
//! ### About Text Layout
//!
//! Text layout means to compose text glyphs and other structures (images, inline-blocks, etc.) in lines.
//! It is a complex problem and deeply coupled with system environment interfaces.
//!
//! This crate does not solve text layout problems. Thus by default it does not support `display: inline` and similar features.
//! However, you are informed the inline layout parts so that you can implement a text layout engine to handle them.

#![warn(missing_docs)]
#![no_std]

#[macro_use]
extern crate alloc;
#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[cfg(target_os = "android")]
extern crate android_logger;

#[cfg(target_os = "android")]
use log::LevelFilter;

#[allow(unused_imports)]
use alloc::{boxed::Box, vec::Vec};
use core::{
    cell::{RefCell, RefMut},
    fmt,
};

use float_pigment_css::typing::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, Direction, Display, FlexDirection, FlexWrap,
    JustifyContent, Position, TextAlign, WritingMode,
};

mod algo;
mod cache;
mod special_positioned;
mod types;
mod unit;

pub(crate) use cache::*;
pub use special_positioned::is_independent_positioning;
pub(crate) use special_positioned::*;
pub use types::*;
pub(crate) use unit::*;

/// A type to get screen size.
#[allow(missing_docs)]
pub trait ScreenQuery<L: LengthNum> {
    fn screen_width(&self) -> L;
    fn screen_height(&self) -> L;
}

/// The main tree node type.
///
/// It should be implemented by a tree implementation and then the layout algorithms can run on it.
pub trait LayoutTreeNode: Sized {
    /// The main length type.
    ///
    /// It can simply be an `f32`.
    /// Sometimes float-point is not accurate enough. You can use the fixed-point types provided by the `fixed` crate.
    type Length: LengthNum;

    /// A custem length type used in `DefLength::Custom`.
    ///
    /// If you do not need it, simply use `i32`.
    type LengthCustom: PartialEq;

    /// A helper type for tree traversal.
    ///
    /// Hint: consider implement `LayoutTreeVisitor` for `Self` if the `Self` type can do tree traversal.
    type TreeVisitor: LayoutTreeVisitor<Self>;

    /// The layout styles of the current node.
    type Style: LayoutStyle<Self::Length, Self::LengthCustom>;

    /// A helper type to represent the current node as an inline node.
    ///
    /// This type is intended to be used in the text layout engine.
    /// You can write a void implementation if you do not need inline layout.
    type InlineUnit: InlineUnit<Self, Env = Self::Env>;

    /// A helper type to measure inline node.
    ///
    /// This type is intended to be used in the text layout engine.
    /// You can write a void implementation if you do not need inline layout.
    type InlineMeasure: InlineMeasure<Self, InlineUnit = Self::InlineUnit, Env = Self::Env>;

    /// Some custom environment data.
    type Env: ScreenQuery<Self::Length>;

    /// Get a reference to the `LayoutNode` of the current tree node.
    fn layout_node(&self) -> &LayoutNode<Self>;

    /// Get a helper for tree traversal.
    fn tree_visitor(&self) -> &Self::TreeVisitor;

    /// Get the styles of the current tree node.
    fn style(&self) -> &Self::Style;

    /// Resolve a `Length::Custom` value.
    ///
    /// If you do not use `Length::Custom` values, simply `unreachable!()`.
    fn resolve_custom_length(
        &self,
        custom: &Self::LengthCustom,
        owner: Self::Length,
    ) -> Self::Length;

    /// Returns if the node is a "measure" node.
    ///
    /// This means this node has a dedicated way to calculate its size (normally `true` for leaf nodes).
    /// For example, images has a natural width/height -
    /// its size should be calculated from the image natural size.
    /// In this case, the `LayoutTreeNode::measure_block_size` will be called.
    fn should_measure(&self, env: &mut Self::Env) -> bool;

    /// Measure a size for the current tree node.
    ///
    /// Only be called if the node is a "measure" node, a.k.a. `LayoutTreeNode::should_measure` returns `true`.
    /// The `env` is the one passed from the `Layoutnode::update*` call.
    /// If `update_position` is set, then the returned result will be used as the new layout result.
    #[allow(clippy::too_many_arguments)]
    fn measure_block_size(
        &self,
        env: &mut Self::Env,
        req_size: OptionSize<Self::Length>,
        min: Size<Self::Length>,
        max: Size<Self::Length>,
        max_content: OptionSize<Self::Length>,
        update_position: bool,
    ) -> MeasureResult<Self::Length>;

    /// Convert the current node to a `Self::InlineUnit`.
    ///
    /// The returned value will be passed to `InlineMeasure::block_size`.
    /// This is intended for the text layout engine.
    #[allow(clippy::too_many_arguments)]
    fn measure_inline_unit(
        &self,
        env: &mut Self::Env,
        req_size: OptionSize<Self::Length>,
        min: Size<Self::Length>,
        max: Size<Self::Length>,
        max_content: OptionSize<Self::Length>,
    ) -> Self::InlineUnit;

    /// A notifier that the layout size of itself (or any node in the subtree) has been re-evaluated.
    ///
    /// Note that the position is the combination of size and origin.
    /// This call indicates that the size may be changed and the origin is still undetermined.
    fn size_updated(
        &self,
        _env: &mut Self::Env,
        _size: Size<Self::Length>,
        _computed_style: &ComputedStyle<Self::Length>,
    ) {
    }
}

/// A helper type for tree traversal.
pub trait LayoutTreeVisitor<T: LayoutTreeNode> {
    /// Get the parent node.
    fn parent(&self) -> Option<&T>;

    /// Get child nodes.
    fn for_each_child<'a, 'b: 'a, F>(&'b self, f: F)
    where
        F: FnMut(&'a T, usize),
        T: 'a;

    /// Get the count of child nodes.
    fn children_len(&self) -> usize;

    /// Get the specified child.
    fn child_at(&self, index: usize) -> Option<&T>;
}

/// The styles of a tree node.
///
/// The values are similar to corresponding CSS properties.
#[allow(missing_docs)]
pub trait LayoutStyle<L: LengthNum, T: PartialEq = i32> {
    fn display(&self) -> Display;
    fn position(&self) -> Position;
    fn direction(&self) -> Direction;
    fn writing_mode(&self) -> WritingMode;
    fn flex_direction(&self) -> FlexDirection;
    fn flex_wrap(&self) -> FlexWrap;
    fn align_items(&self) -> AlignItems;
    fn align_self(&self) -> AlignSelf;
    fn align_content(&self) -> AlignContent;
    fn justify_content(&self) -> JustifyContent;
    fn left(&self) -> DefLength<L, T>;
    fn right(&self) -> DefLength<L, T>;
    fn top(&self) -> DefLength<L, T>;
    fn bottom(&self) -> DefLength<L, T>;
    fn border_left(&self) -> DefLength<L, T>;
    fn border_right(&self) -> DefLength<L, T>;
    fn border_top(&self) -> DefLength<L, T>;
    fn border_bottom(&self) -> DefLength<L, T>;
    fn margin_left(&self) -> DefLength<L, T>;
    fn margin_right(&self) -> DefLength<L, T>;
    fn margin_top(&self) -> DefLength<L, T>;
    fn margin_bottom(&self) -> DefLength<L, T>;
    fn padding_left(&self) -> DefLength<L, T>;
    fn padding_right(&self) -> DefLength<L, T>;
    fn padding_top(&self) -> DefLength<L, T>;
    fn padding_bottom(&self) -> DefLength<L, T>;
    fn flex_grow(&self) -> f32;
    fn flex_shrink(&self) -> f32;
    fn flex_basis(&self) -> DefLength<L, T>;
    fn width(&self) -> DefLength<L, T>;
    fn height(&self) -> DefLength<L, T>;
    fn min_width(&self) -> DefLength<L, T>;
    fn min_height(&self) -> DefLength<L, T>;
    fn max_width(&self) -> DefLength<L, T>;
    fn max_height(&self) -> DefLength<L, T>;
    fn aspect_ratio(&self) -> Option<f32>;
    fn box_sizing(&self) -> BoxSizing;
    fn order(&self) -> i32;
    fn text_align(&self) -> TextAlign {
        TextAlign::Start
    }
}

/// The layout information of a tree node.
pub struct LayoutNode<T: LayoutTreeNode> {
    unit: RefCell<LayoutUnit<T>>,
}

impl<T: LayoutTreeNode> fmt::Debug for LayoutNode<T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }
}

impl<T: LayoutTreeNode> Default for LayoutNode<T> {
    fn default() -> Self {
        Self {
            unit: RefCell::new(LayoutUnit::new()),
        }
    }
}

impl<T: LayoutTreeNode> LayoutNode<T> {
    /// Create a new layout node.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Informs the node styles been changed.
    #[inline]
    pub fn mark_dirty(&self, node: &T::TreeVisitor) -> bool {
        self.unit.borrow_mut().mark_dirty(node)
    }

    /// Get the size and position results (border rect).
    #[inline]
    pub fn result(&self) -> Rect<T::Length> {
        self.unit.borrow().result()
    }

    /// Get the result padding rect.
    #[inline]
    pub fn result_padding_rect(&self) -> Rect<T::Length> {
        self.unit.borrow().result_padding_rect()
    }

    /// Get the result content rect.
    #[inline]
    pub fn result_content_rect(&self) -> Rect<T::Length> {
        self.unit.borrow().result_content_rect()
    }

    /// Get the computed styles, such as margins, borders, and paddings.
    #[inline]
    pub fn computed_style(&self) -> ComputedStyle<T::Length> {
        self.unit.borrow().computed_style()
    }

    /// Check all nodes that has been `mark_dirty`, and update the layout results of the whole tree.
    ///
    /// Should only be called on the tree root node.
    /// The `env` will be received in measure functions.
    #[inline]
    pub fn update(&self, env: &mut T::Env, node: &T, available_size: OptionSize<T::Length>) {
        init_debug_logger();
        self.unit.borrow_mut().compute(env, node, available_size)
    }

    /// Check all nodes that has been `mark_dirty`, and update the layout results of the whole tree with container size given.
    ///
    /// Should only be called on the tree root node.
    /// The `env` will be received in measure functions.
    /// `available_size` is the available size for the root node.
    /// `containing_size` is the size of the viewport.
    #[inline]
    pub fn update_with_containing_size(
        &self,
        env: &mut T::Env,
        node: &T,
        available_size: OptionSize<T::Length>,
        containing_size: OptionSize<T::Length>,
    ) {
        init_debug_logger();
        self.unit.borrow_mut().compute_with_containing_size(
            env,
            node,
            available_size,
            containing_size,
        )
    }

    #[inline]
    pub(crate) fn unit(&self) -> RefMut<LayoutUnit<T>> {
        self.unit.borrow_mut()
    }
}

/// A helper type to measure inline nodes.
///
/// This should be implemented by the text layout engine.
pub trait InlineMeasure<T: LayoutTreeNode> {
    /// A helper type to represent the current node as an inline node.
    type InlineUnit: InlineUnit<T, Env = Self::Env>;

    /// Some custom environment data.
    type Env;

    /// Measure a series of inline nodes.
    ///
    /// Continous inline nodes will be collected together, and treat as a whole block.
    /// The text layout engine should returns:
    /// * the total size;
    /// * the position and detailed measure results for each inline node.
    ///
    /// The `env` will be received in measure functions.
    /// The `block_node` is the parent of these inline nodes.
    /// If `update_position` is set, then the returned result will be used as the new layout result.
    #[allow(clippy::type_complexity)]
    fn block_size(
        env: &mut Self::Env,
        block_node: &T,
        inline_nodes: Vec<(Self::InlineUnit, EdgeOption<T::Length>, Edge<T::Length>)>,
        req_size: OptionSize<T::Length>,
        max_content_with_max_size: OptionSize<T::Length>,
        update_position: bool,
    ) -> (
        Size<T::Length>,
        Vec<(Point<T::Length>, MeasureResult<T::Length>)>,
    );
}

/// A helper type as the inline form of a tree node.
pub trait InlineUnit<T: LayoutTreeNode> {
    /// Some custom environment data.
    type Env;

    /// Construct from a tree node with specified size and baseline information.
    fn inline_block(
        env: &mut Self::Env,
        node: &T,
        size: Size<T::Length>,
        baseline_ascent: T::Length,
    ) -> Self;
}

/// The result of the measure function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasureResult<L: LengthNum> {
    /// The size that occupied.
    pub size: Size<L>,

    /// The first baseline position.
    pub first_baseline_ascent: Vector<L>,

    /// The last baseline position.
    pub last_baseline_ascent: Vector<L>,
}

/// The computed `margin` `padding` `border` width.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedStyle<L: LengthNum> {
    pub margin: Edge<L>,
    pub padding: Edge<L>,
    pub border: Edge<L>,
}

impl<L: LengthNum> Default for ComputedStyle<L> {
    fn default() -> Self {
        Self {
            margin: Edge {
                left: L::zero(),
                right: L::zero(),
                top: L::zero(),
                bottom: L::zero(),
            },
            border: Edge {
                left: L::zero(),
                right: L::zero(),
                top: L::zero(),
                bottom: L::zero(),
            },
            padding: Edge {
                left: L::zero(),
                right: L::zero(),
                top: L::zero(),
                bottom: L::zero(),
            },
        }
    }
}

#[doc(hidden)]
pub fn init_debug_logger() {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Trace));
}
