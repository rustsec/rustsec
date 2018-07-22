//! Security advisories in the RustSec database

#[cfg(feature = "chrono")]
use chrono::{Date as ChronoDate, DateTime, Utc};
use semver::VersionReq;
use std::fmt;

use error::Error;
use package::PackageName;

/// An individual security advisory pertaining to a single vulnerability
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Advisory {
    /// Security advisory ID (e.g. RUSTSEC-YYYY-NNNN)
    pub id: AdvisoryId,

    /// Name of affected crate
    pub package: PackageName,

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

    /// Date this advisory was officially issued
    pub date: Date,

    /// URL with an announcement (e.g. blog post, PR, disclosure issue, CVE)
    pub url: Option<String>,

    /// One-liner description of a vulnerability
    pub title: String,

    /// Extended description of a vulnerability
    pub description: String,
}

/// An identifier for an individual advisory
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct AdvisoryId(pub String);

impl AsRef<str> for AdvisoryId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AdvisoryId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<&'a str> for AdvisoryId {
    fn from(string: &'a str) -> AdvisoryId {
        AdvisoryId(string.into())
    }
}

impl Into<String> for AdvisoryId {
    fn into(self) -> String {
        self.0
    }
}

/// Wrapper struct around advisories since they're each in a table
#[derive(Serialize, Deserialize)]
pub(crate) struct AdvisoryWrapper {
    pub(crate) advisory: Advisory,
}

/// Dates on advisories
// TODO: better validate how these are formed
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Date(pub String);

impl Date {
    /// Convert an advisory RFC 3339 date into a `chrono::Date`
    #[cfg(feature = "chrono")]
    pub fn into_chrono_date(&self) -> Result<ChronoDate<Utc>, Error> {
        let datetime = DateTime::parse_from_rfc3339(self.0.as_ref())?;
        Ok(ChronoDate::from_utc(datetime.naive_utc().date(), Utc))
    }
}

impl Into<String> for Date {
    fn into(self) -> String {
        self.0
    }
}

impl<'a> From<&'a str> for Date {
    // TODO: validate inputs
    fn from(string: &'a str) -> Date {
        Date(string.into())
    }
}
