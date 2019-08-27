//! Version types used by RustSec.
//!
//! These are newtypes of the `semver` crate with slightly different behavior
//! around the handling of prerelease versions.
//!
//! See: <https://github.com/RustSec/cargo-audit/issues/30>

mod predicate;
mod req;

pub use self::req::VersionReq;

use crate::Error;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Version type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Version(semver::Version);

impl Version {
    /// Parse a version from a string
    pub fn parse(input: &str) -> Result<Self, Error> {
        Ok(semver::Version::parse(input)?.into())
    }

    /// Get the major part of the version
    pub fn major(&self) -> u64 {
        self.0.major
    }

    /// Get the minor part of the version
    pub fn minor(&self) -> u64 {
        self.0.minor
    }

    /// Get the patch part of the version
    pub fn patch(&self) -> u64 {
        self.0.patch
    }

    /// Get the prerelease portion of the version as a `String` if it exists
    pub fn pre(&self) -> Option<String> {
        if self.is_prerelease() {
            let pre_identifiers: Vec<_> = self.0.pre.iter().map(|id| id.to_string()).collect();
            Some(pre_identifiers.join("."))
        } else {
            None
        }
    }

    /// Is this version a prerelease?
    pub fn is_prerelease(&self) -> bool {
        self.0.is_prerelease()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<semver::Version> for Version {
    fn from(version: semver::Version) -> Version {
        Version(version)
    }
}

impl From<Version> for semver::Version {
    fn from(version: Version) -> semver::Version {
        version.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Version, VersionReq};

    fn version(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    fn version_req(s: &str) -> VersionReq {
        VersionReq::parse(s).unwrap()
    }

    #[test]
    fn version_parsing() {
        let v = version("1.2.3");
        assert_eq!(v.major(), 1);
        assert_eq!(v.minor(), 2);
        assert_eq!(v.patch(), 3);
        assert!(v.pre().is_none());
    }

    #[test]
    fn prerelease_version_parsing() {
        let pre_v = version("1.2.3-pre.4");
        assert_eq!(pre_v.major(), 1);
        assert_eq!(pre_v.minor(), 2);
        assert_eq!(pre_v.patch(), 3);
        assert_eq!(pre_v.pre().unwrap(), "pre.4");
    }

    #[test]
    fn basic_version_comparisons() {
        assert_eq!(version("0.0.0"), version("0.0.0"));
        assert!(version("0.0.0") < version("0.0.1"));
        assert!(version("1.0.0") > version("0.0.1"));
    }

    #[test]
    fn basic_version_req_matches() {
        assert!(version_req("= 0.0.0").matches(&version("0.0.0")));
        assert!(version_req("> 0.0.0").matches(&version("0.0.1")));
        assert!(version_req("< 1.0.0").matches(&version("0.0.1")));
    }

    // Test case from: <https://github.com/RustSec/cargo-audit/issues/30>
    #[test]
    fn prerelease_version_req_matches() {
        assert!(version_req(">= 0.7.6").matches(&version("0.11.0-dev")));
        assert!(!version_req("< 0.7.6").matches(&version("0.11.0-dev")));
    }

    #[test]
    fn release_is_greater_than_prerelease() {
        assert!(version_req("> 1.0.0-pre").matches(&version("1.0.0")));
        assert!(version_req("< 1.0.0").matches(&version("1.0.0-pre")))
    }
}
