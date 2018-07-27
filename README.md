# Rust "platforms" crate

[![Latest Version][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![MIT/Apache 2 licensed][license-image]
[![Gitter Chat][gitter-image]][gitter-link]

[crate-image]: https://img.shields.io/crates/v/platforms.svg
[crate-link]: https://crates.io/crates/platforms
[docs-image]: https://docs.rs/platforms/badge.svg
[docs-link]: https://docs.rs/platforms/
[build-image]: https://travis-ci.org/RustSec/platforms-crate.svg?branch=master
[build-link]: https://travis-ci.org/RustSec/platforms-crate
[license-image]: https://img.shields.io/badge/license-MIT%2FApache2-blue.svg
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

Rust platform registry: provides programmatic access to information
about valid Rust platforms, sourced from [Rust Forge].

[Documentation]

[Rust Forge]: https://forge.rust-lang.org/platform-support.html
[Documentation]: https://docs.rs/platforms/

## About

This crate provides programmatic access to information about valid Rust
platforms. This is useful for systems which document/inventory information
relevant to Rust platforms: specifically it was created for the
[RustSec Advisory Database] as a way to document and validate which Rust
platforms security advisories are applicable to.

It is not intended to be a tool for gating builds based on the current platform
or as a replacement for Rust's existing conditional compilation features:
please use those for build purposes.

[RustSec Advisory Database]: https://github.com/RustSec

## Registered Platforms

### Tier 1

| target triple                   | target_arch | target_os  | target_env |
|---------------------------------|-------------|------------|------------|
| i686-apple-darwin               | x86         | macos      | ""         |
| i686-pc-windows-gnu             | x86         | windows    | gnu        |
| i686-pc-windows-msvc            | x86         | windows    | msvc       |
| i686-unknown-linux-gnu          | x86         | linux      | gnu        |
| x86-64-apple-darwin             | x86_64      | macos      | ""         |
| x86_64-pc-windows-gnu           | x86_64      | windows    | gnu        |
| x86_64-pc-windows-msvc          | x86_64      | windows    | msvc       |
| x86_64-unknown-linux-gnu        | x86_64      | linux      | gnu        |

### Tier 2

| target triple                   | target_arch | target_os  | target_env |
|---------------------------------|-------------|------------|------------|
| aarch64-apple-ios               | aarch64     | ios        | ""         |
| aarch64-unknown-cloudabi        | aarch64     | cloudabi   | ""         |
| aarch64-linux-android           | aarch64     | android    | ""         |
| aarch64-unknown-fuchsia         | aarch64     | fuchsia    | ""         |
| aarch64-unknown-linux-gnu       | aarch64     | linux      | gnu        |
| aarch64-unknown-linux-musl      | aarch64     | linux      | musl       |
| arm-linux-androideabi           | arm         | android    | ""         |
| arm-unknown-linux-gnueabi       | arm         | linux      | gnu        |
| arm-unknown-linux-gnueabihf     | arm         | linux      | gnu        |
| arm-unknown-linux-musleabi      | arm         | linux      | musl       |
| arm-unknown-linux-musleabihf    | arm         | linux      | musl       |
| armv5te-unknown-linux-gnueabi   | arm         | linux      | gnu        |
| armv7-apple-ios                 | arm         | ios        | ""         |
| armv7-linux-androideabi         | arm         | android    | ""         |
| armv7-unknown-cloudabi-eabihf   | arm         | cloudabi   | ""         |
| armv7-unknown-linux-gnueabihf   | arm         | linux      | gnu        |
| armv7-unknown-linux-musleabihf  | arm         | linux      | musl       |
| armv7s-apple-ios                | arm         | ios        | ""         |
| asmjs-unknown-emscripten        | asmjs       | emscripten | ""         |
| i386-apple-ios                  | x86         | ios        | ""         |
| i586-pc-windows-msvc            | x86         | windows    | msvc       |
| i586-unknown-linux-gnu          | x86         | linux      | gnu        |
| i586-unknown-linux-musl         | x86         | linux      | gnu        |
| i686-linux-android              | x86         | android    | ""         |
| i686-unknown-cloudabi           | x86         | cloudabi   | ""         |
| i686-unknown-freebsd            | x86         | freebsd    | ""         |
| i686-unknown-linux-musl         | x86         | linux      | musl       |
| mips-unknown-linux-gnu          | mips        | linux      | gnu        |
| mips-unknown-linux-musl         | mips        | linux      | musl       |
| mips64-unknown-linux-gnuabi64   | mips64      | linux      | gnu        |
| mips64el-unknown-linux-gnuabi64 | mips64      | linux      | gnu        |
| mipsel-unknown-linux-gnu        | mips        | linux      | gnu        |
| mipsel-unknown-linux-musl       | mips        | linux      | musl       |
| powerpc-unknown-linux-gnu       | powerpc     | linux      | gnu        |
| powerpc64-unknown-linux-gnu     | powerpc64   | linux      | gnu        |
| powerpc64le-unknown-linux-gnu   | powerpc64   | linux      | gnu        |
| s390x-unknown-linux-gnu         | s390x       | linux      | gnu        |
| sparc64-unknown-linux-gnu       | sparc64     | linux      | gnu        |
| sparcv9-sun-solaris             | sparc64     | solaris    | ""         |
| wasm32-unknown-unknown          | wasm32      | unknown    | ""         |
| wasm32-unknown-emscripten       | wasm32      | emscripten | ""         |
| x86_64-apple-ios                | x86_64      | ios        | ""         |
| x86_64-linux-android            | x86_64      | android    | ""         |
| x86_64-rumprun-netbsd           | x86_64      | netbsd     | ""         |
| x86_64-sun-solaris              | x86_64      | solaris    | ""         |
| x86_64-unknown-cloudabi         | x86_64      | cloudabi   | ""         |
| x86_64-unknown-freebsd          | x86_64      | freebsd    | ""         |
| x86_64-unknown-fuchsia          | x86_64      | fuchsia    | ""         |
| x86_64-unknown-linux-gnux32     | x86_64      | linux      | gnu        |
| x86_64-unknown-linux-musl       | x86_64      | linux      | musl       |
| x86_64-unknown-netbsd           | x86_64      | netbsd     | ""         |
| x86_64-unknown-redox            | x86_64      | redox      | ""         |

### Tier 3

| target triple                   | target_arch | target_os  | target_env |
|---------------------------------|-------------|------------|------------|
| i686-unknown-haiku              | x86         | haiku      | ""         |
| i686-unknown-netbsd             | x86         | netbsd     | ""         |
| le32-unknown-nacl               | unknown     | unknown    | ""         |
| mips-unknown-linux-uclibc       | mips        | linux      | uclibc     |
| mipsel-unknown-linux-uclibc     | mips        | linux      | uclibc     |
| msp430-none-elf                 | msp430      | unknown    | ""         |
| sparc64-unknown-netbsd          | sparc64     | netbsd     | ""         |
| thumbv6m-none-eabi              | unknown     | unknown    | ""         |
| thumbv7em-none-eabi             | unknown     | unknown    | ""         |
| thumbv7em-none-eabihf           | unknown     | unknown    | ""         |
| thumbv7m-none-eabi              | unknown     | unknown    | ""         |
| x86_64-unknown-bitrig           | x86_64      | bitrig     | ""         |
| x86_64-unknown-dragonfly        | x86_64      | dragonfly  | ""         |
| x86_64-unknown-haiku            | x86_64      | haiku      | ""         |
| x86_64-unknown-openbsd          | x86_64      | openbsd    | ""         |

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

[LICENSE-APACHE]: https://github.com/RustSec/rustsec-client/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/rustsec-client/blob/master/LICENSE-MIT

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
