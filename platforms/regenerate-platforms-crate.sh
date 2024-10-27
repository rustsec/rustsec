#!/bin/bash

set -e

# Regenerate the code

curl -o ./platform-support.md https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/rustc/src/platform-support.md

(
    cd ./platforms-data-gen
    cargo +nightly run --release -- ../platform-support.md
)

cargo fmt

# Regenerate the README.md

cp -f README.header.md README.md

(
    cd ./markdown-table-gen
    cargo run >> ../README.md
)
