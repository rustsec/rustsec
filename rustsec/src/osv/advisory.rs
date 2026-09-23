//! OSV advisories.
//!
//! It implements the parts of the [OSV schema](https://ossf.github.io/osv-schema) required for
//! RustSec.

#[cfg(feature = "osv-export")]
use super::ranges_for_advisory;
#[cfg(feature = "osv-export")]
use crate::advisory::Versions;
use crate::advisory::{Affected, Category, Id, Informational, affected::FunctionPath};
#[cfg(feature = "osv-export")]
use crate::{
    Advisory,
    repository::git::{GitModificationTimes, GitPath},
};
use cvss::Cvss;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

const ECOSYSTEM: &str = "crates.io";

/// Security advisory in the format defined by <https://github.com/google/osv>
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// A CVSS severity entry with its version-specific vector.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type", content = "score")]
pub enum OsvSeverity {
    /// CVSS version 2.0.
    CVSS_V2(cvss::v2::Vector),
    /// CVSS version 3.0 or 3.1.
    CVSS_V3(cvss::v3::Vector),
    /// CVSS version 4.0.
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

/// A package affected by an OSV advisory, including RustSec-specific metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvAffected {
    package: OsvPackage,
    ecosystem_specific: Option<OsvEcosystemSpecific>,
    database_specific: OsvDatabaseSpecific,
    ranges: Option<Vec<OsvJsonRange>>,
    // FIXME deserialize with deserialize_semver_compat
    versions: Option<Vec<String>>,
}

impl OsvAffected {
    /// Name of the affected package.
    pub fn package_name(&self) -> &str {
        &self.package.name
    }

    /// Ecosystem containing the affected package, such as `crates.io`.
    pub fn ecosystem(&self) -> &str {
        &self.package.ecosystem
    }

    /// RustSec informational classification for this package, if any.
    pub fn informational(&self) -> Option<&Informational> {
        self.database_specific.informational.as_ref()
    }

    /// Affected version ranges, or an empty slice when no ranges are specified.
    pub fn ranges(&self) -> &[OsvJsonRange] {
        self.ranges.as_deref().unwrap_or_default()
    }

    /// Whether any affected range includes a patched version.
    pub fn has_patched_versions(&self) -> bool {
        self.ranges.iter().flatten().any(|range| {
            range
                .events
                .iter()
                .any(|event| matches!(event, OsvTimelineEvent::Fixed(_)))
        })
    }

    /// RustSec vulnerability categories for this package.
    pub fn categories(&self) -> &[Category] {
        &self.database_specific.categories
    }
}

/// An OSV affected range with an ordered sequence of version events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsvJsonRange {
    // 'type' is a reserved keyword in Rust
    #[serde(rename = "type")]
    kind: String,
    events: Vec<OsvTimelineEvent>,
    // 'repo' field is not used because we don't track or export git commit data
}

impl OsvJsonRange {
    /// Range type, such as `SEMVER`, `ECOSYSTEM`, or `GIT`.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Version events in their original order.
    pub fn events(&self) -> &[OsvTimelineEvent] {
        &self.events
    }

    /// Generates the timeline of the bug being introduced and fixed for the
    /// [`affected[].ranges[].events`](https://github.com/ossf/osv-schema/blob/main/schema.md#affectedrangesevents-fields) field.
    #[cfg(feature = "osv-export")]
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

/// A version marking a boundary of an affected range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OsvTimelineEvent {
    /// First affected version; `0` denotes all earlier versions.
    #[serde(rename = "introduced")]
    Introduced(String),
    /// First version containing a fix (excluded from the affected range).
    #[serde(rename = "fixed")]
    Fixed(String),
    /// Last affected version (included in the affected range).
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
    /// A short summary of the advisory.
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Detailed advisory description in Markdown.
    pub fn details(&self) -> &str {
        &self.details
    }

    /// Affected packages and their RustSec-specific metadata.
    pub fn affected(&self) -> &[OsvAffected] {
        &self.affected
    }

    /// CVSS severity entries with their version-specific vectors.
    pub fn severity(&self) -> &[OsvSeverity] {
        &self.severity
    }

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
    #[cfg(feature = "osv-export")]
    #[cfg_attr(docsrs, doc(cfg(feature = "osv-export")))]
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
        let mut refs: Vec<Id> = self
            .references
            .iter()
            .filter(|r| {
                r.url
                    .as_str()
                    .starts_with("https://rustsec.org/advisories/")
            })
            .map(|r| Id::from_str(&r.url.as_str()[31..48]).expect("Invalid rustsec url"))
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

#[cfg(feature = "osv-export")]
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

#[cfg(feature = "osv-export")]
fn rustsec_date_to_rfc3339(d: &crate::advisory::Date) -> String {
    format!("{}-{:02}-{:02}T12:00:00Z", d.year(), d.month(), d.day())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn affected_package_accessors_preserve_individual_classifications() {
        let advisory: OsvAdvisory = serde_json::from_value(json!({
            "id": "RUSTSEC-2026-0299",
            "modified": "2026-09-22T07:35:37Z",
            "published": "2026-09-22T12:00:00Z",
            "summary": "Example summary",
            "details": "Example **details**",
            "affected": [
                {
                    "package": {"ecosystem": "crates.io", "name": "owned-alloc"},
                    "database_specific": {"informational": "unmaintained", "categories": []}
                },
                {
                    "package": {"ecosystem": "crates.io", "name": "another-crate"},
                    "database_specific": {"categories": ["memory-corruption"]}
                }
            ]
        }))
        .unwrap();

        assert_eq!(advisory.summary(), "Example summary");
        assert_eq!(advisory.details(), "Example **details**");
        assert_eq!(advisory.affected().len(), 2);
        let first = &advisory.affected()[0];
        assert_eq!(first.package_name(), "owned-alloc");
        assert_eq!(first.ecosystem(), "crates.io");
        assert_eq!(first.informational(), Some(&Informational::Unmaintained));
        assert!(first.categories().is_empty());
        let second = &advisory.affected()[1];
        assert_eq!(second.package_name(), "another-crate");
        assert_eq!(second.informational(), None);
        assert_eq!(second.categories(), &[Category::MemoryCorruption]);
    }
}

#[cfg(test)]
mod accessor_tests {
    use crate::osv::{OsvAdvisory, OsvAffected, OsvSeverity, OsvTimelineEvent};
    use serde_json::json;

    #[test]
    fn range_accessors_preserve_types_and_event_order() {
        let affected: OsvAffected = serde_json::from_value(json!({
            "package": { "ecosystem": "crates.io", "name": "example" },
            "database_specific": {},
            "ranges": [
                { "type": "SEMVER", "events": [
                    { "introduced": "0" }, { "fixed": "0.7.46" },
                    { "introduced": "0.8.0" }, { "last_affected": "0.8.12" }
                ] },
                { "type": "ECOSYSTEM", "events": [] }
            ]
        }))
        .unwrap();
        assert_eq!(affected.ranges().len(), 2);
        let range = &affected.ranges()[0];
        assert_eq!(range.kind(), "SEMVER");
        assert!(matches!(range.events(), [
            OsvTimelineEvent::Introduced(first), OsvTimelineEvent::Fixed(fixed),
            OsvTimelineEvent::Introduced(second), OsvTimelineEvent::LastAffected(last)
        ] if first == "0" && fixed == "0.7.46" && second == "0.8.0" && last == "0.8.12"));
        assert_eq!(affected.ranges()[1].kind(), "ECOSYSTEM");
        assert!(affected.ranges()[1].events().is_empty());
    }

    #[test]
    fn missing_ranges_are_empty() {
        let affected: OsvAffected = serde_json::from_value(json!({
            "package": { "ecosystem": "crates.io", "name": "example" },
            "database_specific": {}
        }))
        .unwrap();
        assert!(affected.ranges().is_empty());
    }

    #[test]
    fn severity_exposes_versioned_vectors() {
        let v2 = "AV:N/AC:L/Au:N/C:C/I:C/A:C";
        let v3 = "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H";
        let v4 = "CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:H/SI:H/SA:H";
        let mut source = json!({
            "id": "RUSTSEC-2026-0299", "modified": "2026-09-22T07:35:37Z",
            "published": "2026-09-22T12:00:00Z", "summary": "Example", "details": "Example",
            "severity": [
                { "type": "CVSS_V2", "score": v2 },
                { "type": "CVSS_V3", "score": v3 },
                { "type": "CVSS_V4", "score": v4 }
            ]
        });
        let advisory: OsvAdvisory = serde_json::from_value(source.clone()).unwrap();
        assert!(matches!(advisory.severity(), [
            OsvSeverity::CVSS_V2(two), OsvSeverity::CVSS_V3(three), OsvSeverity::CVSS_V4(four)
        ] if two.to_string() == v2 && three.to_string() == v3 && four.to_string() == v4));
        source.as_object_mut().unwrap().remove("severity");
        let advisory: OsvAdvisory = serde_json::from_value(source).unwrap();
        assert!(advisory.severity().is_empty());
    }
}
