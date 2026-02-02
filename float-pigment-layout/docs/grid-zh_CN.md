# CSS Grid Layout 实现文档

本文档描述 `float-pigment-layout` 中 CSS Grid 布局算法的实现。

## 规范参考

- [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/)
- [CSS Box Alignment Module Level 3](https://www.w3.org/TR/css-align-3/)

---

## 模块结构

```
float-pigment-layout/src/algo/grid/
├── mod.rs          # 主入口，Grid 布局算法实现
├── alignment.rs    # 对齐计算 (align/justify-items/self/content)
├── track_size.rs   # 轨道尺寸计算 (fr, auto, fixed)
├── placement.rs    # Grid 项目放置算法
├── matrix.rs       # Grid 矩阵数据结构
├── grid_item.rs    # Grid 项目结构定义
└── dynamic_grid.rs # 动态二维网格数据结构
```

---

## 算法流程

### W3C 规范参考

完整的 Grid 布局流程请参考：[W3C CSS Grid Layout Module Level 1 - §11 Grid Layout Algorithm](https://www.w3.org/TR/css-grid-1/#layout-algorithm)

### float-pigment 已实现的 Grid 布局流程

本实现采用简化的单次遍历方式，共 9 个步骤：

```
+-----------------------------------------------------------------------------------+
|                      float-pigment Grid Layout Flow                               |
+-----------------------------------------------------------------------------------+
|                                                                                   |
|  1. Available Space (§11.1)                                                       |
|     +-- 计算 Grid 容器的 content-box 可用空间                                      |
|     +-- 根据 min/max-width/height 约束可用空间                                     |
|                                                                                   |
|  2. Gutters (§10.1)                                                               |
|     +-- 解析 row-gap / column-gap 属性                                             |
|     +-- 计算间距的实际像素值                                                        |
|                                                                                   |
|  3. Explicit Grid (§7.1)                                                          |
|     +-- 解析 grid-template-rows / grid-template-columns                            |
|     +-- 初始化轨道列表 (TrackList)                                                  |
|                                                                                   |
|  4. Placement (§8.5)                                                              |
|     +-- 过滤 Grid 项目 (排除 absolute / display:none)                              |
|     +-- 使用动态网格 (DynamicGrid) 单次遍历放置项目                                 |
|     +-- 支持 grid-auto-flow: row / column (dense 未实现)                           |
|                                                                                   |
|  5. Track Sizing (§11.3)                                                          |
|     +-- 初始化轨道尺寸 (简化版，无 growth limit)                                    |
|     +-- §11.7 Flex Tracks: 计算 fr 单位的实际像素值                                 |
|     +-- 先计算列尺寸，再计算行尺寸                                                  |
|                                                                                   |
|  6. Item Sizing (§11.5)                                                           |
|     +-- 计算每个项目的 min-content / max-content contribution                      |
|     +-- 使用 outer size (margin-box) 参与 track sizing                            |
|                                                                                   |
|  7. Finalize Tracks (§11.5)                                                       |
|     +-- 根据项目 outer size 调整 auto 轨道尺寸                                      |
|     +-- (§11.6 Maximize Tracks 未独立实现)                                          |
|                                                                                   |
|  8. Content Distribution (§10.5)                                                  |
|     +-- 应用 align-content: 分配 block axis 方向的剩余空间                          |
|     +-- 应用 justify-content: 分配 inline axis 方向的剩余空间                       |
|                                                                                   |
|  9. Item Positioning (§10.3-10.4)                                                 |
|     +-- 应用 align-self: 项目在单元格内的 block axis 对齐                           |
|     +-- 应用 justify-self: 项目在单元格内的 inline axis 对齐                        |
|                                                                                   |
+-----------------------------------------------------------------------------------+
| [!] LIMITATIONS vs W3C Specification:                                             |
|     - §11.1 Step 3-4: iterative re-resolution NOT implemented                     |
|     - §8.5: dense packing mode NOT implemented                                    |
|     - §11.4: base size / growth limit NOT separately maintained                   |
|     - §11.6: Maximize Tracks NOT separately implemented                           |
+-----------------------------------------------------------------------------------+
```

#### 步骤详解

##### Step 1: Available Space

计算 Grid container 的 available grid space (content-box)：

1. 确定容器的 `width` / `height`（或从 containing block 约束推导）
2. 减去 `padding` 和 `border` 得到 content-box 尺寸
3. 输出：`available_inline_size`（inline-axis）、`available_block_size`（block-axis）

##### Step 2: Gutters

解析 `gap` / `row-gap` / `column-gap` 属性：

1. 将 `row-gap` 和 `column-gap` 解析为 used value（支持 `<length>`、`<percentage>`）
2. 计算总 gutter 空间：`total_row_gap = row_gap × (row_count - 1)`
3. 从可用空间中扣除 gutters：`available_for_tracks = available - total_gap`

##### Step 3: Explicit Grid

解析 `grid-template-rows` / `grid-template-columns` 定义 explicit grid：

1. 遍历 track sizing function 列表
2. 分类 track sizing function：
   - Fixed (`100px`, `50%`) → 解析为 used pixel value
   - Flexible (`fr`) → 标记待 free space 分配
   - Intrinsic (`auto`) → 标记待 content-based sizing
3. 输出：row/column track 数量、初始化的 track sizes

##### Step 4: Placement

按 `grid-auto-flow` 使用 auto-placement algorithm 将 items 放入 grid matrix：

1. 过滤掉 `position: absolute` 和 `display: none` 的 items
2. 初始化空的动态网格矩阵 (`DynamicGrid`)
3. 使用 auto-placement algorithm 放置每个 item：
   - `row` 模式：row-major order (从左到右、从上到下)
   - `column` 模式：column-major order (从上到下、从左到右)
   - **Implicit track creation**：超出 explicit grid 边界时，自动创建 implicit tracks
   - ⚠️ `dense` mode 未实现 (当前 `RowDense`/`ColumnDense` 等同于 `Row`/`Column`)
4. 输出：`GridMatrix` (item placement mapping，尺寸为实际 grid 维度)


##### Step 5: Track Sizing

计算每个 track 的 used track size，先 columns 后 rows：

1. **Fixed sizing function**：直接使用解析后的像素值
2. **Flexible sizing function (`fr`)**：
   - 计算 free space：`free_space = available - fixed_tracks - gutters`
   - 计算 hypothetical fr size：`fr_size = free_space / total_fr`
   - Used track size：`track_size = fr_value × fr_size`
3. **`auto` sizing function**：初始化为 0，待 Step 7 根据 content contribution 调整

> ⚠️ 简化实现：未分别维护 W3C §11.4 规定的 `base size` 和 `growth limit`

##### Step 6: Item Sizing

递归计算每个 grid item 的 size contribution：

1. 遍历网格矩阵中的每个项目
2. 确定项目的可用空间（所跨越的 grid area）
3. 递归调用布局算法计算项目尺寸
4. 输出：每个项目的 `width`、`height`

##### Step 7: Finalize Tracks

根据 item contribution 调整 auto track size：

1. 遍历所有 `auto` tracks
2. 取该 track 内所有 items 的最大 outer size (margin-box)
3. 更新 track size
4. 输出：最终的 `each_inline_size[]`、`each_block_size[]`

> ⚠️ 简化实现：W3C §11.6 "Maximize Tracks" 未作为独立步骤实现

##### Step 8: Content Distribution

应用 `align-content` / `justify-content` 进行 content distribution：

1. 计算 free space：`free_space = container_size - total_track_size`
2. 根据 distribution value 计算偏移：
   - `start`：初始偏移 = 0
   - `end`：初始偏移 = free space
   - `center`：初始偏移 = free space / 2
   - `space-between` / `space-around` / `space-evenly`：计算额外的 inter-track spacing
3. 输出：`(initial_offset, gap_addition)`

##### Step 9: Item Positioning

应用 self-alignment 并计算每个 item 的最终位置：

1. 遍历网格矩阵
2. 累加轨道尺寸和 gutters 确定 grid area 位置
3. 应用 content-distribution 偏移
4. 在 grid area 内应用 self-alignment（`align-self` / `justify-self`）偏移
5. 设置项目的 `left`、`top`、`width`、`height`

---

## 支持的属性

### Grid Container 属性

| 属性 | 状态 | 说明 |
|-----|------|-----|
| `display: grid` | ✅ | block-level grid container |
| `display: inline-grid` | ✅ | inline-level grid container |
| `grid-template-columns` | ✅ | explicit column track sizing |
| `grid-template-rows` | ✅ | explicit row track sizing |
| `grid-auto-flow` | ✅ | auto-placement direction (row/column) |
| `grid-auto-flow: dense` | ⚠️ | dense packing mode 未实现 |
| `gap` / `row-gap` / `column-gap` | ✅ | gutters between tracks |
| `align-items` | ✅ | default block-axis alignment for items |
| `justify-items` | ✅ | default inline-axis alignment for items |
| `align-content` | ✅ | content-distribution (block-axis) |
| `justify-content` | ✅ | content-distribution (inline-axis) |

### Grid Item 属性

| 属性 | 状态 | 说明 |
|-----|------|-----|
| `align-self` | ✅ | self-alignment (block-axis) |
| `justify-self` | ✅ | self-alignment (inline-axis) |
| `grid-column-start` | ❌ | line-based placement 未实现 |
| `grid-column-end` | ❌ | line-based placement 未实现 |
| `grid-row-start` | ❌ | line-based placement 未实现 |
| `grid-row-end` | ❌ | line-based placement 未实现 |

### Track Sizing Functions

| 值 | 状态 | 说明 |
|---|------|-----|
| `<length>` | ✅ | fixed track sizing function (如 `100px`) |
| `<percentage>` | ✅ | percentage track sizing function (如 `50%`) |
| `auto` | ✅ | intrinsic track sizing (content-based) |
| `<flex>` (`fr`) | ✅ | flexible track sizing function |
| `min-content` | ⚠️ | intrinsic sizing (部分支持) |
| `max-content` | ⚠️ | intrinsic sizing (部分支持) |
| `minmax()` | ❌ | 未实现 |
| `repeat()` | ❌ | 未实现 |
| `fit-content()` | ❌ | 未实现 |

---

## TODO

### 高优先级

- [ ] **Iterative Re-resolution** (W3C §11.1 Step 3-4)
  - 当 min-content contribution 因 row sizes 改变时，重新计算 column track sizes
  - 当 min-content contribution 因 column sizes 改变时，重新计算 row track sizes
  - 影响场景：text wrapping、`aspect-ratio`、nested flex/grid

- [ ] **Line-based Placement** (W3C §8.3)
  - `grid-column-start` / `grid-column-end`
  - `grid-row-start` / `grid-row-end`
  - `grid-column` / `grid-row` shorthand properties
  - `grid-area` shorthand property
  - `span` keyword 支持

### 中优先级

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

### 低优先级

- [ ] **Intrinsic Sizing Keywords** (W3C §11.5 / CSS Sizing 3)
  - 完整的 `min-content` / `max-content` sizing 支持

- [ ] **Implicit Track Sizing** (W3C §7.6)
  - `grid-auto-rows` property
  - `grid-auto-columns` property

- [ ] **Subgrid** (CSS Grid Level 2)
  - `subgrid` keyword

---

## 测试覆盖

当前共有 **135 个** Grid 测试用例，覆盖：

| 类别 | 测试数 | 文件 |
|-----|-------|-----|
| Explicit Track Sizing | 14 | `grid_template.rs` |
| Auto-placement | 12 | `grid_auto_flow.rs` |
| Gutters | 15 | `gap.rs` |
| Flexible Length (`fr`) | 11 | `fr_unit.rs` |
| Basic Layout | 18 | `grid_basics.rs` |
| Box Alignment | 38 | `alignment.rs` |
| Other | 27 | - |

所有测试断言值均符合 W3C 规范定义的计算逻辑。

---

## 算法复杂度分析

### 符号定义

| 符号 | 含义 |
|-----|------|
| R | 行数 (rows) |
| C | 列数 (columns) |
| N | Grid 项目数 (items) |

### 时间复杂度

| 步骤 | 操作 | 复杂度 | 说明 |
|-----|------|--------|------|
| 1 | Available Space | O(1) | constant-time calculation |
| 2 | Gutters | O(1) | constant-time calculation |
| 3 | Explicit Grid | O(R + C) | 遍历 track definition list |
| 4 | Placement | O(N) | auto-place all grid items |
| 5 | Track Sizing | O(R + C) | 处理 row 和 column tracks |
| 6 | Item Sizing | O(R × C) | 遍历 grid matrix 计算 item sizing |
| 7 | Finalize Tracks | O(R × C) | finalize track base sizes |
| 8 | Content-distribution | O(R + C) | 计算 track distribution offsets |
| 9 | Item Positioning | O(R × C) | 应用 self-alignment 和 positioning |

**总时间复杂度**: **O(R × C)**

> 注：对于 dense grid（N ≈ R × C），复杂度等价于 O(N)

### 空间复杂度

| 数据结构 | 复杂度 | 说明 |
|---------|--------|------|
| DynamicGrid | O(R × C) | 动态扩展的二维矩阵 |
| GridLayoutMatrix | O(R × C) | 存储布局计算结果 |
| Track Lists | O(R + C) | 行/列轨道定义列表 |
| each_inline_size | O(C) | 列尺寸临时向量 |
| each_block_size | O(R) | 行尺寸临时向量 |

**总空间复杂度**: **O(R × C)**

### 复杂度特点

```
+------------------------------------------------------------------------+
|                         Complexity Summary                             |
+------------------------------------------------------------------------+
|                                                                        |
|        +------------------------------------------------+              |
|        |     Time: O(R x C)        Space: O(R x C)      |              |
|        +------------------------------------------------+              |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | Best Case: Small grid (e.g. 3x3)                               |   |
|   |   - Time  = O(1) (constant for fixed small grids)              |   |
|   |   - Space = O(1) (9 cells)                                     |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | General Case: Medium grid (e.g. 10x10)                         |   |
|   |   - Time  ~ O(100)                                             |   |
|   |   - Space ~ 100 cells                                          |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | Notes:                                                         |   |
|   |   - Each item layout is an independent sub-problem             |   |
|   |   - Nested Grid/Flex increases actual computation              |   |
|   |   - Caching can reduce redundant calculations                  |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
+------------------------------------------------------------------------+
```

### 与 Flexbox 对比

| Layout Mode | 时间复杂度 | 空间复杂度 |
|-------------|-----------|-----------|
| Grid | O(R × C) | O(R × C) |
| Flexbox | O(N) | O(N) |

> Grid layout 由于需要维护 2D grid matrix，复杂度高于一维的 Flexbox。
> 但对于实际的 UI 场景，grid dimensions 通常较小，性能差异可忽略。

### 复杂度最优性分析

**时间复杂度 O(R × C) 是渐近最优的 (asymptotically optimal)** ✅

```
+------------------------------------------------------------------------+
|                   Time Complexity Lower Bound Analysis                 |
+------------------------------------------------------------------------+
|                                                                        |
|   Grid layout requires:                                                |
|   +-- Determine each track size  --> At least R + C tracks             |
|   +-- Place N items into cells   --> At least N items                  |
|   +-- Position each item         --> At least N items                  |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | Lower Bound: Big-Omega(R + C + N)                              |   |
|   | When grid is nearly full (N ~ R x C), bound is Big-Omega(R x C)|   |
|   +----------------------------------------------------------------+   |
|                                                                        |
|   [OK] Current: O(R x C) = Achieves theoretical lower bound            |
|                                                                        |
+------------------------------------------------------------------------+
```


**空间复杂度 O(R × C) 符合理论下界**

```
+------------------------------------------------------------------------+
|                  Space Complexity Lower Bound Analysis                 |
+------------------------------------------------------------------------+
|                                                                        |
|   Must store:                                                          |
|   +-- Track size info  --> O(R + C)                                    |
|   +-- Item info        --> O(N)                                        |
|   +-- Cell mapping     --> O(R x C) for 2D positioning                 |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | Lower Bound: Big-Omega(R + C + N)                              |   |
|   | For dense grids: N ~ R x C, so Big-Omega(R x C)                |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
|   [OK] Current: O(R x C) - standard for 2D grid layout                 |
|   [*] Optimization: lazy allocation reduces actual memory usage        |
|                                                                        |
+------------------------------------------------------------------------+
```

### 与业界实现对比

| 实现 | 时间复杂度 | 空间复杂度 | 备注 |
|-----|-----------|-----------|------|
| **float-pigment** | O(R × C) | O(R × C) | single-pass, dynamic grid structure |
| Chromium (Blink) | O(k × R × C) | O(R × C) | k = iteration count (≤2) |
| Firefox (Gecko) | O(k × R × C) | O(R × C) | full W3C conformance |
| WebKit | O(k × R × C) | O(R × C) | full W3C conformance |
| Taffy | O(R × C) | O(R × C) | single-pass, dynamic grid structure |

**说明**:
- 本实现省略了 W3C §11.1 Step 3-4 的 iterative re-resolution，因此是 **single-pass** 执行
- 主流 browser engines 实现完整 W3C 规范，需要 iterative re-resolution，复杂度为 O(k × R × C)
- 实践中 k 通常为 1-2，性能差异可忽略

### 结论

```
+------------------------------------------------------------------------+
|                        Complexity Evaluation                           |
+------------------------------------------------------------------------+
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | TIME COMPLEXITY: O(R x C)                                      |   |
|   |   +-- Asymptotically optimal: achieves theoretical lower bound |   |
|   |   +-- Industry comparison: faster than full W3C (single-pass)  |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | SPACE COMPLEXITY: O(R x C)                                     |   |
|   |   +-- Standard for 2D grid layout data structures              |   |
|   |   +-- Industry comparison: on par with major browser engines   |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
|   +----------------------------------------------------------------+   |
|   | SUMMARY                                                        |   |
|   | Time complexity is asymptotically optimal.                     |   |
|   | Space complexity meets industry standards.                     |   |
|   | Lazy allocation reduces actual memory footprint.               |   |
|   +----------------------------------------------------------------+   |
|                                                                        |
+------------------------------------------------------------------------+
```
