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
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvPackage {
    ecosystem: String,
    name: String,
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
        let mtime = mod_times.for_path(path);

        OsvAdvisory {
            id: metadata.id,
            modified: git2_time_to_rfc3339(mtime),
            published: rustsec_date_to_rfc3339(&metadata.date),
            affects: OsvAffected {
                ranges: ranges_for_advisory(&advisory.versions),
            },
            withdrawn: None, //TODO: actually populate this
            aliases: metadata.aliases,
            related: metadata.related,
            package: OsvPackage {
                ecosystem: ECOSYSTEM.to_string(),
                name: metadata.package.to_string(),
            },
            summary: metadata.title,
            details: metadata.description,
            references: osv_references(metadata.url, metadata.references),
        }
    }
}

fn osv_references(url: Option<Url>, references: Vec<Url>) -> Vec<OsvReference> {
    let mut result: Vec<OsvReference> = Vec::new();
    if let Some(url) = url {
        result.push(rustsec_to_osv_reference(url));
    }
    result.extend(references.into_iter().map(rustsec_to_osv_reference));
    result
}

fn rustsec_to_osv_reference(url: Url) -> OsvReference {
    OsvReference {
        kind: OsvReferenceKind::WEB, //TODO: guess kind
        url: url,
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
