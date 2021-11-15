# RustSec: `platforms` crate

[![Latest Version][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache 2/MIT licensed][license-image]
![MSRV][rustc-image]
[![Project Chat][zulip-image]][zulip-link]

Rust platform registry: provides programmatic access to information
about valid Rust platforms, sourced from the Rust compiler.

[Documentation][docs-link]

## About

This crate provides programmatic access to information about valid Rust
platforms. This is useful for systems which document/inventory information
relevant to Rust platforms.

It was created for the [RustSec Advisory Database] and is maintained by the
[Rust Secure Code Working Group][wg-secure-code].

It is not intended to be a tool for gating builds based on the current platform
or as a replacement for Rust's existing conditional compilation features:
please use those for build purposes.

## Minimum Supported Rust Version

Rust **1.40** or higher.

Minimum supported Rust version may be changed in the future, but it will be
accompanied by a minor version bump.

## Registered Platforms

### Tier 1

| target triple                         | target_arch | target_os  | target_env |
|---------------------------------------|-------------|------------|------------|
| [aarch64-unknown-linux-gnu]           | aarch64     | linux      | gnu        |
| [i686-pc-windows-gnu]                 | x86         | windows    | gnu        |
| [i686-pc-windows-msvc]                | x86         | windows    | msvc       |
| [i686-unknown-linux-gnu]              | x86         | linux      | gnu        |
| [x86_64-apple-darwin]                 | x86_64      | macos      | ""         |
| [x86_64-pc-windows-gnu]               | x86_64      | windows    | gnu        |
| [x86_64-pc-windows-msvc]              | x86_64      | windows    | msvc       |
| [x86_64-unknown-linux-gnu]            | x86_64      | linux      | gnu        |

### Tier 2

| target triple                         | target_arch | target_os  | target_env |
|---------------------------------------|-------------|------------|------------|
| [aarch64-apple-darwin]                | aarch64     | macos      | ""         |
| [aarch64-apple-ios]                   | aarch64     | ios        | ""         |
| [aarch64-pc-windows-msvc]             | aarch64     | windows    | msvc       |
| [aarch64-linux-android]               | aarch64     | android    | ""         |
| [aarch64-fuchsia]                     | aarch64     | fuchsia    | ""         |
| [aarch64-unknown-linux-musl]          | aarch64     | linux      | musl       |
| [aarch64-unknown-none]                | aarch64     | unknown    | ""         |
| [aarch64-unknown-none-softfloat]      | aarch64     | unknown    | ""         |
| [arm-linux-androideabi]               | arm         | android    | ""         |
| [arm-unknown-linux-gnueabi]           | arm         | linux      | gnu        |
| [arm-unknown-linux-gnueabihf]         | arm         | linux      | gnu        |
| [arm-unknown-linux-musleabi]          | arm         | linux      | musl       |
| [arm-unknown-linux-musleabihf]        | arm         | linux      | musl       |
| [armv5te-unknown-linux-gnueabi]       | arm         | linux      | gnu        |
| [armv5te-unknown-linux-musleabi]      | arm         | linux      | musl       |
| [armv7-linux-androideabi]             | arm         | android    | ""         |
| [armv7-unknown-linux-gnueabi]         | arm         | linux      | gnu        |
| [armv7-unknown-linux-gnueabihf]       | arm         | linux      | gnu        |
| [armv7-unknown-linux-musleabi]        | arm         | linux      | musl       |
| [armv7-unknown-linux-musleabihf]      | arm         | linux      | musl       |
| [armebv7r-none-eabi]                  | arm         | unknown    | ""         |
| [armebv7r-none-eabihf]                | arm         | unknown    | ""         |
| [asmjs-unknown-emscripten]            | asmjs       | emscripten | ""         |
| [i586-pc-windows-msvc]                | x86         | windows    | msvc       |
| [i586-unknown-linux-gnu]              | x86         | linux      | gnu        |
| [i586-unknown-linux-musl]             | x86         | linux      | gnu        |
| [i686-linux-android]                  | x86         | android    | ""         |
| [i686-unknown-freebsd]                | x86         | freebsd    | ""         |
| [i686-unknown-linux-musl]             | x86         | linux      | musl       |
| [mips-unknown-linux-gnu]              | mips        | linux      | gnu        |
| [mips-unknown-linux-musl]             | mips        | linux      | musl       |
| [mips64-unknown-linux-gnuabi64]       | mips64      | linux      | gnu        |
| [mips64-unknown-linux-muslabi64]      | mips64      | linux      | musl       |
| [mips64el-unknown-linux-gnuabi64]     | mips64      | linux      | gnu        |
| [mips64el-unknown-linux-muslabi64]    | mips64      | linux      | musl       |
| [mipsel-unknown-linux-gnu]            | mips        | linux      | gnu        |
| [mipsel-unknown-linux-musl]           | mips        | linux      | musl       |
| [nvptx64-nvidia-cuda]                 | nvptx64     | cuda       | ""         |
| [powerpc-unknown-linux-gnu]           | powerpc     | linux      | gnu        |
| [powerpc64-unknown-linux-gnu]         | powerpc64   | linux      | gnu        |
| [powerpc64le-unknown-linux-gnu]       | powerpc64   | linux      | gnu        |
| [s390x-unknown-linux-gnu]             | s390x       | linux      | gnu        |
| [sparc64-unknown-linux-gnu]           | sparc64     | linux      | gnu        |
| [sparcv9-sun-solaris]                 | sparc64     | solaris    | ""         |
| [thumbv6m-none-eabi]                  | thumbv6     | unknown    | ""         |
| [thumbv7em-none-eabi]                 | thumbv7     | unknown    | ""         |
| [thumbv7em-none-eabihf]               | thumbv7     | unknown    | ""         |
| [thumbv7m-none-eabi]                  | thumbv7     | unknown    | ""         |
| [thumbv7neon-linux-androideabi]       | arm         | android    | ""         |
| [thumbv7neon-unknown-linux-gnueabihf] | arm         | linux      | gnu        |
| [wasm32-unknown-unknown]              | wasm32      | unknown    | ""         |
| [wasm32-unknown-emscripten]           | wasm32      | emscripten | ""         |
| [wasm32-wasi]                         | wasm32      | wasi       | ""         |
| [x86_64-apple-ios]                    | x86_64      | ios        | ""         |
| [x86_64-fortanix-unknown-sgx]         | x86_64      | unknown    | sgx        |
| [x86_64-linux-android]                | x86_64      | android    | ""         |
| [x86_64-pc-solaris]                   | x86_64      | solaris    | ""         |
| [x86_64-unknown-freebsd]              | x86_64      | freebsd    | ""         |
| [x86_64-fuchsia]                      | x86_64      | fuchsia    | ""         |
| [x86_64-unknown-illumos]              | x86_64      | illumos    | ""         |
| [x86_64-unknown-linux-gnux32]         | x86_64      | linux      | gnu        |
| [x86_64-unknown-linux-musl]           | x86_64      | linux      | musl       |
| [x86_64-unknown-netbsd]               | x86_64      | netbsd     | ""         |
| [x86_64-unknown-redox]                | x86_64      | redox      | ""         |
| [powerpc-unknown-linux-gnuspe]        | powerpc     | linux      | gnu        |
| [sparc-unknown-linux-gnu]             | sparc       | linux      | gnu        |

### Tier 3

| target triple                         | target_arch | target_os  | target_env |
|---------------------------------------|-------------|------------|------------|
| [aarch64-apple-ios-macabi]            | aarch64     | ios        | ""         |
| [aarch64-apple-ios-sim]               | aarch64     | ios        | ""         |
| [aarch64-apple-tvos]                  | aarch64     | tvos       | ""         |
| [aarch64-unknown-freebsd]             | aarch64     | freebsd    | ""         |
| [aarch64-unknown-hermit]              | aarch64     | hermit     | ""         |
| [aarch64-unknown-linux-gnu_ilp32]     | aarch64     | linux      | gnu        |
| [aarch64-unknown-netbsd]              | aarch64     | netbsd     | ""         |
| [aarch64-unknown-openbsd]             | aarch64     | openbsd    | ""         |
| [aarch64-unknown-redox]               | aarch64     | redox      | ""         |
| [aarch64-uwp-windows-msvc]            | aarch64     | windows    | msvc       |
| [aarch64-wrs-vxworks]                 | aarch64     | vxworks    | gnu        |
| [aarch64_be-unknown-linux-gnu_ilp32]  | aarch64     | linux      | gnu        |
| [aarch64_be-unknown-linux-gnu]        | aarch64     | linux      | gnu        |
| [armv4t-unknown-linux-gnueabi]        | arm         | linux      | gnu        |
| [armv5te-unknown-linux-uclibceabi]    | arm         | linux      | uclibc     |
| [armv6-unknown-freebsd]               | arm         | freebsd    | ""         |
| [armv6-unknown-netbsd-eabihf]         | arm         | netbsd     | ""         |
| [armv7-apple-ios]                     | arm         | ios        | ""         |
| [armv7-unknown-freebsd]               | arm         | freebsd    | ""         |
| [armv7-unknown-netbsd-eabihf]         | arm         | netbsd     | ""         |
| [armv7-wrs-vxworks-eabihf]            | arm         | vxworks    | gnu        |
| [armv7a-none-eabihf]                  | arm         | unknown    | ""         |
| [armv7s-apple-ios]                    | arm         | ios        | ""         |
| [i386-apple-ios]                      | x86         | ios        | ""         |
| [i686-apple-darwin]                   | x86         | macos      | ""         |
| [i686-unknown-haiku]                  | x86         | haiku      | ""         |
| [i686-unknown-netbsd]                 | x86         | netbsd     | ""         |
| [i686-unknown-openbsd]                | x86         | openbsd    | ""         |
| [mips-unknown-linux-uclibc]           | mips        | linux      | uclibc     |
| [mipsel-unknown-linux-uclibc]         | mips        | linux      | uclibc     |
| [msp430-none-elf]                     | msp430      | unknown    | ""         |
| [powerpc-unknown-linux-musl]          | powerpc     | linux      | musl       |
| [powerpc64-unknown-linux-musl]        | powerpc64   | linux      | musl       |
| [powerpc64le-unknown-linux-musl]      | powerpc64   | linux      | musl       |
| [s390x-unknown-linux-musl]            | s390x       | linux      | musl       |
| [sparc64-unknown-netbsd]              | sparc64     | netbsd     | ""         |
| [x86_64-sun-solaris]                  | x86_64      | solaris    | ""         |
| [x86_64-unknown-dragonfly]            | x86_64      | dragonfly  | ""         |
| [x86_64-unknown-haiku]                | x86_64      | haiku      | ""         |
| [x86_64-unknown-openbsd]              | x86_64      | openbsd    | ""         |

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

[crate-image]: https://img.shields.io/crates/v/platforms.svg
[crate-link]: https://crates.io/crates/platforms
[docs-image]: https://docs.rs/platforms/badge.svg
[docs-link]: https://docs.rs/platforms/
[build-image]: https://github.com/RustSec/rustsec/actions/workflows/platforms.yml/badge.svg
[build-link]: https://github.com/RustSec/rustsec/actions/workflows/platforms.yml
[license-image]: https://img.shields.io/badge/license-Apache2%2FMIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.40+-blue.svg
[zulip-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[zulip-link]: https://rust-lang.zulipchat.com/#narrow/stream/146229-wg-secure-code/

[//]: # (general links)

[RustSec Advisory Database]: https://github.com/RustSec
[wg-secure-code]: https://www.rust-lang.org/governance/wgs/wg-secure-code
[LICENSE-APACHE]: https://github.com/RustSec/platforms-crate/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/platforms-crate/blob/main/LICENSE-MIT

[//]: # (platform links)

[aarch64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_LINUX_GNU.html
[i686-pc-windows-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_PC_WINDOWS_GNU.html
[i686-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_PC_WINDOWS_MSVC.html
[i686-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_UNKNOWN_LINUX_GNU.html
[x86_64-apple-darwin]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_APPLE_DARWIN.html
[x86_64-pc-windows-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_PC_WINDOWS_GNU.html
[x86_64-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_PC_WINDOWS_MSVC.html
[x86_64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_LINUX_GNU.html
[aarch64-apple-darwin]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_APPLE_DARWIN.html
[aarch64-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_APPLE_IOS.html
[aarch64-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_PC_WINDOWS_MSVC.html
[aarch64-linux-android]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_LINUX_ANDROID.html
[aarch64-fuchsia]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_FUCHSIA.html
[aarch64-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_LINUX_MUSL.html
[aarch64-unknown-none]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_NONE.html
[aarch64-unknown-none-softfloat]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_NONE_SOFTFLOAT.html
[arm-linux-androideabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARM_LINUX_ANDROIDEABI.html
[arm-unknown-linux-gnueabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARM_UNKNOWN_LINUX_GNUEABI.html
[arm-unknown-linux-gnueabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARM_UNKNOWN_LINUX_GNUEABIHF.html
[arm-unknown-linux-musleabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARM_UNKNOWN_LINUX_MUSLEABI.html
[arm-unknown-linux-musleabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARM_UNKNOWN_LINUX_MUSLEABIHF.html
[armv5te-unknown-linux-gnueabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV5TE_UNKNOWN_LINUX_GNUEABI.html
[armv5te-unknown-linux-musleabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV5TE_UNKNOWN_LINUX_MUSLEABI.html
[armv7-linux-androideabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_LINUX_ANDROIDEABI.html
[armv7-unknown-linux-gnueabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_UNKNOWN_LINUX_GNUEABI.html
[armv7-unknown-linux-gnueabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_UNKNOWN_LINUX_GNUEABIHF.html
[armv7-unknown-linux-musleabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_UNKNOWN_LINUX_MUSLEABI.html
[armv7-unknown-linux-musleabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_UNKNOWN_LINUX_MUSLEABIHF.html
[armebv7r-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMEBV7R_NONE_EABI.html
[armebv7r-none-eabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMEBV7R_NONE_EABIHF.html
[asmjs-unknown-emscripten]: https://docs.rs/platforms/latest/platforms/platform/constant.ASMJS_UNKNOWN_EMSCRIPTEN.html
[i586-pc-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/constant.I586_PC_WINDOWS_MSVC.html
[i586-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.I586_UNKNOWN_LINUX_GNU.html
[i586-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.I586_UNKNOWN_LINUX_MUSL.html
[i686-linux-android]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_LINUX_ANDROID.html
[i686-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_UNKNOWN_FREEBSD.html
[i686-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_UNKNOWN_LINUX_MUSL.html
[mips-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS_UNKNOWN_LINUX_GNU.html
[mips-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS_UNKNOWN_LINUX_MUSL.html
[mips64-unknown-linux-gnuabi64]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS64_UNKNOWN_LINUX_GNUABI64.html
[mips64-unknown-linux-muslabi64]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS64_UNKNOWN_LINUX_MUSLABI64.html
[mips64el-unknown-linux-gnuabi64]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS64EL_UNKNOWN_LINUX_GNUABI64.html
[mips64el-unknown-linux-muslabi64]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS64EL_UNKNOWN_LINUX_MUSLABI64.html
[mipsel-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPSEL_UNKNOWN_LINUX_GNU.html
[mipsel-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPSEL_UNKNOWN_LINUX_MUSL.html
[nvptx64-nvidia-cuda]: https://docs.rs/platforms/latest/platforms/platform/constant.NVPTX64_NVIDIA_CUDA.html
[powerpc-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC_UNKNOWN_LINUX_GNU.html
[powerpc64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC64_UNKNOWN_LINUX_GNU.html
[powerpc64le-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC64LE_UNKNOWN_LINUX_GNU.html
[s390x-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.S390X_UNKNOWN_LINUX_GNU.html
[sparc64-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.SPARC64_UNKNOWN_LINUX_GNU.html
[sparcv9-sun-solaris]: https://docs.rs/platforms/latest/platforms/platform/constant.SPARCV9_SUN_SOLARIS.html
[thumbv6m-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/constant.THUMBV6M_NONE_EABI.html
[thumbv7em-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/constant.THUMBV7EM_NONE_EABI.html
[thumbv7em-none-eabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.THUMBV7EM_NONE_EABIHF.html
[thumbv7m-none-eabi]: https://docs.rs/platforms/latest/platforms/platform/constant.THUMBV7M_NONE_EABI.html
[thumbv7neon-linux-androideabi]: https://docs.rs/platforms/latest/platforms/platform/constant.THUMBV7NEON_LINUX_ANDROIDEABI.html
[thumbv7neon-unknown-linux-gnueabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.THUMBV7NEON_UNKNOWN_LINUX_GNUEABIHF.html
[wasm32-unknown-unknown]: https://docs.rs/platforms/latest/platforms/platform/constant.WASM32_UNKNOWN_UNKNOWN.html
[wasm32-unknown-emscripten]: https://docs.rs/platforms/latest/platforms/platform/constant.WASM32_UNKNOWN_EMSCRIPTEN.html
[wasm32-wasi]: https://docs.rs/platforms/latest/platforms/platform/constant.WASM32_WASI.html
[x86_64-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_APPLE_IOS.html
[x86_64-fortanix-unknown-sgx]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_FORTANIX_UNKNOWN_SGX.html
[x86_64-linux-android]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_LINUX_ANDROID.html
[x86_64-pc-solaris]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_PC_SOLARIS.html
[x86_64-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_FREEBSD.html
[x86_64-fuchsia]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_FUCHSIA.html
[x86_64-unknown-illumos]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_ILLUMOS.html
[x86_64-unknown-linux-gnux32]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_LINUX_GNUX32.html
[x86_64-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_LINUX_MUSL.html
[x86_64-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_NETBSD.html
[x86_64-unknown-redox]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_REDOX.html
[powerpc-unknown-linux-gnuspe]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC_UNKNOWN_LINUX_GNUSPE.html
[sparc-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.SPARC_UNKNOWN_LINUX_GNU.html
[aarch64-apple-ios-macabi]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_APPLE_IOS_MACABI.html
[aarch64-apple-ios-sim]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_APPLE_IOS_SIM.html
[aarch64-apple-tvos]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_APPLE_TVOS.html
[aarch64-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_FREEBSD.html
[aarch64-unknown-hermit]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_HERMIT.html
[aarch64-unknown-linux-gnu_ilp32]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_LINUX_GNU_ILP32.html
[aarch64-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_NETBSD.html
[aarch64-unknown-openbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_OPENBSD.html
[aarch64-unknown-redox]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UNKNOWN_REDOX.html
[aarch64-uwp-windows-msvc]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_UWP_WINDOWS_MSVC.html
[aarch64-wrs-vxworks]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_WRS_VXWORKS.html
[aarch64_be-unknown-linux-gnu_ilp32]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_BE_UNKNOWN_LINUX_GNU_ILP32.html
[aarch64_be-unknown-linux-gnu]: https://docs.rs/platforms/latest/platforms/platform/constant.AARCH64_BE_UNKNOWN_LINUX_GNU.html
[armv4t-unknown-linux-gnueabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV4T_UNKNOWN_LINUX_GNUEABI.html
[armv5te-unknown-linux-uclibceabi]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV5TE_UNKNOWN_LINUX_UCLIBCEABI.html
[armv6-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV6_UNKNOWN_FREEBSD.html
[armv6-unknown-netbsd-eabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV6_UNKNOWN_NETBSD_EABIHF.html
[armv7-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_APPLE_IOS.html
[armv7-unknown-freebsd]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_UNKNOWN_FREEBSD.html
[armv7-unknown-netbsd-eabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_UNKNOWN_NETBSD_EABIHF.html
[armv7-wrs-vxworks-eabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7_WRS_VXWORKS_EABIHF.html
[armv7a-none-eabihf]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7A_NONE_EABIHF.html
[armv7s-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/constant.ARMV7S_APPLE_IOS.html
[i386-apple-ios]: https://docs.rs/platforms/latest/platforms/platform/constant.I386_APPLE_IOS.html
[i686-apple-darwin]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_APPLE_DARWIN.html
[i686-unknown-haiku]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_UNKNOWN_HAIKU.html
[i686-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_UNKNOWN_NETBSD.html
[i686-unknown-openbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.I686_UNKNOWN_OPENBSD.html
[mips-unknown-linux-uclibc]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPS_UNKNOWN_LINUX_UCLIBC.html
[mipsel-unknown-linux-uclibc]: https://docs.rs/platforms/latest/platforms/platform/constant.MIPSEL_UNKNOWN_LINUX_UCLIBC.html
[msp430-none-elf]: https://docs.rs/platforms/latest/platforms/platform/constant.MSP430_NONE_ELF.html
[powerpc-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC_UNKNOWN_LINUX_MUSL.html
[powerpc64-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC64_UNKNOWN_LINUX_MUSL.html
[powerpc64le-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.POWERPC64LE_UNKNOWN_LINUX_MUSL.html
[s390x-unknown-linux-musl]: https://docs.rs/platforms/latest/platforms/platform/constant.S390X_UNKNOWN_LINUX_MUSL.html
[sparc64-unknown-netbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.SPARC64_UNKNOWN_NETBSD.html
[x86_64-sun-solaris]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_SUN_SOLARIS.html
[x86_64-unknown-dragonfly]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_DRAGONFLY.html
[x86_64-unknown-haiku]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_HAIKU.html
[x86_64-unknown-openbsd]: https://docs.rs/platforms/latest/platforms/platform/constant.X86_64_UNKNOWN_OPENBSD.html
