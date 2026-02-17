#!/bin/bash

set -e

cd hello-world
cargo build --release
cp target/release/hello-world ../binary-without-audit-info

cargo clean
cargo auditable build --release
cp target/release/hello-world ../binary-with-audit-info

cd ../vulnerable-binary
cargo auditable build --release
cp target/release/vulnerable-binary ../binary-with-vuln

cd ../vulnerable-binary-with-panic-message
cargo build --release
cp target/release/vulnerable-binary-with-panic-message ../binary-with-vuln-panic

cd ../vulnerable-binary-with-affected-functions
RUSTFLAGS='-C link-dead-code' cargo auditable build --release --config profile.release.opt-level=0
cp target/release/vulnerable-binary-with-affected-functions ../binary-with-affected-functions
