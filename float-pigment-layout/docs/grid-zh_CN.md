# CSS Grid Layout 实现文档

本文档描述 `float-pigment-layout` 中 CSS Grid 布局算法的实现。

## 规范参考

- [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/)
- [CSS Box Alignment Module Level 3](https://www.w3.org/TR/css-align-3/)

---

## 模块结构

```
float-pigment-layout/src/algo/grid/
├── mod.rs           # 主入口：9 步布局算法编排 (GridContainer::compute)
├── alignment.rs     # §10.3-10.5：自身对齐 (align/justify-self) & 内容分配 (align/justify-content)
├── grid_item.rs     # §6：GridItem（放置阶段）& GridLayoutItem（布局阶段）数据结构
├── matrix.rs        # §7.1：OccupiedBitmap（1 bit/cell）+ GridMatrix（放置）+ GridLayoutMatrix（定位）
├── placement.rs     # §8.5：自动放置算法 (row/column × sparse/dense)
├── template.rs      # §7.1-7.2：解析 grid-template-rows/columns 为轨道列表
├── track.rs         # §11.4-11.8：GridTrack/GridTracks + TrackSizingFunction + maximize (§11.6) + stretch (§11.8)
├── track_size.rs    # §11.3-11.4：初始轨道尺寸解析 (fixed/fr → used values)
└── track_sizing.rs  # §11.5+§11.7：内在轨道尺寸计算 + fr 迭代冻结算法
```

---

## 算法流程

### W3C 规范参考

完整的 Grid 布局流程请参考：[W3C CSS Grid Layout Module Level 1 - §11 Grid Layout Algorithm](https://www.w3.org/TR/css-grid-1/#layout-algorithm)

### float-pigment 已实现的 Grid 布局流程

本实现遵循 W3C Grid 布局算法（§11）的 9 步流水线。
其中步骤 5–7 构成核心的轨道尺寸计算循环（列 → 行 → 可选重解析 → 最终化）。

```
┌───────────────────────────────────────────────────────────────────┐
│              float-pigment Grid Layout Flow                      │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 1  Resolve Available Grid Space (§11.1)           │   │
│  │  从容器的已用尺寸中减去 padding 和 border，             │   │
│  │  确定两个轴的可用空间。                                 │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 2  Resolve Gutters (§10.1)                        │   │
│  │  将 row-gap 和 column-gap 解析为 used value。           │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 3  Establish Grid (§7.1, §7.5-7.6)               │   │
│  │  解析 grid-template-rows / grid-template-columns        │   │
│  │  形成 explicit grid；读取 grid-auto-rows /              │   │
│  │  grid-auto-columns 作为 implicit track sizing function。│   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 4  Place Grid Items (§8.5)                        │   │
│  │  按 grid-auto-flow (row | column) x (sparse | dense)   │   │
│  │  执行 auto-placement algorithm。                        │   │
│  │  必要时创建 implicit tracks。                           │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 5  Initialize Track Sizes (§11.3-11.4)           │   │
│  │  对每个轨道（先列后行）：                               │   │
│  │   ├─ 根据 track sizing function 初始化每个轨道的       │   │
│  │   │   base size 和 growth limit (§11.4)。               │   │
│  │   └─ 若两个轴都包含 intrinsic tracks，执行             │   │
│  │       迭代重解析 (§11.1 Step 3-4)。                     │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 6  Calculate Item Size Contributions (§11.5)      │   │
│  │  对每个 grid item 计算：                                │   │
│  │   ├─ min-content contribution (§11.5 Step 2)            │   │
│  │   ├─ max-content contribution (§11.5 Step 4)            │   │
│  │   └─ 在已解析轨道约束下的 computed size。               │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 7  Resolve Final Track Sizes (§11.5-11.8)         │   │
│  │                                                          │   │
│  │  7a. Resolve Intrinsic Track Sizes (§11.5)               │   │
│  │      根据项目的 min/max-content contribution            │   │
│  │      增大 base size 和 growth limit。                   │   │
│  │                                                          │   │
│  │  7b. Expand Flexible Tracks (§11.7)                      │   │
│  │      使用迭代冻结算法将剩余 free space                  │   │
│  │      分配给 fr 轨道。                                   │   │
│  │                                                          │   │
│  │  7c. Maximize Tracks (§11.6)                             │   │
│  │      若 grid container 有 definite size，               │   │
│  │      将剩余 free space 平均分配给可增长的轨道。         │   │
│  │                                                          │   │
│  │  7d. Stretch auto Tracks (§11.8)                         │   │
│  │      当 align/justify-content 为 normal 或 stretch      │   │
│  │      时，扩展 auto 轨道以填充剩余空间。                 │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 8  Align Grid Tracks (§10.5)                      │   │
│  │  按 align-content / justify-content 在轨道间分配        │   │
│  │  free space (start | end | center | space-between |     │   │
│  │  space-around | space-evenly | stretch | normal)。      │   │
│  └──────────────────────────┬─────────────────────────────┘   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │  STEP 9  Position Grid Items (§10.3-10.4)               │   │
│  │  对每个项目：                                           │   │
│  │   ├─ 根据轨道偏移确定其 grid area 位置。               │   │
│  │   ├─ 在 grid area 内应用 align-self / justify-self。   │   │
│  │   ├─ 处理 stretch 对齐（必要时重新布局）。             │   │
│  │   └─ 解析 writing direction (ltr / rtl)。               │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                  │
└───────────────────────────────────────────────────────────────────┘
```

#### 步骤详解

##### Step 1: 解析可用网格空间 (§11.1)

确定 grid container 的 content box 可用空间：

1. 确定容器的已用 `width` 和 `height`（或从 containing block 约束推导）
2. 减去 `padding` 和 `border` 得到 content-box 尺寸
3. 输出：inline 轴可用尺寸、block 轴可用尺寸

##### Step 2: 解析间距 (§10.1)

将 `gap` / `row-gap` / `column-gap` 解析为 used value：

1. 解析 `row-gap` 和 `column-gap`（支持 `<length>` 和 `<percentage>`）
2. 计算每轴的总 gutter 空间：`gutter_total = gap × (track_count − 1)`

##### Step 3: 建立网格 (§7.1, §7.5-7.6)

构建 explicit grid 和 implicit grid：

1. 解析 `grid-template-rows` / `grid-template-columns` 为 track sizing function 列表
2. 分类每个 track sizing function：
   - **Fixed** (`<length>`、`<percentage>`) — 解析为确定尺寸
   - **Flexible** (`<flex>`，即 `fr`) — 标记为待 free-space 分配
   - **Intrinsic** (`auto`、`min-content`、`max-content`) — 标记为待内容尺寸确定
3. 读取 `grid-auto-rows` / `grid-auto-columns` 作为 implicit track sizing function

##### Step 4: 放置网格项目 (§8.5)

按 `grid-auto-flow` 执行 auto-placement algorithm：

1. 排除 `position: absolute` 和 `display: none` 的子项
2. 将每个剩余项目放入网格：
   - `row`（默认）/ `column`：sparse packing — 游标只向前推进
   - `row dense` / `column dense`：dense packing — 游标重置以搜索前方空洞
3. 当项目超出 explicit grid 边界时，按需创建 implicit tracks

##### Step 5: 初始化轨道尺寸 (§11.3-11.4)

初始化每个轨道的 **base size** 和 **growth limit**，先列后行：

1. 根据 track sizing function 设置初始值 (§11.4)：
   - **Fixed**：base size = 解析值；growth limit = 解析值
   - **Flexible** (`fr`)：base size = 0；growth limit = ∞
   - **Intrinsic** (`auto`、`min-content`、`max-content`)：base size = 0；growth limit = ∞
2. 若两个轴都包含 intrinsic tracks，执行迭代重解析 (§11.1 Step 3-4)：
   先解析列、再解析行；若列尺寸发生变化则重新解析行（最多额外一轮）

##### Step 6: 计算项目尺寸贡献 (§11.5)

为 track sizing algorithm 计算每个 grid item 的尺寸贡献：

1. 对每个项目计算 **min-content contribution** (§11.5 Step 2) — 给定最小可用空间时的项目尺寸
2. 对每个项目计算 **max-content contribution** (§11.5 Step 4) — 给定无限可用空间时的项目尺寸
3. 在已解析的轨道约束下计算项目的最终尺寸

##### Step 7: 解析最终轨道尺寸 (§11.5-11.8)

1. **解析内在轨道尺寸 (§11.5)**：使用项目的 min-content 和 max-content contribution 增大每个轨道的 base size 和 growth limit
2. **扩展弹性轨道 (§11.7)**：使用迭代冻结算法将剩余 free space 分配给 `fr` 轨道 — 若轨道的假设尺寸低于其 base size 则冻结该轨道，并重新分配剩余空间
3. **最大化轨道 (§11.6)**：若 grid container 有 definite size，将剩余 free space 平均分配给可增长的轨道
4. **拉伸 `auto` 轨道 (§11.8)**：当 `align-content` 或 `justify-content` 计算值为 `normal` 或 `stretch` 时，扩展 `auto` 轨道以填充剩余空间

##### Step 8: 对齐网格轨道 — 内容分配 (§10.5)

按 `align-content` / `justify-content` 在网格轨道间分配 free space：

1. 计算 free space：容器尺寸 − 总轨道尺寸 − 总 gutter
2. 应用对齐值：
   - `start` / `end` / `center`：按对应偏移量移动所有轨道
   - `space-between` / `space-around` / `space-evenly`：在轨道间分配额外间距
   - `stretch` / `normal`：已在 Step 7d 处理

##### Step 9: 定位网格项目 — 自身对齐 (§10.3-10.4)

确定每个 grid item 在其 grid area 内的最终位置：

1. 累加轨道尺寸和 gutter 确定 grid area 的位置和尺寸
2. 应用 Step 8 的 content-distribution 偏移
3. 在 grid area 内应用 `align-self`（block 轴）和 `justify-self`（inline 轴）
4. 处理 `stretch` 对齐：若项目无显式尺寸且无 `auto` margin 则重新布局
5. 解析 writing direction (`ltr` / `rtl`) 用于 inline 轴定位

---

## W3C 规范对比

### 规范章节对照表

| W3C 章节                     | 规范内容                           | 状态 | 说明                                                |
| ---------------------------- | ---------------------------------- | ---- | --------------------------------------------------- |
| §5 Grid Containers           | `display: grid/inline-grid`        | ✅    | 完整支持                                            |
| §6 Grid Items                | 网格项目定义                       | ✅    | 正确过滤 `display: none`，支持 `position: absolute` |
| §7.1 Explicit Grid           | `grid-template-rows/columns`       | ✅    | 支持 `<length>`, `<percentage>`, `auto`, `fr`, `min-content`, `max-content` |
| §7.5-7.6 Implicit Grid       | `grid-auto-rows/columns`           | ✅    | 支持固定值、百分比、fr、多值循环                    |
| §8.1-8.4 Line Placement      | 基于线的放置                       | ❌    | 未实现 `grid-column/row-start/end`                  |
| §8.5 Auto-placement          | 自动放置算法                       | ✅    | 完整实现 sparse 和 dense 模式                       |
| §9 Absolute Positioning      | 绝对定位                           | ✅    | 正确处理 `position: absolute` 项目                  |
| §10.1 Gutters                | `gap`, `row-gap`, `column-gap`     | ✅    | 完整支持                                            |
| §10.3 Row-axis Alignment     | `justify-self`                     | ✅    | 完整支持所有值                                      |
| §10.4 Column-axis Alignment  | `align-self`                       | ✅    | 完整支持所有值                                      |
| §10.5 Grid Alignment         | `align-content`, `justify-content` | ✅    | 完整支持包括 `space-between` 等                     |
| §11.1 Grid Sizing Algorithm  | 总体流程                           | ✅    | 实现迭代重解析 (Step 3-4)                           |
| §11.3 Track Sizing Algorithm | 轨道大小计算                       | ✅    | 按规范顺序：列→行                                   |
| §11.4 Initialize Track Sizes | 初始化 `base_size`/`growth_limit`  | ✅    | 正确初始化                                          |
| §11.5 Intrinsic Track Sizes  | 解析内在轨道尺寸                   | ✅    | 使用 min-content 和 max-content                     |
| §11.6 Maximize Tracks        | 分配剩余空间                       | ✅    | 平均分配给 `growth_limit=∞` 的轨道                  |
| §11.7 Expand Flexible Tracks | fr 迭代算法                        | ✅    | 完整实现迭代冻结算法                                |
| §11.8 Stretch auto Tracks    | 拉伸 auto 轨道                     | ✅    | 当 `align-content: normal/stretch` 时执行           |
| CSS Writing Modes §2.1       | `direction: ltr/rtl`               | ✅    | 完整支持 RTL 布局，区分逻辑/物理关键字              |

### 未实现功能

| 功能                     | W3C 章节         | 优先级 | 说明                                        |
| ------------------------ | ---------------- | ------ | ------------------------------------------- |
| Line-based Placement     | §8.1-8.4         | 高     | `grid-column/row-start/end`, `span` keyword |
| `repeat()`               | §7.2             | 中     | 重复轨道定义                                |
| `minmax()`               | §7.2             | 中     | 轨道最小/最大尺寸约束                       |
| `auto-fill` / `auto-fit` | §7.2             | 中     | 自动填充轨道                                |
| Named Grid Areas         | §7.3             | 中     | `grid-template-areas`                       |
| `fit-content()`          | §7.2             | 低     | 内容适应尺寸                                |
| Shorthand Properties     | §7.4             | 低     | `grid-template`, `grid` 简写                |
| Named Lines              | §8.4             | 低     | `[line-name]` 命名网格线                    |
| Subgrid                  | CSS Grid Level 2 | 低     | 子网格                                      |

---

## 算法复杂度分析

### 符号定义

| 符号 | 含义                |
| ---- | ------------------- |
| R    | 行数 (rows)         |
| C    | 列数 (columns)      |
| N    | Grid 项目数 (items) |

### 时间复杂度

| 步骤 | 操作                 | 复杂度       | 说明                                   |
| ---- | -------------------- | ------------ | -------------------------------------- |
| 1    | Available Space      | O(1)         | constant-time calculation              |
| 2    | Gutters              | O(1)         | constant-time calculation              |
| 3    | Explicit Grid        | O(R + C)     | 遍历 track definition list             |
| 4    | Placement            | O(N + R × C) | Dense 搜索可能扫描空洞；sparse 为 O(N) |
| 5    | Track Sizing         | O(N)         | 遍历 items 应用 track sizing           |
| 6    | Item Sizing          | O(N)         | 遍历 items 计算 item sizing            |
| 7    | Finalize Tracks      | O(N + R + C) | finalize track base sizes              |
| 8    | Content-distribution | O(R + C)     | 计算 track distribution offsets        |
| 9    | Item Positioning     | O(N)         | 遍历 items 应用 self-alignment         |

**总时间复杂度**: **O(N + R × C)**

- 对于 dense grid（N ≈ R × C），复杂度为 O(R × C)
- 对于 sparse grid（N << R × C），复杂度接近 O(N + R + C)

### 空间复杂度

| 数据结构                 | 复杂度        | 说明                                                        |
| ------------------------ | ------------- | ----------------------------------------------------------- |
| GridMatrix.occupancy     | O(R × C / 64) | 占用状态按位存储（每个单元 1 bit，空间约为密集矩阵的 1/64） |
| GridMatrix.items         | O(N)          | grid items 列表                                             |
| GridLayoutMatrix.items   | O(N)          | layout items 列表                                           |
| GridLayoutMatrix.offsets | O(R + C)      | 预计算的行/列偏移                                           |
| Track Lists              | O(R + C)      | 行/列轨道定义列表                                           |
| each_inline_size         | O(C)          | 列尺寸临时向量                                              |
| each_block_size          | O(R)          | 行尺寸临时向量                                              |

**总空间复杂度**: **O(R × C / 64 + N)**

- 占用状态使用 `OccupiedBitmap` 按位存储，每个单元仅占 1 bit，空间开销约为密集矩阵的 1/64
- Items 独立存储在 Vec 中，支持 O(N) 遍历

---

## 测试覆盖

当前共有 **~243 个** Grid 测试用例，覆盖：

### WPT 测试 (`tests/wpt/css_grid/`)

| 类别                      | 测试数 | 文件                   |
| ------------------------- | ------ | ---------------------- |
| Box Alignment             | 47     | `alignment.rs`         |
| Intrinsic Track Sizing    | 33     | `intrinsic_tracks.rs`  |
| Maximize Tracks           | 25     | `maximize_tracks.rs`   |
| Auto-placement (含 dense) | 22     | `grid_auto_flow.rs`    |
| Basic Layout              | 17     | `grid_basics.rs`       |
| Gutters                   | 15     | `gap.rs`               |
| Explicit Track Sizing     | 14     | `grid_template.rs`     |
| Direction (RTL)           | 13     | `direction.rs`         |
| Flexible Length (`fr`)    | 11     | `fr_unit.rs`           |
| Margin                    | 11     | `margin.rs`            |
| Auto Tracks               | 9      | `grid_auto.rs`         |
| Writing Mode              | 7      | `writing_mode.rs`      |

所有测试断言值均根据 W3C 规范推算，确保符合规范定义的计算逻辑。
