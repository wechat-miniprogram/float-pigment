/**
 * Build for current platform only (convenience for local dev).
 */
const { execSync } = require('child_process');
const path = require('path');
const os = require('os');

const root = path.resolve(__dirname, '..');
const napi = path.join(root, 'node_modules', '.bin', 'napi');

// Detect current platform target triple
const PLATFORM_MAP = {
  'darwin-arm64': 'aarch64-apple-darwin',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'win32-x64': 'x86_64-pc-windows-msvc',
  'win32-ia32': 'i686-pc-windows-msvc',
};

const key = `${process.platform}-${process.arch}`;
const target = PLATFORM_MAP[key];

if (!target) {
  console.error(`Unsupported platform: ${key}`);
  process.exit(1);
}

const profile = process.argv.includes('--debug') ? '' : '--release';

console.log(`Building for ${target}...\n`);
execSync(
  `${napi} build --platform ${profile} --target ${target} --js false --dts type.d.ts`,
  { cwd: root, stdio: 'inherit' }
);

require('./postbuild.js');
