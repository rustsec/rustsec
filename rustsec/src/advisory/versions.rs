//! The `[versions]` subsection of an advisory.
//!
//! This is meant to eventually take the place of the `patched_versions`
//! and `unaffected_versions` sections of the `[advisory]`, but can't be
//! used

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::osv;

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

// TODO: deserialization with validation of range sanity.
// Ideally this needs an immutable type (i.e. with private fields)
// so that it would be impossible to construct invalid range requirements at any point,
// but that would require an API break.

impl Versions {
    /// Is the given version of a package vulnerable?
    pub fn is_vulnerable(&self, version: &Version) -> bool {
        // we .unwrap() here because the version specification has been validated on deserialization
        for range in osv::ranges_for_advisory(self).unwrap().iter() {
            if range.affects(version) {
                return true;
            }
        }
        false
    }
}
