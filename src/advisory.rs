//! Advisory type and related parsing code

use error::Result;
use semver::VersionReq;
use toml;
use util;

/// An individual security advisory pertaining to a single vulnerability
#[derive(Debug, PartialEq)]
pub struct Advisory {
    /// Security advisory ID (e.g. RUSTSEC-YYYY-NNNN)
    pub id: String,

    /// Name of affected crate
    pub package: String,

    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    pub patched_versions: Vec<VersionReq>,

    /// Date vulnerability was originally disclosed (optional)
    pub date: Option<String>,

    /// URL with an announcement (e.g. blog post, PR, disclosure issue, CVE)
    pub url: Option<String>,

    /// One-liner description of a vulnerability
    pub title: String,

    /// Extended description of a vulnerability
    pub description: String,
}

impl Advisory {
    /// Parse an Advisory from a TOML table object
    pub fn from_toml_table(value: &toml::value::Table) -> Result<Advisory> {
        Ok(Advisory {
            id: util::parse_mandatory_string(value, "id")?,
            package: util::parse_mandatory_string(value, "package")?,
            patched_versions: util::parse_versions(value, "patched_versions")?,
            date: util::parse_optional_string(value, "date")?,
            url: util::parse_optional_string(value, "url")?,
            title: util::parse_mandatory_string(value, "title")?,
            description: util::parse_mandatory_string(value, "description")?,
        })
    }
}
