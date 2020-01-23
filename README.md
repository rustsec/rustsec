# cargo audit

[![Latest Version][crate-image]][crate-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][rustc-image]
![Apache 2.0 OR MIT licensed][license-image]
[![Gitter Chat][gitter-image]][gitter-link]

Audit `Cargo.lock` files for crates with security vulnerabilities reported to the
[RustSec Advisory Database].

## Requirements

`cargo audit` requires Rust **1.39** or later.

## Installation

`cargo audit` is a Cargo subcommand and can be installed with `cargo install`:

```
$ cargo install cargo-audit
```

Once installed, run `cargo audit` at the toplevel of any Cargo project.

## Screenshot

<img src="https://github.com/RustSec/cargo-audit/raw/master/screenshot.png" alt="Screenshot" style="max-width:100%;">

## `cargo audit fix` subcommand

This tool supports an experimental feature to automatically update `Cargo.toml`
to fix vulnerable dependency requirements.

To enable it, install `cargo audit` with the `fix` feature enabled:

```
$ cargo install cargo-audit --features=fix
```

Once installed, run `cargo audit fix` to automatically fix vulnerable
dependency requirements.

This will modify `Cargo.toml` in place. To perform a dry run instead, which
shows a preview of what dependencies would be upgraded, run
`cargo audit fix --dry-run`.

## Using `cargo audit` on Travis CI

To automatically run `cargo audit` on every build in Travis CI, you can add the following to your `.travis.yml`:

```yaml
language: rust
cache: cargo # cache cargo-audit once installed
before_script:
  - cargo install --force cargo-audit
  - cargo generate-lockfile
script:
  - cargo audit
```

## Reporting Vulnerabilities

Report vulnerabilities by opening pull requests against the [RustSec Advisory Database]
GitHub repo:

<a href="https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md">
  <img alt="Report Vulnerability" width="250px" height="60px" src="https://rustsec.org/assets/img/report-vuln-button.svg">
</a>

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cargo-audit.svg
[crate-link]: https://crates.io/crates/cargo-audit
[build-image]: https://github.com/rustsec/cargo-audit/workflows/Rust/badge.svg
[build-link]: https://github.com/rustsec/cargo-audit/actions
[license-image]: https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.39+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

[//]: # (general links)

[RustSec Advisory Database]: https://github.com/RustSec/advisory-db/
[LICENSE-APACHE]: https://github.com/RustSec/cargo-audit/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/cargo-audit/blob/master/LICENSE-MIT
