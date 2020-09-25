## 6.0.0 (2020-09-25)

- Bump semver from 0.10.0 to 0.11.0 ([#83])

[#83]: https://github.com/RustSec/cargo-lock/pull/83

## 5.0.0 (2020-09-23)

- CLI: support for listing a single dependency ([#77])
- Cargo-compatible serializer ([#76])
- CLI: add `--dependencies` and `--sources` flags to `cargo lock list` ([#75])
- CLI: implement `cargo lock tree` without arguments ([#74])
- Add `dependency::Tree::roots()` method ([#73])
- CLI: make `list` the default command ([#72])
- Make `cli` feature non-default ([#70])
- WASM support; MSRV 1.41+ ([#69])
- Bump `semver` dependency from v0.9 to v0.10 ([#57])

[#77]: https://github.com/RustSec/cargo-lock/pull/77
[#76]: https://github.com/RustSec/cargo-lock/pull/76
[#75]: https://github.com/RustSec/cargo-lock/pull/75
[#74]: https://github.com/RustSec/cargo-lock/pull/74
[#73]: https://github.com/RustSec/cargo-lock/pull/73
[#72]: https://github.com/RustSec/cargo-lock/pull/72
[#70]: https://github.com/RustSec/cargo-lock/pull/70
[#69]: https://github.com/RustSec/cargo-lock/pull/69
[#57]: https://github.com/RustSec/cargo-lock/pull/57

## 4.0.1 (2020-01-22)

- CLI: fix executable name ([#45])

[#45]: https://github.com/RustSec/cargo-lock/pull/46

## 4.0.0 (2020-01-22)

- Command line interface ([#40], [#42], [#43])
- Add helper methods for working with checksum metadata ([#38])
- Use minified version of Cargo's `SourceId` type ([#36])
- Overhaul encoding: use serde_derive, proper V1/V2 support ([#35])
- Add support Cargo.lock `patch` and `root` ([#33])
- Detect V1 vs V2 Cargo.lock files ([#31])
- Update `petgraph` requirement from 0.4 to 0.5 ([#28])
- Add `package::Checksum` ([#29])

[#43]: https://github.com/RustSec/cargo-lock/pull/43
[#42]: https://github.com/RustSec/cargo-lock/pull/42
[#40]: https://github.com/RustSec/cargo-lock/pull/40
[#38]: https://github.com/RustSec/cargo-lock/pull/38
[#36]: https://github.com/RustSec/cargo-lock/pull/36
[#35]: https://github.com/RustSec/cargo-lock/pull/35
[#33]: https://github.com/RustSec/cargo-lock/pull/33
[#31]: https://github.com/RustSec/cargo-lock/pull/31
[#29]: https://github.com/RustSec/cargo-lock/pull/29
[#28]: https://github.com/RustSec/cargo-lock/pull/28

## 3.0.0 (2019-10-01)

- Support `[package.dependencies]` without versions ([#23])

[#23]: https://github.com/RustSec/cargo-lock/pull/23

## 2.0.0 (2019-09-25)

- Use two-pass dependency tree computation ([#20])
- Remove `Lockfile::root_package()` ([#18])

[#20]: https://github.com/RustSec/cargo-lock/pull/20
[#18]: https://github.com/RustSec/cargo-lock/pull/18

## 1.0.0 (2019-09-24)

- dependency/tree: Render trees to an `io::Write` ([#16])
- metadata: Generalize into `Key` and `Value` types ([#14])
- Refactor dependency handling ([#11])

[#16]: https://github.com/RustSec/cargo-lock/pull/16
[#14]: https://github.com/RustSec/cargo-lock/pull/14
[#11]: https://github.com/RustSec/cargo-lock/pull/11

## 0.2.1 (2019-09-21)

- Allow empty `[metadata]` in Cargo.lock files ([#9])

[#9]: https://github.com/RustSec/cargo-lock/pull/9

## 0.2.0 (2019-09-21)

- dependency_graph: Move `petgraph` types into a module ([#7])

[#7]: https://github.com/RustSec/cargo-lock/pull/7

## 0.1.0 (2019-09-21)

- Initial release
