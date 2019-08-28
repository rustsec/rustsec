//! The `[versions]` subsection of an advisory.
//!
//! This is meant to eventually take the place of the `patched_versions`
//! and `unaffected_versions` sections of the `[advisory]`, but can't be
//! used

use crate::version::VersionReq;
use serde::{Deserialize, Serialize};

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
