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
