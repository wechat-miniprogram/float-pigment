# float-pigment-css-napi

[float-pigment-css](https://github.com/wechat-miniprogram/float-pigment) 的 Node.js N-API 绑定 —— 以原生速度把 WXSS/CSS 编译成 bincode 或 JSON，基于 [napi-rs](https://napi.rs/) 的纯 Rust addon。

## 使用

### 安装

```bash
npm install float-pigment-css-napi
```

支持的平台：

| 平台 | 子包 |
|------|------|
| macOS arm64 | `float-pigment-css-napi-darwin-arm64` |
| macOS x64 | `float-pigment-css-napi-darwin-x64` |
| Windows x64 | `float-pigment-css-napi-win32-x64-msvc` |
| Windows x86 | `float-pigment-css-napi-win32-ia32-msvc` |
| Linux x64 (glibc) | `float-pigment-css-napi-linux-x64-gnu` |
| Linux x64 (musl) | `float-pigment-css-napi-linux-x64-musl` |
| Linux arm64 (glibc) | `float-pigment-css-napi-linux-arm64-gnu` |
| Linux arm64 (musl) | `float-pigment-css-napi-linux-arm64-musl` |

### 最小示例

```javascript
const { compileSingleSync } = require('float-pigment-css-napi')

const result = compileSingleSync({
  fileName: 'app.wxss',
  fileContent: Buffer.from('.a { color: red; }'),
  outputType: 'bincode',  // 'bincode' | 'json' | 'none'
})
// result.content: Buffer（序列化产物）
// result.warnings: [{ start: { line, column }, end: { line, column }, message }]
```

批量编译（`compile` / `compileSync`）及异步变体（`compileSingle`）的完整类型定义见自动生成的 [`index.d.ts`](./index.d.ts) —— 所有导出函数和类型都在那里声明。

## 开发

### 构建

前置条件：Rust 工具链 (1.92.0+)、Node.js 16+、pnpm。

| 命令 | 说明 |
|------|------|
| `pnpm install` | 安装开发依赖（`@napi-rs/cli`） |
| `npm run build` | 为当前平台构建原生模块（release） |
| `npm run build:debug` | 当前平台 debug 模式构建 |
| `npm test` | 针对刚构建出的原生模块跑冒烟测试 |

`npm run build` 会产出 `float-pigment-css-napi.<triple>.node`，以及自动生成的 `index.js` 加载器和 `index.d.ts` 类型声明。跨平台产物由 CI（`.github/workflows/napi.yml`）在各自的原生 runner 上构建 —— 本地无需交叉编译工具链。

### 发布

发布由 CI 驱动。推送 `v*` tag 后，`publish.yml` 工作流会在原生 runner 上构建全部 8 个 napi 目标平台，用 `napi artifacts` 收集产物，然后通过 OIDC trusted publishing（无需 npm token）将各平台子包与主包发布到 npm。同一个 `v*` tag 也会发布 Rust crate 和 wasm 包。

### 运行时兼容性

| 运行时 | 支持 |
|--------|------|
| Node.js 16+ | ✓ |
| Node.js 22 | ✓ |
| Electron (主进程) | ✓ |
| Electron (utility process) | ✓ |
| NW.js | ✓ |

N-API 是 ABI 稳定的 —— 同一个 `.node` 二进制在所有兼容的 Node.js 和 Electron 版本上通用，无需重新编译。

## 许可证

MIT
