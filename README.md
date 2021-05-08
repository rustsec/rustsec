# RustSec Crates 📦

The RustSec Advisory Database is a repository of security advisories filed
against Rust crates published via [crates.io](https://crates.io).

The advisory database itself can be found at:

https://github.com/RustSec/advisory-db

## About this repository

This repository contains a Cargo Workspace with all of the crates maintained
by the RustSec project:

| Name              | Description                              | Crate | Documentation | Build |
|-------------------|------------------------------------------|-------|---------------|-------|
| [`cargo-audit`]   | Audit Cargo.lock against the advisory DB | [![crates.io](https://img.shields.io/crates/v/cargo-audit.svg)](https://crates.io/crates/cargo-audit) | [![Documentation](https://docs.rs/cargo-audit/badge.svg)](https://docs.rs/cargo-audit) | TODO |
| [`cargo-lock`]    | Self-contained Cargo.lock parser         | [![crates.io](https://img.shields.io/crates/v/cargo-lock.svg)](https://crates.io/crates/cargo-lock) | [![Documentation](https://docs.rs/cargo-lock/badge.svg)](https://docs.rs/cargo-lock) | TODO |
| [`cvss`]          | Common Vulnerability Scoring System      | [![crates.io](https://img.shields.io/crates/v/cvss.svg)](https://crates.io/crates/cvss) | [![Documentation](https://docs.rs/cvss/badge.svg)](https://docs.rs/cvss) | TODO |
| [`platforms`]     | Rust platform registry                   | [![crates.io](https://img.shields.io/crates/v/platforms.svg)](https://crates.io/crates/platforms) | [![Documentation](https://docs.rs/platforms/badge.svg)](https://docs.rs/platforms) | TODO |
| [`rustsec`]       | Advisory DB client library               | [![crates.io](https://img.shields.io/crates/v/rustsec.svg)](https://crates.io/crates/rustsec) | [![Documentation](https://docs.rs/rustsec/badge.svg)](https://docs.rs/rustsec) | TODO |
| [`rustsec-admin`] | Linter and web site generator            | [![crates.io](https://img.shields.io/crates/v/rustsec-admin.svg)](https://crates.io/crates/rustsec-admin) | [![Documentation](https://docs.rs/rustsec-admin/badge.svg)](https://docs.rs/rustsec-admin) | TODO |

## License

All crates licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

[//]: # (crates)

[`cargo-audit`]: https://github.com/RustSec/rustsec/tree/main/cargo-audit
[`cargo-lock`]: https://github.com/RustSec/rustsec/tree/main/cargo-lock
[`cvss`]: https://github.com/RustSec/rustsec/tree/main/cvss
[`platforms`]: https://github.com/RustSec/rustsec/tree/main/platforms
[`rustsec`]: https://github.com/RustSec/rustsec/tree/main/rustsec
[`rustsec-admin`]: https://github.com/RustSec/rustsec/tree/main/admin
