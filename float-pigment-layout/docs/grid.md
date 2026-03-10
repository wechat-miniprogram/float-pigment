# CSS Grid Layout Implementation

This document describes the CSS Grid layout algorithm implementation in `float-pigment-layout`.

## Specification References

- [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/)
- [CSS Box Alignment Module Level 3](https://www.w3.org/TR/css-align-3/)

---

## Module Structure

```
float-pigment-layout/src/algo/grid/
├── mod.rs           # Main entry: 9-step layout algorithm orchestration (GridContainer::compute)
├── alignment.rs     # §10.3-10.5: Self-alignment (align/justify-self) & content distribution (align/justify-content)
├── grid_item.rs     # §6: GridItem (placement phase) & GridLayoutItem (layout phase) data structures
├── matrix.rs        # §7.1: OccupiedBitmap (1 bit/cell) + GridMatrix (placement) + GridLayoutMatrix (positioning)
├── placement.rs     # §8.5: Auto-placement algorithm (row/column × sparse/dense)
├── template.rs      # §7.1-7.2: Parse grid-template-rows/columns into track lists
├── track.rs         # §11.4-11.8: GridTrack/GridTracks + TrackSizingFunction + maximize (§11.6) + stretch (§11.8)
├── track_size.rs    # §11.3-11.4: Initial track size resolution (fixed/fr → used values)
└── track_sizing.rs  # §11.5+§11.7: Intrinsic track sizing + fr iterative freeze algorithm
```

## Algorithm Flow

### W3C Specification Reference

For the complete Grid layout flow, see: [W3C CSS Grid Layout Module Level 1 - §11 Grid Layout Algorithm](https://www.w3.org/TR/css-grid-1/#layout-algorithm)

### float-pigment Implementation Flow

This implementation follows the W3C Grid Layout Algorithm (§11) in a 9-step pipeline.
Steps 5–7 form the core track sizing loop (columns → rows → optional re-resolution → finalize).

```
┌──────────────────────────────────────────────────────────────────────┐
│               float-pigment Grid Layout Flow                       │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 1  Resolve Available Grid Space (§11.1)             │   │
│  │  Determine the available space in both axes by subtracting │   │
│  │  the container's padding and border from the used size.    │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 2  Resolve Gutters (§10.1)                           │   │
│  │  Resolve row-gap and column-gap to used values.            │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 3  Establish Grid (§7.1, §7.5-7.6)                  │   │
│  │  Parse grid-template-rows / grid-template-columns to form  │   │
│  │  the explicit grid; read grid-auto-rows / grid-auto-columns│   │
│  │  for implicit track sizing functions.                       │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 4  Place Grid Items (§8.5)                           │   │
│  │  Run the auto-placement algorithm according to             │   │
│  │  grid-auto-flow (row | column) × (sparse | dense).        │   │
│  │  Implicit tracks are created as needed.                    │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 5  Initialize Track Sizes (§11.3-11.4)              │   │
│  │  For each track (columns first, then rows):                │   │
│  │   ├─ Initialize each track's base size and growth limit    │   │
│  │   │   according to its track sizing function (§11.4).      │   │
│  │   └─ If both axes contain intrinsic tracks, perform        │   │
│  │       iterative re-resolution (§11.1 Step 3-4).            │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 6  Calculate Item Size Contributions (§11.5)         │   │
│  │  For each grid item, calculate its:                        │   │
│  │   ├─ min-content contribution (§11.5 Step 2)               │   │
│  │   ├─ max-content contribution (§11.5 Step 4)               │   │
│  │   └─ computed size under resolved track constraints.       │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 7  Resolve Final Track Sizes (§11.5-11.8)            │   │
│  │                                                             │   │
│  │  7a. Resolve Intrinsic Track Sizes (§11.5)                  │   │
│  │      Increase base sizes and growth limits using items'     │   │
│  │      min-content and max-content contributions.             │   │
│  │                                                             │   │
│  │  7b. Expand Flexible Tracks (§11.7)                         │   │
│  │      Distribute remaining free space to fr tracks using     │   │
│  │      the iterative freeze algorithm.                        │   │
│  │                                                             │   │
│  │  7c. Maximize Tracks (§11.6)                                │   │
│  │      If the grid container has a definite size, distribute  │   │
│  │      remaining free space equally among growable tracks.    │   │
│  │                                                             │   │
│  │  7d. Stretch auto Tracks (§11.8)                            │   │
│  │      When align/justify-content is normal or stretch,       │   │
│  │      expand auto tracks to fill remaining space.            │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 8  Align Grid Tracks — Content Distribution (§10.5)  │   │
│  │  Distribute free space among tracks per align-content /    │   │
│  │  justify-content (start | end | center | space-between |   │   │
│  │  space-around | space-evenly | stretch | normal).          │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               ▼                                    │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  STEP 9  Position Grid Items — Self-Alignment (§10.3-10.4) │   │
│  │  For each item:                                            │   │
│  │   ├─ Determine its grid area position from track offsets.  │   │
│  │   ├─ Apply align-self / justify-self within the grid area. │   │
│  │   ├─ Handle stretch alignment (re-layout if needed).       │   │
│  │   └─ Resolve writing direction (ltr / rtl).                │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                    │
└──────────────────────────────────────────────────────────────────────┘
```

#### Step Details

##### Step 1: Resolve Available Grid Space (§11.1)

Determine the available space for the grid container's content box:

1. Determine the grid container's used `width` and `height` (or derive from containing block constraints)
2. Subtract `padding` and `border` to obtain the content-box dimensions
3. Output: available inline-axis size and available block-axis size

##### Step 2: Resolve Gutters (§10.1)

Resolve `gap` / `row-gap` / `column-gap` to used values:

1. Resolve `row-gap` and `column-gap` (supports `<length>` and `<percentage>`)
2. Compute total gutter space per axis: `gutter_total = gap × (track_count − 1)`

##### Step 3: Establish Grid (§7.1, §7.5-7.6)

Build the explicit and implicit grid:

1. Parse `grid-template-rows` / `grid-template-columns` into a list of track sizing functions
2. Classify each track sizing function:
   - **Fixed** (`<length>`, `<percentage>`) — resolve to a definite size
   - **Flexible** (`<flex>`, i.e. `fr`) — marked for free-space distribution
   - **Intrinsic** (`auto`, `min-content`, `max-content`) — marked for content-based sizing
3. Read `grid-auto-rows` / `grid-auto-columns` for implicit track sizing functions

##### Step 4: Place Grid Items (§8.5)

Run the auto-placement algorithm per `grid-auto-flow`:

1. Exclude `position: absolute` and `display: none` children
2. Place each remaining item into the grid:
   - `row` (default) / `column`: sparse packing — the cursor only advances forward
   - `row dense` / `column dense`: dense packing — the cursor resets to search for earlier gaps
3. Create implicit tracks as needed when items exceed the explicit grid boundary

##### Step 5: Initialize Track Sizes (§11.3-11.4)

Initialize each track's **base size** and **growth limit**, columns first then rows:

1. For each track, set initial values according to its track sizing function (§11.4):
   - **Fixed**: base size = resolved value; growth limit = resolved value
   - **Flexible** (`fr`): base size = 0; growth limit = ∞
   - **Intrinsic** (`auto`, `min-content`, `max-content`): base size = 0; growth limit = ∞
2. If both axes contain intrinsic tracks, perform iterative re-resolution (§11.1 Step 3-4):
   resolve columns, then rows; if column sizes change, re-resolve rows (at most one extra pass)

##### Step 6: Calculate Item Size Contributions (§11.5)

Compute each grid item's size contributions for the track sizing algorithm:

1. For each item, calculate its **min-content contribution** (§11.5 Step 2) — the item's size when given the minimum possible space
2. For each item, calculate its **max-content contribution** (§11.5 Step 4) — the item's size when given unlimited space
3. Compute the item's final size under the resolved track constraints

##### Step 7: Resolve Final Track Sizes (§11.5-11.8)

1. **Resolve Intrinsic Track Sizes (§11.5)**: increase each track's base size and growth limit using items' min-content and max-content contributions
2. **Expand Flexible Tracks (§11.7)**: distribute remaining free space to `fr` tracks using the iterative freeze algorithm — any track whose hypothetical size is below its base size is frozen, and the remaining space is redistributed
3. **Maximize Tracks (§11.6)**: if the grid container has a definite size, distribute any remaining free space equally among growable tracks
4. **Stretch `auto` Tracks (§11.8)**: when `align-content` or `justify-content` computes to `normal` or `stretch`, expand `auto` tracks to fill remaining space

##### Step 8: Align Grid Tracks — Content Distribution (§10.5)

Distribute free space among grid tracks per `align-content` / `justify-content`:

1. Compute free space: container size minus total track sizes minus total gutters
2. Apply the alignment value:
   - `start` / `end` / `center`: shift all tracks by the corresponding offset
   - `space-between` / `space-around` / `space-evenly`: distribute extra spacing between tracks
   - `stretch` / `normal`: handled in Step 7d

##### Step 9: Position Grid Items — Self-Alignment (§10.3-10.4)

Determine each grid item's final position within its grid area:

1. Accumulate track sizes and gutters to determine the grid area's position and size
2. Apply content-distribution offsets from Step 8
3. Apply `align-self` (block axis) and `justify-self` (inline axis) within the grid area
4. Handle `stretch` alignment: re-layout the item if it has no explicit size and no `auto` margins
5. Resolve writing direction (`ltr` / `rtl`) for inline-axis positioning

---

## W3C Specification Comparison

### Specification Section Mapping

| W3C Section                  | Content                               | Status | Notes                                                            |
| ---------------------------- | ------------------------------------- | ------ | ---------------------------------------------------------------- |
| §5 Grid Containers           | `display: grid/inline-grid`           | ✅      | Full support                                                     |
| §6 Grid Items                | Grid item definition                  | ✅      | Correctly filters `display: none`, supports `position: absolute` |
| §7.1 Explicit Grid           | `grid-template-rows/columns`          | ✅      | Supports `<length>`, `<percentage>`, `auto`, `fr`, `min-content`, `max-content` |
| §7.5-7.6 Implicit Grid       | `grid-auto-rows/columns`              | ✅      | Supports fixed, percentage, fr, multiple values cycling          |
| §8.1-8.4 Line Placement      | Line-based placement                  | ❌      | `grid-column/row-start/end` not implemented                      |
| §8.5 Auto-placement          | Auto-placement algorithm              | ✅      | Full sparse and dense mode support                               |
| §9 Absolute Positioning      | Absolute positioning                  | ✅      | Correctly handles `position: absolute` items                     |
| §10.1 Gutters                | `gap`, `row-gap`, `column-gap`        | ✅      | Full support                                                     |
| §10.3 Row-axis Alignment     | `justify-self`                        | ✅      | All values supported                                             |
| §10.4 Column-axis Alignment  | `align-self`                          | ✅      | All values supported                                             |
| §10.5 Grid Alignment         | `align-content`, `justify-content`    | ✅      | Full support including `space-between` etc.                      |
| §11.1 Grid Sizing Algorithm  | Overall flow                          | ✅      | Implements iterative re-resolution (Step 3-4)                    |
| §11.3 Track Sizing Algorithm | Track size calculation                | ✅      | Follows spec order: columns→rows                                 |
| §11.4 Initialize Track Sizes | Initialize `base_size`/`growth_limit` | ✅      | Correct initialization                                           |
| §11.5 Intrinsic Track Sizes  | Resolve intrinsic track sizes         | ✅      | Uses min-content and max-content                                 |
| §11.6 Maximize Tracks        | Distribute free space                 | ✅      | Equal distribution to `growth_limit=∞` tracks                    |
| §11.7 Expand Flexible Tracks | fr iterative algorithm                | ✅      | Full iterative freezing algorithm                                |
| §11.8 Stretch auto Tracks    | Stretch auto tracks                   | ✅      | When `align-content: normal/stretch`                             |
| CSS Writing Modes §2.1       | `direction: ltr/rtl`                  | ✅      | Full RTL support, distinguishes logical/physical keywords        |

### Unimplemented Features

| Feature                  | W3C Section      | Priority | Notes                                       |
| ------------------------ | ---------------- | -------- | ------------------------------------------- |
| Line-based Placement     | §8.1-8.4         | High     | `grid-column/row-start/end`, `span` keyword |
| `repeat()`               | §7.2             | Medium   | Repeat track definitions                    |
| `minmax()`               | §7.2             | Medium   | Track min/max size constraints              |
| `auto-fill` / `auto-fit` | §7.2             | Medium   | Auto-fill tracks                            |
| Named Grid Areas         | §7.3             | Medium   | `grid-template-areas`                       |
| `fit-content()`          | §7.2             | Low      | Content-fit sizing                          |
| Shorthand Properties     | §7.4             | Low      | `grid-template`, `grid` shorthands          |
| Named Lines              | §8.4             | Low      | `[line-name]` named grid lines              |
| Subgrid                  | CSS Grid Level 2 | Low      | Subgrid feature                             |

---

## Algorithm Complexity Analysis

### Symbol Definitions

| Symbol | Meaning              |
| ------ | -------------------- |
| R      | Number of rows       |
| C      | Number of columns    |
| N      | Number of grid items |

### Time Complexity

| Step | Operation            | Complexity   | Description                                 |
| ---- | -------------------- | ------------ | ------------------------------------------- |
| 1    | Available Space      | O(1)         | Constant-time calculation                   |
| 2    | Gutters              | O(1)         | Constant-time calculation                   |
| 3    | Explicit Grid        | O(R + C)     | Iterate track definition list               |
| 4    | Placement            | O(N + R × C) | Dense search may scan holes; sparse is O(N) |
| 5    | Track Sizing         | O(N)         | Iterate items for track sizing              |
| 6    | Item Sizing          | O(N)         | Iterate items for item sizing               |
| 7    | Finalize Tracks      | O(N + R + C) | Finalize track base sizes                   |
| 8    | Content-distribution | O(R + C)     | Calculate track distribution offsets        |
| 9    | Item Positioning     | O(N)         | Iterate items for self-alignment            |

**Total Time Complexity**: **O(N + R × C)**

- For dense grids (N ≈ R × C), complexity is O(R × C)
- For sparse grids (N << R × C), complexity approaches O(N + R + C)

### Space Complexity

| Data Structure           | Complexity    | Description                                                               |
| ------------------------ | ------------- | ------------------------------------------------------------------------- |
| GridMatrix.occupancy     | O(R × C / 64) | Bit-level occupancy storage (1 bit per cell, ~1/64 space of dense matrix) |
| GridMatrix.items         | O(N)          | Grid items list                                                           |
| GridLayoutMatrix.items   | O(N)          | Layout items list                                                         |
| GridLayoutMatrix.offsets | O(R + C)      | Precomputed row/column offsets                                            |
| Track Lists              | O(R + C)      | Row/column track definition lists                                         |
| each_inline_size         | O(C)          | Column size temporary vector                                              |
| each_block_size          | O(R)          | Row size temporary vector                                                 |

**Total Space Complexity**: **O(R × C / 64 + N)**

- Occupancy state is stored in `OccupiedBitmap` at bit granularity (1 bit per cell), using about 1/64 of the space of a dense matrix
- Items stored separately in Vec for O(N) iteration

---

## Test Coverage

Currently **~248** Grid test cases covering:

### WPT Tests (`tests/wpt/css_grid/`)

| Category                     | Test Count | File                  |
| ---------------------------- | ---------- | --------------------- |
| Box Alignment                | 47         | `alignment.rs`        |
| Intrinsic Track Sizing       | 33         | `intrinsic_tracks.rs` |
| Maximize Tracks              | 25         | `maximize_tracks.rs`  |
| Auto-placement (incl. dense) | 22         | `grid_auto_flow.rs`   |
| Basic Layout                 | 17         | `grid_basics.rs`      |
| Gutters                      | 15         | `gap.rs`              |
| Explicit Track Sizing        | 14         | `grid_template.rs`    |
| Direction (RTL)              | 13         | `direction.rs`        |
| Flexible Length (`fr`)       | 16         | `fr_unit.rs`          |
| Margin                       | 11         | `margin.rs`           |
| Auto Tracks                  | 9          | `grid_auto.rs`        |
| Writing Mode                 | 7          | `writing_mode.rs`     |
| **WPT Subtotal**             | **229**    |                       |

All test assertion values are derived from W3C specification, ensuring compliance with spec-defined calculation logic.
