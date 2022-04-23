//! Queries against the RustSec database
//!
use crate::{
    advisory::{Advisory, Severity},
    collection::Collection,
    database::scope,
    package,
};
use platforms::target::{Arch, OS};
use semver::Version;

/// Queries against the RustSec database
#[derive(Clone, Debug, Default)]
pub struct Query {
    /// Collection to query against
    pub(super) collection: Option<Collection>,

    /// Package name to search for
    pub(super) package: Option<package::Name>,

    /// Version of a package to search for
    version: Option<Version>,

    /// Severity threshold (i.e. minimum severity)
    severity: Option<Severity>,

    /// Target architecture
    target_arch: Option<Arch>,

    /// Target operating system
    target_os: Option<OS>,

    /// Year associated with the advisory ID
    year: Option<u32>,

    /// Query for withdrawn advisories
    /// (i.e. advisories which were soft-deleted from the database,
    /// as opposed to yanked crates)
    withdrawn: Option<bool>,

    /// Query for informational advisories
    informational: Option<bool>,

    /// Scope of packages which should be considered for audit
    _package_scope: Option<scope::Package>,
}

impl Query {
    /// Create a new query
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new query which uses the default scope rules for crates:
    ///
    /// - Only `Collection::Crates`
    /// - Ignore withdrawn advisories
    /// - Ignore informational advisories
    pub fn crate_scope() -> Self {
        Self::new()
            .collection(Collection::Crates)
            .withdrawn(false)
            .informational(false)
    }

    /// Set collection to query against
    pub fn collection(mut self, collection: Collection) -> Self {
        self.collection = Some(collection);
        self
    }

    /// Set package name to search for
    pub fn package(mut self, package: impl Into<package::Name>) -> Self {
        self.package = Some(package.into());
        self
    }

    /// Set package name to search for along with an associated version
    pub fn package_version(
        mut self,
        package: impl Into<package::Name>,
        version: impl Into<Version>,
    ) -> Self {
        self.package = Some(package.into());
        self.version = Some(version.into());
        self
    }

    /// Set minimum severity threshold according to the CVSS
    /// Qualitative Severity Rating Scale.
    ///
    /// Vulnerabilities without associated CVSS information will always
    /// match regardless of what this is set to.
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Set target architecture
    pub fn target_arch(mut self, arch: Arch) -> Self {
        self.target_arch = Some(arch);
        self
    }

    /// Set target operating system
    pub fn target_os(mut self, os: OS) -> Self {
        self.target_os = Some(os);
        self
    }

    /// Query for vulnerabilities occurring in a specific year.
    pub fn year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }

    /// Query for withdrawn advisories.
    ///
    /// By default they will be omitted from query results.
    pub fn withdrawn(mut self, setting: bool) -> Self {
        self.withdrawn = Some(setting);
        self
    }

    /// Query for informational advisories. By default they will be omitted
    /// from query results.
    pub fn informational(mut self, setting: bool) -> Self {
        self.informational = Some(setting);
        self
    }

    /// Does this query match a given advisory?
    pub fn matches(&self, advisory: &Advisory) -> bool {
        if let Some(collection) = self.collection {
            if Some(collection) != advisory.metadata.collection {
                return false;
            }
        }

        if let Some(package) = &self.package {
            if package != &advisory.metadata.package {
                return false;
            }
        }

        if let Some(version) = &self.version {
            if !advisory.versions.is_vulnerable(version) {
                return false;
            }
        }

        if let Some(severity_threshold) = self.severity {
            if let Some(advisory_severity) = advisory.severity() {
                if advisory_severity < severity_threshold {
                    return false;
                }
            }
        }

        if let Some(affected) = &advisory.affected {
            if let Some(target_arch) = self.target_arch {
                if !affected.arch.is_empty() && !affected.arch.contains(&target_arch) {
                    return false;
                }
            }

            if let Some(target_os) = self.target_os {
                if !affected.os.is_empty() && !affected.os.contains(&target_os) {
                    return false;
                }
            }
        }

        if let Some(query_year) = self.year {
            if let Some(advisory_year) = advisory.metadata.id.year() {
                if query_year != advisory_year {
                    return false;
                }
            }
        }

        if let Some(withdrawn) = self.withdrawn {
            if withdrawn != advisory.metadata.withdrawn.is_some() {
                return false;
            }
        }

        if let Some(informational) = self.informational {
            if informational != advisory.metadata.informational.is_some() {
                return false;
            }
        }

        true
    }
}
