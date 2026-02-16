//! Grid Alignment Implementation
//!
//! CSS Box Alignment Module Level 3
//! <https://www.w3.org/TR/css-align-3/>
//!
//! This module implements alignment for CSS Grid Layout:
//! - **Self-Alignment** (§6): `align-self` and `justify-self` for positioning
//!   items within their grid areas.
//! - **Content Distribution** (§5): `align-content` and `justify-content` for
//!   distributing tracks within the grid container.

use float_pigment_css::length_num::LengthNum;
use float_pigment_css::typing::{
    AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf,
};

use crate::{LayoutStyle, LayoutTreeNode};

// ═══════════════════════════════════════════════════════════════════════════════
// Self-Alignment (§6)
// https://www.w3.org/TR/css-align-3/#self-alignment
// ═══════════════════════════════════════════════════════════════════════════════

/// Resolve the effective align-self value for a grid item (block axis / vertical).
///
/// CSS Box Alignment Module Level 3 - §6.1 Self-Alignment
/// <https://www.w3.org/TR/css-align-3/#self-alignment>
///
/// The `align-self` property aligns the item within its containing block along
/// the block/column/cross axis. If `align-self` is `auto`, it computes to the
/// parent's `align-items` value.
///
/// In grid layout, the default alignment is `stretch` (via `align-items: normal`).
/// Reference: <https://www.w3.org/TR/css-align-3/#align-items-property>
pub(crate) fn resolve_grid_align_self<T: LayoutTreeNode>(
    child_style: &T::Style,
    parent_style: &T::Style,
) -> AlignSelf {
    let align_self = child_style.align_self();
    if align_self == AlignSelf::Auto {
        // CSS Box Alignment §6.1: "auto" computes to the parent's align-items value
        match parent_style.align_items() {
            AlignItems::Stretch => AlignSelf::Stretch,
            AlignItems::Center => AlignSelf::Center,
            AlignItems::Start | AlignItems::FlexStart | AlignItems::SelfStart => AlignSelf::Start,
            AlignItems::End | AlignItems::FlexEnd | AlignItems::SelfEnd => AlignSelf::End,
            AlignItems::Baseline => AlignSelf::Baseline,
            // Grid default: "normal" behaves as "stretch" for non-replaced grid items
            // https://www.w3.org/TR/css-align-3/#align-items-property
            AlignItems::Normal => AlignSelf::Stretch,
        }
    } else {
        align_self
    }
}

/// Resolve the effective justify-self value for a grid item (inline axis / horizontal).
///
/// CSS Box Alignment Module Level 3 - §6.1 Self-Alignment
/// <https://www.w3.org/TR/css-align-3/#justify-self-property>
///
/// The `justify-self` property aligns the item within its containing block along
/// the inline/row/main axis. If `justify-self` is `auto`, it computes to the
/// parent's `justify-items` value.
///
/// In grid layout, the default justification is `stretch` (via `justify-items: normal`).
pub(crate) fn resolve_grid_justify_self<T: LayoutTreeNode>(
    child_style: &T::Style,
    parent_style: &T::Style,
) -> JustifySelf {
    let justify_self = child_style.justify_self();
    if justify_self == JustifySelf::Auto {
        // CSS Box Alignment §6.1: "auto" computes to the parent's justify-items value
        match parent_style.justify_items() {
            JustifyItems::Stretch => JustifySelf::Stretch,
            JustifyItems::Center => JustifySelf::Center,
            JustifyItems::Start | JustifyItems::FlexStart | JustifyItems::SelfStart => {
                JustifySelf::Start
            }
            JustifyItems::End | JustifyItems::FlexEnd | JustifyItems::SelfEnd => JustifySelf::End,
            JustifyItems::Left => JustifySelf::Left,
            JustifyItems::Right => JustifySelf::Right,
            JustifyItems::Normal => JustifySelf::Stretch,
        }
    } else {
        justify_self
    }
}

/// Calculate the alignment offset for a grid item within its cell (block axis).
///
/// CSS Box Alignment Module Level 3 - §6 Self-Alignment
/// <https://www.w3.org/TR/css-align-3/#self-align>
///
/// Computes the offset needed to position an item according to `align-self`:
/// - `start`/`flex-start`/`self-start`: Item at the start edge (offset = 0)
/// - `end`/`flex-end`/`self-end`: Item at the end edge (offset = available_space)
/// - `center`: Item centered (offset = available_space / 2)
/// - `stretch`: Item stretches to fill (offset = 0, handled elsewhere)
pub(crate) fn calculate_alignment_offset<L: LengthNum>(
    align_self: AlignSelf,
    item_size: L,
    cell_size: L,
) -> L {
    if cell_size <= item_size {
        return L::zero();
    }
    let available_space = cell_size - item_size;
    match align_self {
        AlignSelf::Start | AlignSelf::FlexStart | AlignSelf::SelfStart => L::zero(),
        AlignSelf::End | AlignSelf::FlexEnd | AlignSelf::SelfEnd => available_space,
        AlignSelf::Center => available_space.div_f32(2.0),
        AlignSelf::Stretch | AlignSelf::Auto | AlignSelf::Normal | AlignSelf::Baseline => L::zero(),
    }
}

/// Calculate the justification offset for a grid item within its cell (inline axis).
///
/// CSS Box Alignment Module Level 3 - §6 Self-Alignment
/// <https://www.w3.org/TR/css-align-3/#justify-self-property>
///
/// Computes the offset needed to position an item according to `justify-self`:
/// - `start`/`flex-start`/`self-start`/`left`: Item at the start/left edge
/// - `end`/`flex-end`/`self-end`/`right`: Item at the end/right edge
/// - `center`: Item centered horizontally
/// - `stretch`: Item stretches to fill (offset = 0, handled elsewhere)
pub(crate) fn calculate_justify_offset<L: LengthNum>(
    justify_self: JustifySelf,
    item_size: L,
    cell_size: L,
) -> L {
    if cell_size <= item_size {
        return L::zero();
    }
    let available_space = cell_size - item_size;
    match justify_self {
        JustifySelf::Start
        | JustifySelf::FlexStart
        | JustifySelf::SelfStart
        | JustifySelf::Left => L::zero(),
        JustifySelf::End | JustifySelf::FlexEnd | JustifySelf::SelfEnd | JustifySelf::Right => {
            available_space
        }
        JustifySelf::Center => available_space.div_f32(2.0),
        JustifySelf::Stretch | JustifySelf::Auto | JustifySelf::Normal => L::zero(),
    }
}

/// Calculate justify offset for RTL (right-to-left) direction.
///
/// CSS Writing Modes Level 3 - §2.1 Specifying Directionality
/// <https://www.w3.org/TR/css-writing-modes-3/#direction>
///
/// In RTL mode, `start` and `end` are reversed:
/// - `start`: Aligns to right edge (offset = available_space)
/// - `end`: Aligns to left edge (offset = 0)
/// - `left`/`right`: Physical directions remain unchanged
/// - `center`: Unchanged
pub(crate) fn calculate_justify_offset_rtl<L: LengthNum>(
    justify_self: JustifySelf,
    item_size: L,
    cell_size: L,
) -> L {
    if cell_size <= item_size {
        return L::zero();
    }
    let available_space = cell_size - item_size;
    match justify_self {
        // In RTL, `start` means right edge, so offset from left = available_space
        JustifySelf::Start | JustifySelf::FlexStart | JustifySelf::SelfStart => available_space,
        // In RTL, `end` means left edge, so offset = 0
        JustifySelf::End | JustifySelf::FlexEnd | JustifySelf::SelfEnd => L::zero(),
        // Physical directions remain unchanged
        JustifySelf::Left => L::zero(),
        JustifySelf::Right => available_space,
        JustifySelf::Center => available_space.div_f32(2.0),
        JustifySelf::Stretch | JustifySelf::Auto | JustifySelf::Normal => L::zero(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Content Distribution (§5)
// https://www.w3.org/TR/css-align-3/#content-distribution
// ═══════════════════════════════════════════════════════════════════════════════

/// Calculate the content alignment offset for grid tracks within the container (block axis).
///
/// CSS Box Alignment Module Level 3 - §5 Content Distribution
/// <https://www.w3.org/TR/css-align-3/#content-distribution>
///
/// The `align-content` property aligns the grid tracks within the grid container
/// along the block axis when there is extra space.
///
/// Returns `(initial_offset, gap_addition)`:
/// - `initial_offset`: Space before the first track
/// - `gap_addition`: Extra space added between each track (for space-* values)
///
/// Distribution values (§5.3):
/// - `space-between`: First track at start, last at end, equal space between
///   - gap = available_space / (track_count - 1)
/// - `space-around`: Equal space around each track, half at edges
///   - gap = available_space / track_count; initial = gap / 2
/// - `space-evenly`: Equal space between and around all tracks
///   - gap = available_space / (track_count + 1); initial = gap
pub(crate) fn calculate_align_content_offset<L: LengthNum>(
    align_content: AlignContent,
    total_track_size: L,
    container_size: L,
    track_count: usize,
) -> (L, L) {
    if container_size <= total_track_size || track_count == 0 {
        return (L::zero(), L::zero());
    }
    let available_space = container_size - total_track_size;
    match align_content {
        AlignContent::Start | AlignContent::FlexStart => (L::zero(), L::zero()),
        AlignContent::End | AlignContent::FlexEnd => (available_space, L::zero()),
        AlignContent::Center => (available_space.div_f32(2.0), L::zero()),
        AlignContent::SpaceBetween => {
            // space-between: n tracks → (n-1) gaps
            if track_count <= 1 {
                (L::zero(), L::zero())
            } else {
                let gap = available_space.div_f32((track_count - 1) as f32);
                (L::zero(), gap)
            }
        }
        AlignContent::SpaceAround => {
            // space-around: n tracks → n portions, half at edges
            let gap = available_space.div_f32(track_count as f32);
            (gap.div_f32(2.0), gap)
        }
        AlignContent::SpaceEvenly => {
            // space-evenly: n tracks → (n+1) equal gaps
            let gap = available_space.div_f32((track_count + 1) as f32);
            (gap, gap)
        }
        AlignContent::Stretch | AlignContent::Normal | AlignContent::Baseline => {
            (L::zero(), L::zero())
        }
    }
}

/// Calculate the content justification offset for grid tracks within the container (inline axis).
///
/// CSS Box Alignment Module Level 3 - §5 Content Distribution
/// <https://www.w3.org/TR/css-align-3/#justify-content-property>
///
/// The `justify-content` property aligns the grid tracks within the grid container
/// along the inline axis when there is extra space.
///
/// Returns `(initial_offset, gap_addition)`:
/// - `initial_offset`: Space before the first track
/// - `gap_addition`: Extra space added between each track (for space-* values)
///
/// Behaves identically to `align-content` but on the inline/horizontal axis.
pub(crate) fn calculate_justify_content_offset<L: LengthNum>(
    justify_content: JustifyContent,
    total_track_size: L,
    container_size: L,
    track_count: usize,
) -> (L, L) {
    if container_size <= total_track_size || track_count == 0 {
        return (L::zero(), L::zero());
    }
    let available_space = container_size - total_track_size;
    match justify_content {
        JustifyContent::Start | JustifyContent::FlexStart | JustifyContent::Left => {
            (L::zero(), L::zero())
        }
        JustifyContent::End | JustifyContent::FlexEnd | JustifyContent::Right => {
            (available_space, L::zero())
        }
        JustifyContent::Center => (available_space.div_f32(2.0), L::zero()),
        JustifyContent::SpaceBetween => {
            // space-between: n tracks → (n-1) gaps
            if track_count <= 1 {
                (L::zero(), L::zero())
            } else {
                let gap = available_space.div_f32((track_count - 1) as f32);
                (L::zero(), gap)
            }
        }
        JustifyContent::SpaceAround => {
            // space-around: n tracks → n portions, half at edges
            let gap = available_space.div_f32(track_count as f32);
            (gap.div_f32(2.0), gap)
        }
        JustifyContent::SpaceEvenly => {
            // space-evenly: n tracks → (n+1) equal gaps
            let gap = available_space.div_f32((track_count + 1) as f32);
            (gap, gap)
        }
        JustifyContent::Stretch | JustifyContent::Baseline => (L::zero(), L::zero()),
    }
}
