## [4.0.0] (2020-01-22)

- Command line interface ([#40], [#42], [#43])
- Add helper methods for working with checksum metadata ([#38])
- Use minified version of Cargo's `SourceId` type ([#36])
- Overhaul encoding: use serde_derive, proper V1/V2 support ([#35])
- Add support Cargo.lock `patch` and `root` ([#33])
- Detect V1 vs V2 Cargo.lock files ([#31])
- Update `petgraph` requirement from 0.4 to 0.5 ([#28])
- Add `package::Checksum` ([#29])

## [3.0.0] (2019-10-01)

- Support `[package.dependencies]` without versions ([#23])

## [2.0.0] (2019-09-25)

- Use two-pass dependency tree computation ([#20])
- Remove `Lockfile::root_package()` ([#18])

## [1.0.0] (2019-09-24)

- dependency/tree: Render trees to an `io::Write` ([#16])
- metadata: Generalize into `Key` and `Value` types ([#14])
- Refactor dependency handling ([#11])

## [0.2.1] (2019-09-21)

- Allow empty `[metadata]` in Cargo.lock files ([#9])

## [0.2.0] (2019-09-21)

- dependency_graph: Move `petgraph` types into a module ([#7])

## [0.1.0] (2019-09-21)

- Initial release

[4.0.0]: https://github.com/RustSec/cargo-lock/pull/44
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

[3.0.0]: https://github.com/RustSec/cargo-lock/pull/24
[#23]: https://github.com/RustSec/cargo-lock/pull/23

[2.0.0]: https://github.com/RustSec/cargo-lock/pull/21
[#20]: https://github.com/RustSec/cargo-lock/pull/20
[#18]: https://github.com/RustSec/cargo-lock/pull/18

[1.0.0]: https://github.com/RustSec/cargo-lock/pull/17
[#16]: https://github.com/RustSec/cargo-lock/pull/16
[#14]: https://github.com/RustSec/cargo-lock/pull/14
[#11]: https://github.com/RustSec/cargo-lock/pull/11

[0.2.1]: https://github.com/RustSec/cargo-lock/pull/10
[#9]: https://github.com/RustSec/cargo-lock/pull/9

[0.2.0]: https://github.com/RustSec/cargo-lock/pull/8
[#7]: https://github.com/RustSec/cargo-lock/pull/7

[0.1.0]: https://github.com/RustSec/cargo-lock/pull/5
