//! The `[versions]` subsection of an advisory.
//!
//! This is meant to eventually take the place of the `patched_versions`
//! and `unaffected_versions` sections of the `[advisory]`, but can't be
//! used

use std::convert::TryFrom;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::osv;

// Right now ranges are only validated during deserialization;
// since the fields are public, it's possible to mutate them and get them to
// TODO: Ideally this needs an immutable type (i.e. with private fields)
// so that it would be impossible to construct invalid range requirements at any point,
// but that would require an API break.

/// The `[versions]` subsection of an advisory: future home to information
/// about which versions are patched and/or unaffected.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "RawVersions")]
pub struct Versions {
    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    pub patched: Vec<VersionReq>,

    /// Versions which were never affected in the first place
    #[serde(default)]
    pub unaffected: Vec<VersionReq>,
}

impl TryFrom<RawVersions> for Versions {
    type Error = crate::Error;

    fn try_from(raw: RawVersions) -> Result<Self, Self::Error> {
        validate_ranges(&raw)?;
        Ok(Versions {
            patched: raw.patched,
            unaffected: raw.unaffected,
        })
    }
}

impl Versions {
    /// Is the given version of a package vulnerable?
    pub fn is_vulnerable(&self, version: &Version) -> bool {
        // we .unwrap() here because the version specification has been validated on deserialization
        for range in osv::ranges_for_advisory(self).iter() {
            if range.affects(version) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
/// Raw deserialized data that didn't pass validation yet
pub(crate) struct RawVersions {
    pub patched: Vec<VersionReq>,

    #[serde(default)]
    pub unaffected: Vec<VersionReq>,
}

fn validate_ranges(versions: &RawVersions) -> Result<(), crate::Error> {
    let _ = osv::ranges_for_unvalidated_advisory(versions)?;
    Ok(())
}