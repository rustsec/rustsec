# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.24.0 (2021-06-28)
### Added
- OSV export ([#366])

### Changed
- Bump `cargo-lock` to v7.0 ([#379])

[#366]: https://github.com/RustSec/rustsec-crate/pull/366
[#379]: https://github.com/RustSec/rustsec-crate/pull/379

## 0.23.3 (2021-03-08)
### Fixed
- Workaround for stale git refs ([#309])

[#309]: https://github.com/RustSec/rustsec-crate/pull/309

## 0.23.2 (2021-03-07)
### Changed
- Rename advisory-db `master` branch to `main` ([#307])

[#307]: https://github.com/RustSec/rustsec-crate/pull/307

## 0.23.1 (2021-02-24)
### Fixed
- Parsing error on Windows ([#295])

[#295]: https://github.com/RustSec/rustsec-crate/pull/295

## 0.23.0 (2021-01-26)
### Added
- Advisory `references` as a URL list ([#266])
- Support for omitting leading `[advisory]` table ([#268])
- `thread-safety` category ([#290])

### Changed
- Rename previous `references` field to `related` ([#261])
- Use `url` crate to parse metadata URL ([#263])
- Bump `smol_str` to v0.1.17; MSRV 1.46+ ([#264])
- Replace `chrono` with `humantime` ([#265])
- Mark enums as non_exhaustive ([#267])
- Use `SystemTime` instead of a `git::Timestamp` type ([#269])
- Rename `fetch` Cargo feature to `git` ([#270])
- Rename `repository::GitRepository` to `repository::git::Repository` ([#271])

### Removed
- `markdown` feature ([#262])

[#261]: https://github.com/RustSec/rustsec-crate/pull/261
[#262]: https://github.com/RustSec/rustsec-crate/pull/262
[#263]: https://github.com/RustSec/rustsec-crate/pull/263
[#264]: https://github.com/RustSec/rustsec-crate/pull/264
[#265]: https://github.com/RustSec/rustsec-crate/pull/265
[#266]: https://github.com/RustSec/rustsec-crate/pull/266
[#267]: https://github.com/RustSec/rustsec-crate/pull/267
[#268]: https://github.com/RustSec/rustsec-crate/pull/268
[#269]: https://github.com/RustSec/rustsec-crate/pull/269
[#270]: https://github.com/RustSec/rustsec-crate/pull/270
[#271]: https://github.com/RustSec/rustsec-crate/pull/271
[#290]: https://github.com/RustSec/rustsec-crate/pull/290

## 0.22.2 (2020-10-27)
### Changed
- Revert "Refactor Advisory type handling" ([#249])

[#249]: https://github.com/RustSec/rustsec-crate/pull/249

## 0.22.1 (2020-10-26) [YANKED]
### Changed
- Refactor `Advisory` and `VulnerabilityInfo` ([#246])

[#246]: https://github.com/RustSec/rustsec-crate/pull/246

## 0.22.0 (2020-10-25) [YANKED]
### Added
- `fetch` feature ([#213], [#226])

### Changed
- Bump `cargo-lock` to v6; `semver` to v0.11 ([#244])
- Make `advisory.title` and `advisory.description` struct fields ([#242])
- Remove support for the V2 advisory format ([#238], [#242], [#243])
- Mark the `advisory::parser` module as `pub` ([#240])
- Bump `cargo-edit` to 0.7.0 ([#231])
- Bump `crates-index` from 0.15.4 to 0.16.0 ([#237])
- `advisory`: laxer function path handling ([#229])
- `linter`: fully deprecate `obsolete` in favor of `yanked` ([#228])
- `advisory`: `markdown` feature and `Advisory::description_html` ([#227])
- `linter`: add support for V3 advisory format ([#225])
- MSRV 1.41+ ([#217])
- Bump `platforms` crate to v1 ([#210])

### Fixed
- `linter`: correctly handle crates with dashes in names ([#221])

### Removed
- `advisory.metadata.title` and `advisory.metadata.description` ([#242])

[#244]: https://github.com/RustSec/rustsec-crate/pull/244
[#243]: https://github.com/RustSec/rustsec-crate/pull/243
[#242]: https://github.com/RustSec/rustsec-crate/pull/242
[#240]: https://github.com/RustSec/rustsec-crate/pull/240
[#238]: https://github.com/RustSec/rustsec-crate/pull/238
[#237]: https://github.com/RustSec/rustsec-crate/pull/237
[#231]: https://github.com/RustSec/rustsec-crate/pull/231
[#229]: https://github.com/RustSec/rustsec-crate/pull/229
[#228]: https://github.com/RustSec/rustsec-crate/pull/228
[#227]: https://github.com/RustSec/rustsec-crate/pull/227
[#226]: https://github.com/RustSec/rustsec-crate/pull/226
[#225]: https://github.com/RustSec/rustsec-crate/pull/225
[#221]: https://github.com/RustSec/rustsec-crate/pull/221
[#217]: https://github.com/RustSec/rustsec-crate/pull/217
[#213]: https://github.com/RustSec/rustsec-crate/pull/213
[#210]: https://github.com/RustSec/rustsec-crate/pull/210

## 0.21.0 (2020-06-23)
### Added
- `year`, `month`, and `day` methods to `advisory::Date` ([#191])
- `unsound` informational advisory kind ([#189])

### Changed
- Bump `crates-index` from 0.14 to 0.15 ([#183])
- Rename `obsolete` advisories to `yanked` ([#196])
- Rename `warning::Kind::Informational` to `::Notice` ([#195])
- Make `warning::Kind` a `#[non_exhausive]` enum ([#195])
- Make `Informational` a `#[non_exhausive]` enum ([#194])

### Removed
- Legacy `patched_versions` and `unaffected_versions` ([#197])

[#197]: https://github.com/RustSec/rustsec-crate/pull/197
[#196]: https://github.com/RustSec/rustsec-crate/pull/196
[#195]: https://github.com/RustSec/rustsec-crate/pull/195
[#194]: https://github.com/RustSec/rustsec-crate/pull/194
[#191]: https://github.com/RustSec/rustsec-crate/pull/191
[#189]: https://github.com/RustSec/rustsec-crate/pull/189
[#183]: https://github.com/RustSec/rustsec-crate/pull/183

## 0.20.1 (2020-06-14)
### Added
- `advisory::Id::numerical_part()` ([#185])

[#185]: https://github.com/RustSec/rustsec-crate/pull/185

## 0.20.0 (2020-05-06)
### Changed
- Make `WarningInfo` into a simple type alias ([#170])

[#170]: https://github.com/RustSec/rustsec-crate/pull/170

## 0.19.0 (2020-05-04)

- Refactor package scopes ([#168])
- Prototype V3 Advisory Format ([#167])
- Bump dependencies to link `libgit2` dynamically ([#163])
- Add `WarningInfo` and modify `Warning` struct ([#156])
- Drop support for the V1 advisory format ([#154])

[#168]: https://github.com/RustSec/rustsec-crate/pull/168
[#167]: https://github.com/RustSec/rustsec-crate/pull/167
[#163]: https://github.com/RustSec/rustsec-crate/pull/163
[#156]: https://github.com/RustSec/rustsec-crate/pull/156
[#154]: https://github.com/RustSec/rustsec-crate/pull/154

## 0.18.0 (2020-02-05)

- Move yanked crate auditing to `cargo-audit` ([#147])

[#147]: https://github.com/RustSec/rustsec-crate/pull/147

## 0.17.1 (2020-01-22)

- Update `cargo-lock` requirement from 3.0 to 4.0 ([#143])

[#143]: https://github.com/RustSec/rustsec-crate/pull/143

## 0.17.0 (2020-01-19)

- Bump MSRV to 1.39 ([#140])
- Extract `cargo audit fix` logic into `Fixer` ([#136])
- Warn for yanked crates ([#135])
- Add `vendored-openssl` feature ([#130])
- Support crate sources as a vulnerability query attribute ([#128])
- Try to auto-detect proxy setting ([#126])

[#140]: https://github.com/RustSec/rustsec-crate/pull/140
[#136]: https://github.com/RustSec/rustsec-crate/pull/136
[#135]: https://github.com/RustSec/rustsec-crate/pull/135
[#130]: https://github.com/RustSec/rustsec-crate/pull/130
[#128]: https://github.com/RustSec/rustsec-crate/pull/128
[#126]: https://github.com/RustSec/rustsec-crate/pull/126

## 0.16.0 (2019-10-13)

- Remove `support.toml` parsing ([#124])

[#124]: https://github.com/RustSec/rustsec-crate/pull/124

## 0.15.2 (2019-10-08)

- version: Fix matching bug for `>` version requirements ([#122])

[#122]: https://github.com/RustSec/rustsec-crate/pull/122

## 0.15.1 (2019-10-07)

- linter: Add `informational` as an allowable `[advisory]` key ([#118])
- repository: Expose `authentication` module ([#117])

[#118]: https://github.com/RustSec/rustsec-crate/pull/118
[#117]: https://github.com/RustSec/rustsec-crate/pull/117

## 0.15.0 (2019-10-01)

- Upgrade to `cargo-lock` crate v3.0 ([#115])

[#115]: https://github.com/RustSec/rustsec-crate/pull/115

## 0.14.1 (2019-09-25)

- Upgrade to `cargo-lock` crate v2.0 ([#113])

[#113]: https://github.com/RustSec/rustsec-crate/pull/113

## 0.14.0 (2019-09-24)

- warning: Extract into module; make more like `Vulnerability` ([#110])
- Upgrade to `cvss` crate v1.0 ([#109])
- Upgrade to `cargo-lock` crate v1.0 ([#107])

[#110]: https://github.com/RustSec/rustsec-crate/pull/110
[#109]: https://github.com/RustSec/rustsec-crate/pull/109
[#107]: https://github.com/RustSec/rustsec-crate/pull/107

## 0.13.0 (2019-09-23)

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

## 0.12.1 (2019-07-29)

- Use new inclusive range syntax ([#66])

[#66]: https://github.com/RustSec/rustsec-crate/pull/66

## 0.12.0 (2019-07-15)

- Update dependencies and use 2018 import conventions; Rust 1.32+ ([#64])
- Re-export all types in `advisory::paths::*` ([#61])

[#64]: https://github.com/RustSec/rustsec-crate/pull/64
[#61]: https://github.com/RustSec/rustsec-crate/pull/61

## 0.11.0 (2019-01-13)

- Cargo.toml: Update `platforms` crate to v0.2 ([#59])
- Redo advisory's `affected_functions` as `affected_paths` ([#58])

[#59]: https://github.com/RustSec/rustsec-crate/pull/58
[#58]: https://github.com/RustSec/rustsec-crate/pull/59

## 0.10.0 (2018-12-14)

- Implement `affected_functions` advisory attribute ([#54])
- Fix handling of `unaffected_versions` ([#53])
- Update to Rust 2018 edition ([#52])

[#54]: https://github.com/RustSec/rustsec-crate/pull/54
[#53]: https://github.com/RustSec/rustsec-crate/pull/53
[#52]: https://github.com/RustSec/rustsec-crate/pull/52

## 0.9.3 (2018-10-14)

- Create parents of the `advisory-db` repo dir  ([#49])

[#49]: https://github.com/RustSec/rustsec-crate/pull/49

## 0.9.2 (2018-10-14)

- Handle cloning `advisory-db` into existing, empty dir ([#47])

[#47]: https://github.com/RustSec/rustsec-crate/pull/47

## 0.9.1 (2018-07-29)

- Use Cargo's git authentication helper ([#40])

[#40]: https://github.com/RustSec/rustsec-crate/pull/40

## 0.9.0 (2018-07-26)

- Use `platforms` crate for platform-related functionality ([#39])

[#39]: https://github.com/RustSec/rustsec-crate/pull/39

## 0.8.0 (2018-07-24)

- Advisory platform requirements ([#38])
- Cargo-like keyword support ([#37])

[#38]: https://github.com/RustSec/rustsec-crate/pull/38
[#37]: https://github.com/RustSec/rustsec-crate/pull/37

## 0.7.5 (2018-07-24)

- Allow `AdvisoryId::new()` to parse `RUSTSEC-0000-0000` ([#36])

[#36]: https://github.com/RustSec/rustsec-crate/pull/36

## 0.7.4 (2018-07-23)

- Add link to logo image for docs.rs ([#35])

[#35]: https://github.com/RustSec/rustsec-crate/pull/35

## 0.7.3 (2018-07-23)

- Fix builds with `--no-default-features` ([#34])

[#34]: https://github.com/RustSec/rustsec-crate/pull/34

## 0.7.2 (2018-07-23)

- README.md: Badge fixups, add gitter badge ([#32])

[#32]: https://github.com/RustSec/rustsec-crate/pull/32

## 0.7.1 (2018-07-23)

- Cargo.toml: Formatting fixups, add `readme` attribute ([#31])

[#31]: https://github.com/RustSec/rustsec-crate/pull/31

## 0.7.0 (2018-07-22)

- Validate dates are well-formed ([#29])
- Add `AdvisoryIdKind` and limited support for parsing advisory IDs ([#28])
- Add a `Vulnerabilities` collection struct ([#27])
- Parse aliases, references, and unaffected versions ([#23])
- Parse (but do not yet verify) signatures on advisory-db commits ([#22])
- Parse individual advisory `.toml` files rather than Advisories.toml ([#21])
- Switch to `git2`-based fetcher for `advisory-db` ([#20])
- Use serde to parse advisories TOML and `Cargo.lock` files ([#18])
- Use `failure` crate for error handling ([#17])

[#29]: https://github.com/RustSec/rustsec-crate/pull/29
[#28]: https://github.com/RustSec/rustsec-crate/pull/28
[#27]: https://github.com/RustSec/rustsec-crate/pull/27
[#23]: https://github.com/RustSec/rustsec-crate/pull/23
[#22]: https://github.com/RustSec/rustsec-crate/pull/22
[#21]: https://github.com/RustSec/rustsec-crate/pull/21
[#20]: https://github.com/RustSec/rustsec-crate/pull/20
[#18]: https://github.com/RustSec/rustsec-crate/pull/18
[#17]: https://github.com/RustSec/rustsec-crate/pull/17

## 0.6.0 (2017-03-05)

- Use `semver::Version` for `lockfile::Package` versions ([#11])
- Move `AdvisoryDatabase` under the `::db` module ([#10])
- Lockfile support ([#9])

[#11]: https://github.com/RustSec/rustsec-crate/pull/11
[#10]: https://github.com/RustSec/rustsec-crate/pull/10
[#9]: https://github.com/RustSec/rustsec-crate/pull/9

## 0.5.2 (2017-02-26)

- Add `AdvisoryDatabase::fetch_from_url()` ([#8])

[#8]: https://github.com/RustSec/rustsec-crate/pull/8

## 0.5.1 (2017-02-26)

- Make `advisory` and `error` modules public ([#7])

[#7]: https://github.com/RustSec/rustsec-crate/pull/7

## 0.5.0 (2017-02-26)

- Use str version param for `AdvisoryDatabase::find_vulns_for_crate()` ([#6])

[#6]: https://github.com/RustSec/rustsec-crate/pull/6

## 0.4.0 (2017-02-26)

- Add `AdvisoryDatabase::find_vulns_for_crate()` ([#5])

[#5]: https://github.com/RustSec/rustsec-crate/pull/5

## 0.3.0 (2017-02-26)

- Rename `crate_name` TOML attribute back to `package` ([#4])

[#4]: https://github.com/RustSec/rustsec-crate/pull/4

## 0.2.0 (2017-02-25)

- Rename `package` TOML attribute to `crate_name` ([#3])
- Add iterator support to `AdvisoryDatabase` ([#2])

[#3]: https://github.com/RustSec/rustsec-crate/pull/3
[#2]: https://github.com/RustSec/rustsec-crate/pull/2

## 0.1.0 (2017-02-25)

- Initial release
