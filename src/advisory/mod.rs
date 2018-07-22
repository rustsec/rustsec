//! Security advisories in the RustSec database

use semver::VersionReq;

use package::PackageName;

mod date;
mod id;
mod iter;

pub use self::date::*;
pub use self::id::*;
pub use self::iter::Iter;

/// An individual security advisory pertaining to a single vulnerability
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Advisory {
    /// Security advisory ID (e.g. RUSTSEC-YYYY-NNNN)
    pub id: AdvisoryId,

    /// Name of affected crate
    pub package: PackageName,

    /// Date this advisory was officially issued
    pub date: Date,

    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    pub patched_versions: Vec<VersionReq>,

    /// Versions which were never affected in the first place
    #[serde(default)]
    pub unaffected_versions: Vec<VersionReq>,

    /// Advisory IDs in other databases which point to the same advisory
    #[serde(default)]
    pub aliases: Vec<AdvisoryId>,

    /// Advisory IDs which are related to this advisory
    #[serde(default)]
    pub references: Vec<AdvisoryId>,

    /// URL with an announcement (e.g. blog post, PR, disclosure issue, CVE)
    pub url: Option<String>,

    /// One-liner description of a vulnerability
    pub title: String,

    /// Extended description of a vulnerability
    pub description: String,
}

/// Wrapper struct around advisories since they're each in a table
#[derive(Serialize, Deserialize)]
pub(crate) struct AdvisoryWrapper {
    pub(crate) advisory: Advisory,
}
