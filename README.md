# `cargo-lock` crate

[![Latest Version][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![Rust 1.35+][rustc-image]
![Apache 2.0 OR MIT licensed][license-image]
[![Gitter Chat][gitter-image]][gitter-link]

Self-contained [serde]-powered `Cargo.lock` parser/serializer with support for
both the V1 and V2 (merge-friendly) formats, as well as optional dependency
tree analysis features. Used by [RustSec].

When the `dependency-tree` feature of this crate is enabled, it supports
computing a directed graph of the dependency tree, modeled using the
[`petgraph`] crate, along with support for printing dependency trees ala
the [`cargo-tree`] crate.

[Documentation][docs-link]

## Requirements

`cargo-lock` requires Rust **1.35** or later.

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cargo-lock.svg
[crate-link]: https://crates.io/crates/cargo-lock
[docs-image]: https://docs.rs/cargo-lock/badge.svg
[docs-link]: https://docs.rs/cargo-lock/
[build-image]: https://github.com/rustsec/cargo-lock/workflows/Rust/badge.svg?branch=master&event=push
[build-link]: https://github.com/rustsec/cargo-lock/actions
[license-image]: https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

[//]: # (general links)

[serde]: https://serde.rs/
[RustSec]: https://rustsec.org/
[`petgraph`]: https://github.com/petgraph/petgraph
[`cargo-tree`]: https://github.com/sfackler/cargo-tree
[LICENSE-APACHE]: https://github.com/RustSec/cargo-lock/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/cargo-lock/blob/master/LICENSE-MIT
