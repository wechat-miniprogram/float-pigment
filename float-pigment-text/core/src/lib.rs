//! Text layout engine: itemization, shaping, line breaking, and justification.
//!
//! The layout process is split into two stages:
//!
//! * stage A (`ShapedBlock::new`): depends on the content and style properties only.
//!   The result can be reused when the container size changes.
//! * stage B (`ShapedBlock::layout`): depends on the container inline size,
//!   producing the final `LayoutResult`.
//!
//! Platform dependent abilities (font lookup, shaping, rasterization) are
//! abstracted as traits in `font`, `shaper` and `raster` modules, and are
//! implemented by platform specific crates.

#![warn(missing_docs)]

mod analyze;
mod axis;
pub mod font;
mod geometry;
mod itemize;
pub mod layout;
mod line;
pub mod model;
pub mod raster;
#[cfg(feature = "rust-font")]
pub mod rust_font;
mod shaped;
pub mod shaper;

pub use layout::LayoutResult;
pub use shaped::ShapedBlock;
