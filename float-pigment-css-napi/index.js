'use strict';

const path = require('path');
const fs = require('fs');

const platform = process.platform;
const arch = process.arch;

const archMap = { ia32: 'x32', x64: 'x64', arm64: 'arm64' };
const dirName = platform + '-' + (archMap[arch] || arch);

const bindingPath = path.join(__dirname, 'prebuilds', dirName, 'node.napi.node');

if (!fs.existsSync(bindingPath)) {
  throw new Error(
    'No native binding found for ' + platform + '-' + arch + '. ' +
    'Looked in: ' + bindingPath
  );
}

module.exports = require(bindingPath);
