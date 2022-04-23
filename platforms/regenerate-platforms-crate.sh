#!/bin/bash

set -e

curl -o ./platform-support.md https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/rustc/src/platform-support.md

(
    cd ./platforms-data-gen
    cargo +nightly run --release -- ../platform-support.md
)

cargo fmt
