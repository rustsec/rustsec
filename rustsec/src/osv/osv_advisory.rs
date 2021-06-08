use std::path::Path;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::Serialize;

use super::{OsvRange, ranges_for_advisory};

use crate::{Advisory, repository::git::GitModificationTimes};

const ECOSYSTEM: &'static str = "crates.io";

#[derive(Debug, Clone, Serialize)]
pub struct OsvAdvisory {
    id: String,
    modified: String, // maybe add an rfc3339 newtype?
    published: String, // maybe add an rfc3339 newtype?
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn: Option<String>, // maybe add an rfc3339 newtype?
    aliases: Vec<String>,
    related: Vec<String>,
    package: OsvPackage,
    summary: String,
    details: String,
    affects: OsvAffected,
    references: Vec<OsvReference>
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
    ranges: Vec<OsvRange>
}

#[derive(Debug, Clone, Serialize)]
pub struct OsvReference {
    // 'type' is a reserved keyword in Rust
    #[serde(alias = "type")]
    kind: OsvReferenceKind,
    url: String
}

#[derive(Debug, Clone, Serialize)]
pub enum OsvReferenceKind {
    ADVISORY,
    ARTICLE,
    REPORT,
    FIX,
    PACKAGE,
    WEB
}

impl OsvAdvisory {
    /// Converts a single RustSec advisory to OSV format.
    /// `path` must be relative to the git repository root.
    pub fn from_rustsec(advisory: &Advisory, mod_times: GitModificationTimes, path: &Path) -> Self {
        let mtime = mod_times.for_path(path)
            .expect(&format!("Could not find file {:?}, make sure the path is specified relative to the git repo root", path));

        OsvAdvisory {
            id: advisory.metadata.id.to_string(),
            modified: git2_time_to_chrono(mtime).to_rfc3339(),
            published: rustsec_date_to_chrono(&advisory.metadata.date).to_rfc3339(),
            affects: OsvAffected{ranges: ranges_for_advisory(&advisory.versions)},
            withdrawn: None, //TODO: actually populate this
            aliases: advisory.metadata.aliases.iter().map(|id| id.to_string()).collect(),
            related: Vec::new(), //TODO: do we even track this?
            package: OsvPackage {
                ecosystem: ECOSYSTEM.to_string(),
                name: advisory.metadata.package.to_string()
            },
            summary: advisory.metadata.title.clone(),
            details: advisory.metadata.description.clone(),
            references: rustsec_to_osv_references(&advisory.metadata.references),
        }
    }
}

fn rustsec_to_osv_references(refs: &[url::Url]) -> Vec<OsvReference> {
    refs.iter().map(|rustsec_url| {
        OsvReference {
            kind: OsvReferenceKind::WEB, //TODO: guess kind
            url: rustsec_url.as_str().to_string(),
        }
    }).collect()
}

fn git2_time_to_chrono(time: &git2::Time) -> DateTime::<Utc> {
    let unix_timestamp = time.seconds();
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(unix_timestamp, 0), Utc)
}

fn rustsec_date_to_chrono(date: &crate::advisory::Date) -> DateTime::<Utc> {
    let pub_date: NaiveDate = date.into();
    let pub_time = NaiveDateTime::new(pub_date, NaiveTime::from_hms(12,0,0));
    DateTime::<Utc>::from_utc(pub_time, Utc)
}