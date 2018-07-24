# cargo audit

[![Latest Version][crate-image]][crate-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]
![MIT/Apache 2 licensed][license-image]
[![Gitter Chat][gitter-image]][gitter-link]

[crate-image]: https://img.shields.io/crates/v/cargo-audit.svg
[crate-link]: https://crates.io/crates/cargo-audit
[build-image]: https://travis-ci.org/RustSec/cargo-audit.svg?branch=master
[build-link]: https://travis-ci.org/RustSec/cargo-audit
[appveyor-image]: https://ci.appveyor.com/api/projects/status/oa39c0in9qkxpoiv?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tarcieri/cargo-audit
[license-image]: https://img.shields.io/badge/license-MIT%2FApache2-blue.svg
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

Audit Cargo.lock for crates with security vulnerabilities reported to the
[RustSec Advisory Database].

This implements an idea originally proposed in this (closed) RFC:

https://github.com/rust-lang/rfcs/pull/1752

[RustSec Advisory Database]: https://github.com/RustSec/advisory-db/

## Installation

`cargo audit` is a Cargo subcommand and can be installed with `cargo install`:

```
$ cargo install cargo-audit
```

Once installed, it can be run on the toplevel of any Cargo project.

## Reporting Vulnerabilities

Vulneraties can be reported by opening pull requests against the
[RustSec Advisory Database] GitHub repo:

<a href="https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md">
  <img alt="Report Vulnerability" width="250px" height="60px" src="https://rustsec.org/assets/img/report-vuln-button.svg">
</a>

## Screenshot

<img src="https://github.com/RustSec/cargo-audit/raw/master/screenshot.png" alt="Screenshot" style="max-width:100%;">

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

[LICENSE-APACHE]: https://github.com/RustSec/cargo-audit/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/cargo-audit/blob/master/LICENSE-MIT

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
