//! The `[versions]` subsection of an advisory.
//!
//! This is meant to eventually take the place of the `patched_versions`
//! and `unaffected_versions` sections of the `[advisory]`, but can't be
//! used

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use super::version_ranges;

/// The `[versions]` subsection of an advisory: future home to information
/// about which versions are patched and/or unaffected.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Versions {
    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    pub patched: Vec<VersionReq>,

    /// Versions which were never affected in the first place
    #[serde(default)]
    pub unaffected: Vec<VersionReq>,
}

impl Versions {
    /// Is the given version of a package vulnerable?
    pub fn is_vulnerable(&self, version: &Version) -> bool {
        for range in version_ranges::ranges_for_advisory(self).iter() {
            if range.contains(version) {
                return true;
            }
        }
        false
    }
}
