//! Tracking file for supported versions

use crate::{advisory::date::Date, version::VersionReq};
use serde::{Deserialize, Serialize};

/// Tracking file for supported versions
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Support {
    /// Versions of the RustSec crate
    pub rustsec: RustSec,
}

/// Supported `rustsec` crate version metadata
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RustSec {
    /// Supported versions of the RustSec crate
    pub version: VersionReq,

    /// Information about the next (breaking) update
    pub next_update: Option<NextUpdate>,
}

impl RustSec {
    /// Is the current version of this crate supported?
    pub fn is_supported(&self) -> bool {
        self.version.matches(&crate::VERSION.parse().unwrap())
    }
}

/// Information about the next breaking change to the advisory DB.
/// This allows us to both warn in advance when the file format used in
/// the `RustSec/advisory-db` repo will have breaking changes, and also
/// notify users with out-of-date copies of e.g. `cargo-audit` that they
/// need to upgrade.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NextUpdate {
    /// New minimum supported versions
    pub version: VersionReq,

    /// Date when the breaking changes are planned to be made
    pub date: Date,
}
