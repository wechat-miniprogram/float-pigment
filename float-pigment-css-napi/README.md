# float-pigment-css-napi

Node.js N-API binding for [float-pigment-css](https://github.com/wechat-miniprogram/float-pigment), providing high-performance CSS compilation with binary/JSON serialization support.

## Features

- Pure Rust implementation via [napi-rs](https://napi.rs/) — no C++ intermediate layer
- ABI-stable N-API — works across Node.js and Electron versions without recompilation
- Async compilation via libuv thread pool (non-blocking)
- Sync compilation for simple use cases
- Output formats: `bincode` (binary, production), `json` (readable, debug), `none` (validation only)
- Cross-platform prebuilds: macOS (arm64/x64), Windows (x64/x86), Linux (x64/arm64, glibc & musl)

## Installation

```bash
npm install float-pigment-css-napi
```

The native binary for your platform is delivered automatically via
`optionalDependencies` — npm/pnpm resolves the matching
`float-pigment-css-napi-<platform>` package based on `os`/`cpu`/`libc`, so you
only download the binary you actually need.

Supported platforms:

| Platform | Package |
|----------|---------|
| macOS arm64 | `float-pigment-css-napi-darwin-arm64` |
| macOS x64 | `float-pigment-css-napi-darwin-x64` |
| Windows x64 | `float-pigment-css-napi-win32-x64-msvc` |
| Windows x86 | `float-pigment-css-napi-win32-ia32-msvc` |
| Linux x64 (glibc) | `float-pigment-css-napi-linux-x64-gnu` |
| Linux x64 (musl) | `float-pigment-css-napi-linux-x64-musl` |
| Linux arm64 (glibc) | `float-pigment-css-napi-linux-arm64-gnu` |
| Linux arm64 (musl) | `float-pigment-css-napi-linux-arm64-musl` |

Then in your code:

```javascript
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
- pnpm

### Build & test

| Command | Description |
|---------|-------------|
| `pnpm install` | Install dev dependencies (`@napi-rs/cli`) |
| `npm run build` | Build the native addon for the current platform (release) |
| `npm run build:debug` | Build for the current platform in debug mode |
| `npm test` | Run the smoke tests against the freshly built addon |

`npm run build` emits `float-pigment-css-napi.<triple>.node` plus the generated
`index.js` loader and `index.d.ts` types. Cross-platform artifacts are produced
by CI (`.github/workflows/napi.yml`) on native runners — no local
cross-compilation toolchain is required.

### Verify build

```bash
node -e "const m = require('./'); console.log(Object.keys(m))"
# Expected: [ 'OutputType', 'compileSync', 'compileSingleSync', 'compile', 'compileSingle' ]
```

## Publishing

Releases are driven by CI. Push a `napi-v*` tag; the workflow builds all eight
targets on native runners, collects the artifacts with `napi artifacts`, and
publishes the per-platform packages plus the main package to npm (requires the
`NPM_TOKEN` repository secret).

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
