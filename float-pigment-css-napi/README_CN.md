# float-pigment-css-napi

[float-pigment-css](https://github.com/wechat-miniprogram/float-pigment) 的 Node.js N-API 绑定，提供高性能 CSS 编译与二进制/JSON 序列化支持。

## 特性

- 纯 Rust 实现，通过 [napi-rs](https://napi.rs/) 绑定 — 无 C++ 中间层
- ABI 稳定的 N-API — 跨 Node.js 和 Electron 版本无需重新编译
- 异步编译通过 libuv 线程池执行（不阻塞主线程）
- 同步编译用于简单场景
- 输出格式：`bincode`（二进制，生产环境）、`json`（可读，调试用）、`none`（仅校验）
- 跨平台预编译：macOS (arm64/x64)、Windows (x64/x86)

## 安装

从源码构建（尚未发布到 npm）：

```bash
cd float-pigment-css-napi
npm install
npm run build
```

一条命令编译全部 4 个平台：
- `darwin-arm64`（macOS Apple Silicon）
- `darwin-x64`（macOS Intel）
- `win32-x64`（Windows 64 位）
- `win32-x32`（Windows 32 位）

构建后目录结构：

```
prebuilds/
├── darwin-arm64/node.napi.node
├── darwin-x64/node.napi.node
├── win32-x32/node.napi.node
└── win32-x64/node.napi.node
index.js          ← 平台加载器
type.d.ts         ← TypeScript 类型声明
```

### 集成到你的项目

```bash
# 方式一：直接拷贝产物
cp -r prebuilds index.js type.d.ts /path/to/your-project/deps/float-pigment-css-napi/

# 方式二：通过 file: 相对路径引用（适合 monorepo）
# 在项目 package.json 中：
#   "dependencies": {
#     "float-pigment-css-napi": "file:../float-pigment-css-napi"
#   }
```

然后在代码中：

```javascript
// 方式一：拷贝到项目内
const { compile, compileSync } = require('./deps/float-pigment-css-napi')

// 方式二：file: 引用
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
- npm 或 pnpm
- 交叉编译依赖（从 macOS 编译 Windows 目标）：
  ```bash
  rustup target add x86_64-pc-windows-msvc i686-pc-windows-msvc
  cargo install cargo-xwin
  ```

### 构建脚本

| 命令 | 说明 |
|------|------|
| `npm run build` | 编译全部 4 个平台 (darwin-arm64, darwin-x64, win32-x64, win32-x32) |
| `npm run build:current` | 仅编译当前平台 |
| `npm run build:debug` | 当前平台 debug 模式 |
| `npm run build:target <triple>` | 编译指定 Rust target triple |

### 验证构建

```bash
# 检查 prebuilds 结构
ls prebuilds/*/node.napi.node

# 测试加载
node -e "const m = require('./'); console.log(Object.keys(m))"
# 期望输出: [ 'compile', 'compileSync', 'compileSingle', 'compileSingleSync', ... ]
```

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
