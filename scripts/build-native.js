const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const targets = {
  'x86_64-pc-windows-msvc': ['win32', 'x64', 'gatm.exe'],
  'x86_64-unknown-linux-gnu': ['linux', 'x64', 'gatm'],
  'aarch64-unknown-linux-gnu': ['linux', 'arm64', 'gatm'],
};

const target = process.argv[process.argv.indexOf('--target') + 1];
const selected = targets[target];
if (!selected) {
  console.error(`Unknown target. Choose one of: ${Object.keys(targets).join(', ')}`);
  process.exit(1);
}

const result = spawnSync('cargo', ['build', '--release', '--target', target], {
  stdio: 'inherit',
});
if (result.error || result.status !== 0) {
  process.exit(result.status || 1);
}

const [platform, arch, executable] = selected;
const packageDirectory = path.join(__dirname, '..', 'packages', `gatm-${platform}-${arch}`, 'bin');
fs.mkdirSync(packageDirectory, { recursive: true });
fs.copyFileSync(
  path.join(__dirname, '..', 'target', target, 'release', executable),
  path.join(packageDirectory, executable),
);
if (process.platform !== 'win32') {
  fs.chmodSync(path.join(packageDirectory, executable), 0o755);
}