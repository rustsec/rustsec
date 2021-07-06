# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 7.0.1 (2021-07-05)
### Changed
- Bump `petgraph` dependency from 0.5.1 to 0.6.0 ([#396])

[#396]: https://github.com/RustSec/rustsec/pull/396

## 7.0.0 (2021-05-27) [YANKED]
### Added
- Support for V3 lockfile format ([#363])

### Changed
- Bump `semver` to v1.0.0 ([#378])

[#363]: https://github.com/RustSec/rustsec/pull/363
[#378]: https://github.com/RustSec/rustsec/pull/378

## 6.0.1 (2021-01-25)
### Changed
-  Rename default branch to `main`

## 6.0.0 (2020-09-25)
- Bump semver from 0.10.0 to 0.11.0

## 5.0.0 (2020-09-23)
- CLI: support for listing a single dependency
- Cargo-compatible serializer
- CLI: add `--dependencies` and `--sources` flags to `cargo lock list`
- CLI: implement `cargo lock tree` without arguments
- Add `dependency::Tree::roots()` method
- CLI: make `list` the default command
- Make `cli` feature non-default
- WASM support; MSRV 1.41+
- Bump `semver` dependency from v0.9 to v0.10

## 4.0.1 (2020-01-22)
- CLI: fix executable name

## 4.0.0 (2020-01-22)
- Command line interface
- Add helper methods for working with checksum metadata
- Use minified version of Cargo's `SourceId` type
- Overhaul encoding: use serde_derive, proper V1/V2 support
- Add support Cargo.lock `patch` and `root`
- Detect V1 vs V2 Cargo.lock files
- Update `petgraph` requirement from 0.4 to 0.5
- Add `package::Checksum`

## 3.0.0 (2019-10-01)
- Support `[package.dependencies]` without versions

## 2.0.0 (2019-09-25)
- Use two-pass dependency tree computation
- Remove `Lockfile::root_package()`

## 1.0.0 (2019-09-24)
- dependency/tree: Render trees to an `io::Write`
- metadata: Generalize into `Key` and `Value` types
- Refactor dependency handling

## 0.2.1 (2019-09-21)
- Allow empty `[metadata]` in Cargo.lock files

## 0.2.0 (2019-09-21)
- dependency_graph: Move `petgraph` types into a module

## 0.1.0 (2019-09-21)
- Initial release
