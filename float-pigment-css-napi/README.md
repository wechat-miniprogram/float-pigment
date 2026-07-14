# float-pigment-css-napi

Node.js N-API binding for [float-pigment-css](https://github.com/wechat-miniprogram/float-pigment) — compiles WXSS/CSS into bincode or JSON at native speed, via a pure-Rust addon built with [napi-rs](https://napi.rs/).

## Usage

### Install

```bash
npm install float-pigment-css-napi
```

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

### Minimal example

```javascript
const { compileSingleSync } = require('float-pigment-css-napi')

const result = compileSingleSync({
  fileName: 'app.wxss',
  fileContent: Buffer.from('.a { color: red; }'),
  outputType: 'bincode',  // 'bincode' | 'json' | 'none'
})
// result.content: Buffer (serialized output)
// result.warnings: [{ start: { line, column }, end: { line, column }, message }]
```

For batch compilation (`compile` / `compileSync`) and the async variants (`compileSingle`), see the generated [`index.d.ts`](./index.d.ts) for full type definitions — every exported function and type is declared there.

## Development

### Build

Prerequisites: Rust toolchain (1.92.0+), Node.js 16+, pnpm.

| Command | Description |
|---------|-------------|
| `pnpm install` | Install dev dependencies (`@napi-rs/cli`) |
| `npm run build` | Build the native addon for the current platform (release) |
| `npm run build:debug` | Build for the current platform in debug mode |
| `npm test` | Run the smoke tests against the freshly built addon |

`npm run build` emits `float-pigment-css-napi.<triple>.node` plus the generated `index.js` loader and `index.d.ts` types. Cross-platform artifacts are produced by CI (`.github/workflows/napi.yml`) on native runners — no local cross-compilation toolchain is required.

### Publishing

Releases are driven by CI. Push a `napi-v*` tag; the workflow builds all eight targets on native runners, collects the artifacts with `napi artifacts`, and publishes the per-platform packages plus the main package to npm.

### Runtime Compatibility

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
