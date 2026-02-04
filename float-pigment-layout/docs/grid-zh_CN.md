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
├── track.rs        # 轨道数据结构 (GridTrack, GridTracks)
├── placement.rs    # Grid 项目放置算法
├── matrix.rs       # Grid 矩阵数据结构 (使用 HashSet 跟踪占用)
└── grid_item.rs    # Grid 项目结构定义
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
|     +-- grid-auto-flow: row / column (sparse packing)                             |
|     +-- grid-auto-flow: row dense / column dense (dense packing)                  |
|                                                                                   |
|  5. Track Sizing (§11.1 + §11.3-11.4)                                             |
|     +-- 初始化轨道 base_size 和 growth_limit (§11.4)                               |
|     +-- §11.7 Flex Tracks: 计算 fr 单位的实际像素值                                 |
|     +-- 先计算列尺寸，再计算行尺寸                                                  |
|     +-- §11.1 Step 3-4: Iterative re-resolution (当 auto tracks 存在时)           |
|                                                                                   |
|  6. Item Sizing (§11.5)                                                           |
|     +-- 计算每个项目的 min-content / max-content contribution                      |
|     +-- 使用 outer size (margin-box) 参与 track sizing                            |
|                                                                                   |
|  7. Finalize Tracks (§11.5-11.6)                                                  |
|     +-- 根据项目 outer size 调整 auto 轨道尺寸 (§11.5)                              |
|     +-- §11.6 Maximize Tracks: 分配 free space 给 auto tracks                     |
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
   - `row` (default): row-major order，cursor 只向前移动 (sparse)
   - `column`: column-major order，cursor 只向前移动 (sparse)
   - `row dense`: row-major order，从头搜索空洞 (dense)
   - `column dense`: column-major order，从头搜索空洞 (dense)
   - **Implicit track creation**：超出 explicit grid 边界时，自动创建 implicit tracks
4. 输出：`GridMatrix` (item placement mapping，尺寸为实际 grid 维度)


##### Step 5: Track Sizing (Initial Pass)

计算每个 track 的初始 used track size，先 columns 后 rows：

1. **初始化 (§11.4)**：为每个 track 初始化 `base_size` 和 `growth_limit`
   - Fixed sizing function：`base_size` = 解析后的像素值
   - Flexible sizing function (`fr`)：`base_size` = 0，`growth_limit` = infinity
   - `auto` sizing function：`base_size` = 0，`growth_limit` = infinity
2. **Flexible tracks (§11.7)** - 仅当没有 auto tracks 时在此步骤计算：
   - 若无 auto tracks：`fr_size = (available - fixed_tracks) / total_fr`
   - 若有 auto + fr 混用：延迟到 Step 7 计算（需先确定 auto 的 content size）
3. **Iterative Re-resolution (§11.1 Step 3-4)**：
   - 当同时存在 auto 行和 auto 列时，检查是否需要重新计算
   - 如果 column track sizes 因 row sizes 而改变，重新运行 track sizing
   - 最多重复一次，避免无限循环

##### Step 6: Item Sizing

递归计算每个 grid item 的 size contribution：

1. 遍历网格矩阵中的每个项目
2. 确定项目的可用空间（所跨越的 grid area）
3. 递归调用布局算法计算项目尺寸
4. 输出：每个项目的 `width`、`height`

##### Step 7: Finalize Tracks

根据 item contribution 调整 track size 并完成 fr 计算：

1. **§11.5 Resolve Intrinsic Track Sizes**：
   - 遍历所有 `auto` tracks
   - 取该 track 内所有 items 的最大 outer size (margin-box)
   - 更新 track `base_size`
2. **§11.7 Expand Flexible Tracks** (迭代算法)：
   - 计算 `hypothetical_fr_size = free_space / total_flex`
   - 如果任何 fr track 的 size < 其 min-content，冻结该 track 在 min-content
   - 重复迭代直到稳定
   - 最终：`track_size = hypothetical_fr_size × fr_value`
3. **§11.6 Maximize Tracks**：
   - 仅当 container 有 definite size 时执行
   - 计算 free space：`free_space = container_size - total_base_size - gutters`
   - 将 free space 平均分配给 `growth_limit` 为 infinity 的 tracks (auto tracks)
4. 输出：最终的 `each_inline_size[]`、`each_block_size[]`

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

## W3C 规范对比

### 规范章节对照表

| W3C 章节 | 规范内容 | 状态 | 说明 |
|---------|---------|------|------|
| §6 Grid Items | 网格项目定义 | ✅ | 正确过滤 `display: none`，支持 `position: absolute` |
| §7.1 Explicit Grid | `grid-template-rows/columns` | ✅ | 支持 `<length>`, `<percentage>`, `auto`, `fr` |
| §7.2 Implicit Grid | `grid-auto-rows/columns` | ⚠️ | 隐式轨道使用默认 `auto` 大小 |
| §8.1-8.4 Line Placement | 基于线的放置 | ❌ | 未实现 `grid-column/row-start/end` |
| §8.5 Auto-placement | 自动放置算法 | ✅ | 完整实现 sparse 和 dense 模式 |
| §9 Absolute Positioning | 绝对定位 | ✅ | 正确处理 `position: absolute` 项目 |
| §10.1 Gutters | `gap`, `row-gap`, `column-gap` | ✅ | 完整支持 |
| §10.3 Row-axis Alignment | `justify-self` | ✅ | 完整支持所有值 |
| §10.4 Column-axis Alignment | `align-self` | ✅ | 完整支持所有值 |
| §10.5 Grid Alignment | `align-content`, `justify-content` | ✅ | 完整支持包括 `space-between` 等 |
| §11.1 Grid Sizing Algorithm | 总体流程 | ✅ | 实现迭代重解析 (Step 3-4) |
| §11.3 Track Sizing Algorithm | 轨道大小计算 | ✅ | 按规范顺序：列→行 |
| §11.4 Initialize Track Sizes | 初始化 `base_size`/`growth_limit` | ✅ | 正确初始化 |
| §11.5 Intrinsic Track Sizes | 解析内在轨道尺寸 | ✅ | 使用 min-content 和 max-content |
| §11.6 Maximize Tracks | 分配剩余空间 | ✅ | 平均分配给 `growth_limit=∞` 的轨道 |
| §11.7 Expand Flexible Tracks | fr 迭代算法 | ✅ | 完整实现迭代冻结算法 |
| §11.8 Stretch auto Tracks | 拉伸 auto 轨道 | ✅ | 当 `align-content: normal/stretch` 时执行 |

### 未实现功能

| 功能 | W3C 章节 | 优先级 | 说明 |
|-----|---------|--------|------|
| Line-based Placement | §8.1-8.4 | 高 | `grid-column/row-start/end`, `span` keyword |
| `minmax()` | §7.2.3 | 中 | 轨道最小/最大尺寸约束 |
| `repeat()` | §7.2.2 | 中 | 重复轨道定义 |
| `fit-content()` | §7.2.4 | 低 | 内容适应尺寸 |
| `auto-fill` / `auto-fit` | §7.2.2.1 | 中 | 自动填充轨道 |
| Named Grid Areas | §7.3 | 中 | `grid-template-areas` |
| `grid-auto-rows/columns` | §7.6 | 低 | 隐式轨道大小控制 |
| Subgrid | CSS Grid Level 2 | 低 | 子网格 |

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
| 5 | Track Sizing | O(N) | 遍历 items 应用 track sizing |
| 6 | Item Sizing | O(N) | 遍历 items 计算 item sizing |
| 7 | Finalize Tracks | O(N + R + C) | finalize track base sizes |
| 8 | Content-distribution | O(R + C) | 计算 track distribution offsets |
| 9 | Item Positioning | O(N) | 遍历 items 应用 self-alignment |

**总时间复杂度**: **O(N + R + C)**

- 对于 dense grid（N ≈ R × C），复杂度为 O(R × C)
- 对于 sparse grid（N << R × C），复杂度接近 O(N)，达到理论最优

### 空间复杂度

| 数据结构 | 复杂度 | 说明 |
|---------|--------|------|
| GridMatrix.occupancy | O(R × C) | occupancy 状态 (1 byte per cell) |
| GridMatrix.items | O(N) | grid items 列表 |
| GridLayoutMatrix.items | O(N) | layout items 列表 |
| GridLayoutMatrix.offsets | O(R + C) | 预计算的行/列偏移 |
| Track Lists | O(R + C) | 行/列轨道定义列表 |
| each_inline_size | O(C) | 列尺寸临时向量 |
| each_block_size | O(R) | 行尺寸临时向量 |

**总空间复杂度**: **O(R × C + N)**

- Occupancy 追踪使用 1 byte per cell，比存储完整 GridItem 更高效
- Items 独立存储在 Vec 中，支持 O(N) 遍历

---

## 测试覆盖

当前共有 **~160 个** Grid 测试用例，覆盖：

| 类别 | 测试数 | 文件 |
|-----|-------|-----|
| Explicit Track Sizing | 14 | `grid_template.rs` |
| Auto-placement (含 dense) | 24 | `grid_auto_flow.rs` |
| Gutters | 15 | `gap.rs` |
| Flexible Length (`fr`) | 11 | `fr_unit.rs` |
| Basic Layout | 18 | `grid_basics.rs` |
| Box Alignment | 38 | `alignment.rs` |
| Maximize Tracks | 14 | `maximize_tracks.rs` |
| Other | 27 | - |

所有测试断言值均根据 W3C 规范推算，确保符合规范定义的计算逻辑。
