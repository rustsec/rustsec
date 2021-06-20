use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::Serialize;
use url::Url;

use super::{ranges_for_advisory, OsvRange};

use crate::{
    advisory::{affected::FunctionPath, Affected, Category, Id, Informational},
    repository::git::{GitModificationTimes, GitPath},
    Advisory,
};

const ECOSYSTEM: &'static str = "crates.io";

/// Security advisory in the format defined by https://github.com/google/osv
#[derive(Debug, Clone, Serialize)]
pub struct OsvAdvisory {
    id: Id,
    modified: String,  // maybe add an rfc3339 newtype?
    published: String, // maybe add an rfc3339 newtype?
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn: Option<String>, // maybe add an rfc3339 newtype?
    aliases: Vec<Id>,
    related: Vec<Id>,
    package: OsvPackage,
    summary: String,
    details: String,
    affects: OsvAffected,
    references: Vec<OsvReference>,
    ecosystem_specific: OsvEcosystemSpecific,
    database_specific: OsvDatabaseSpecific,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvPackage {
    /// Set to a constant identifying crates.io
    ecosystem: &'static str,
    /// Crate name
    name: String,
    /// https://github.com/package-url/purl-spec derived from the other two
    purl: String,
}

impl From<&cargo_lock::Name> for OsvPackage {
    fn from(package: &cargo_lock::Name) -> Self {
        OsvPackage {
            ecosystem: ECOSYSTEM,
            name: package.to_string(),
            purl: "pkg:cargo/".to_string() + package.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvAffected {
    // Other fields are specified, but we never use them.
    // Ranges alone are sufficient.
    ranges: Vec<OsvJsonRange>,
}

/// Same as `OsvRange`, but also has `type` field specified
/// which is required in the OSV JSON representation.
#[derive(Debug, Clone, Serialize)]
pub struct OsvJsonRange {
    // 'type' is a reserved keyword in Rust
    #[serde(rename = "type")]
    kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    introduced: Option<semver::Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed: Option<semver::Version>,
}

impl From<OsvRange> for OsvJsonRange {
    fn from(range: OsvRange) -> Self {
        OsvJsonRange {
            kind: "SEMVER",
            introduced: range.introduced,
            fixed: range.fixed,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvReference {
    // 'type' is a reserved keyword in Rust
    #[serde(rename = "type")]
    kind: OsvReferenceKind,
    url: Url,
}

impl From<Url> for OsvReference {
    fn from(url: Url) -> Self {
        OsvReference {
            kind: guess_url_kind(&url),
            url: url,
        }
    }
}

#[allow(dead_code)] // we don't (yet) construct all the variants
#[derive(Debug, Clone, Serialize)]
pub enum OsvReferenceKind {
    ADVISORY,
    ARTICLE,
    REPORT,
    FIX,
    PACKAGE,
    WEB,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvEcosystemSpecific {
    affects: OsvEcosystemSpecificAffected,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvEcosystemSpecificAffected {
    arch: Vec<platforms::target::Arch>,
    os: Vec<platforms::target::OS>,
    /// We include function names only in order to allow changing
    /// the way versions are specified without an API break
    functions: Vec<FunctionPath>,
}

impl From<Affected> for OsvEcosystemSpecificAffected {
    fn from(a: Affected) -> Self {
        OsvEcosystemSpecificAffected {
            arch: a.arch,
            os: a.os,
            functions: a.functions.into_iter().map(|(f, _v)| f).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvDatabaseSpecific {
    categories: Vec<Category>,
    cvss: Option<cvss::v3::Base>,
    informational: Option<Informational>,
}

impl OsvAdvisory {
    /// Converts a single RustSec advisory to OSV format.
    /// `path` is the path to the advisory file. It must be relative to the git repository root.
    pub fn from_rustsec(
        advisory: Advisory,
        mod_times: &GitModificationTimes,
        path: GitPath<'_>,
    ) -> Self {
        let metadata = advisory.metadata;

        // Assemble the URLs to put into 'references' field
        let mut reference_urls: Vec<Url> = Vec::new();
        // link to the package on crates.io
        let package_url = "https://crates.io/crates/".to_owned() + metadata.package.as_str();
        reference_urls.push(Url::parse(&package_url).unwrap());
        // link to human-readable RustSec advisory
        let advisory_url = format!(
            "https://rustsec.org/advisories/{}.html",
            metadata.id.as_str()
        );
        reference_urls.push(Url::parse(&advisory_url).unwrap());
        // primary URL for the issue specified in the advisory
        if let Some(url) = metadata.url {
            reference_urls.push(url);
        }
        // other references
        reference_urls.extend(metadata.references.into_iter());

        OsvAdvisory {
            id: metadata.id,
            modified: git2_time_to_rfc3339(mod_times.for_path(path)),
            published: rustsec_date_to_rfc3339(&metadata.date),
            affects: OsvAffected {
                ranges: json_ranges_for_advisory(&advisory.versions),
            },
            withdrawn: metadata.withdrawn.map(|d| rustsec_date_to_rfc3339(&d)),
            aliases: metadata.aliases,
            related: metadata.related,
            package: (&metadata.package).into(),
            summary: metadata.title,
            details: metadata.description,
            references: osv_references(reference_urls),
            ecosystem_specific: OsvEcosystemSpecific {
                affects: advisory.affected.unwrap_or_default().into(),
            },
            database_specific: OsvDatabaseSpecific {
                categories: metadata.categories,
                cvss: metadata.cvss,
                informational: metadata.informational,
            },
        }
    }
}

fn osv_references(references: Vec<Url>) -> Vec<OsvReference> {
    references.into_iter().map(|u| u.into()).collect()
}

fn guess_url_kind(url: &Url) -> OsvReferenceKind {
    let str = url.as_str();
    if (str.contains("://github.com/") || str.contains("://gitlab.")) && str.contains("/issues/") {
        OsvReferenceKind::REPORT
    // the check for "/advisories/" matches both RustSec and GHSA URLs
    } else if str.contains("/advisories/") || str.contains("://cve.mitre.org/") {
        OsvReferenceKind::ADVISORY
    } else if str.contains("://crates.io/crates/") {
        OsvReferenceKind::PACKAGE
    } else {
        OsvReferenceKind::WEB
    }
}

/// Like ``ranges_for_advisory``, but also converts from ``OsvRange`` to ``OsvJsonRange``
/// Assumes that the input has already been validated; panics if passed an invalid advisory.
fn json_ranges_for_advisory(versions: &crate::advisory::Versions) -> Vec<OsvJsonRange> {
    ranges_for_advisory(versions)
        .into_iter()
        .map(|x| x.into())
        .collect()
}

fn git2_time_to_rfc3339(time: &git2::Time) -> String {
    let unix_timestamp = time.seconds();
    let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    DateTime::<Utc>::from_utc(time, Utc).to_rfc3339()
}

fn rustsec_date_to_rfc3339(d: &crate::advisory::Date) -> String {
    let pub_date: NaiveDate = NaiveDate::from_ymd(d.year() as i32, d.month(), d.day());
    let pub_time = NaiveDateTime::new(pub_date, NaiveTime::from_hms(12, 0, 0));
    DateTime::<Utc>::from_utc(pub_time, Utc).to_rfc3339()
}
