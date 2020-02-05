## [0.18.0] (2020-02-05)

- Move yanked crate auditing to `cargo-audit` ([#147])

## [0.17.1] (2020-01-22)

- Update `cargo-lock` requirement from 3.0 to 4.0 ([#143])

## [0.17.0] (2020-01-19)

- Bump MSRV to 1.39 ([#140])
- Extract `cargo audit fix` logic into `Fixer` ([#136])
- Warn for yanked crates ([#135])
- Add `vendored-openssl` feature ([#130])
- Support crate sources as a vulnerability query attribute ([#128])
- Try to auto-detect proxy setting ([#126])

## [0.16.0] (2019-10-13)

- Remove `support.toml` parsing ([#124])

## [0.15.2] (2019-10-08)

- version: Fix matching bug for `>` version requirements ([#122])

## [0.15.1] (2019-10-07)

- linter: Add `informational` as an allowable `[advisory]` key ([#118])
- repository: Expose `authentication` module ([#117])

## [0.15.0] (2019-10-01)

- Upgrade to `cargo-lock` crate v3.0 ([#115])

## [0.14.1] (2019-09-25)

- Upgrade to `cargo-lock` crate v2.0 ([#113])

## [0.14.0] (2019-09-24)

- warning: Extract into module; make more like `Vulnerability` ([#110])
- Upgrade to `cvss` crate v1.0 ([#109])
- Upgrade to `cargo-lock` crate v1.0 ([#107])

## [0.13.0] (2019-09-23)

- linter: Ensure advisory date's year matches year in advisory ID ([#99])
- Use the `cargo-lock` crate ([#97])
- lockfile: Add (optional) DependencyGraph analysis ([#95])
- Rename `rustsec::db` module to `rustsec::database` ([#90])
- report: Generate warnings for selected informational advisories ([#89])
- vulnerability: Add `affected_functions()` ([#88])
- Add `rustsec::advisory::Linter` ([#87])
- package: Parse dependencies from Cargo.lock ([#84])
- Initial `report` module and built-in report-generating ([#83])
- Basic query support ([#81])
- Index the `rust` advisory directory from `RustSec/advisory-db` ([#80])
- Add first-class support for GitHub Security Advisories (GHSA) ([#79])
- Re-vendor Cargo's git authentication code ([#78])
- `support.toml` for indicating supported versions ([#76])
- Add support for "informational" advisories ([#75])
- Add `rustsec::advisory::Category` ([#74])
- Refactor advisory types: add `[affected]` and `[versions]` sections ([#73])
- advisory: Add (optional) `cvss` field with CVSS v3.1 score ([#72])
- Freshen deps: add `home`, remove `directories` and `failure` ([#71])
- Improved handling of prereleases; MSRV 1.35+ ([#69])
- Add `Version` and `VersionReq` newtypes ([#68])

## [0.12.1] (2019-07-29)

- Use new inclusive range syntax ([#66])

## [0.12.0] (2019-07-15)

- Update dependencies and use 2018 import conventions; Rust 1.32+ ([#64])
- Re-export all types in `advisory::paths::*` ([#61])

## [0.11.0] (2019-01-13)

- Cargo.toml: Update `platforms` crate to v0.2 ([#59])
- Redo advisory's `affected_functions` as `affected_paths` ([#58])

## [0.10.0] (2018-12-14)

- Implement `affected_functions` advisory attribute ([#54])
- Fix handling of `unaffected_versions` ([#53])
- Update to Rust 2018 edition ([#52])

## [0.9.3] (2018-10-14)

- Create parents of the `advisory-db` repo dir  ([#49])

## [0.9.2] (2018-10-14)

- Handle cloning `advisory-db` into existing, empty dir ([#47])

## [0.9.1] (2018-07-29)

- Use Cargo's git authentication helper ([#40])

## [0.9.0] (2018-07-26)

- Use `platforms` crate for platform-related functionality ([#39])

## [0.8.0] (2018-07-24)

- Advisory platform requirements ([#38])
- Cargo-like keyword support ([#37])

## [0.7.5] (2018-07-24)

- Allow `AdvisoryId::new()` to parse `RUSTSEC-0000-0000` ([#36])

## [0.7.4] (2018-07-23)

- Add link to logo image for docs.rs ([#35])

## [0.7.3] (2018-07-23)

- Fix builds with `--no-default-features` ([#34])

## [0.7.2] (2018-07-23)

- README.md: Badge fixups, add gitter badge ([#32])

## [0.7.1] (2018-07-23)

- Cargo.toml: Formatting fixups, add `readme` attribute ([#31])

## [0.7.0] (2018-07-22)

- Validate dates are well-formed ([#29])
- Add `AdvisoryIdKind` and limited support for parsing advisory IDs ([#28])
- Add a `Vulnerabilities` collection struct ([#27])
- Parse aliases, references, and unaffected versions ([#23])
- Parse (but do not yet verify) signatures on advisory-db commits ([#22])
- Parse individual advisory `.toml` files rather than Advisories.toml ([#21])
- Switch to `git2`-based fetcher for `advisory-db` ([#20])
- Use serde to parse advisories TOML and `Cargo.lock` files ([#18])
- Use `failure` crate for error handling ([#17])

## 0.6.0 (2017-03-05)

- Use `semver::Version` for `lockfile::Package` versions ([#11])
- Move `AdvisoryDatabase` under the `::db` module ([#10])
- Lockfile support ([#9])

## 0.5.2 (2017-02-26)

- Add `AdvisoryDatabase::fetch_from_url()` ([#8])

## 0.5.1 (2017-02-26)

- Make `advisory` and `error` modules public ([#7])

## 0.5.0 (2017-02-26)

- Use str version param for `AdvisoryDatabase::find_vulns_for_crate()` ([#6])

## 0.4.0 (2017-02-26)

- Add `AdvisoryDatabase::find_vulns_for_crate()` ([#5])

## 0.3.0 (2017-02-26)

- Rename `crate_name` TOML attribute back to `package` ([#4])

## 0.2.0 (2017-02-25)

- Rename `package` TOML attribute to `crate_name` ([#3])
- Add iterator support to `AdvisoryDatabase` ([#2])

## 0.1.0 (2017-02-25)

- Initial release

[0.18.0]: https://github.com/RustSec/rustsec-crate/pull/148
[#147]: https://github.com/RustSec/rustsec-crate/pull/147
[0.17.1]: https://github.com/RustSec/rustsec-crate/pull/144
[#143]: https://github.com/RustSec/rustsec-crate/pull/143
[0.17.0]: https://github.com/RustSec/rustsec-crate/pull/141
[#140]: https://github.com/RustSec/rustsec-crate/pull/140
[#136]: https://github.com/RustSec/rustsec-crate/pull/136
[#135]: https://github.com/RustSec/rustsec-crate/pull/135
[#130]: https://github.com/RustSec/rustsec-crate/pull/130
[#128]: https://github.com/RustSec/rustsec-crate/pull/128
[#126]: https://github.com/RustSec/rustsec-crate/pull/126
[0.16.0]: https://github.com/RustSec/rustsec-crate/pull/125
[#124]: https://github.com/RustSec/rustsec-crate/pull/124
[0.15.2]: https://github.com/RustSec/rustsec-crate/pull/123
[#122]: https://github.com/RustSec/rustsec-crate/pull/122
[0.15.1]: https://github.com/RustSec/rustsec-crate/pull/121
[#118]: https://github.com/RustSec/rustsec-crate/pull/118
[#117]: https://github.com/RustSec/rustsec-crate/pull/117
[0.15.0]: https://github.com/RustSec/rustsec-crate/pull/116
[#115]: https://github.com/RustSec/rustsec-crate/pull/115
[0.14.1]: https://github.com/RustSec/rustsec-crate/pull/114
[#113]: https://github.com/RustSec/rustsec-crate/pull/113
[0.14.0]: https://github.com/RustSec/rustsec-crate/pull/111
[#110]: https://github.com/RustSec/rustsec-crate/pull/110
[#109]: https://github.com/RustSec/rustsec-crate/pull/109
[#107]: https://github.com/RustSec/rustsec-crate/pull/107
[0.13.0]: https://github.com/RustSec/rustsec-crate/pull/103
[#99]: https://github.com/RustSec/rustsec-crate/pull/99
[#97]: https://github.com/RustSec/rustsec-crate/pull/97
[#95]: https://github.com/RustSec/rustsec-crate/pull/95
[#90]: https://github.com/RustSec/rustsec-crate/pull/90
[#89]: https://github.com/RustSec/rustsec-crate/pull/89
[#88]: https://github.com/RustSec/rustsec-crate/pull/88
[#87]: https://github.com/RustSec/rustsec-crate/pull/87
[#84]: https://github.com/RustSec/rustsec-crate/pull/84
[#83]: https://github.com/RustSec/rustsec-crate/pull/83
[#81]: https://github.com/RustSec/rustsec-crate/pull/81
[#80]: https://github.com/RustSec/rustsec-crate/pull/80
[#79]: https://github.com/RustSec/rustsec-crate/pull/79
[#78]: https://github.com/RustSec/rustsec-crate/pull/78
[#76]: https://github.com/RustSec/rustsec-crate/pull/76
[#75]: https://github.com/RustSec/rustsec-crate/pull/75
[#74]: https://github.com/RustSec/rustsec-crate/pull/74
[#73]: https://github.com/RustSec/rustsec-crate/pull/73
[#72]: https://github.com/RustSec/rustsec-crate/pull/72
[#71]: https://github.com/RustSec/rustsec-crate/pull/71
[#69]: https://github.com/RustSec/rustsec-crate/pull/69
[#68]: https://github.com/RustSec/rustsec-crate/pull/68
[0.12.1]: https://github.com/RustSec/rustsec-crate/pull/67
[#66]: https://github.com/RustSec/rustsec-crate/pull/66
[0.12.0]: https://github.com/RustSec/rustsec-crate/pull/65
[#64]: https://github.com/RustSec/rustsec-crate/pull/64
[#61]: https://github.com/RustSec/rustsec-crate/pull/61
[0.11.0]: https://github.com/RustSec/rustsec-crate/pull/60
[#59]: https://github.com/RustSec/rustsec-crate/pull/58
[#58]: https://github.com/RustSec/rustsec-crate/pull/59
[0.10.0]: https://github.com/RustSec/rustsec-crate/pull/56
[#54]: https://github.com/RustSec/rustsec-crate/pull/54
[#53]: https://github.com/RustSec/rustsec-crate/pull/53
[#52]: https://github.com/RustSec/rustsec-crate/pull/52
[0.9.3]: https://github.com/RustSec/rustsec-crate/pull/50
[#49]: https://github.com/RustSec/rustsec-crate/pull/49
[0.9.2]: https://github.com/RustSec/rustsec-crate/pull/48
[#47]: https://github.com/RustSec/rustsec-crate/pull/47
[0.9.1]: https://github.com/RustSec/rustsec-crate/compare/v0.9.0...v0.9.1
[#40]: https://github.com/RustSec/rustsec-crate/pull/40
[0.9.0]: https://github.com/RustSec/rustsec-crate/compare/v0.8.0...v0.9.0
[#39]: https://github.com/RustSec/rustsec-crate/pull/39
[0.8.0]: https://github.com/RustSec/rustsec-crate/compare/v0.7.5...v0.8.0
[#38]: https://github.com/RustSec/rustsec-crate/pull/38
[#37]: https://github.com/RustSec/rustsec-crate/pull/37
[0.7.5]: https://github.com/RustSec/rustsec-crate/compare/v0.7.4...v0.7.5
[#36]: https://github.com/RustSec/rustsec-crate/pull/36
[0.7.4]: https://github.com/RustSec/rustsec-crate/compare/v0.7.3...v0.7.4
[#35]: https://github.com/RustSec/rustsec-crate/pull/35
[0.7.3]: https://github.com/RustSec/rustsec-crate/compare/v0.7.2...v0.7.3
[#34]: https://github.com/RustSec/rustsec-crate/pull/34
[0.7.2]: https://github.com/RustSec/rustsec-crate/compare/v0.7.1...v0.7.2
[#32]: https://github.com/RustSec/rustsec-crate/pull/32
[0.7.1]: https://github.com/RustSec/rustsec-crate/compare/v0.7.0...v0.7.1
[#31]: https://github.com/RustSec/rustsec-crate/pull/31
[0.7.0]: https://github.com/RustSec/rustsec-crate/compare/v0.6.0...v0.7.0
[#29]: https://github.com/RustSec/rustsec-crate/pull/29
[#28]: https://github.com/RustSec/rustsec-crate/pull/28
[#27]: https://github.com/RustSec/rustsec-crate/pull/27
[#23]: https://github.com/RustSec/rustsec-crate/pull/23
[#22]: https://github.com/RustSec/rustsec-crate/pull/22
[#21]: https://github.com/RustSec/rustsec-crate/pull/21
[#20]: https://github.com/RustSec/rustsec-crate/pull/20
[#18]: https://github.com/RustSec/rustsec-crate/pull/18
[#17]: https://github.com/RustSec/rustsec-crate/pull/17
[#11]: https://github.com/RustSec/rustsec-crate/pull/11
[#10]: https://github.com/RustSec/rustsec-crate/pull/10
[#9]: https://github.com/RustSec/rustsec-crate/pull/9
[#8]: https://github.com/RustSec/rustsec-crate/pull/8
[#7]: https://github.com/RustSec/rustsec-crate/pull/7
[#6]: https://github.com/RustSec/rustsec-crate/pull/6
[#5]: https://github.com/RustSec/rustsec-crate/pull/5
[#4]: https://github.com/RustSec/rustsec-crate/pull/4
[#3]: https://github.com/RustSec/rustsec-crate/pull/3
[#2]: https://github.com/RustSec/rustsec-crate/pull/2
