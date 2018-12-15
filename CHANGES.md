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

- Allow AdvisoryId::new() to parse `RUSTSEC-0000-0000` ([#36])

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
