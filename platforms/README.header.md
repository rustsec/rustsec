# RustSec: `platforms` crate

[![Latest Version][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache 2/MIT licensed][license-image]
![MSRV][rustc-image]
[![Project Chat][zulip-image]][zulip-link]

Rust platform registry: provides programmatic access to information
about valid Rust platforms, sourced from the Rust compiler.

[Documentation][docs-link]

## About

This crate provides programmatic access to information about valid Rust
platforms. This is useful for systems which document/inventory information
relevant to Rust platforms.

It was created for the [RustSec Advisory Database] and is maintained by the
[Rust Secure Code Working Group][wg-secure-code].

It is not intended to be a tool for gating builds based on the current platform
or as a replacement for Rust's existing conditional compilation features:
please use those for build purposes.

## Minimum Supported Rust Version

Rust **1.40** or higher.

Minimum supported Rust version may be changed in the future, but it will be
accompanied by a minor version bump.

## SemVer Policy

We reserve the right to add and remove platforms from the registry without
bumping major versions. This doesn't change the API, but can break crates that
expect platforms to be there if they are removed.

If we remove platforms, we will bump the minor version of this crate.

[//]: # (badges)

[crate-image]:  https://buildstats.info/crate/platforms
[crate-link]: https://crates.io/crates/platforms
[docs-image]: https://docs.rs/platforms/badge.svg
[docs-link]: https://docs.rs/platforms/
[build-image]: https://github.com/RustSec/rustsec/actions/workflows/platforms.yml/badge.svg
[build-link]: https://github.com/RustSec/rustsec/actions/workflows/platforms.yml
[license-image]: https://img.shields.io/badge/license-Apache2%2FMIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.40+-blue.svg
[zulip-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[zulip-link]: https://rust-lang.zulipchat.com/#narrow/stream/146229-wg-secure-code/

[//]: # (general links)

[RustSec Advisory Database]: https://github.com/RustSec
[wg-secure-code]: https://www.rust-lang.org/governance/wgs/wg-secure-code

## Registered Platforms

