# CSS Margin Collapsing Implementation

This document describes the margin collapsing implementation in `float-pigment-layout`, aligned with W3C §8.3.1.

## Specification References

- [CSS 2.1 §8.3.1 Collapsing margins](https://www.w3.org/TR/CSS21/box.html#collapsing-margins)
- [CSS 2.1 §9.4.1 Block formatting contexts](https://www.w3.org/TR/CSS21/visuren.html#block-formatting)
- [CSS Flexbox §3 Flex Containers](https://www.w3.org/TR/css-flexbox-1/#flex-containers)
- [CSS Grid Layout §3](https://www.w3.org/TR/css-grid-1/#grid-containers)

---

## Module Structure

```
float-pigment-layout/src/algo/flow.rs
├── establishes_bfc()              # unified BFC predicate (within current accessor coverage)
├── is_margin_start_collapsible()  # §8.3.1 relation (a): parent–first-child top
├── is_margin_end_collapsible()    # §8.3.1 relation (c): parent–last-child bottom (incl. direct height==auto check)
├── is_empty_block()               # §8.3.1 relation (d): empty-block collapsed-through
└── Flow::compute_block_or_inline_series()  # main flow: parent-child + sibling collapse

float-pigment-layout/src/unit.rs
├── CollapsedBlockMargin / CollapsedMargin  # positive/negative-split collapse margin data
└── unit.compute_with_containing_size       # layout root entry (reads collapsed_margin back into root)

float-pigment-layout/src/algo/flex_box.rs / grid/mod.rs
└── their own Flow::compute — build collapsed_margin via from_margin(), bypassing flow.rs collapse path
```

---

## Core Concepts

### Root Element

The "root element" in this engine = a node where **`node.parent().is_none()`** (CSS semantics: no parent = document tree root).

`.layout()` may be called on a subtree (incremental layout), but a subtree's top node still has a parent and participates in collapse normally — only a truly parentless node plays the root role and does not collapse with its children.

### Two-directional BFC Blocking

CSS §8.3.1 requires adjoining margins to belong to the **same BFC**. Margins crossing a BFC boundary do not collapse, which requires blocking in both directions:

- **Parent side**: a BFC container does not collapse with its child's margins (e.g., flex container vs. flex item)
- **Child side**: a BFC child does not collapse with its parent/siblings (e.g., a flex child inside a regular block parent)

### Signal Propagation: `bfc_established` Computed Locally

The BFC-blocking signal `bfc_established` is a **node-intrinsic property** (display / position / has-parent), computed locally at the top of `compute_block_or_inline_series` and reused at three call sites (start / end / empty-block checks).

**It is not propagated through `ComputeRequest`** — this keeps BFC blocking confined to the block-flow path. flex/grid layouts go through their own `Flex::compute` / `Grid::compute` and never read `bfc_established`, so they are unaffected.

---

## Algorithm Flow

### W3C §8.3.1 Adjoining Conditions

Two margins are **adjoining** (collapsible) if and only if:

1. Both belong to in-flow block-level boxes in the **same BFC**
2. No line boxes, no clearance, no padding, and no border separate them
3. They form one of these four vertically-adjacent pairs:
   - **(a)** top margin of a box and top margin of its **first in-flow child**
   - **(b)** bottom margin of a box and top margin of its **next in-flow following sibling**
   - **(c)** bottom margin of a **last in-flow child** and the parent's bottom margin (with **parent `height: auto`**)
   - **(d)** top and bottom margins of a box that **does not establish a new BFC**, with `min-height: 0`, `height: 0|auto`, and no in-flow children (empty-block collapsed-through)

### float-pigment Implementation Flow

`Flow::compute_block_or_inline_series` processes the collapse computation for a block container and its child sequence. The collapse chain is held in the `prev_sibling_collapsed_margin` state variable, advanced child-by-child:

```
┌───────────────────────────────────────────────────────────────────┐
│              float-pigment Margin Collapse Flow                   │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 1  bfc_established = establishes_bfc(node)                   │  │
│  │  Does this node establish a BFC? (no collapse with children)│  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 2  parent_margin_start_collapsible =                  │  │
│  │    is_margin_start_collapsible(bfc_established, parent_is_block,   │  │
│  │                                 padding_border_start)       │  │
│  │  §8.3.1 relation (a)                                        │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 3  Iterate in-flow children (skip display:none / abs/fixed) │  │
│  │                                                             │  │
│  │   for child in node.children:                              │  │
│  │     child_res = child.compute_internal(...)                │  │
│  │                                                             │  │
│  │     ┌── child establishes BFC? ───────────┐                │  │
│  │     │                                     │                │  │
│  │     ▼ yes                                 ▼ no             │  │
│  │  Settle prev chain into offset       Sibling collapse:     │  │
│  │  BFC child start/end margins as      prev.adjoin(          │  │
│  │    direct offsets (no adjoin)         child.start)         │  │
│  │  Seed prev = zero sentinel           Handle collapsed_through│  │
│  │    (so next sibling takes             Update prev          │  │
│  │    sibling branch, not first-child)                        │  │
│  │                                                             │  │
│  │     Parent-first-child collapse (prev == None and STEP 2 = true): │  │
│  │       parent_collapsed_margin_start.adjoin(child.start)     │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 4  height_is_auto = matches!(node.style().height(),  │  │
│  │            DefLength::Auto | DefLength::Undefined)          │  │
│  │          parent_margin_end_collapsible =                    │  │
│  │    is_margin_end_collapsible(bfc_established, height_is_auto, ...) │  │
│  │  §8.3.1 relation (c)                                        │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 5  If STEP 4 = true, fold last child's bottom into    │  │
│  │          parent_collapsed_margin_end                        │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 6  collapsed_through =                                │  │
│  │    is_empty_block(bfc_established, ...)                            │  │
│  │  §8.3.1 relation (d) (BFC containers do not collapse through)│  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 7  Emit BlockOrInlineSeriesComputeResult              │  │
│  │  { size, collapsed_margin: (start, end, collapsed_through) }│  │
│  │  Caller uses this to position the container                 │  │
│  └─────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────┘
```

### BFC Child Branch Handling (the "yes" branch of STEP 3)

When `establishes_bfc(child)` is true:

1. Settle the prior sibling collapse chain: solve the accumulated `prev_sibling_collapsed_margin` and add it to `main_offset` and `total_main_size`
2. Apply the BFC child's `start` / `end` margins as **direct offsets** (do **not** `adjoin` them into the chain)
3. Seed `prev_sibling_collapsed_margin = Some((CollapsedMargin::zero(), false))` — a zero sentinel

**Why a zero sentinel, not `None`**: setting `None` would make the next non-BFC sibling route through the first-child branch, which adjoins its top margin into `parent_collapsed_margin_start` — incorrectly propagating the sibling's margin up to the parent's top. The zero sentinel forces the next sibling through the sibling-collapse branch, where `adjoin(zero, x) = x` applies its margin as a direct offset with no spurious propagation. The sentinel is benign at end-of-children wrap-up (zero adjoin into `parent_end` is a no-op; zero solve added to total is a no-op).

This guarantees both margins of a BFC child act independently and do not collapse with parent or siblings, and the collapse chain stays broken for all subsequent siblings.

---

## W3C Conformance

### 12-item Audit Table

| # | Item | Standard | Status | Evidence |
|---|------|:---:|:---:|----------|
| 1 | (a) parent–first-child top | blocked by border/padding + BFC + clearance | ⚠️ partial | `is_margin_start_collapsible`: border/padding ✓, bfc_established ✓; **clearance missing** (needs `clear()` accessor) |
| 2 | (b) sibling bottom–top | blocked by clearance on latter | ⚠️ partial | `flow.rs` sibling branch algorithm correct; **clearance missing** |
| 3 | (c) parent–last-child bottom | height:auto + min-height:0 + no pb/border + clearance | ✅ | `is_margin_end_collapsible`: directly checks `node.style().height()` for Auto/Undefined |
| 4 | (d) empty-block collapsed-through | no BFC + min:0 + h:0\|auto + no children | ✅ | `is_empty_block`: includes `bfc_established` short-circuit (BFC containers do not collapse through) |
| 5 | root element | absolute rule | ✅ | `establishes_bfc`: `parent().is_none()` |
| 6 | BFC (overflow) | overflow≠visible | ⚠️ partial | display-based ✓; **overflow-based needs `overflow()` accessor** |
| 7 | float | float≠none | ❌ | **needs `float()` accessor** |
| 8 | absolutely positioned | abs/fixed | ✅ | `establishes_bfc`: Absolute/Fixed |
| 9 | inline-block | inline-block | ✅ | `establishes_bfc`: InlineBlock |
| 10 | horizontal margins | never collapse | ✅ | collapse only on main axis (`flow.rs:345-355`) |
| 11 | positive/negative margins | max/min/sum | ✅ | `CollapsedMargin` splits positive/negative; `adjoin` max/min; `solve` sums |
| 12 | same-BFC precondition | cross-BFC does not collapse | ✅ | parent-side `bfc_established` + child-side BFC branch, two-way blocking (within current accessor coverage) |

**Additional coverage** (display-based BFC): `Flex` / `InlineFlex` / `Grid` / `InlineGrid` / `FlowRoot` — all routed through `establishes_bfc`, conforming to Flexbox §3 / Grid §3.

### `establishes_bfc()` Implementation

```rust
fn establishes_bfc<T: LayoutTreeNode>(node: &T) -> bool {
    if node.tree_visitor().parent().is_none() { return true; }  // root element
    let style = node.style();
    if matches!(style.display(),
        Display::Flex | Display::InlineFlex
        | Display::Grid | Display::InlineGrid
        | Display::InlineBlock
        | Display::FlowRoot) { return true; }
    if matches!(style.position(), Position::Absolute | Position::Fixed) {
        return true;
    }
    false
}
```

### `CollapsedMargin` Positive/Negative Split

CSS §8.3.1 specifies: positive margins take max, negative margins take min, mixed positive+negative sum to (positive max + negative min).

```rust
struct CollapsedMargin<L> {
    positive: L,  // >= 0
    negative: L,  // <= 0
}

impl CollapsedMargin {
    fn adjoin(&self, other: &Self) -> Self {
        Self {
            positive: self.positive.max(other.positive),
            negative: self.negative.min(other.negative),
        }
    }
    fn solve(&self) -> L { self.positive + self.negative }
}
```

---

## Unimplemented Features (TODO)

| Priority | Item | Blocker |
|:---:|------|---------|
| medium | add `LayoutStyle::float()` accessor | trait extension |
| medium | add `LayoutStyle::overflow()` accessor | trait extension |
| medium | add `LayoutStyle::clear()` accessor | trait extension |
| medium | #7 float does not collapse | depends on `float()` accessor |
| medium | #6 overflow≠visible establishes BFC | depends on `overflow()` accessor |
| medium | #1/#2 clearance blocking (relations a/b/c/d) | depends on `clear()` accessor |
| low | `Display::FlowRoot` layout path | `flow.rs:128` is `todo!()`, independent layout algorithm |

Adding the accessors is a separate engineering effort touching 5 places:
1. `LayoutStyle` trait methods
2. forest `Node` implementation (`layout_impl.rs`)
3. forest `StyleManager` field storage
4. forest `StyleSetter` set methods
5. forest `set_style` parsing "overflow"/"float"/"clear" strings + mlp `Element` attribute reading

---

## Test Coverage

### Test Organization

- **Dedicated**: `float-pigment-forest/tests/custom/css_margin_collapse.rs` — collapse-specific, covers root-no-collapse + non-root parent-child collapse positive/negative cases
- **Existing**: `float-pigment-forest/tests/custom/css_margin.rs` — various margin scenarios, including the cross_flex series (BFC child collapse)
- **WPT**: `float-pigment-forest/tests/wpt/css_display/display_flex.rs` — flex + margin

### Key Tests

| Test | Verifies | Standard basis |
|------|----------|---------------|
| `entry_node_margin_does_not_collapse_with_child` | root (no parent) margin does not collapse with child | CSS §8.3.1 |
| `non_entry_parent_collapses_with_first_child` | non-root parent-child collapses normally | CSS §8.3.1 relation (a) |
| `block_bfc_block_sequence_no_propagation` | [block, BFC, block] — sibling after BFC does not propagate margin to parent | CSS §8.3.1 (BFC breaks chain) |
| `margin_collapse_cross_flex` 1/2/4/5/6 | flex container as child does not collapse with parent | Flexbox §3 |
| `display_flex_with_margin` | flex container margin acts independently | Flexbox §3 |
| `margin_collapse_min_height` / `_2` | min-height does not block last-child collapse (only height does) | CSS §8.3.1 relation (c) |
| `margin_collapse_max_height` / `_2` | max-height does not block last-child collapse | CSS §8.3.1 relation (c) |
| `margin_root` / `_3` / `_4` | entry node (no parent) margin is independent | CSS §8.3.1 |

### Test Results

- `cargo test -p float-pigment-forest`: 958 passed / 0 failed / 1 ignored
- `cargo clippy -p float-pigment-layout -- -D warnings`: no issues

---

**References**:
- [CSS 2.1 §8.3.1 Collapsing margins](https://www.w3.org/TR/CSS21/box.html#collapsing-margins)
- [CSS 2.1 §9.4.1 Block formatting contexts](https://www.w3.org/TR/CSS21/visuren.html#block-formatting)
- [CSS Flexbox §3 Flex Containers](https://www.w3.org/TR/css-flexbox-1/#flex-containers)
- [MDN Mastering margin collapsing](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_model/Mastering_margin_collapsing)
