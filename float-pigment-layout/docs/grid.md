# CSS Grid Layout Implementation

This document describes the CSS Grid layout algorithm implementation in `float-pigment-layout`.

## Specification References

- [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/)
- [CSS Box Alignment Module Level 3](https://www.w3.org/TR/css-align-3/)

---

## Module Structure

```
float-pigment-layout/src/algo/grid/
├── mod.rs          # Main entry, Grid layout algorithm implementation
├── alignment.rs    # Alignment calculations (align/justify-items/self/content)
├── track_size.rs   # Track sizing calculations (fr, auto, fixed)
├── placement.rs    # Grid item placement algorithm
├── matrix.rs       # Grid matrix data structure
├── grid_item.rs    # Grid item structure definitions
└── dynamic_grid.rs # Dynamic 2D grid data structure
```

---

## Algorithm Flow

### W3C Specification Reference

For the complete Grid layout flow, see: [W3C CSS Grid Layout Module Level 1 - §11 Grid Layout Algorithm](https://www.w3.org/TR/css-grid-1/#layout-algorithm)

### float-pigment Implementation Flow

This implementation uses a simplified single-pass approach with 9 steps:

```
+-----------------------------------------------------------------------------------+
|                      float-pigment Grid Layout Flow                               |
+-----------------------------------------------------------------------------------+
|                                                                                   |
|  1. Available Space (§11.1)                                                       |
|     +-- Calculate grid container's content-box available space                    |
|     +-- Constrain available space by min/max-width/height                         |
|                                                                                   |
|  2. Gutters (§10.1)                                                               |
|     +-- Parse row-gap / column-gap properties                                     |
|     +-- Calculate actual pixel values for gaps                                    |
|                                                                                   |
|  3. Explicit Grid (§7.1)                                                          |
|     +-- Parse grid-template-rows / grid-template-columns                          |
|     +-- Initialize track list (TrackList)                                         |
|                                                                                   |
|  4. Placement (§8.5)                                                              |
|     +-- Filter grid items (exclude absolute / display:none)                       |
|     +-- Single-pass placement using DynamicGrid                                   |
|     +-- Support grid-auto-flow: row / column (dense NOT implemented)              |
|                                                                                   |
|  5. Track Sizing (§11.3)                                                          |
|     +-- Initialize track sizes (simplified, no growth limit)                      |
|     +-- §11.7 Flex Tracks: Calculate fr unit pixel values                         |
|     +-- Size columns first, then rows                                             |
|                                                                                   |
|  6. Item Sizing (§11.5)                                                           |
|     +-- Calculate min-content / max-content contribution for each item            |
|     +-- Use outer size (margin-box) for track sizing                              |
|                                                                                   |
|  7. Finalize Tracks (§11.5)                                                        |
|     +-- Adjust auto track sizes based on item outer size                          |
|     +-- (§11.6 Maximize Tracks NOT implemented)                                   |
|                                                                                   |
|  8. Content Distribution (§10.5)                                                  |
|     +-- Apply align-content: distribute remaining space on block axis             |
|     +-- Apply justify-content: distribute remaining space on inline axis          |
|                                                                                   |
|  9. Item Positioning (§10.3-10.4)                                                 |
|     +-- Apply align-self: item alignment within cell on block axis                |
|     +-- Apply justify-self: item alignment within cell on inline axis             |
|                                                                                   |
+-----------------------------------------------------------------------------------+
| [!] LIMITATIONS vs W3C Specification:                                             |
|     - §11.1 Step 3-4: iterative re-resolution NOT implemented                     |
|     - §8.5: dense packing mode NOT implemented                                    |
|     - §11.4: base size / growth limit NOT separately maintained                   |
|     - §11.6: Maximize Tracks NOT implemented (free space not distributed)         |
+-----------------------------------------------------------------------------------+
```

#### Step Details

##### Step 1: Available Space

Calculate the available grid space (content-box) of the grid container:

1. Determine container's `width` / `height` (or derive from containing block constraints)
2. Subtract `padding` and `border` to get content-box dimensions
3. Output: `available_inline_size` (inline-axis), `available_block_size` (block-axis)

##### Step 2: Gutters

Resolve `gap` / `row-gap` / `column-gap` properties:

1. Resolve `row-gap` and `column-gap` to used values (supports `<length>`, `<percentage>`)
2. Calculate total gutter space: `total_row_gap = row_gap × (row_count - 1)`
3. Subtract gutters from available space: `available_for_tracks = available - total_gap`

##### Step 3: Explicit Grid

Parse `grid-template-rows` / `grid-template-columns` to define explicit grid:

1. Iterate through track sizing function list
2. Classify track sizing functions:
   - Fixed sizing (`100px`, `50%`) → Resolve to used pixel values
   - Flexible sizing (`fr`) → Mark for free space distribution
   - Intrinsic sizing (`auto`) → Mark for content-based sizing
3. Output: Row/column track counts, initialized track sizes

##### Step 4: Placement

Auto-place items into grid matrix according to `grid-auto-flow`:

1. Filter out `position: absolute` and `display: none` items
2. Initialize empty dynamic grid matrix (`DynamicGrid`)
3. Place each item using auto-placement algorithm:
   - `row` mode: row-major order (left to right, top to bottom)
   - `column` mode: column-major order (top to bottom, left to right)
   - **Implicit track creation**: Automatically creates implicit tracks when exceeding explicit grid
   - ⚠️ `dense` mode NOT implemented (`RowDense`/`ColumnDense` behave same as `Row`/`Column`)
4. Output: `GridMatrix` (item placement mapping, sized to actual grid dimensions)


##### Step 5: Track Sizing

Calculate used track size for each track, columns first then rows:

1. **Fixed sizing function**: Use resolved pixel values directly
2. **Flexible sizing function (`fr`)**:
   - Calculate free space: `free_space = available - fixed_tracks - gutters`
   - Calculate hypothetical fr size: `fr_size = free_space / total_fr`
   - Used track size: `track_size = fr_value × fr_size`
3. **`auto` sizing function**: Initialize to 0, adjust in Step 7 based on content contribution

> ⚠️ Simplified: Does NOT separately maintain W3C §11.4 `base size` and `growth limit`

##### Step 6: Item Sizing

Recursively calculate size contribution of each grid item:

1. Iterate through each item in grid matrix
2. Determine item's available space (grid area it spans)
3. Recursively invoke layout algorithm to compute item size
4. Output: Each item's `width`, `height`

##### Step 7: Finalize Tracks

Adjust auto track sizes based on item contribution:

1. Iterate through all `auto` tracks
2. Take maximum outer size (margin-box) of all items spanning that track
3. Update track size
4. Output: Final `each_inline_size[]`, `each_block_size[]`

> ⚠️ Simplified: W3C §11.6 "Maximize Tracks" NOT separately implemented

##### Step 8: Content Distribution

Apply `align-content` / `justify-content` for content distribution:

1. Calculate free space: `free_space = container_size - total_track_size`
2. Calculate offset based on distribution value:
   - `start`: initial offset = 0
   - `end`: initial offset = free space
   - `center`: initial offset = free space / 2
   - `space-between` / `space-around` / `space-evenly`: Calculate additional inter-track spacing
3. Output: `(initial_offset, gap_addition)`

##### Step 9: Item Positioning

Apply self-alignment and calculate final item position:

1. Iterate through grid matrix
2. Accumulate track sizes and gutters to determine grid area position
3. Apply content-distribution offset
4. Apply self-alignment (`align-self` / `justify-self`) offset within grid area
5. Set item's `left`, `top`, `width`, `height`

---

## Supported Properties

### Grid Container Properties

| Property | Status | Description |
|----------|--------|-------------|
| `display: grid` | ✅ | block-level grid container |
| `display: inline-grid` | ✅ | inline-level grid container |
| `grid-template-columns` | ✅ | explicit column track sizing |
| `grid-template-rows` | ✅ | explicit row track sizing |
| `grid-auto-flow` | ✅ | auto-placement direction (row/column) |
| `grid-auto-flow: dense` | ⚠️ | dense packing mode not implemented |
| `gap` / `row-gap` / `column-gap` | ✅ | gutters between tracks |
| `align-items` | ✅ | default block-axis alignment for items |
| `justify-items` | ✅ | default inline-axis alignment for items |
| `align-content` | ✅ | content-distribution (block-axis) |
| `justify-content` | ✅ | content-distribution (inline-axis) |

### Grid Item Properties

| Property | Status | Description |
|----------|--------|-------------|
| `align-self` | ✅ | self-alignment (block-axis) |
| `justify-self` | ✅ | self-alignment (inline-axis) |
| `grid-column-start` | ❌ | line-based placement not implemented |
| `grid-column-end` | ❌ | line-based placement not implemented |
| `grid-row-start` | ❌ | line-based placement not implemented |
| `grid-row-end` | ❌ | line-based placement not implemented |

### Track Sizing Functions

| Value | Status | Description |
|-------|--------|-------------|
| `<length>` | ✅ | fixed track sizing function (e.g., `100px`) |
| `<percentage>` | ✅ | percentage track sizing function (e.g., `50%`) |
| `auto` | ✅ | intrinsic track sizing (content-based) |
| `<flex>` (`fr`) | ✅ | flexible track sizing function |
| `min-content` | ⚠️ | intrinsic sizing (partial support) |
| `max-content` | ⚠️ | intrinsic sizing (partial support) |
| `minmax()` | ❌ | not implemented |
| `repeat()` | ❌ | not implemented |
| `fit-content()` | ❌ | not implemented |

---

## TODO

### High Priority

- [ ] **Iterative Re-resolution** (W3C §11.1 Step 3-4)
  - Re-resolve column track sizes when min-content contribution changes due to row sizes
  - Re-resolve row track sizes when min-content contribution changes due to column sizes
  - Affected scenarios: text wrapping, `aspect-ratio`, nested flex/grid

- [ ] **Line-based Placement** (W3C §8.3)
  - `grid-column-start` / `grid-column-end`
  - `grid-row-start` / `grid-row-end`
  - `grid-column` / `grid-row` shorthand properties
  - `grid-area` shorthand property
  - `span` keyword support

### Medium Priority

- [ ] **Track Sizing Functions** (W3C §7.2)
  - `minmax(min, max)` sizing function
  - `repeat(count, tracks)` notation
  - `fit-content(limit)` sizing function
  - `auto-fill` / `auto-fit` keywords

- [ ] **Named Grid Areas** (W3C §7.3)
  - `grid-template-areas` property
  - Named area-based placement

- [ ] **Dense Packing Mode** (W3C §8.5)
  - `grid-auto-flow: dense`
  - `grid-auto-flow: row dense`
  - `grid-auto-flow: column dense`

### Low Priority

- [ ] **Intrinsic Sizing Keywords** (W3C §11.5 / CSS Sizing 3)
  - Full `min-content` / `max-content` sizing support

- [ ] **Implicit Track Sizing** (W3C §7.6)
  - `grid-auto-rows` property
  - `grid-auto-columns` property

- [ ] **Subgrid** (CSS Grid Level 2)
  - `subgrid` keyword

---

## Algorithm Complexity Analysis

### Symbol Definitions

| Symbol | Meaning |
|--------|---------|
| R | Number of rows |
| C | Number of columns |
| N | Number of grid items |

### Time Complexity

| Step | Operation | Complexity | Description |
|------|-----------|------------|-------------|
| 1 | Available Space | O(1) | Constant-time calculation |
| 2 | Gutters | O(1) | Constant-time calculation |
| 3 | Explicit Grid | O(R + C) | Iterate track definition list |
| 4 | Placement | O(N) | Auto-place all grid items |
| 5 | Track Sizing | O(N) | Iterate items for track sizing |
| 6 | Item Sizing | O(N) | Iterate items for item sizing |
| 7 | Finalize Tracks | O(N + R + C) | Finalize track base sizes |
| 8 | Content-distribution | O(R + C) | Calculate track distribution offsets |
| 9 | Item Positioning | O(N) | Iterate items for self-alignment |

**Total Time Complexity**: **O(N + R + C)**

- For dense grids (N ≈ R × C), complexity is O(R × C)
- For sparse grids (N << R × C), complexity approaches O(N), achieving theoretical optimum

### Space Complexity

| Data Structure | Complexity | Description |
|----------------|------------|-------------|
| GridMatrix.occupancy | O(R × C) | Occupancy state (1 byte per cell) |
| GridMatrix.items | O(N) | Grid items list |
| GridLayoutMatrix.items | O(N) | Layout items list |
| GridLayoutMatrix.offsets | O(R + C) | Precomputed row/column offsets |
| Track Lists | O(R + C) | Row/column track definition lists |
| each_inline_size | O(C) | Column size temporary vector |
| each_block_size | O(R) | Row size temporary vector |

**Total Space Complexity**: **O(R × C + N)**

- Occupancy tracking uses 1 byte per cell, more efficient than storing full GridItem
- Items stored separately in Vec for O(N) iteration

---

## Test Coverage

Currently **135** Grid test cases covering:

| Category | Test Count | File |
|----------|------------|------|
| Explicit Track Sizing | 14 | `grid_template.rs` |
| Auto-placement | 12 | `grid_auto_flow.rs` |
| Gutters | 15 | `gap.rs` |
| Flexible Length (`fr`) | 11 | `fr_unit.rs` |
| Basic Layout | 18 | `grid_basics.rs` |
| Box Alignment | 38 | `alignment.rs` |
| Other | 27 | - |

All test assertion values conform to W3C specification-defined calculation logic.
