//! OSV advisories.
//!
//! It implements the parts of the [OSV schema](https://ossf.github.io/osv-schema) required for
//! RustSec.

use super::ranges_for_advisory;
use crate::advisory::Versions;
use crate::{
    Advisory,
    advisory::{Affected, Category, Id, Informational, affected::FunctionPath},
    repository::git::{GitModificationTimes, GitPath},
};
use cvss::Cvss;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

const ECOSYSTEM: &str = "crates.io";

/// Security advisory in the format defined by <https://github.com/google/osv>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(docsrs, doc(cfg(feature = "osv-export")))]
pub struct OsvAdvisory {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_version: Option<semver::Version>,
    id: Id,
    modified: String,  // maybe add an rfc3339 newtype?
    published: String, // maybe add an rfc3339 newtype?
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn: Option<String>, // maybe add an rfc3339 newtype?
    #[serde(default)]
    aliases: Vec<Id>,
    #[serde(default)]
    related: Vec<Id>,
    summary: String,
    details: String,
    #[serde(default)]
    severity: Vec<OsvSeverity>,
    #[serde(default)]
    affected: Vec<OsvAffected>,
    #[serde(default)]
    references: Vec<OsvReference>,
    #[serde(default)]
    database_specific: MainOsvDatabaseSpecific,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvPackage {
    /// Set to a constant identifying crates.io
    pub(crate) ecosystem: String,
    /// Crate name
    pub(crate) name: String,
    /// https://github.com/package-url/purl-spec derived from the other two
    #[serde(default)]
    purl: Option<String>,
}

impl From<&cargo_lock::Name> for OsvPackage {
    fn from(package: &cargo_lock::Name) -> Self {
        Self {
            ecosystem: ECOSYSTEM.to_string(),
            name: package.to_string(),
            purl: Some("pkg:cargo/".to_string() + package.as_str()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "score")]
enum OsvSeverity {
    CVSS_V2(cvss::v2::Vector),
    CVSS_V3(cvss::v3::Vector),
    CVSS_V4(cvss::v4::Vector),
}

impl TryFrom<Cvss> for OsvSeverity {
    type Error = &'static str;

    fn try_from(cvss: Cvss) -> Result<Self, Self::Error> {
        match cvss {
            Cvss::CvssV20(vector) => Ok(Self::CVSS_V2(vector)),
            Cvss::CvssV30(base) => Ok(Self::CVSS_V3(base)),
            Cvss::CvssV31(base) => Ok(Self::CVSS_V3(base)),
            Cvss::CvssV40(vector) => Ok(Self::CVSS_V4(vector)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvAffected {
    pub(crate) package: OsvPackage,
    ecosystem_specific: Option<OsvEcosystemSpecific>,
    database_specific: OsvDatabaseSpecific,
    ranges: Option<Vec<OsvJsonRange>>,
    // FIXME deserialize with deserialize_semver_compat
    versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvJsonRange {
    // 'type' is a reserved keyword in Rust
    #[serde(rename = "type")]
    kind: String,
    events: Vec<OsvTimelineEvent>,
    // 'repo' field is not used because we don't track or export git commit data
}

impl OsvJsonRange {
    /// Generates the timeline of the bug being introduced and fixed for the
    /// [`affected[].ranges[].events`](https://github.com/ossf/osv-schema/blob/main/schema.md#affectedrangesevents-fields) field.
    fn new(versions: &Versions) -> Self {
        let ranges = ranges_for_advisory(versions);
        assert!(!ranges.is_empty()); // zero ranges means nothing is affected, so why even have an advisory?
        let mut timeline = Vec::new();
        for range in ranges {
            match range.introduced {
                Some(ver) => timeline.push(OsvTimelineEvent::Introduced(ver.to_string())),
                None => timeline.push(OsvTimelineEvent::Introduced("0.0.0-0".to_owned())),
            }
            #[allow(clippy::single_match)]
            match range.fixed {
                Some(ver) => timeline.push(OsvTimelineEvent::Fixed(ver.to_string())),
                None => (), // "everything after 'introduced' is affected" is implicit in OSV
            }
        }

        Self {
            kind: "SEMVER".to_string(),
            events: timeline,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OsvTimelineEvent {
    #[serde(rename = "introduced")]
    Introduced(String),
    #[serde(rename = "fixed")]
    Fixed(String),
    #[serde(rename = "last_affected")]
    LastAffected(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvReference {
    // 'type' is a reserved keyword in Rust
    #[serde(rename = "type")]
    pub kind: OsvReferenceKind,
    pub url: Url,
}

impl From<Url> for OsvReference {
    fn from(url: Url) -> Self {
        Self {
            kind: guess_url_kind(&url),
            url,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
enum OsvReferenceKind {
    ADVISORY,
    #[allow(dead_code)]
    ARTICLE,
    REPORT,
    #[allow(dead_code)]
    FIX,
    PACKAGE,
    WEB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvEcosystemSpecific {
    affects: Option<OsvEcosystemSpecificAffected>,
    affected_functions: Option<Vec<FunctionPath>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvEcosystemSpecificAffected {
    arch: Vec<String>,
    os: Vec<String>,
    /// We include function names only in order to allow changing
    /// the way versions are specified without an API break
    functions: Vec<FunctionPath>,
}

impl From<Affected> for OsvEcosystemSpecificAffected {
    fn from(a: Affected) -> Self {
        Self {
            arch: a.arch,
            os: a.os,
            functions: a.functions.into_keys().collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OsvDatabaseSpecific {
    #[serde(default)]
    categories: Vec<Category>,
    cvss: Option<Cvss>,
    informational: Option<Informational>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct MainOsvDatabaseSpecific {
    #[serde(default)]
    license: Option<String>,
}

impl OsvAdvisory {
    /// Advisory ID
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Publication date
    pub fn published(&self) -> &str {
        &self.published
    }

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
        reference_urls.extend(metadata.references);

        Self {
            schema_version: None,
            id: metadata.id,
            modified: mod_times
                .for_path(path)
                .format(&time::format_description::well_known::Rfc3339)
                .expect("well-known format to heap never fails"),
            published: rustsec_date_to_rfc3339(&metadata.date),
            affected: vec![OsvAffected {
                package: (&metadata.package).into(),
                ranges: Some(vec![OsvJsonRange::new(&advisory.versions)]),
                versions: Some(vec![]),
                ecosystem_specific: Some(OsvEcosystemSpecific {
                    affects: Some(advisory.affected.unwrap_or_default().into()),
                    affected_functions: None,
                }),
                database_specific: OsvDatabaseSpecific {
                    categories: metadata.categories,
                    cvss: metadata.cvss.clone(),
                    informational: metadata.informational,
                },
            }],
            withdrawn: metadata.withdrawn.map(|d| rustsec_date_to_rfc3339(&d)),
            aliases: metadata.aliases,
            related: metadata.related,
            summary: metadata.title,
            severity: match metadata.cvss {
                Some(cvss) => match cvss.try_into() {
                    Ok(sev) => vec![sev],
                    Err(_) => vec![],
                },
                None => vec![],
            },
            details: metadata.description,
            references: osv_references(reference_urls),
            database_specific: MainOsvDatabaseSpecific {
                license: Some(metadata.license.spdx().to_string()),
            },
        }
    }

    /// Try to extract RustSec alias id from OSV advisory metadata
    pub fn rustsec_refs_imported(&self) -> Vec<Id> {
        const PREFIX: &str = "https://rustsec.org/advisories/";
        let mut refs: Vec<Id> = self
            .references
            .iter()
            .filter_map(|r| {
                // Strip the known prefix and the human-readable ".html"
                // suffix that the exporter appends, then parse the remaining
                // advisory id. A URL that merely starts with the prefix but is
                // truncated or malformed simply yields no id, rather than
                // panicking on an out-of-bounds byte slice or an `expect` on an
                // unparseable value. Only genuine RustSec ids are kept, since
                // that is what this function is documented to return.
                let rest = r.url.as_str().strip_prefix(PREFIX)?;
                let id_str = rest.strip_suffix(".html").unwrap_or(rest);
                Id::from_str(id_str).ok().filter(Id::is_rustsec)
            })
            .collect();
        refs.sort();
        refs.dedup();
        refs
    }

    /// Get crates in crates.io ecosystem referenced in this advisory
    pub fn crates(&self) -> Vec<&str> {
        let mut res = self
            .affected
            .iter()
            .filter_map(|a| {
                if a.package.ecosystem == ECOSYSTEM {
                    Some(a.package.name.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        res.sort();
        res.dedup();
        res
    }

    /// Get aliases ids
    pub fn aliases(&self) -> &[Id] {
        self.aliases.as_slice()
    }

    /// Is this advisory withdrawn?
    pub fn withdrawn(&self) -> bool {
        self.withdrawn.is_some()
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
    } else if str.contains("/advisories/") || str.contains("://www.cve.org/") {
        OsvReferenceKind::ADVISORY
    } else if str.contains("://crates.io/crates/") {
        OsvReferenceKind::PACKAGE
    } else {
        OsvReferenceKind::WEB
    }
}

fn rustsec_date_to_rfc3339(d: &crate::advisory::Date) -> String {
    format!("{}-{:02}-{:02}T12:00:00Z", d.year(), d.month(), d.day())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn advisory_with_reference_urls(urls: &[&str]) -> OsvAdvisory {
        let refs: String = urls
            .iter()
            .map(|u| format!("{{\"type\":\"ADVISORY\",\"url\":\"{}\"}}", u))
            .collect::<Vec<_>>()
            .join(",");
        let json = format!(
            r#"{{
                "id": "RUSTSEC-2021-0001",
                "modified": "2021-01-01T00:00:00Z",
                "published": "2021-01-01T00:00:00Z",
                "summary": "s",
                "details": "d",
                "references": [{refs}]
            }}"#,
            refs = refs
        );
        serde_json::from_str(&json).expect("valid OSV advisory json")
    }

    #[test]
    fn extracts_ids_from_well_formed_rustsec_urls() {
        let advisory = advisory_with_reference_urls(&[
            "https://rustsec.org/advisories/RUSTSEC-2018-0001.html",
            "https://crates.io/crates/foo",
        ]);
        let ids = advisory.rustsec_refs_imported();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].as_str(), "RUSTSEC-2018-0001");
    }

    #[test]
    fn short_or_malformed_rustsec_urls_do_not_panic() {
        // Regression test for #1681: a URL that starts with the advisories
        // prefix but is truncated (shorter than the old hardcoded [31..48]
        // slice) or otherwise unparseable must be skipped, not panic.
        let advisory = advisory_with_reference_urls(&[
            "https://rustsec.org/advisories/",
            "https://rustsec.org/advisories/RUSTSEC",
            "https://rustsec.org/advisories/not-an-id.html",
            "https://rustsec.org/advisories/RUSTSEC-2018-0002.html",
        ]);
        let ids = advisory.rustsec_refs_imported();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].as_str(), "RUSTSEC-2018-0002");
    }
}
