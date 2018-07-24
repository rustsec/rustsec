use serde::{de::Error as DeError, Deserialize, Deserializer};

use error::{Error, ErrorKind};

/// Platform requirements: matchers on Rust platform "target triple"s.
///
/// Supports a simple wildcard syntax:
///
/// - Prefix: e.g. `x86_64-*`
/// - Suffix: e.g. `*-gnu`
/// - Contains: e.g. `*windows*`
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PlatformReq(String);

/// Wildcard character
pub const WILDCARD: char = '*';

impl PlatformReq {
    /// Create a new platform requirement. Platforms support glob-like
    /// wildcards on the beginning and end, e.g. `*windows*`.
    ///
    /// Must match at least one known Rust platform "target triple"
    /// (e.g. `x86_64-unknown-linux-gnu`) to be considered valid.
    pub fn new<S: Into<String>>(into_string: S) -> Result<Self, Error> {
        let platform_req = PlatformReq(into_string.into());

        if platform_req.0.is_empty() {
            fail!(
                ErrorKind::Parse,
                "platform requirements cannot be empty strings"
            );
        }

        if platform_req.matching_platforms().is_empty() {
            fail!(
                ErrorKind::Parse,
                "platform requirement '{}' does not match any known platforms",
                platform_req.as_str()
            )
        } else {
            Ok(platform_req)
        }
    }

    /// Borrow this platform requirement as a string slice
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Does this platform requirement match the given platform string?
    ///
    /// This matcher accepts a platform "target triple" string ala
    /// `x86_64-unknown-linux-gnu` and matches it against this
    /// `Platform`, using simple glob like rules.
    pub fn matches(&self, platform: Platform) -> bool {
        let self_len = self.as_str().len();

        // Universal matcher
        if self.0 == "*" {
            return true;
        }

        let mut chars = self.as_str().chars();
        let starts_with_wildcard = chars.next().unwrap() == WILDCARD;
        let ends_with_wildcard = chars.last() == Some(WILDCARD);

        if starts_with_wildcard {
            if ends_with_wildcard {
                // Contains expression: `*windows*`
                platform
                    .as_str()
                    .contains(&self.0[1..self_len.checked_sub(1).unwrap()])
            } else {
                // Suffix expression: `*-gnu`
                platform.as_str().ends_with(&self.0[1..])
            }
        } else if ends_with_wildcard {
            // Prefix expression: `x86_64-*`
            platform
                .as_str()
                .starts_with(&self.0[..self_len.checked_sub(1).unwrap()])
        } else {
            // No wildcards: direct comparison
            self.as_str() == platform.as_str()
        }
    }

    /// Does this platform requirement match the current platform?
    ///
    /// If we can't detect the current platform, this defaults to `true`
    pub fn matches_current(&self) -> bool {
        match Platform::current() {
            Some(platform) => self.matches(platform),
            None => true,
        }
    }

    /// Expand glob expressions into a list of all known matching platforms
    pub fn matching_platforms(&self) -> Vec<Platform> {
        Platform::all()
            .iter()
            .filter(|platform| self.matches(**platform))
            .cloned()
            .collect()
    }
}

impl<'de> Deserialize<'de> for PlatformReq {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{}", e)))
    }
}

/// Valid Rust platforms
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Platform(&'static str);

impl Platform {
    /// All valid Rust platforms
    pub fn all() -> &'static [Platform] {
        ALL_PLATFORMS
    }

    /// Detect the current platform
    pub fn current() -> Option<Platform> {
        current_platform().map(|platform| Platform(platform))
    }

    /// Borrow this platform requirement as a string slice
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

/// Valid Rust platforms
///
/// Sourced from https://forge.rust-lang.org/platform-support.html
// TODO: find a better source of these than a web site
const ALL_PLATFORMS: &[Platform] = &[
    // Tier 1
    Platform("i686-apple-darwin"),
    Platform("i686-pc-windows-gnu"),
    Platform("i686-pc-windows-msvc"),
    Platform("i686-unknown-linux-gnu"),
    Platform("x86_64-apple-darwin"),
    Platform("x86_64-pc-windows-gnu"),
    Platform("x86_64-pc-windows-msvc"),
    Platform("x86_64-unknown-linux-gnu"),
    // Tier 2
    Platform("aarch64-apple-ios"),
    Platform("aarch64-unknown-cloudabi"),
    Platform("aarch64-linux-android"),
    Platform("aarch64-unknown-fuchsia"),
    Platform("aarch64-unknown-linux-gnu"),
    Platform("aarch64-unknown-linux-musl"),
    Platform("arm-linux-androideabi"),
    Platform("arm-unknown-linux-gnueabi"),
    Platform("arm-unknown-linux-gnueabihf"),
    Platform("arm-unknown-linux-musleabi"),
    Platform("arm-unknown-linux-musleabihf"),
    Platform("armv5te-unknown-linux-gnueabi"),
    Platform("armv7-apple-ios"),
    Platform("armv7-linux-androideabi"),
    Platform("armv7-unknown-cloudabi-eabihf"),
    Platform("armv7-unknown-linux-gnueabihf"),
    Platform("armv7-unknown-linux-musleabihf"),
    Platform("armv7s-apple-ios"),
    Platform("asmjs-unknown-emscripten"),
    Platform("i386-apple-ios"),
    Platform("i586-pc-windows-msvc"),
    Platform("i586-unknown-linux-gnu"),
    Platform("i586-unknown-linux-musl"),
    Platform("i686-linux-android"),
    Platform("i686-unknown-cloudabi"),
    Platform("i686-unknown-freebsd"),
    Platform("i686-unknown-linux-musl"),
    Platform("mips-unknown-linux-gnu"),
    Platform("mips-unknown-linux-musl"),
    Platform("mips64-unknown-linux-gnuabi64"),
    Platform("mips64el-unknown-linux-gnuabi64"),
    Platform("mipsel-unknown-linux-gnu"),
    Platform("mipsel-unknown-linux-musl"),
    Platform("powerpc-unknown-linux-gnu"),
    Platform("powerpc64-unknown-linux-gnu"),
    Platform("powerpc64le-unknown-linux-gnu"),
    Platform("s390x-unknown-linux-gnu"),
    Platform("sparc64-unknown-linux-gnu"),
    Platform("sparcv9-sun-solaris"),
    Platform("wasm32-unknown-unknown"),
    Platform("wasm32-unknown-emscripten"),
    Platform("x86_64-apple-ios"),
    Platform("x86_64-linux-android"),
    Platform("x86_64-rumprun-netbsd"),
    Platform("x86_64-sun-solaris"),
    Platform("x86_64-unknown-cloudabi"),
    Platform("x86_64-unknown-freebsd"),
    Platform("x86_64-unknown-fuchsia"),
    Platform("x86_64-unknown-linux-gnux32"),
    Platform("x86_64-unknown-linux-musl"),
    Platform("x86_64-unknown-netbsd"),
    Platform("x86_64-unknown-redox"),
    // Tier 3
    Platform("i686-pc-windows-msvc"),
    Platform("i686-unknown-haiku"),
    Platform("i686-unknown-netbsd"),
    Platform("le32-unknown-nacl"),
    Platform("mips-unknown-linux-uclibc"),
    Platform("mipsel-unknown-linux-uclibc"),
    Platform("msp430-none-elf"),
    Platform("sparc64-unknown-netbsd"),
    Platform("thumbv6m-none-eabi"),
    Platform("thumbv7em-none-eabi"),
    Platform("thumbv7em-none-eabihf"),
    Platform("thumbv7m-none-eabi"),
    Platform("x86_64-pc-windows-msvc"),
    Platform("x86_64-unknown-bitrig"),
    Platform("x86_64-unknown-dragonfly"),
    Platform("x86_64-unknown-haiku"),
    Platform("x86_64-unknown-openbsd"),
];

/// Attempt to detect the current platform based on attributes
#[allow(unreachable_code)]
fn current_platform() -> Option<&'static str> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return Some("x86_64-unknown-linux-gnu");

    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    return Some("i686-unknown-linux-gnu");

    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    return Some("arm-unknown-linux-gnueabihf");

    #[cfg(all(target_os = "linux", target_arch = "mips"))]
    return Some("mips-unknown-linux-gnu");

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return Some("aarch64-unknown-linux-gnu");

    #[cfg(all(target_os = "linux", target_env = "musl"))]
    return Some("x86_64-unknown-linux-musl");

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return Some("x86_64-apple-darwin");

    #[cfg(all(target_os = "macos", target_arch = "x86"))]
    return Some("i686-apple-darwin");

    #[cfg(all(windows, target_arch = "x86_64", target_env = "gnu"))]
    return Some("x86_64-pc-windows-gnu");

    #[cfg(all(windows, target_arch = "x86", target_env = "gnu"))]
    return Some("i686-pc-windows-gnu");

    #[cfg(all(windows, target_arch = "x86_64", target_env = "msvc"))]
    return Some("x86_64-pc-windows-msvc");

    #[cfg(all(windows, target_arch = "x86", target_env = "msvc"))]
    return Some("i686-pc-windows-msvc");

    #[cfg(target_os = "android")]
    return Some("arm-linux-androideabi");

    #[cfg(target_os = "freebsd")]
    return Some("x86_64-unknown-freebsd");

    #[cfg(target_os = "openbsd")]
    return Some("x86_64-unknown-openbsd");

    #[cfg(target_os = "bitrig")]
    return Some("x86_64-unknown-bitrig");

    #[cfg(target_os = "netbsd")]
    return Some("x86_64-unknown-netbsd");

    #[cfg(target_os = "dragonfly")]
    return Some("x86_64-unknown-dragonfly");

    #[cfg(target_os = "solaris")]
    return Some("x86_64-sun-solaris");

    #[cfg(all(target_os = "emscripten", target_arch = "asmjs"))]
    return Some("asmjs-unknown-emscripten");

    #[cfg(all(target_os = "emscripten", target_arch = "wasm32"))]
    return Some("wasm32-unknown-emscripten");

    #[cfg(all(target_os = "linux", target_arch = "sparc64"))]
    return Some("sparc64-unknown-linux-gnu");

    // Couldn't detect platform
    None
}

#[cfg(test)]
mod tests {
    use super::{Platform, PlatformReq};

    #[test]
    fn prefix_glob_test() {
        let req = PlatformReq::new("sparc*").unwrap();

        assert_eq!(
            req.matching_platforms(),
            [
                Platform("sparc64-unknown-linux-gnu"),
                Platform("sparcv9-sun-solaris"),
                Platform("sparc64-unknown-netbsd")
            ]
        );
    }

    #[test]
    fn suffix_glob_test() {
        let req = PlatformReq::new("*-musl").unwrap();

        assert_eq!(
            req.matching_platforms(),
            [
                Platform("aarch64-unknown-linux-musl"),
                Platform("i586-unknown-linux-musl"),
                Platform("i686-unknown-linux-musl"),
                Platform("mips-unknown-linux-musl"),
                Platform("mipsel-unknown-linux-musl"),
                Platform("x86_64-unknown-linux-musl")
            ]
        );
    }

    #[test]
    fn contains_glob_test() {
        let req = PlatformReq::new("*windows*").unwrap();

        assert_eq!(
            req.matching_platforms(),
            [
                Platform("i686-pc-windows-gnu"),
                Platform("i686-pc-windows-msvc"),
                Platform("x86_64-pc-windows-gnu"),
                Platform("x86_64-pc-windows-msvc"),
                Platform("i586-pc-windows-msvc"),
                Platform("i686-pc-windows-msvc"),
                Platform("x86_64-pc-windows-msvc")
            ]
        );
    }

    #[test]
    fn direct_match_test() {
        let req = PlatformReq::new("x86_64-unknown-dragonfly").unwrap();

        assert_eq!(
            req.matching_platforms(),
            [Platform("x86_64-unknown-dragonfly")]
        );
    }

    #[test]
    fn wildcard_test() {
        let req = PlatformReq::new("*").unwrap();
        assert_eq!(req.matching_platforms().len(), Platform::all().len())
    }

    // How to handle this is debatable...
    #[test]
    fn double_wildcard_test() {
        let req = PlatformReq::new("**").unwrap();
        assert_eq!(req.matching_platforms().len(), Platform::all().len())
    }

    #[test]
    fn invalid_req_tests() {
        assert!(PlatformReq::new("").is_err());
        assert!(PlatformReq::new(" ").is_err());
        assert!(PlatformReq::new("derp").is_err());
        assert!(PlatformReq::new("***").is_err());
    }
}
