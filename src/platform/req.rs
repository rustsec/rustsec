use platform::{Platform, ALL_PLATFORMS};
use std::{string::String, vec::Vec};

/// Platform requirements: glob-like expressions for matching Rust platforms
/// as identified by a "target triple", e.g. `i686-apple-darwin`.
///
/// For a list of all valid platforms, "target triples", see:
///
/// <https://forge.rust-lang.org/platform-support.html>
///
/// Platforms can be grouped with simple globbing rules:
///
/// - Start with wildcard: `*-gnu`
/// - End with wildcard: `x86_64-*`
/// - Start and end with wildcard: `*windows*`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformReq(String);

/// Wildcard character used for globbing
pub const WILDCARD: char = '*';

impl PlatformReq {
    /// Create a new platform requirement. Platforms support glob-like
    /// wildcards on the beginning and end, e.g. `*windows*`.
    ///
    /// Must match at least one known Rust platform "target triple"
    /// (e.g. `x86_64-unknown-linux-gnu`) to be considered valid.
    pub fn new<S: Into<String>>(into_string: S) -> Result<Self, ()> {
        let platform_req = PlatformReq(into_string.into());

        if platform_req.0.is_empty() || platform_req.matching_platforms().is_empty() {
            Err(())
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
    pub fn matches(&self, platform: &Platform) -> bool {
        let self_len = self.as_str().len();

        // Universal matcher
        if self.0.len() == 1 && self.0.chars().next().unwrap() == WILDCARD {
            return true;
        }

        let mut chars = self.as_str().chars();
        let starts_with_wildcard = chars.next().unwrap() == WILDCARD;
        let ends_with_wildcard = chars.last() == Some(WILDCARD);

        if starts_with_wildcard {
            if ends_with_wildcard {
                // Contains expression: `*windows*`
                platform
                    .target_triple
                    .contains(&self.0[1..self_len.checked_sub(1).unwrap()])
            } else {
                // Suffix expression: `*-gnu`
                platform.target_triple.ends_with(&self.0[1..])
            }
        } else if ends_with_wildcard {
            // Prefix expression: `x86_64-*`
            platform
                .target_triple
                .starts_with(&self.0[..self_len.checked_sub(1).unwrap()])
        } else {
            // No wildcards: direct comparison
            self.as_str() == platform.target_triple
        }
    }

    /// Expand glob expressions into a list of all known matching platforms
    pub fn matching_platforms(&self) -> Vec<Platform> {
        ALL_PLATFORMS
            .iter()
            .filter(|platform| self.matches(*platform))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{PlatformReq, ALL_PLATFORMS};
    use std::vec::Vec;

    #[test]
    fn prefix_glob_test() {
        let req = PlatformReq::new("sparc*").unwrap();

        assert_eq!(
            req.matching_platforms()
                .iter()
                .map(|p| p.target_triple)
                .collect::<Vec<_>>(),
            [
                "sparc64-unknown-linux-gnu",
                "sparcv9-sun-solaris",
                "sparc64-unknown-netbsd"
            ]
        );
    }

    #[test]
    fn suffix_glob_test() {
        let req = PlatformReq::new("*-musl").unwrap();

        assert_eq!(
            req.matching_platforms()
                .iter()
                .map(|p| p.target_triple)
                .collect::<Vec<_>>(),
            [
                "aarch64-unknown-linux-musl",
                "i586-unknown-linux-musl",
                "i686-unknown-linux-musl",
                "mips-unknown-linux-musl",
                "mipsel-unknown-linux-musl",
                "x86_64-unknown-linux-musl"
            ]
        );
    }

    #[test]
    fn contains_glob_test() {
        let req = PlatformReq::new("*windows*").unwrap();

        assert_eq!(
            req.matching_platforms()
                .iter()
                .map(|p| p.target_triple)
                .collect::<Vec<_>>(),
            [
                "i686-pc-windows-gnu",
                "i686-pc-windows-msvc",
                "x86_64-pc-windows-gnu",
                "x86_64-pc-windows-msvc",
                "i586-pc-windows-msvc"
            ]
        );
    }

    #[test]
    fn direct_match_test() {
        let req = PlatformReq::new("x86_64-unknown-dragonfly").unwrap();

        assert_eq!(
            req.matching_platforms()
                .iter()
                .map(|p| p.target_triple)
                .collect::<Vec<_>>(),
            ["x86_64-unknown-dragonfly"]
        );
    }

    #[test]
    fn wildcard_test() {
        let req = PlatformReq::new("*").unwrap();
        assert_eq!(req.matching_platforms().len(), ALL_PLATFORMS.len())
    }

    // How to handle this is debatable...
    #[test]
    fn double_wildcard_test() {
        let req = PlatformReq::new("**").unwrap();
        assert_eq!(req.matching_platforms().len(), ALL_PLATFORMS.len())
    }

    #[test]
    fn invalid_req_tests() {
        assert!(PlatformReq::new("").is_err());
        assert!(PlatformReq::new(" ").is_err());
        assert!(PlatformReq::new("derp").is_err());
        assert!(PlatformReq::new("***").is_err());
    }
}
