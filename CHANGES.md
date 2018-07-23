## [0.3.1] (2018-07-23)

[0.3.1]: https://github.com/tendermint/yubihsm-rs/compare/v0.3.0...v0.3.1

* [#25](https://github.com/RustSec/cargo-audit/pull/25)
  Use ` OR ` delimiter to display patched versions (fixes #18).
  ([@tarcieri])

* [#24](https://github.com/RustSec/cargo-audit/pull/24)
  Fix `cargo audit --version`.
  ([@tarcieri])

## [0.3.0] (2018-07-23)

[0.3.0]: https://github.com/tendermint/yubihsm-rs/compare/v0.2.1...v0.3.0

* [#22](https://github.com/RustSec/cargo-audit/pull/22)
  Near rewrite of cargo-audit (upgrades to rustsec 0.7.0).
  See also the [rustsec 0.7.0 release notes].
  ([@tarcieri])

[rustsec 0.7.0 release notes]: https://github.com/RustSec/rustsec-client/blob/master/CHANGES.md#070-2018-07-22

## 0.2.1 (2017-09-24)

* [#14](https://github.com/RustSec/cargo-audit/pull/14)
  Use crate isatty to resolve Windows build errors.
  ([@unv-annihilator])

## 0.2.0 (2017-03-05)

* [#12](https://github.com/RustSec/cargo-audit/pull/12)
  Upgrade to rustsec 0.6.0 crate.
  ([@tarcieri])

* [#10](https://github.com/RustSec/cargo-audit/pull/10)
  Configurable colors (fixes #5).
  ([@tarcieri])

* [#8](https://github.com/RustSec/cargo-audit/pull/8)
  Avoid panicking if there are no dependencies (fixes #4).
  ([@tarcieri])

* [#6](https://github.com/RustSec/cargo-audit/pull/6)
  Handle error and instruct the user to generate a lockfile before audit.
  ([@zmanian])

## 0.1.1 (2017-02-27)

* [#2](https://github.com/RustSec/cargo-audit/pull/2)
  Make cargo-audit a proper cargo subcommand.
  ([@tarcieri])

## 0.1.0 (2017-02-27)

* Initial release

[@tarcieri]: https://github.com/tarcieri
[@zmanian]: https://github.com/zmanian
[@unv-annihilator]: https://github.com/unv-annihilator
