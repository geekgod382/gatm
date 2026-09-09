# GATM 
Git Automation Tool by Manan

Tired of running multiple git commands? Just run `gatm` and it will automatically perform your routine tasks.

## Install

```
npm install -g @geekgod382/gatm 
```

## Usage 

Run these commands INSIDE project directory

```
gatm 
gatm -m "commit message"
gatm -d
gatm -b new_branch
```

See more about options below.

## How it works
gatm runs the core git commands : `add`, `commit`, `push` and `branch`. When you runs `gatm` it will extract the 
untracked/modified files and run `git add <the changed files>`. After this, it deterministically generates a commit message
and run `git commit -m "message" `. At last, it runs `git push -u origin branch_name` to push the commit.

If there are no modified files, the tool returns "nothing to commit".

If push fails because of changes in remote origin, the tool returns "run git pull first".

The extraction of untracked/modified files happens by keeping an eye on `.gitignore` file, to avoid node_modules or .env files
from being committed. 

## Prerequisites 
Git SHOULD BE installed

gatm requires that `git init` was already ran in the project directory, and a remote origin was added. 

## Branching 
The default commits are made to the main branch. But you can change it using the `-b` option described below.

## Options
gatm supports these options:

- `-m "custom_commit_message"` : use this option for adding a custom commit message. This will bypass the deterministic message generation.
- `-b "new_branch_name"` : use this option for creating a new branch and commiting to it. When this option is used, gatm will run `git checkout -b new_branch_name` to create and switch to new branch, and then pushes to it.
- `-d` : a dry run option. This will give you the details about changed files, commit message generated and branching and won't run any command. This option is just for your own satisfaction. 

The deterministic commit message is generated from `git status --porcelain`. gatm
extracts the destination path for renames, sorts and deduplicates all changed
paths, and joins them in this format:

```
chore: update path/to/file, another/file
```

The `-m` option bypasses this generation.

## Additional details
gatm is built in Rust, for blazing speed, and published to npm.

gatm DOES NOT keep a record of remote origins because git already does that.

## Development

Build and test the native binary with Cargo:

```
cargo build --release
cargo test
```

The npm package exposes `bin/gatm.js`. npm installs one matching optional native
package for the current operating system and CPU architecture:

- Windows x64: `gatm-win32-x64`
- Linux x64: `gatm-linux-x64`
- Linux arm64: `gatm-linux-arm64`

Build a native package from a host with the required Rust target and linker:

```
npm run build:native -- --target x86_64-pc-windows-msvc
npm run build:native -- --target x86_64-unknown-linux-gnu
npm run build:native -- --target aarch64-unknown-linux-gnu
```

Publish each generated package in `packages/` before publishing the root
package. In CI, build each target on its native runner, publish its package,
then publish the root package. npm will install only the binary it needs.
macOS is not currently supported.

The root package is scoped because npm reserves unscoped names that are too
similar to existing packages. Publish it publicly with:

```
npm publish --access public
```

On PowerShell, publish a native package from its own directory. For example,
after building Windows x64:

```
Push-Location .\packages\gatm-win32-x64
npm pack --dry-run
npm publish
Pop-Location
```

Do not use `npm pack --prefix .\packages\gatm-win32-x64` from the repository
root; npm may pack the root package instead. The dry-run should show
`gatm-win32-x64@1.0.0` and `bin/gatm.exe`.
