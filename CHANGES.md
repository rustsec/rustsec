## [0.7.1] (2018-07-23)

[0.7.1]: https://github.com/tendermint/yubihsm-rs/compare/v0.7.0...v0.7.1

* [#31](https://github.com/RustSec/rustsec-client/pull/31)
  Cargo.toml: Formatting fixups, add "readme" attribute.

## [0.7.0] (2018-07-22)

[0.7.0]: https://github.com/tendermint/yubihsm-rs/compare/v0.6.0...v0.7.0

* [#29](https://github.com/RustSec/rustsec-client/pull/29)
  Validate dates are well-formed.

* [#28](https://github.com/RustSec/rustsec-client/pull/28)
  Add AdvisoryIdKind and limited support for parsing advisory IDs.

* [#27](https://github.com/RustSec/rustsec-client/pull/27)
  Add a "Vulnerabilities" collection struct.

* [#23](https://github.com/RustSec/rustsec-client/pull/23)
  Parse aliases, references, and unaffected versions.

* [#22](https://github.com/RustSec/rustsec-client/pull/22)
  Parse (but do not yet verify) signatures on advisory-db commits.

* [#21](https://github.com/RustSec/rustsec-client/pull/21)
  Parse individual advisory .toml files rather than Advisories.toml.

* [#20](https://github.com/RustSec/rustsec-client/pull/20)
  Switch to git2-based fetcher for advisory-db.

* [#18](https://github.com/RustSec/rustsec-client/pull/18)
  Use serde to parse advisories TOML and Cargo.lock files.

* [#17](https://github.com/RustSec/rustsec-client/pull/17)
  Use 'failure' crate for error handling.

## 0.6.0 (2017-03-05)

* [#11](https://github.com/RustSec/rustsec-client/pull/11)
  Use semver::Version for lockfile::Package versions.
  ([@tarcieri])

* [#10](https://github.com/RustSec/rustsec-client/pull/10)
  Move AdvisoryDatabase under the ::db module.
  ([@tarcieri])
 
* [#9](https://github.com/RustSec/rustsec-client/pull/9)
  Lockfile support.
  ([@tarcieri])

## 0.5.2 (2017-02-26)

* [#8](https://github.com/RustSec/rustsec-client/pull/8)
  Add `AdvisoryDatabase::fetch_from_url()`.
  ([@tarcieri])

## 0.5.1 (2017-02-26)

* [#7](https://github.com/RustSec/rustsec-client/pull/7)
  Make `advisory` and `error` modules public.
  ([@tarcieri])

## 0.5.0 (2017-02-26)

* [#6](https://github.com/RustSec/rustsec-client/pull/6)
  Use str version param for `AdvisoryDatabase::find_vulns_for_crate()`.
  ([@tarcieri])

## 0.4.0 (2017-02-26)

* [#5](https://github.com/RustSec/rustsec-client/pull/5)
  Add `AdvisoryDatabase::find_vulns_for_crate()`.
  ([@tarcieri])

## 0.3.0 (2017-02-26)

* [#4](https://github.com/RustSec/rustsec-client/pull/4)
  Rename `crate_name` TOML attribute back to `package`.
  ([@tarcieri])

## 0.2.0 (2017-02-25)

* [#3](https://github.com/RustSec/rustsec-client/pull/3)
  Rename `package` TOML attribute to `crate_name`.
  ([@tarcieri])

* [#2](https://github.com/RustSec/rustsec-client/pull/2)
  Add iterator support to AdvisoryDatabase.
  ([@tarcieri])

## 0.1.0 (2017-02-25)

* Initial release

[@tarcieri]: https://github.com/tarcieri
