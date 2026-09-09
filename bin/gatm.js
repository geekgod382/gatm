#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const executable = process.platform === 'win32' ? 'gatm.exe' : 'gatm';
const candidates = [
  path.join(__dirname, '..', 'target', 'release', executable),
  path.join(__dirname, '..', 'target', 'debug', executable),
];
const binary = candidates.find((candidate) => fs.existsSync(candidate));

if (!binary) {
  console.error('gatm: native binary not found; run `cargo build --release` first');
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' });
if (result.error) {
  console.error(`gatm: ${result.error.message}`);
  process.exit(1);
}
process.exit(result.status ?? 1);