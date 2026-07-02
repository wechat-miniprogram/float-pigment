# float-pigment-css-napi

Node.js N-API binding for [float-pigment-css](https://github.com/wechat-miniprogram/float-pigment), providing high-performance CSS compilation with binary/JSON serialization support.

## Features

- Pure Rust implementation via [napi-rs](https://napi.rs/) — no C++ intermediate layer
- ABI-stable N-API — works across Node.js and Electron versions without recompilation
- Async compilation via libuv thread pool (non-blocking)
- Sync compilation for simple use cases
- Output formats: `bincode` (binary, production), `json` (readable, debug), `none` (validation only)
- Cross-platform prebuilds: macOS (arm64/x64), Windows (x64/x86)

## Installation

Build from source (not yet published to npm):

```bash
cd float-pigment-css-napi
npm install
npm run build
```

This builds all 4 supported platforms in one command:
- `darwin-arm64` (macOS Apple Silicon)
- `darwin-x64` (macOS Intel)
- `win32-x64` (Windows 64-bit)
- `win32-x32` (Windows 32-bit)

After build, the directory structure is:

```
prebuilds/
├── darwin-arm64/node.napi.node
├── darwin-x64/node.napi.node
├── win32-x32/node.napi.node
└── win32-x64/node.napi.node
index.js          ← platform loader
type.d.ts         ← TypeScript declarations
```

### Integrating into your project

```bash
# Option 1: Copy artifacts directly
cp -r prebuilds index.js type.d.ts /path/to/your-project/deps/float-pigment-css-napi/

# Option 2: Use file: reference (suitable for monorepo)
# In your project's package.json:
#   "dependencies": {
#     "float-pigment-css-napi": "file:../float-pigment-css-napi"
#   }
```

Then in your code:

```javascript
// Option 1: copied into project
const { compile, compileSync } = require('./deps/float-pigment-css-napi')

// Option 2: file: reference
const { compile, compileSync } = require('float-pigment-css-napi')
```

## Usage

```javascript
const { compile, compileSync, compileSingle, compileSingleSync } = require('float-pigment-css-napi')

// Batch compile multiple files (async)
const result = await compile({
  src: [
    { path: 'pages/index/index.wxss', content: Buffer.from('.container { display: flex; }') },
    { path: 'pages/home/home.wxss', content: Buffer.from('.title { color: red; }') },
  ],
  outputType: 'bincode',
  tagNamePrefix: 'wx-',  // optional, default "wx-"
})

// result.files: [{ path, file: { content: Buffer, warnings: [] } }, ...]
// result.importIndex: Buffer

// Batch compile (sync)
const syncResult = compileSync({
  src: [{ path: 'app.wxss', content: Buffer.from('page { margin: 0; }') }],
  outputType: 'json',
})

// Single file compile (async)
const file = await compileSingle({
  fileName: 'components/button/button.wxss',
  fileContent: Buffer.from('.btn { padding: 10px; }'),
  outputType: 'bincode',
})

// file.content: Buffer
// file.warnings: [{ start: { line, column }, end: { line, column }, message }]

// Single file compile (sync)
const fileSync = compileSingleSync({
  fileName: 'app.wxss',
  fileContent: Buffer.from('body { font-size: 14px; }'),
  outputType: 'none',  // validation only, content will be null
})
```

## API

### `compile(args: CompileArgument): Promise<CompileResult>`

Compile multiple CSS files asynchronously (runs on libuv thread pool).

### `compileSync(args: CompileArgument): CompileResult`

Synchronous version of `compile`.

### `compileSingle(args: CompileSingleArgument): Promise<FileResult>`

Compile a single CSS file asynchronously.

### `compileSingleSync(args: CompileSingleArgument): FileResult`

Synchronous version of `compileSingle`.

### Types

```typescript
type OutputType = 'bincode' | 'json' | 'none'

interface CompileArgument {
  src: SourceEntry[]
  outputType: OutputType
  tagNamePrefix?: string  // default: "wx-"
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
  tagNamePrefix?: string  // default: "wx-"
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

## Build

### Prerequisites

- Rust toolchain (1.92.0+)
- Node.js 16+
- npm or pnpm
- Cross-compile dependencies (for Windows targets from macOS):
  ```bash
  rustup target add x86_64-pc-windows-msvc i686-pc-windows-msvc
  cargo install cargo-xwin
  ```

### Build scripts

| Command | Description |
|---------|-------------|
| `npm run build` | Build all 4 platforms (darwin-arm64, darwin-x64, win32-x64, win32-x32) |
| `npm run build:current` | Build for current platform only |
| `npm run build:debug` | Build current platform in debug mode |
| `npm run build:target <triple>` | Build for a specific Rust target triple |

### Verify build

```bash
# Check prebuilds structure
ls prebuilds/*/node.napi.node

# Test loading
node -e "const m = require('./'); console.log(Object.keys(m))"
# Expected: [ 'compile', 'compileSync', 'compileSingle', 'compileSingleSync', ... ]
```

## Runtime Compatibility

| Runtime | Supported |
|---------|-----------|
| Node.js 16+ | ✓ |
| Node.js 22 | ✓ |
| Electron (main process) | ✓ |
| Electron (utility process) | ✓ |
| NW.js | ✓ |

N-API is ABI-stable — the same `.node` binary works across all compatible Node.js and Electron versions without recompilation.

## License

MIT
