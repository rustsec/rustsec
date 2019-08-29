//! Advisory information (i.e. the `[advisory]` section)

use super::{category::Category, date::Date, id::Id, keyword::Keyword};
use crate::{package, version::VersionReq};
use serde::{Deserialize, Serialize};

/// The `[advisory]` section of a RustSec security advisory
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Info {
    /// Security advisory ID (e.g. RUSTSEC-YYYY-NNNN)
    pub id: Id,

    /// Name of affected crate
    pub package: package::Name,

    /// Date this advisory was officially issued
    pub date: Date,

    /// Advisory IDs in other databases which point to the same advisory
    #[serde(default)]
    pub aliases: Vec<Id>,

    /// CVSS v3.1 Base Metrics vector string containing severity information.
    ///
    /// Example:
    ///
    /// ```text
    /// CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:L/I:L/A:N
    /// ```
    pub cvss: Option<cvss::v3::Base>,

    /// Advisory IDs which are related to this advisory
    #[serde(default)]
    pub references: Vec<Id>,

    /// RustSec vulnerability categories: one of a fixed list of vulnerability
    /// categorizations accepted by the project.
    #[serde(default)]
    pub categories: Vec<Category>,

    /// Freeform keywords which succinctly describe this vulnerability (e.g. "ssl", "rce", "xss")
    #[serde(default)]
    pub keywords: Vec<Keyword>,

    /// URL with an announcement (e.g. blog post, PR, disclosure issue, CVE)
    pub url: Option<String>,

    /// One-liner description of a vulnerability
    pub title: String,

    /// Extended description of a vulnerability
    pub description: String,

    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    // TODO(tarcieri): phase this out
    #[serde(default)]
    pub(super) patched_versions: Vec<VersionReq>,

    /// Versions which were never affected in the first place
    #[serde(default)]
    pub(super) unaffected_versions: Vec<VersionReq>,
}
