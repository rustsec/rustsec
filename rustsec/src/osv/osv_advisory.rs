use std::path::Path;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::Serialize;

use super::{ranges_for_advisory, OsvRange};

use crate::{advisory::Metadata, repository::git::GitModificationTimes, Advisory};

const ECOSYSTEM: &'static str = "crates.io";

/// Security advisory in the format defined by https://github.com/google/osv
#[derive(Debug, Clone, Serialize)]
pub struct OsvAdvisory {
    id: String,
    modified: String,  // maybe add an rfc3339 newtype?
    published: String, // maybe add an rfc3339 newtype?
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn: Option<String>, // maybe add an rfc3339 newtype?
    aliases: Vec<String>,
    related: Vec<String>,
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
    url: String,
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
    /// `path` must be relative to the git repository root.
    pub fn from_rustsec(advisory: &Advisory, mod_times: GitModificationTimes, path: &Path) -> Self {
        let metadata = &advisory.metadata;
        let mtime = mod_times.for_path(path)
            .expect(&format!("Could not find file {:?}, make sure the path is specified relative to the git repo root", path));

        OsvAdvisory {
            id: metadata.id.to_string(),
            modified: git2_time_to_chrono(mtime).to_rfc3339(),
            published: rustsec_date_to_chrono(&metadata.date).to_rfc3339(),
            affects: OsvAffected {
                ranges: ranges_for_advisory(&advisory.versions),
            },
            withdrawn: None, //TODO: actually populate this
            aliases: metadata.aliases.iter().map(|id| id.to_string()).collect(),
            related: Vec::new(), //TODO: do we even track this?
            package: OsvPackage {
                ecosystem: ECOSYSTEM.to_string(),
                name: metadata.package.to_string(),
            },
            summary: metadata.title.clone(),
            details: metadata.description.clone(),
            references: osv_references(&metadata),
        }
    }
}

fn osv_references(meta: &Metadata) -> Vec<OsvReference> {
    let mut result: Vec<OsvReference> = Vec::new();
    if let Some(url) = &meta.url {
        result.push(rustsec_to_osv_reference(url));
    }
    result.extend(meta.references.iter().map(rustsec_to_osv_reference));
    result
}

fn rustsec_to_osv_reference(url: &url::Url) -> OsvReference {
    OsvReference {
        kind: OsvReferenceKind::WEB, //TODO: guess kind
        url: url.as_str().to_string(),
    }
}

fn git2_time_to_chrono(time: &git2::Time) -> DateTime<Utc> {
    let unix_timestamp = time.seconds();
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(unix_timestamp, 0), Utc)
}

fn rustsec_date_to_chrono(date: &crate::advisory::Date) -> DateTime<Utc> {
    let pub_date: NaiveDate = date.into();
    let pub_time = NaiveDateTime::new(pub_date, NaiveTime::from_hms(12, 0, 0));
    DateTime::<Utc>::from_utc(pub_time, Utc)
}
