/**
 * Build for a specific target platform.
 * Usage: node scripts/build-target.js <target-triple>
 * Example: node scripts/build-target.js x86_64-pc-windows-msvc
 */
const { execSync } = require('child_process');
const path = require('path');

const target = process.argv[2];
if (!target) {
  console.error('Usage: node scripts/build-target.js <target-triple>');
  console.error('Example: node scripts/build-target.js x86_64-pc-windows-msvc');
  process.exit(1);
}

const root = path.resolve(__dirname, '..');
const napi = path.join(root, 'node_modules', '.bin', 'napi');

execSync(
  `${napi} build --platform --release --target ${target} --js false --dts type.d.ts`,
  { cwd: root, stdio: 'inherit' }
);

// Move .node into prebuilds/
require('./postbuild.js');
