# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.26.5 (2023-03-22)
### Changed

- Migrated to a maintained fork of `cargo-edit` v0.9.x to fix [CVE-2023-22742] in the transitive dependency `libgit2-sys` ([#831])
- Removed the experimental check for the presence of a signature on the advisory-db repository. It only verified the presence of a signature without checking for any particular key, so it provided no additional security. ([#816])
- Fixed a build failure with certain dependency versions on recent compilers due to failing type inference ([#836])

[CVE-2023-22742]: https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2023-22742
[#816]: https://github.com/rustsec/rustsec/pull/816
[#831]: https://github.com/rustsec/rustsec/pull/831
[#836]: https://github.com/rustsec/rustsec/pull/836

## 0.26.4 (2022-11-15)
### Fixed
- `registry::CachedIndex` now correctly handles invalid semver versions in crates.io registry, which crates.io allows for some reason ([#762])

[#762]: https://github.com/rustsec/rustsec/pull/762

## 0.26.3 (2022-11-01)
### Added
- `registry::CachedIndex` which is orders of magnitude faster than `registry::Index` when scanning multiple `Cargo.lock` files or binaries ([#730])

[#730]: https://github.com/rustsec/rustsec/pull/730

## 0.26.2 (2022-08-15)
### Fixed
- Fixed `withdrawn` ([#642])

[#642]: https://github.com/RustSec/rustsec/pull/642
	
## 0.26.1 (2022-08-14)
### Changed
- Deprecate `yanked` ([#631])

[#631]: https://github.com/RustSec/rustsec/pull/631

## 0.26.0 (2022-05-21)
### Added
- `[advisory.source]` ([#541])
- `doc_cfg` annotations when building on docs.rs ([#571])

### Changed
- Bump `git2` dependency to v0.14; MSRV 1.57 ([#524])
- Bump `platforms` dependency to v3.0 ([#532])
- Update to 2021 edition ([#538])
- Use `Query::crate_scope()` as the `Default` ([#544])
- Bump `cvss` dependency to v2.0 ([#550])
- Bump `cargo-lock` dependency to v8.0 ([#561])
- Flatten `warnings` module; rename `WarningKind` ([#572])
- Flatten `advisory::id` module; rename `IdKind` ([#573])

### Removed
- Legacy database scopes ([#541])

[#524]: https://github.com/RustSec/rustsec/pull/524
[#532]: https://github.com/RustSec/rustsec/pull/532
[#538]: https://github.com/RustSec/rustsec/pull/538
[#541]: https://github.com/RustSec/rustsec/pull/541
[#544]: https://github.com/RustSec/rustsec/pull/544
[#550]: https://github.com/RustSec/rustsec/pull/550
[#561]: https://github.com/RustSec/rustsec/pull/561
[#571]: https://github.com/RustSec/rustsec/pull/571
[#572]: https://github.com/RustSec/rustsec/pull/572
[#573]: https://github.com/RustSec/rustsec/pull/573

## 0.25.1 (2021-11-15)
### Changed
- Bump `platforms` dependency to v2.0.0 ([#485])

[#485]: https://github.com/RustSec/rustsec/pull/485

## 0.25.0 (2021-11-12) [YANKED]
### Changed
- Bump `cargo-edit` dependency from 0.7.0 to 0.8.0 ([#439])
- Make `advisory::id::Kind` lowercase ([#471])
- Bump MSRV to 1.52 ([#476])
- Flatten API: make modules with one type non-`pub`; re-export type from parent ([#478])

[#439]: https://github.com/RustSec/rustsec/pull/439
[#471]: https://github.com/RustSec/rustsec/pull/471
[#476]: https://github.com/RustSec/rustsec/pull/476
[#478]: https://github.com/RustSec/rustsec/pull/478

## 0.24.3 (2021-09-11)
### Added
- `vendored-libgit2` feature ([#432])

### Changed
- OSV v1.0 ([#421])

[#421]: https://github.com/RustSec/rustsec/pull/421
[#432]: https://github.com/RustSec/rustsec/pull/432

## 0.24.2 (2021-07-20)
### Changed
- Support `~` and `=` operators in version specification ([#402])
- Bump `crates-index` from 0.16.7 to 0.17.0 ([#403])

[#402]: https://github.com/RustSec/rustsec/pull/402
[#403]: https://github.com/RustSec/rustsec/pull/403

## 0.24.1 (2021-07-02)
### Changed
- Do not lint year in CVE IDs ([#393])

[#393]: https://github.com/RustSec/rustsec/pull/393

## 0.24.0 (2021-06-28)
### Added
- OSV export ([#366])

### Changed
- Bump `cargo-lock` to v7.0 ([#379])

[#366]: https://github.com/RustSec/rustsec/pull/366
[#379]: https://github.com/RustSec/rustsec/pull/379

## 0.23.3 (2021-03-08)
### Fixed
- Workaround for stale git refs

## 0.23.2 (2021-03-07)
### Changed
- Rename advisory-db `master` branch to `main`

## 0.23.1 (2021-02-24)
### Fixed
- Parsing error on Windows

## 0.23.0 (2021-01-26)
### Added
- Advisory `references` as a URL list
- Support for omitting leading `[advisory]` table
- `thread-safety` category

### Changed
- Rename previous `references` field to `related`
- Use `url` crate to parse metadata URL
- Bump `smol_str` to v0.1.17; MSRV 1.46+
- Replace `chrono` with `humantime`
- Mark enums as non_exhaustive
- Use `SystemTime` instead of a `git::Timestamp` type
- Rename `fetch` Cargo feature to `git`
- Rename `repository::GitRepository` to `repository::git::Repository`

### Removed
- `markdown` feature

## 0.22.2 (2020-10-27)
### Changed
- Revert "Refactor Advisory type handling"

## 0.22.1 (2020-10-26) [YANKED]
### Changed
- Refactor `Advisory` and `VulnerabilityInfo`

## 0.22.0 (2020-10-25) [YANKED]
### Added
- `fetch` feature

### Changed
- Bump `cargo-lock` to v6; `semver` to v0.11
- Make `advisory.title` and `advisory.description` struct fields
- Remove support for the V2 advisory format
- Mark the `advisory::parser` module as `pub`
- Bump `cargo-edit` to 0.7.0
- Bump `crates-index` from 0.15.4 to 0.16.0
- `advisory`: laxer function path handling
- `linter`: fully deprecate `obsolete` in favor of `yanked`
- `advisory`: `markdown` feature and `Advisory::description_html`
- `linter`: add support for V3 advisory format
- MSRV 1.41+
- Bump `platforms` crate to v1

### Fixed
- `linter`: correctly handle crates with dashes in names

### Removed
- `advisory.metadata.title` and `advisory.metadata.description`

## 0.21.0 (2020-06-23)
### Added
- `year`, `month`, and `day` methods to `advisory::Date`
- `unsound` informational advisory kind

### Changed
- Bump `crates-index` from 0.14 to 0.15
- Rename `obsolete` advisories to `yanked`
- Rename `warning::Kind::Informational` to `::Notice`
- Make `warning::Kind` a `#[non_exhausive]` enum
- Make `Informational` a `#[non_exhausive]` enum

### Removed
- Legacy `patched_versions` and `unaffected_versions`

## 0.20.1 (2020-06-14)
### Added
- `advisory::Id::numerical_part()`

## 0.20.0 (2020-05-06)
### Changed
- Make `WarningInfo` into a simple type alias

## 0.19.0 (2020-05-04)

- Refactor package scopes
- Prototype V3 Advisory Format
- Bump dependencies to link `libgit2` dynamically
- Add `WarningInfo` and modify `Warning` struct
- Drop support for the V1 advisory format

## 0.18.0 (2020-02-05)

- Move yanked crate auditing to `cargo-audit`

## 0.17.1 (2020-01-22)

- Update `cargo-lock` requirement from 3.0 to 4.0

## 0.17.0 (2020-01-19)

- Bump MSRV to 1.39
- Extract `cargo audit fix` logic into `Fixer`
- Warn for yanked crates
- Add `vendored-openssl` feature
- Support crate sources as a vulnerability query attribute
- Try to auto-detect proxy setting

## 0.16.0 (2019-10-13)

- Remove `support.toml` parsing

## 0.15.2 (2019-10-08)

- version: Fix matching bug for `>` version requirements


## 0.15.1 (2019-10-07)

- linter: Add `informational` as an allowable `[advisory]` key
- repository: Expose `authentication` module

## 0.15.0 (2019-10-01)

- Upgrade to `cargo-lock` crate v3.0

## 0.14.1 (2019-09-25)

- Upgrade to `cargo-lock` crate v2.0

## 0.14.0 (2019-09-24)

- warning: Extract into module; make more like `Vulnerability`
- Upgrade to `cvss` crate v1.0
- Upgrade to `cargo-lock` crate v1.0

## 0.13.0 (2019-09-23)

- linter: Ensure advisory date's year matches year in advisory ID
- Use the `cargo-lock` crate
- lockfile: Add (optional) DependencyGraph analysis
- Rename `rustsec::db` module to `rustsec::database`
- report: Generate warnings for selected informational advisories
- vulnerability: Add `affected_functions()`
- Add `rustsec::advisory::Linter`
- package: Parse dependencies from Cargo.lock
- Initial `report` module and built-in report-generating
- Basic query support
- Index the `rust` advisory directory from `RustSec/advisory-db`
- Add first-class support for GitHub Security Advisories (GHSA)
- Re-vendor Cargo's git authentication code
- `support.toml` for indicating supported versions
- Add support for "informational" advisories
- Add `rustsec::advisory::Category`
- Refactor advisory types: add `[affected]` and `[versions]` sections
- advisory: Add (optional) `cvss` field with CVSS v3.1 score
- Freshen deps: add `home`, remove `directories` and `failure`
- Improved handling of prereleases; MSRV 1.35+
- Add `Version` and `VersionReq` newtypes

## 0.12.1 (2019-07-29)

- Use new inclusive range syntax

## 0.12.0 (2019-07-15)

- Update dependencies and use 2018 import conventions; Rust 1.32+
- Re-export all types in `advisory::paths::*`

## 0.11.0 (2019-01-13)

- Cargo.toml: Update `platforms` crate to v0.2
- Redo advisory's `affected_functions` as `affected_paths`

## 0.10.0 (2018-12-14)

- Implement `affected_functions` advisory attribute
- Fix handling of `unaffected_versions`
- Update to Rust 2018 edition

## 0.9.3 (2018-10-14)

- Create parents of the `advisory-db` repo dir 

## 0.9.2 (2018-10-14)

- Handle cloning `advisory-db` into existing, empty dir

## 0.9.1 (2018-07-29)

- Use Cargo's git authentication helper

## 0.9.0 (2018-07-26)

- Use `platforms` crate for platform-related functionality

## 0.8.0 (2018-07-24)

- Advisory platform requirements
- Cargo-like keyword support

## 0.7.5 (2018-07-24)

- Allow `AdvisoryId::new()` to parse `RUSTSEC-0000-0000`

## 0.7.4 (2018-07-23)

- Add link to logo image for docs.rs

## 0.7.3 (2018-07-23)

- Fix builds with `--no-default-features`

## 0.7.2 (2018-07-23)

- README.md: Badge fixups, add gitter badge

## 0.7.1 (2018-07-23)

- Cargo.toml: Formatting fixups, add `readme` attribute

## 0.7.0 (2018-07-22)

- Validate dates are well-formed
- Add `AdvisoryIdKind` and limited support for parsing advisory IDs
- Add a `Vulnerabilities` collection struct
- Parse aliases, references, and unaffected versions
- Parse (but do not yet verify) signatures on advisory-db commits
- Parse individual advisory `.toml` files rather than Advisories.toml
- Switch to `git2`-based fetcher for `advisory-db`
- Use serde to parse advisories TOML and `Cargo.lock` files
- Use `failure` crate for error handling

## 0.6.0 (2017-03-05)

- Use `semver::Version` for `lockfile::Package` versions
- Move `AdvisoryDatabase` under the `::db` module
- Lockfile support

## 0.5.2 (2017-02-26)

- Add `AdvisoryDatabase::fetch_from_url()`

## 0.5.1 (2017-02-26)

- Make `advisory` and `error` modules public

## 0.5.0 (2017-02-26)

- Use str version param for `AdvisoryDatabase::find_vulns_for_crate()`

## 0.4.0 (2017-02-26)

- Add `AdvisoryDatabase::find_vulns_for_crate()`

## 0.3.0 (2017-02-26)

- Rename `crate_name` TOML attribute back to `package`

## 0.2.0 (2017-02-25)

- Rename `package` TOML attribute to `crate_name`
- Add iterator support to `AdvisoryDatabase`

## 0.1.0 (2017-02-25)

- Initial release
