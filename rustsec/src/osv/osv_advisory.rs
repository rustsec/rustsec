use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::Serialize;
use url::Url;

use super::{ranges_for_advisory, OsvRange};

use crate::{
    advisory::Id,
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
    //ecosystem_specific: TODO,
    //database_specific: TODO,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvPackage {
    /// Must be set to a constant identifying crates.io
    ecosystem: String,
    /// Crate name
    name: String,
    /// https://github.com/package-url/purl-spec derived from the other two
    purl: String,
}

impl From<&cargo_lock::Name> for OsvPackage {
    fn from(package: &cargo_lock::Name) -> Self {
        OsvPackage {
            ecosystem: ECOSYSTEM.to_string(),
            name: package.to_string(),
            purl: "pkg:cargo/".to_string() + package.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvAffected {
    // Other fields are specified, but we never use them.
    // Ranges alone are sufficient.
    ranges: Vec<OsvRange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvReference {
    // 'type' is a reserved keyword in Rust
    #[serde(alias = "type")]
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
        let url_string = format!(
            "https://rustsec.org/advisories/{}.html",
            metadata.id.as_str()
        );
        reference_urls.push(Url::parse(&url_string).unwrap());
        if let Some(url) = metadata.url {
            reference_urls.push(url);
        }
        reference_urls.extend(metadata.references.into_iter());

        OsvAdvisory {
            id: metadata.id,
            modified: git2_time_to_rfc3339(mod_times.for_path(path)),
            published: rustsec_date_to_rfc3339(&metadata.date),
            affects: OsvAffected {
                ranges: ranges_for_advisory(&advisory.versions),
            },
            withdrawn: None, //TODO: actually populate this
            aliases: metadata.aliases,
            related: metadata.related,
            package: (&metadata.package).into(),
            summary: metadata.title,
            details: metadata.description,
            references: osv_references(reference_urls),
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
    } else if str.contains("://rustsec.org/advisories/")
        || str.contains("/security/advisories/GHSA-")
        || str.contains("://cve.mitre.org/")
    {
        OsvReferenceKind::ADVISORY
    } else {
        OsvReferenceKind::WEB
    }
}

fn git2_time_to_rfc3339(time: &git2::Time) -> String {
    let unix_timestamp = time.seconds();
    let time = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    DateTime::<Utc>::from_utc(time, Utc).to_rfc3339()
}

fn rustsec_date_to_rfc3339(date: &crate::advisory::Date) -> String {
    let pub_date: NaiveDate = date.into();
    let pub_time = NaiveDateTime::new(pub_date, NaiveTime::from_hms(12, 0, 0));
    DateTime::<Utc>::from_utc(pub_time, Utc).to_rfc3339()
}
