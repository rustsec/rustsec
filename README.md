# `rustsec` crate: advisory DB client

[![Latest Version][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
[![dependency status][deps-image]][deps-link]
![MSRV][rustc-image]
![Apache 2.0 OR MIT licensed][license-image]
[![Gitter Chat][gitter-image]][gitter-link]

Client library for accessing the [RustSec Security Advisory Database]:
fetches the [advisory-db] (or other compatible) git repository and
audits `Cargo.lock` files against it.

[Documentation]

## About

The `rustsec` crate is primarily intended to be used by the [cargo-audit] crate
for the purposes of identifying vulnerable crates in Cargo.lock files.

However, it may be useful if you would like to consume the RustSec advisory
database in other capacities.

## Requirements

- Rust **1.39+**

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/rustsec.svg
[crate-link]: https://crates.io/crates/rustsec
[docs-image]: https://docs.rs/rustsec/badge.svg
[docs-link]: https://docs.rs/rustsec/
[build-image]: https://github.com/rustsec/rustsec-crate/workflows/Rust/badge.svg?branch=master&event=push
[build-link]: https://github.com/rustsec/rustsec-crate/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[deps-image]: https://deps.rs/repo/github/RustSec/rustsec-crate/status.svg
[deps-link]: https://deps.rs/repo/github/RustSec/rustsec-crate
[rustc-image]: https://img.shields.io/badge/rustc-1.39+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

[//]: # (general links)

[RustSec Security Advisory Database]: https://rustsec.org/
[advisory-db]: https://github.com/RustSec/advisory-db
[Documentation]: https://docs.rs/rustsec/
[cargo-audit]: https://github.com/rustsec/cargo-audit
[LICENSE-APACHE]: https://github.com/RustSec/rustsec-crate/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/rustsec-crate/blob/master/LICENSE-MIT
