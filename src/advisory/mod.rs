//! Security advisories in the RustSec database

mod date;
mod function_path;
mod id;
mod iter;
mod keyword;

pub use self::{date::*, function_path::FunctionPath, id::*, iter::Iter, keyword::Keyword};
use crate::package::PackageName;
use platforms::target::{Arch, OS};
use semver::VersionReq;

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

    /// CPU architectures that this vulnerability is specific to
    pub affected_arch: Option<Vec<Arch>>,

    /// Operating systems that this vulnerability is specific to
    pub affected_os: Option<Vec<OS>>,

    /// Functions containing vulnerable code, enumerated as canonical Rust
    /// paths (i.e. starting with the crate name), sans any path parameters.
    /// (e.g. `mycrate::path::to::VulnerableStruct::vulnerable_func`)
    pub affected_functions: Option<Vec<FunctionPath>>,

    /// Advisory IDs in other databases which point to the same advisory
    #[serde(default)]
    pub aliases: Vec<AdvisoryId>,

    /// Advisory IDs which are related to this advisory
    #[serde(default)]
    pub references: Vec<AdvisoryId>,

    /// Freeform keywords which succinctly describe this vulnerability (e.g. "ssl", "rce", "xss")
    #[serde(default)]
    pub keywords: Vec<Keyword>,

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
