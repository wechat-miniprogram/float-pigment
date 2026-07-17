# HTML 测试框架重构设计

> **For agentic workers:** This is a design spec. After user review, use superpowers:writing-plans to create the implementation plan.

**Goal:** 将 float-pigment-layout 的测试从 `assert_xml!` 宏（XML 字符串 in Rust）重构为 HTML 文件 + build.rs codegen 生成 Rust 测试，且同一批 HTML 能跑 Chrome headless 交叉验证（验证人写 expect == Chrome layout），减少人写 expect 错误。

**背景：** 当前 970 个测试（custom + wpt）都用 `assert_xml!(r#"<div style="..." expect_width="50">...</div>"#)` 宏，XML 字符串嵌在 Rust 里，`expect_*` 是人写期望值（曾出现 `aspect_ratio_with_flex_wrap` 人写错的 pre-existing failure）。HTML 无法直接跑 Chrome 验证。

---

## 架构总览

```
tests/cases/<topic>/<case>.html        ← HTML case（data-expect-* + data-chrome）
  │
  ├─ build.rs 扫描 → tests/generated/html_tests.rs  ← float-pigment 测试
  │     ↓
  │  cargo test（float-pigment layout 验证 data-expect-*）
  │
  └─ node scripts/chrome-cross-check.mjs（puppeteer）
        ↓
     Chrome headless 跑 → 对比 data-expect-*（验证人写 expect == Chrome layout）
        ↓
     CI 独立 step / 本地按需
```

同一批 HTML 文件，两个 runner：
- **Rust**（float-pigment）：build.rs 生成测试，cargo test 跑 layout 断言
- **node**（Chrome 交叉验证）：puppeteer 跑 Chrome，对比人写 expect vs Chrome layout

## HTML case 格式

```html
<!DOCTYPE html>
<div style="display: flex; width: 100px; justify-content: space-around" data-chrome="true">
  <div style="flex-shrink: 0; width: 60px;" data-expect-width="60" data-expect-left="0"></div>
  <div style="flex-shrink: 0; width: 60px;" data-expect-width="60" data-expect-left="60"></div>
</div>
```

- `data-expect-width|height|left|top`：float-pigment 断言值（替代当前 `expect_*`）
- `data-chrome="true|false"`：是否跑 Chrome 验证。默认 true；float-pigment 特有行为（与 Chrome 不同，如 CSS 子集差异）标 false
- 根元素：测试 container；子元素：items，`data-expect-*` 挂在要断言的元素上
- 文件结构：`tests/cases/<topic>/<case>.html`（一目录一主题，每 case 一文件，如 `tests/cases/justify_content/start.html`）

## build.rs codegen（float-pigment 测试）

- 扫描 `tests/cases/**/*.html`
- 每个 HTML 生成一个 `#[test]` 函数（函数名 = 路径转下划线，如 `justify_content__start`）
- 测试函数体：
  1. `TestCtx::from_str(include_str!(html_path))`（编译期嵌入 HTML，运行时不读文件）
  2. `ctx.layout(false)`
  3. 遍历 `[data-expect-*]` 元素，读 `data-expect-*` 属性值
  4. 对比 `layout_position()`（width/height/left/top）
  5. 不一致 panic（断言失败）
- 复用现有 `TestCtx` + `assert_xml!` 内部的 layout/assert 逻辑（从宏提取成可复用函数 `assert_html_layout`）
- 生成到 `tests/generated/html_tests.rs`，`tests/mod.rs` 用 `include!("generated/html_tests.rs")` 引入
- build.rs 在 `float-pigment-forest` crate 的 build script（或 tests 的 build script）

## Chrome cross-check runner（node）

- 脚本：`scripts/chrome-cross-check.mjs`（puppeteer）
- 扫描 `tests/cases/**/*.html`，过滤 `data-chrome="true"`（默认 true，未标也跑）
- 每个 HTML：
  1. puppeteer 打开 `file://` URL
  2. 遍历 `[data-expect-*]` 元素
  3. `getBoundingClientRect()` 取 Chrome layout（width/height/left/top）
  4. 对比 `data-expect-*` 值
  5. 不一致：报告 diff（人写 expect vs Chrome 实际）
- 输出：summary（pass/fail/diff 列表）
- 并行：puppeteer 多 page 复用单 browser，加速 970 case
- 失败语义：Chrome 验证 fail = 「人写 expect != Chrome」，需人工判断（人写错 → 修 expect；float-pigment 特有 → 标 data-chrome=false）

## 迁移策略（970 测试）

- 写转换脚本（Rust 或 node）：解析 `assert_xml!(r#"..."#)` → 提取 XML 字符串 + `expect_*` 属性 → 生成 HTML（`expect_*` → `data-expect-*`，加 `data-chrome="true"`）
- custom + wpt 都转（格式相同，都是 `assert_xml!`）
- 转换后删旧 `.rs` 测试文件，改用 generated
- 渐进：先转一个主题（如 `justify_content/`）验证全流程（codegen + Chrome runner + CI），再批量转剩余
- 转换脚本保留（新测试可手写 HTML，旧宏测试若残留可再转）

## CI 集成

- **float-pigment 测试**（cargo test）：现有 CI 不变，跑 generated `html_tests` + 现有非 HTML 测试
- **Chrome cross-check**：新 CI step
  - `browser-actions/setup-chrome` 或 `actions/setup-node` + puppeteer 自带 Chrome
  - `pnpm install puppeteer`（在 float-pigment-forest 或 repo 根）
  - `node scripts/chrome-cross-check.mjs`
  - 失败 fail CI
- 970 × Chrome 慢：puppeteer 并行（多 page）+ 可分片（CI matrix）

## 非目标

- 不改 float-pigment layout 引擎本身（只改测试框架）
- 不支持 Chrome 不支持的 CSS（float-pigment 特有 CSS 标 `data-chrome=false` 跳过）
- 不自动从 Chrome 生成 expect（expect 仍人写，Chrome 只验证）
- 不替换非 flex 的测试（如 grid、css parsing 等，但框架通用，后续可扩）

## 风险

1. **970 测试迁移工作量大**：转换脚本要处理所有 `assert_xml!` 变体（含 `assert_xml!(xml, true)` 的 dump_style 参数）
2. **Chrome vs float-pigment 差异多**：迁移后大量 Chrome 验证 fail（float-pigment 特有行为），要逐一标 `data-chrome=false` 或修 expect。可能先跑一遍 Chrome runner 摸底
3. **build.rs codegen 调试**：生成代码出错难定位（生成 .rs 看）。生成代码要可读（格式化）
4. **puppeteer 依赖**：CI 装 Chrome + puppeteer，增加 CI 复杂度
5. **TestCtx::from_str 是否支持完整 HTML**：当前 `assert_xml!` 用 XML 片段，HTML 文件要 `<!DOCTYPE html>` 等。确认 TestCtx 解析 HTML 文件（或 codegen 提取 body 内容）

## 成功标准

- 970 测试全迁移到 HTML 文件，cargo test 全过（行为不变）
- Chrome cross-check runner 能跑所有 `data-chrome=true` 的 HTML，报告 diff
- CI 两个 step（cargo test + Chrome cross-check）
- 新测试直接写 HTML，不用 `assert_xml!` 宏
