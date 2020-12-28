# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.1.0 (2020-12-28)
### Added
- `aarch64-apple-darwin` platform definition ([#32])

[#32]: https://github.com/RustSec/platforms-crate/pull/32

## 1.0.3 (2020-10-29)
### Changed
- Source `Platform::guess_current` from `$TARGET` environment variable when
  available ([#29])

[#29]: https://github.com/RustSec/platforms-crate/pull/29

## 1.0.2 (2020-09-14)
### Removed
- `const fn` on `Platforms::all` ([#27])

[#27]: https://github.com/RustSec/platforms-crate/pull/27

## 1.0.1 (2020-09-14) [YANKED]
### Changed
- Make `Platform::all()` a `const fn` ([#24])
- Refactor `Platform::find` and `::guess_current` ([#23])
- Rename `ALL_PLATFORMS` to `Platform::all()` ([#22])

[#24]: https://github.com/RustSec/platforms-crate/pull/24
[#23]: https://github.com/RustSec/platforms-crate/pull/23
[#22]: https://github.com/RustSec/platforms-crate/pull/22

## 1.0.0 (2020-09-13) [YANKED]
### Added
- Ensure all types have `FromStr`, `Display`, and `serde` impls ([#20])
- `aarch64-pc-windows-msvc` platform ([#17])

### Changed
- Make extensible enums `non_exhaustive`; MSRV 1.40+ ([#18])

[#20]: https://github.com/RustSec/platforms-crate/pull/20
[#18]: https://github.com/RustSec/platforms-crate/pull/18
[#17]: https://github.com/RustSec/platforms-crate/pull/17

## 0.2.1 (2019-09-24)

- Initial GitHub Actions config ([#12])
- Properly set up `target::os::TARGET_OS` const for unknown OS ([#11])

[#12]: https://github.com/RustSec/platforms-crate/pull/12
[#11]: https://github.com/RustSec/platforms-crate/pull/11

## 0.2.0 (2019-01-13)

- Update platforms to match RustForge ([#9])
- Update to Rust 2018 edition ([#8])

[#9]: https://github.com/RustSec/platforms-crate/pull/9
[#8]: https://github.com/RustSec/platforms-crate/pull/8

## 0.1.4 (2018-07-29)

- `x86_64-apple-darwin`: fix typo in target triple name ([#6])
- Have markdown-table-gen output links to Platform structs on docs.rs ([#5])

[#6]: https://github.com/RustSec/platforms-crate/pull/6
[#5]: https://github.com/RustSec/platforms-crate/pull/5

## 0.1.3 (2018-07-28)

- Fix Travis CI badge in Cargo.toml

## 0.1.2 (2018-07-27)

- Add table of supported platforms to README.md using Markdown generator ([#4])

[#4]: https://github.com/RustSec/platforms-crate/pull/4

## 0.1.1 (2018-07-27)

- Impl `Display` and `std::error::Error` traits for `packages::Error` ([#3])

[#3]: https://github.com/RustSec/platforms-crate/pull/3

## 0.1.0 (2018-07-26)

- Add `guess_current()` ([#2])
- Optional serde support ([#1])

[#2]: https://github.com/RustSec/platforms-crate/pull/2
[#1]: https://github.com/RustSec/platforms-crate/pull/1

## 0.0.1 (2018-07-26)

- Initial release
