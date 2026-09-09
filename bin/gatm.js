#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const platformPackage = `gatm-${process.platform}-${process.arch}`;
const executable = process.platform === 'win32' ? 'gatm.exe' : 'gatm';
const candidates = [
  tryResolve(path.join(platformPackage, 'bin', executable)),
  path.join(__dirname, '..', 'target', 'release', executable),
  path.join(__dirname, '..', 'target', 'debug', executable),
].filter(Boolean);
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

function tryResolve(request) {
  try {
    return require.resolve(request);
  } catch {
    return null;
  }
}