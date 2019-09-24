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
[build-image]: https://github.com/rustsec/platforms-crate/workflows/Rust/badge.svg
[build-link]: https://github.com/rustsec/platforms-crate/actions
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

| target triple                     | target_arch | target_os  | target_env |
|-----------------------------------|-------------|------------|------------|
| [i686-apple-darwin]               | x86         | macos      | ""         |
| [i686-pc-windows-gnu]             | x86         | windows    | gnu        |
| [i686-pc-windows-msvc]            | x86         | windows    | msvc       |
| [i686-unknown-linux-gnu]          | x86         | linux      | gnu        |
| [x86_64-apple-darwin]             | x86_64      | macos      | ""         |
| [x86_64-pc-windows-gnu]           | x86_64      | windows    | gnu        |
| [x86_64-pc-windows-msvc]          | x86_64      | windows    | msvc       |
| [x86_64-unknown-linux-gnu]        | x86_64      | linux      | gnu        |

### Tier 2

| target triple                     | target_arch | target_os  | target_env |
|-----------------------------------|-------------|------------|------------|
| [aarch64-apple-ios]               | aarch64     | ios        | ""         |
| [aarch64-linux-android]           | aarch64     | android    | ""         |
| [aarch64-fuchsia]                 | aarch64     | fuchsia    | ""         |
| [aarch64-unknown-linux-gnu]       | aarch64     | linux      | gnu        |
| [aarch64-unknown-linux-musl]      | aarch64     | linux      | musl       |
| [arm-linux-androideabi]           | arm         | android    | ""         |
| [arm-unknown-linux-gnueabi]       | arm         | linux      | gnu        |
| [arm-unknown-linux-gnueabihf]     | arm         | linux      | gnu        |
| [arm-unknown-linux-musleabi]      | arm         | linux      | musl       |
| [arm-unknown-linux-musleabihf]    | arm         | linux      | musl       |
| [armv5te-unknown-linux-gnueabi]   | arm         | linux      | gnu        |
| [armv7-apple-ios]                 | arm         | ios        | ""         |
| [armv7-linux-androideabi]         | arm         | android    | ""         |
| [armv7-unknown-linux-gnueabihf]   | arm         | linux      | gnu        |
| [armv7-unknown-linux-musleabihf]  | arm         | linux      | musl       |
| [armv7s-apple-ios]                | arm         | ios        | ""         |
| [asmjs-unknown-emscripten]        | asmjs       | emscripten | ""         |
| [i386-apple-ios]                  | x86         | ios        | ""         |
| [i586-pc-windows-msvc]            | x86         | windows    | msvc       |
| [i586-unknown-linux-gnu]          | x86         | linux      | gnu        |
| [i586-unknown-linux-musl]         | x86         | linux      | gnu        |
| [i686-linux-android]              | x86         | android    | ""         |
| [i686-unknown-freebsd]            | x86         | freebsd    | ""         |
| [i686-unknown-linux-musl]         | x86         | linux      | musl       |
| [mips-unknown-linux-gnu]          | mips        | linux      | gnu        |
| [mips-unknown-linux-musl]         | mips        | linux      | musl       |
| [mips64-unknown-linux-gnuabi64]   | mips64      | linux      | gnu        |
| [mips64el-unknown-linux-gnuabi64] | mips64      | linux      | gnu        |
| [mipsel-unknown-linux-gnu]        | mips        | linux      | gnu        |
| [mipsel-unknown-linux-musl]       | mips        | linux      | musl       |
| [powerpc-unknown-linux-gnu]       | powerpc     | linux      | gnu        |
| [powerpc64-unknown-linux-gnu]     | powerpc64   | linux      | gnu        |
| [powerpc64le-unknown-linux-gnu]   | powerpc64   | linux      | gnu        |
| [s390x-unknown-linux-gnu]         | s390x       | linux      | gnu        |
| [sparc64-unknown-linux-gnu]       | sparc64     | linux      | gnu        |
| [sparcv9-sun-solaris]             | sparc64     | solaris    | ""         |
| [wasm32-unknown-unknown]          | wasm32      | unknown    | ""         |
| [wasm32-unknown-emscripten]       | wasm32      | emscripten | ""         |
| [x86_64-apple-ios]                | x86_64      | ios        | ""         |
| [x86_64-linux-android]            | x86_64      | android    | ""         |
| [x86_64-rumprun-netbsd]           | x86_64      | netbsd     | ""         |
| [x86_64-sun-solaris]              | x86_64      | solaris    | ""         |
| [x86_64-unknown-cloudabi]         | x86_64      | cloudabi   | ""         |
| [x86_64-unknown-freebsd]          | x86_64      | freebsd    | ""         |
| [x86_64-fuchsia]                  | x86_64      | fuchsia    | ""         |
| [x86_64-unknown-linux-gnux32]     | x86_64      | linux      | gnu        |
| [x86_64-unknown-linux-musl]       | x86_64      | linux      | musl       |
| [x86_64-unknown-netbsd]           | x86_64      | netbsd     | ""         |
| [x86_64-unknown-redox]            | x86_64      | redox      | ""         |
| [aarch64-unknown-cloudabi]        | aarch64     | cloudabi   | ""         |
| [armv7-unknown-cloudabi-eabihf]   | arm         | cloudabi   | ""         |
| [i686-unknown-cloudabi]           | x86         | cloudabi   | ""         |
| [powerpc-unknown-linux-gnuspe]    | powerpc     | linux      | gnu        |
| [sparc-unknown-linux-gnu]         | sparc       | linux      | gnu        |

### Tier 3

| target triple                     | target_arch | target_os  | target_env |
|-----------------------------------|-------------|------------|------------|
| [i686-unknown-haiku]              | x86         | haiku      | ""         |
| [i686-unknown-netbsd]             | x86         | netbsd     | ""         |
| [mips-unknown-linux-uclibc]       | mips        | linux      | uclibc     |
| [mipsel-unknown-linux-uclibc]     | mips        | linux      | uclibc     |
| [msp430-none-elf]                 | msp430      | unknown    | ""         |
| [sparc64-unknown-netbsd]          | sparc64     | netbsd     | ""         |
| [thumbv6m-none-eabi]              | thumbv6     | unknown    | ""         |
| [thumbv7em-none-eabi]             | thumbv7     | unknown    | ""         |
| [thumbv7em-none-eabihf]           | thumbv7     | unknown    | ""         |
| [thumbv7m-none-eabi]              | thumbv7     | unknown    | ""         |
| [x86_64-fortanix-unknown-sgx]     | x86_64      | unknown    | sgx        |
| [x86_64-unknown-bitrig]           | x86_64      | bitrig     | ""         |
| [x86_64-unknown-dragonfly]        | x86_64      | dragonfly  | ""         |
| [x86_64-unknown-haiku]            | x86_64      | haiku      | ""         |
| [x86_64-unknown-openbsd]          | x86_64      | openbsd    | ""         |

[i686-apple-darwin]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.I686_APPLE_DARWIN.html
[i686-pc-windows-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.I686_PC_WINDOWS_GNU.html
[i686-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.I686_PC_WINDOWS_MSVC.html
[i686-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.I686_UNKNOWN_LINUX_GNU.html
[x86_64-apple-darwin]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.X86_64_APPLE_DARWIN.html
[x86_64-pc-windows-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.X86_64_PC_WINDOWS_GNU.html
[x86_64-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.X86_64_PC_WINDOWS_MSVC.html
[x86_64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier1/constant.X86_64_UNKNOWN_LINUX_GNU.html
[aarch64-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.AARCH64_APPLE_IOS.html
[aarch64-linux-android]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.AARCH64_LINUX_ANDROID.html
[aarch64-fuchsia]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.AARCH64_FUCHSIA.html
[aarch64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.AARCH64_UNKNOWN_LINUX_GNU.html
[aarch64-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.AARCH64_UNKNOWN_LINUX_MUSL.html
[arm-linux-androideabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARM_LINUX_ANDROIDEABI.html
[arm-unknown-linux-gnueabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARM_UNKNOWN_LINUX_GNUEABI.html
[arm-unknown-linux-gnueabihf]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARM_UNKNOWN_LINUX_GNUEABIHF.html
[arm-unknown-linux-musleabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARM_UNKNOWN_LINUX_MUSLEABI.html
[arm-unknown-linux-musleabihf]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARM_UNKNOWN_LINUX_MUSLEABIHF.html
[armv5te-unknown-linux-gnueabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV5TE_UNKNOWN_LINUX_GNUEABI.html
[armv7-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV7_APPLE_IOS.html
[armv7-linux-androideabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV7_LINUX_ANDROIDEABI.html
[armv7-unknown-linux-gnueabihf]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV7_UNKNOWN_LINUX_GNUEABIHF.html
[armv7-unknown-linux-musleabihf]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV7_UNKNOWN_LINUX_MUSLEABIHF.html
[armv7s-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV7S_APPLE_IOS.html
[asmjs-unknown-emscripten]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ASMJS_UNKNOWN_EMSCRIPTEN.html
[i386-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I386_APPLE_IOS.html
[i586-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I586_PC_WINDOWS_MSVC.html
[i586-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I586_UNKNOWN_LINUX_GNU.html
[i586-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I586_UNKNOWN_LINUX_MUSL.html
[i686-linux-android]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I686_LINUX_ANDROID.html
[i686-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I686_UNKNOWN_FREEBSD.html
[i686-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I686_UNKNOWN_LINUX_MUSL.html
[mips-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.MIPS_UNKNOWN_LINUX_GNU.html
[mips-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.MIPS_UNKNOWN_LINUX_MUSL.html
[mips64-unknown-linux-gnuabi64]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.MIPS64_UNKNOWN_LINUX_GNUABI64.html
[mips64el-unknown-linux-gnuabi64]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.MIPS64EL_UNKNOWN_LINUX_GNUABI64.html
[mipsel-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.MIPSEL_UNKNOWN_LINUX_GNU.html
[mipsel-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.MIPSEL_UNKNOWN_LINUX_MUSL.html
[powerpc-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.POWERPC_UNKNOWN_LINUX_GNU.html
[powerpc64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.POWERPC64_UNKNOWN_LINUX_GNU.html
[powerpc64le-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.POWERPC64LE_UNKNOWN_LINUX_GNU.html
[s390x-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.S390X_UNKNOWN_LINUX_GNU.html
[sparc64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.SPARC64_UNKNOWN_LINUX_GNU.html
[sparcv9-sun-solaris]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.SPARCV9_SUN_SOLARIS.html
[wasm32-unknown-unknown]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.WASM32_UNKNOWN_UNKNOWN.html
[wasm32-unknown-emscripten]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.WASM32_UNKNOWN_EMSCRIPTEN.html
[x86_64-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_APPLE_IOS.html
[x86_64-linux-android]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_LINUX_ANDROID.html
[x86_64-rumprun-netbsd]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_RUMPRUN_NETBSD.html
[x86_64-sun-solaris]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_SUN_SOLARIS.html
[x86_64-unknown-cloudabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_UNKNOWN_CLOUDABI.html
[x86_64-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_UNKNOWN_FREEBSD.html
[x86_64-fuchsia]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_FUCHSIA.html
[x86_64-unknown-linux-gnux32]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_UNKNOWN_LINUX_GNUX32.html
[x86_64-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_UNKNOWN_LINUX_MUSL.html
[x86_64-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_UNKNOWN_NETBSD.html
[x86_64-unknown-redox]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.X86_64_UNKNOWN_REDOX.html
[aarch64-unknown-cloudabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.AARCH64_UNKNOWN_CLOUDABI.html
[armv7-unknown-cloudabi-eabihf]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.ARMV7_UNKNOWN_CLOUDABI_EABIHF.html
[i686-unknown-cloudabi]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.I686_UNKNOWN_CLOUDABI.html
[powerpc-unknown-linux-gnuspe]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.POWERPC_UNKNOWN_LINUX_GNUSPE.html
[sparc-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/tier2/constant.SPARC_UNKNOWN_LINUX_GNU.html
[i686-unknown-haiku]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.I686_UNKNOWN_HAIKU.html
[i686-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.I686_UNKNOWN_NETBSD.html
[mips-unknown-linux-uclibc]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.MIPS_UNKNOWN_LINUX_UCLIBC.html
[mipsel-unknown-linux-uclibc]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.MIPSEL_UNKNOWN_LINUX_UCLIBC.html
[msp430-none-elf]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.MSP430_NONE_ELF.html
[sparc64-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.SPARC64_UNKNOWN_NETBSD.html
[thumbv6m-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.THUMBV6M_NONE_EABI.html
[thumbv7em-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.THUMBV7EM_NONE_EABI.html
[thumbv7em-none-eabihf]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.THUMBV7EM_NONE_EABIHF.html
[thumbv7m-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.THUMBV7M_NONE_EABI.html
[x86_64-fortanix-unknown-sgx]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.X86_64_FORTANIX_UNKNOWN_SGX.html
[x86_64-unknown-bitrig]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.X86_64_UNKNOWN_BITRIG.html
[x86_64-unknown-dragonfly]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.X86_64_UNKNOWN_DRAGONFLY.html
[x86_64-unknown-haiku]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.X86_64_UNKNOWN_HAIKU.html
[x86_64-unknown-openbsd]: https://docs.rs/platforms/latest/platforms/platform/tier3/constant.X86_64_UNKNOWN_OPENBSD.html

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
