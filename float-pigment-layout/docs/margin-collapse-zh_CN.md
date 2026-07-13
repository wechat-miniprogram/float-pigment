# CSS Margin Collapsing 实现文档

本文档描述 `float-pigment-layout` 中 margin collapsing（外边距合并）的实现，对照 W3C §8.3.1 标准。

## 规范参考

- [CSS 2.1 §8.3.1 Collapsing margins](https://www.w3.org/TR/CSS21/box.html#collapsing-margins)
- [CSS 2.1 §9.4.1 Block formatting contexts](https://www.w3.org/TR/CSS21/visuren.html#block-formatting)
- [CSS Flexbox §3 Flex Containers](https://www.w3.org/TR/css-flexbox-1/#flex-containers)
- [CSS Grid Layout §3](https://www.w3.org/TR/css-grid-1/#grid-containers)

---

## 模块结构

```
float-pigment-layout/src/algo/flow.rs
├── establishes_bfc()              # 统一 BFC 判定（当前 accessor 覆盖范围）
├── is_margin_start_collapsible()  # §8.3.1 关系 (a)：父-首子 top
├── is_margin_end_collapsible()    # §8.3.1 关系 (c)：父-末子 bottom（含 height==auto 直判）
├── is_empty_block()               # §8.3.1 关系 (d)：空块 collapsed-through
└── Flow::compute_block_or_inline_series()  # 主流程：父子+兄弟塌陷计算

float-pigment-layout/src/unit.rs
├── CollapsedBlockMargin / CollapsedMargin  # 正/负分量分离的塌陷 margin 数据结构
└── unit.compute_with_containing_size       # 布局根入口（读 collapsed_margin 写回根）

float-pigment-layout/src/algo/flex_box.rs / grid/mod.rs
└── 各自的 Flow::compute —— 走 from_margin() 直接构造 collapsed_margin，不经 flow.rs 塌陷路径
```

---

## 核心概念

### 根元素

引擎里的"根元素"= **`node.parent().is_none()`** 的节点（CSS 语义：无父 = 文档树根）。

`.layout()` 可对子树调用（增量布局），但子树顶点有父时仍正常参与 collapse——只有真正无父的节点才扮演根角色、不与子 collapse。

### BFC 阻断的双向语义

CSS §8.3.1 规定 adjoining margins 必须属**同一 BFC**。跨 BFC 边界的 margin 不塌陷，这要求两个方向都阻断：

- **父端**：BFC 容器不与子的 margin 塌陷（如 flex 容器与 flex item 之间）
- **子端**：BFC 子不与父/兄弟的 margin 塌陷（如 flex 子在普通 block 父里）

### 信号传递：`bfc_established` 局部计算

BFC 阻断信号 `bfc_established` 是**节点固有属性**（display/position/有无父），在 `compute_block_or_inline_series` 开头局部计算，三处复用（start/end/empty 判定）。

**不通过 `ComputeRequest` 全局传播**——保证 BFC 阻断只影响 block flow 路径，flex/grid 布局零波及（它们走自己的 `Flex::compute` / `Grid::compute`，不读 `bfc_established`）。

---

## 算法流程

### W3C §8.3.1 的 adjoining 条件

两条 margin 是 **adjoining**（可合并）当且仅当：

1. 都属于**同一 BFC** 内的 in-flow block-level boxes
2. 中间无 line boxes、无 clearance、无 padding、无 border 分隔
3. 构成以下四种相邻关系之一：
   - **(a)** 盒的 top margin 与其**第一个 in-flow 子**的 top margin
   - **(b)** 盒的 bottom margin 与其**下一个 in-flow 兄弟**的 top margin
   - **(c)** **最后一个 in-flow 子**的 bottom margin 与父的 bottom margin（**父 `height:auto`**）
   - **(d)** 不建立新 BFC 且 `min-height:0`、`height:0|auto`、无 in-flow 子的盒，其 top 与 bottom margin（空块穿透 collapsed-through）

### float-pigment 实现流程

`Flow::compute_block_or_inline_series` 处理一个 block 容器与其子序列的塌陷计算。塌陷链维护在 `prev_sibling_collapsed_margin` 状态变量，逐子推进：

```
┌───────────────────────────────────────────────────────────────────┐
│              float-pigment Margin Collapse Flow                   │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 1  算 bfc_established = establishes_bfc(node)                │  │
│  │  当前节点是否建立 BFC（不与子塌陷）                         │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 2  parent_margin_start_collapsible =                  │  │
│  │    is_margin_start_collapsible(bfc_established, parent_is_block,   │  │
│  │                                 padding_border_start)       │  │
│  │  §8.3.1 关系 (a)                                            │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 3  遍历 in-flow 子（跳过 display:none / abs/fixed）   │  │
│  │                                                             │  │
│  │   for child in node.children:                              │  │
│  │     child_res = child.compute_internal(...)                │  │
│  │                                                             │  │
│  │     ┌── 子是 BFC？(establishes_bfc(child)) ──┐             │  │
│  │     │                                        │             │  │
│  │     ▼ 是                                     ▼ 否          │  │
│  │  结算 prev chain 到 offset              兄弟塌陷：           │  │
│  │  BFC 子 start/end margin 作             prev.adjoin(        │  │
│  │    直接位移（不 adjoin）                  child.start)       │  │
│  │  设 prev = zero sentinel              处理 collapsed_through│  │
│  │    （让下一个兄弟走 sibling 分支，        更新 prev           │  │
│  │     而非 first-child 分支）                                  │  │
│  │                                                             │  │
│  │     父-首子塌陷（prev == None 且 STEP 2 = true）:           │  │
│  │       parent_collapsed_margin_start.adjoin(child.start)     │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 4  height_is_auto = matches!(node.style().height(),  │  │
│  │            DefLength::Auto | DefLength::Undefined)          │  │
│  │          parent_margin_end_collapsible =                    │  │
│  │    is_margin_end_collapsible(bfc_established, height_is_auto, ...) │  │
│  │  §8.3.1 关系 (c)                                            │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 5  若 STEP 4 = true，末子 bottom 并入                 │  │
│  │          parent_collapsed_margin_end                        │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 6  collapsed_through =                                │  │
│  │    is_empty_block(bfc_established, ...)                            │  │
│  │  §8.3.1 关系 (d)（BFC 容器不 collapsed-through）            │  │
│  └──────────────────────────┬──────────────────────────────────┘  │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │  STEP 7  输出 BlockOrInlineSeriesComputeResult              │  │
│  │  { size, collapsed_margin: (start, end, collapsed_through) }│  │
│  │  上层据此定位容器                                           │  │
│  └─────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────┘
```

### BFC 子分支的处理（STEP 3 的"是"分支）

当 `establishes_bfc(child)` 为真时：

1. 结算 prev sibling collapse chain：把累积的 `prev_sibling_collapsed_margin` solve 后加到 `main_offset` 和 `total_main_size`
2. BFC 子的 `start` / `end` margin **solve 后作直接位移**加到 offset（不 `adjoin` 进 chain）
3. 设 `prev_sibling_collapsed_margin = Some((CollapsedMargin::zero(), false))`——零值哨兵

**为什么用零值哨兵而不是 `None`**：设 `None` 会让下一个非 BFC 兄弟走 first-child 分支，该分支把它的 top margin adjoin 到 `parent_collapsed_margin_start`——错误地把兄弟的 margin 上浮到父的 top。零值哨兵强制下一个兄弟走 sibling-collapse 分支，那里 `adjoin(zero, x) = x`，margin 作为直接位移应用，无错误上浮。哨兵在末子收尾时无害（zero adjoin 进 parent_end 是 no-op；zero solve 加到 total 是 no-op）。

这保证 BFC 子的两端 margin 都独立作用，不与父/兄弟塌陷，且塌陷链对所有后续兄弟保持切断状态。

---

## W3C 规范对比

### 12 条对照表

| # | 审查项 | 标准 | 状态 | 关键证据 |
|---|--------|:---:|:---:|----------|
| 1 | (a) 父-首子 top | border/padding 阻断 + BFC + clearance | ⚠️ 部分 | `is_margin_start_collapsible`：border/padding ✓、bfc_established ✓；**clearance 缺失**（需 `clear()` accessor） |
| 2 | (b) 兄弟 bottom-top | 后兄弟 clearance 阻断 | ⚠️ 部分 | `flow.rs` 兄弟分支算法对；**clearance 缺失** |
| 3 | (c) 父-末子 bottom | height:auto + min-height:0 + 无 pb/border + clearance | ✅ | `is_margin_end_collapsible`：直接判 `node.style().height()` 是否 Auto/Undefined |
| 4 | (d) 空块穿透 | 不建 BFC + min:0 + h:0\|auto + 无子 | ✅ | `is_empty_block`：含 `bfc_established` 短路（BFC 容器不穿透） |
| 5 | 根元素不合并 | 绝对规则 | ✅ | `establishes_bfc`：`parent().is_none()` |
| 6 | BFC(overflow) 不与子合并 | overflow≠visible | ⚠️ 部分 | display 类 ✓；**overflow 类需 `overflow()` accessor** |
| 7 | float 不合并 | float≠none | ❌ | **需 `float()` accessor** |
| 8 | 绝对定位不合并 | abs/fixed | ✅ | `establishes_bfc`：Absolute/Fixed |
| 9 | inline-block 不与子合并 | inline-block | ✅ | `establishes_bfc`：InlineBlock |
| 10 | 水平 margin 不合并 | 水平永不 | ✅ | collapse 只在 main axis（`flow.rs:345-355`） |
| 11 | 正负 margin 合并 | 正max/负min/相加 | ✅ | `CollapsedMargin` 分离正负、`adjoin` max/min、`solve` 相加 |
| 12 | 同一 BFC 前提 | 跨 BFC 不合并 | ✅ | 父端 `bfc_established` + 子端 BFC 分支，双向阻断（当前 accessor 覆盖内） |

**额外覆盖**（display 类 BFC）：`Flex` / `InlineFlex` / `Grid` / `InlineGrid` / `FlowRoot` —— 都通过 `establishes_bfc` 判定，符合 Flexbox §3 / Grid §3。

### `establishes_bfc()` 实现

```rust
fn establishes_bfc<T: LayoutTreeNode>(node: &T) -> bool {
    if node.tree_visitor().parent().is_none() { return true; }  // 根元素
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

### `CollapsedMargin` 正负分量分离

CSS §8.3.1 规定：正 margin 取 max，负 margin 取 min，正负混合时正 max + 负 min。

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

## 未实现功能（TODO）

| 优先级 | 项 | 阻塞原因 |
|:---:|------|---------|
| 中 | 加 `LayoutStyle::float()` accessor | trait 扩展 |
| 中 | 加 `LayoutStyle::overflow()` accessor | trait 扩展 |
| 中 | 加 `LayoutStyle::clear()` accessor | trait 扩展 |
| 中 | #7 float 不合并 | 依赖 `float()` accessor |
| 中 | #6 overflow≠visible 建 BFC | 依赖 `overflow()` accessor |
| 中 | #1/#2 clearance 阻断（关系 a/b/c/d） | 依赖 `clear()` accessor |
| 低 | `Display::FlowRoot` 布局路径 | `flow.rs:128` 是 `todo!()`，独立布局算法 |

加 accessor 是独立工程，涉及 5 处改动：
1. `LayoutStyle` trait 加方法
2. forest `Node` 实现（`layout_impl.rs`）
3. forest `StyleManager` 存字段
4. forest `StyleSetter` 加 set 方法
5. forest `set_style` 解析 "overflow"/"float"/"clear" 字符串 + mlp `Element` 属性读取

---

## 测试覆盖

### 测试组织

- **核心用例**：`float-pigment-forest/tests/custom/css_margin_collapse.rs` —— collapse 专项，覆盖根不塌陷 + 非根父子塌陷正反例
- **既有用例**：`float-pigment-forest/tests/custom/css_margin.rs` —— margin 各种场景，含 cross_flex 系列（BFC 子塌陷）
- **WPT 用例**：`float-pigment-forest/tests/wpt/css_display/display_flex.rs` —— flex + margin

### 关键测试

| 测试 | 验证 | 期望值依据 |
|------|------|----------|
| `entry_node_margin_does_not_collapse_with_child` | 根（无父）的 margin 不与子塌陷 | CSS §8.3.1 |
| `non_entry_parent_collapses_with_first_child` | 非根父子正常塌陷 | CSS §8.3.1 关系 (a) |
| `block_bfc_block_sequence_no_propagation` | [block, BFC, block]——BFC 之后的兄弟 margin 不上浮到父 | CSS §8.3.1（BFC 切断链）|
| `margin_collapse_cross_flex` 1/2/4/5/6 | flex 容器作子不与父塌陷 | Flexbox §3 |
| `display_flex_with_margin` | flex 容器 margin 独立作用 | Flexbox §3 |
| `margin_collapse_min_height` / `_2` | min-height 不阻断末子塌陷（只 height 才阻断） | CSS §8.3.1 关系 (c) |
| `margin_collapse_max_height` / `_2` | max-height 不阻断末子塌陷 | CSS §8.3.1 关系 (c) |
| `margin_root` / `_3` / `_4` | 入口节点（无父）margin 独立 | CSS §8.3.1 |

### 测试结果

- `cargo test -p float-pigment-forest`：958 passed / 0 failed / 1 ignored
- `cargo clippy -p float-pigment-layout -- -D warnings`：无 issue

---

**参考**：
- [CSS 2.1 §8.3.1 Collapsing margins](https://www.w3.org/TR/CSS21/box.html#collapsing-margins)
- [CSS 2.1 §9.4.1 Block formatting contexts](https://www.w3.org/TR/CSS21/visuren.html#block-formatting)
- [CSS Flexbox §3 Flex Containers](https://www.w3.org/TR/css-flexbox-1/#flex-containers)
- [MDN Mastering margin collapsing](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_model/Mastering_margin_collapsing)
