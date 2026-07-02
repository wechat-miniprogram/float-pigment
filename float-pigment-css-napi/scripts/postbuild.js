/**
 * Post-build script: moves napi-rs output (.node) into
 * prebuilds/<platform>-<arch>/node.napi.node
 * to match the node-gyp-build directory convention used by the original C++ addon.
 */
const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..');

// napi-rs generates files like: float-pigment-css.darwin-arm64.node
const nodeFiles = fs.readdirSync(root).filter(f => f.endsWith('.node') && f.startsWith('float-pigment-css.'));

// Mapping from napi-rs triple suffix to node-gyp-build directory name
const TRIPLE_MAP = {
  'darwin-arm64': 'darwin-arm64',
  'darwin-x64': 'darwin-x64',
  'darwin-universal': 'darwin-universal',
  'win32-x64-msvc': 'win32-x64',
  'win32-ia32-msvc': 'win32-x32',
  'linux-x64-gnu': 'linux-x64',
  'linux-arm64-gnu': 'linux-arm64',
  'linux-x64-musl': 'linux-x64-musl',
  'linux-arm64-musl': 'linux-arm64-musl',
};

for (const file of nodeFiles) {
  const match = file.match(/^float-pigment-css\.(.+)\.node$/);
  if (!match) continue;

  const triple = match[1];
  const dirName = TRIPLE_MAP[triple] || triple;
  const destDir = path.join(root, 'prebuilds', dirName);

  fs.mkdirSync(destDir, { recursive: true });

  const src = path.join(root, file);
  const dest = path.join(destDir, 'node.napi.node');

  fs.renameSync(src, dest);
  console.log(`  ${file} -> prebuilds/${dirName}/node.napi.node`);
}

if (nodeFiles.length === 0) {
  console.warn('Warning: no .node files found to move');
}
