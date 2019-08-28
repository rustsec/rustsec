//! Security advisories in the RustSec database

mod date;
mod id;
mod iter;
mod keyword;
mod paths;

pub use self::{date::*, id::*, iter::Iter, keyword::Keyword, paths::*};
pub use cvss::Severity;

use crate::{
    error::{Error, ErrorKind},
    package::PackageName,
    version::VersionReq,
};
use platforms::target::{Arch, OS};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr};

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

    /// Paths to types and/or functions containing vulnerable code, enumerated
    /// as canonical Rust paths (i.e. starting with the crate name), sans any
    /// path parameters.
    ///
    /// (e.g. `mycrate::path::to::VulnerableStruct::vulnerable_func`)
    pub affected_paths: Option<AffectedPaths>,

    /// Advisory IDs in other databases which point to the same advisory
    #[serde(default)]
    pub aliases: Vec<AdvisoryId>,

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

impl Advisory {
    /// Load an advisory from a `RUSTSEC-20XX-NNNN.toml` file
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?
            .parse()
    }

    /// Get the severity of this advisory if it has a CVSS v3 associated
    pub fn severity(&self) -> Option<Severity> {
        self.cvss.as_ref().map(|cvss| cvss.severity())
    }
}

impl FromStr for Advisory {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        let wrapper: AdvisoryWrapper = toml::from_str(toml_string)?;
        Ok(wrapper.advisory)
    }
}

/// Wrapper struct around advisories since they're each in a table
#[derive(Serialize, Deserialize)]
pub(crate) struct AdvisoryWrapper {
    pub(crate) advisory: Advisory,
}
