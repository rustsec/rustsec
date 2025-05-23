#!/bin/bash

set -e

# Regenerate the code

curl -o ./platform-support.md https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/rustc/src/platform-support.md

cargo +nightly run --manifest-path ../Cargo.toml --bin platforms-data-gen --release -- platform-support.md

cargo fmt

# Regenerate the README.md

cp -f README.header.md README.md

cargo run --manifest-path ../Cargo.toml --bin platforms-table-gen >> README.md
