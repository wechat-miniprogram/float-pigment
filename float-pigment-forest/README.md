# float-pigment-forest

A node tree implementation for float-pigment-layout.

This is a crate of the [float-pigment](https://github.com/wechat-miniprogram/float-pigment) project. See its documentation for details.

## Testing

The test suite uses an **HTML-based imperative codegen** framework. You author declarative HTML test cases; `build.rs` compiles each into a self-contained imperative Rust `#[test]` at build time (no hand-written test code).

### Run tests

```bash
cargo test -p float-pigment-forest
```

### Add a test case

1. Create `tests/cases/<topic>/<name>.html`:

```html
<div style="display: flex; width: 200px; height: 100px;">
  <div style="width: 80px; height: 40px;" data-expect-width="80" data-expect-left="0"></div>
  <div style="width: 120px; height: 40px;" data-expect-width="120" data-expect-left="80"></div>
</div>
```

2. Run `cargo test -p float-pigment-forest`. `build.rs` auto-generates `tests/generated/<topic>/<name>.rs` (imperative) and runs it.

3. Inspect the generated code:

```bash
cat tests/generated/<topic>/<name>.rs
```

### Assertion attributes

Write `data-expect-*` on the element(s) you want to assert. Values are numbers (pixels); only write the axes you check.

| HTML attribute | Generated assertion |
|---|---|
| `data-expect-width="V"` | `assert_eq!(ctx.width(n).round(), V.0)` |
| `data-expect-height="V"` | `assert_eq!(ctx.height(n).round(), V.0)` |
| `data-expect-left="V"` | `assert_eq!(ctx.left(n).round(), V.0)` |
| `data-expect-top="V"` | `assert_eq!(ctx.top(n).round(), V.0)` |
| `data-expect-margin-top/right/bottom/left="V"` | `assert_eq!(ctx.margin_<side>(n).round(), V.0)` |
| `data-ignore="true"` | marks the `#[test]` as `#[ignore]` |

Text nodes (`>text<`) compile to `ctx.create_text("text")`.

### Generated code shape

Each case compiles to an imperative test under `tests/generated/` (gitignored, regenerated every build):

```rust
// AUTO-GENERATED from tests/cases/<topic>/<name>.html. Do not edit.
use crate::TestCtx;

#[test]
fn html_<topic>_<name>() {
    let mut ctx = TestCtx::new();
    let n0 = ctx.create_node("div");
    ctx.set_style(n0, "display: flex; width: 200px; ...");
    let n1 = ctx.create_node("div");
    ctx.append(n0, n1);
    ctx.layout_imperative();
    assert_eq!(ctx.width(n1).round(), 80.0);
}
```

- **Test fn name**: `html_<topic>_<name>` (path separators and `-` → `_`). Filter with `cargo test html_<keyword>`.
- **Edit a case** → `cargo test` regenerates automatically (`rerun-if-changed=tests/cases`).
- **`tests/generated/` is gitignored** — HTML cases are the source of truth, generated code is a build artifact.

### Chrome cross-check (optional)

`scripts/chrome-cross-check.mjs` opens each case whose `data-chrome != "false"` in headless Chrome and compares `getBoundingClientRect` against `data-expect-*` — useful for catching cases where float-pigment's layout diverges from a real browser. Run from the repo root:

```bash
cd scripts && pnpm install && node chrome-cross-check.mjs
```

Set `data-chrome="false"` on the root element of a case that intentionally diverges from Chrome (e.g. float-pigment-specific behavior) to exclude it.
