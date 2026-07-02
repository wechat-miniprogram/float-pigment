# float-pigment-css-napi

[float-pigment-css](https://github.com/wechat-miniprogram/float-pigment) 的 Node.js N-API 绑定，提供高性能 CSS 编译与二进制/JSON 序列化支持。

## 特性

- 纯 Rust 实现，通过 [napi-rs](https://napi.rs/) 绑定 — 无 C++ 中间层
- ABI 稳定的 N-API — 跨 Node.js 和 Electron 版本无需重新编译
- 异步编译通过 libuv 线程池执行（不阻塞主线程）
- 同步编译用于简单场景
- 输出格式：`bincode`（二进制，生产环境）、`json`（可读，调试用）、`none`（仅校验）
- 跨平台预编译：macOS (arm64/x64)、Windows (x64/x86)、Linux (x64/arm64，glibc 与 musl)

## 安装

```bash
npm install float-pigment-css-napi
```

对应平台的原生二进制通过 `optionalDependencies` 自动分发 — npm/pnpm 会根据
`os`/`cpu`/`libc` 解析出匹配的 `float-pigment-css-napi-<平台>` 子包，因此你只会
下载自己平台真正需要的那个二进制。

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

然后在代码中：

```javascript
const { compile, compileSync } = require('float-pigment-css-napi')
```

## 使用方法

```javascript
const { compile, compileSync, compileSingle, compileSingleSync } = require('float-pigment-css-napi')

// 批量编译多个文件（异步）
const result = await compile({
  src: [
    { path: 'pages/index/index.wxss', content: Buffer.from('.container { display: flex; }') },
    { path: 'pages/home/home.wxss', content: Buffer.from('.title { color: red; }') },
  ],
  outputType: 'bincode',
  tagNamePrefix: 'wx-',  // 可选，默认 "wx-"
})

// result.files: [{ path, file: { content: Buffer, warnings: [] } }, ...]
// result.importIndex: Buffer

// 批量编译（同步）
const syncResult = compileSync({
  src: [{ path: 'app.wxss', content: Buffer.from('page { margin: 0; }') }],
  outputType: 'json',
})

// 单文件编译（异步）
const file = await compileSingle({
  fileName: 'components/button/button.wxss',
  fileContent: Buffer.from('.btn { padding: 10px; }'),
  outputType: 'bincode',
})

// file.content: Buffer
// file.warnings: [{ start: { line, column }, end: { line, column }, message }]

// 单文件编译（同步）
const fileSync = compileSingleSync({
  fileName: 'app.wxss',
  fileContent: Buffer.from('body { font-size: 14px; }'),
  outputType: 'none',  // 仅校验，content 为 null
})
```

## API

### `compile(args: CompileArgument): Promise<CompileResult>`

异步批量编译多个 CSS 文件（在 libuv 线程池中执行）。

### `compileSync(args: CompileArgument): CompileResult`

`compile` 的同步版本。

### `compileSingle(args: CompileSingleArgument): Promise<FileResult>`

异步编译单个 CSS 文件。

### `compileSingleSync(args: CompileSingleArgument): FileResult`

`compileSingle` 的同步版本。

### 类型定义

```typescript
type OutputType = 'bincode' | 'json' | 'none'

interface CompileArgument {
  src: SourceEntry[]
  outputType: OutputType
  tagNamePrefix?: string  // 默认: "wx-"
}

interface SourceEntry {
  path: string
  content: Buffer
}

interface CompileResult {
  files: FileEntry[]
  importIndex: Buffer | null
}

interface FileEntry {
  path: string
  file: FileResult
}

interface CompileSingleArgument {
  fileName: string
  fileContent: Buffer
  outputType: OutputType
  tagNamePrefix?: string  // 默认: "wx-"
}

interface FileResult {
  content: Buffer | null
  warnings: CompileWarning[]
}

interface CompileWarning {
  start: { line: number; column: number }
  end: { line: number; column: number }
  message: string
}
```

## 构建

### 前置条件

- Rust 工具链 (1.92.0+)
- Node.js 16+
- pnpm

### 构建与测试

| 命令 | 说明 |
|------|------|
| `pnpm install` | 安装开发依赖（`@napi-rs/cli`） |
| `npm run build` | 为当前平台构建原生模块（release） |
| `npm run build:debug` | 当前平台 debug 模式构建 |
| `npm test` | 针对刚构建出的原生模块跑冒烟测试 |

`npm run build` 会产出 `float-pigment-css-napi.<triple>.node`，以及自动生成的
`index.js` 加载器和 `index.d.ts` 类型声明。跨平台产物由 CI
（`.github/workflows/napi.yml`）在各自的原生 runner 上构建 — 本地无需交叉编译
工具链。

### 验证构建

```bash
node -e "const m = require('./'); console.log(Object.keys(m))"
# 期望输出: [ 'OutputType', 'compileSync', 'compileSingleSync', 'compile', 'compileSingle' ]
```

## 发布

发布由 CI 驱动。推送 `napi-v*` tag 后，工作流会在原生 runner 上构建全部 8 个
目标平台，用 `napi artifacts` 收集产物，然后将各平台子包与主包一并发布到 npm
（需要仓库配置 `NPM_TOKEN` secret）。

## 运行时兼容性

| 运行时 | 支持 |
|--------|------|
| Node.js 16+ | ✓ |
| Node.js 22 | ✓ |
| Electron (主进程) | ✓ |
| Electron (utility process) | ✓ |
| NW.js | ✓ |

N-API 是 ABI 稳定的 — 同一个 `.node` 二进制在所有兼容的 Node.js 和 Electron 版本上通用，无需重新编译。

## 许可证

MIT
