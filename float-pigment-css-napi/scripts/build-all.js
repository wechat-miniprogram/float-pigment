/**
 * Build all target platforms sequentially.
 * Targets: darwin-arm64, darwin-x64, win32-x64, win32-x32
 */
const { execSync } = require('child_process');
const path = require('path');

const root = path.resolve(__dirname, '..');
const napi = path.join(root, 'node_modules', '.bin', 'napi');

const TARGETS = [
  'aarch64-apple-darwin',
  'x86_64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'i686-pc-windows-msvc',
];

for (const target of TARGETS) {
  console.log(`\n=== Building ${target} ===\n`);
  try {
    execSync(
      `${napi} build --platform --release --target ${target} --js false --dts type.d.ts`,
      { cwd: root, stdio: 'inherit' }
    );
  } catch (e) {
    console.error(`Failed to build ${target}: ${e.message}`);
    console.error('Make sure the target is installed: rustup target add ' + target);
    console.error('For Windows cross-compile, install cargo-xwin: cargo install cargo-xwin');
    process.exit(1);
  }
}

// Move all .node files into prebuilds/<platform>/node.napi.node
require('./postbuild.js');
