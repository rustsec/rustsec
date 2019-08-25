//! Version types used by RustSec.
//!
//! These are newtypes of the `semver` crate with slightly different behavior
//! around the handling of prerelease versions.
//!
//! See: <https://github.com/RustSec/cargo-audit/issues/30>

use crate::Error;
use serde::{Deserialize, Serialize};

/// Version type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Version(semver::Version);

impl Version {
    /// Parse a version from a string
    pub fn parse(input: &str) -> Result<Self, Error> {
        Ok(semver::Version::parse(input)?.into())
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

/// Version requirements type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct VersionReq(semver::VersionReq);

impl VersionReq {
    /// Parse a version requirement from a string
    pub fn parse(input: &str) -> Result<Self, Error> {
        Ok(semver::VersionReq::parse(input)?.into())
    }

    /// Match the given `Version` against this `VersionReq`
    pub fn matches(&self, version: &Version) -> bool {
        self.0.matches(&version.0)
    }
}

impl From<semver::VersionReq> for VersionReq {
    fn from(version_req: semver::VersionReq) -> VersionReq {
        VersionReq(version_req)
    }
}

impl From<VersionReq> for semver::VersionReq {
    fn from(version_req: VersionReq) -> semver::VersionReq {
        version_req.0
    }
}
