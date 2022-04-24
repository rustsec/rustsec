//! Queries against the RustSec database
//!
use crate::{
    advisory::{Advisory, Severity},
    collection::Collection,
    package::{self, Package},
    SourceId,
};
use platforms::target::{Arch, OS};
use semver::Version;

/// Queries against the RustSec database
#[derive(Clone, Debug)]
pub struct Query {
    /// Collection to query against
    pub(super) collection: Option<Collection>,

    /// Package name to search for
    pub(super) package_name: Option<package::Name>,

    /// Package version to search for
    package_version: Option<Version>,

    /// Source of the package advisories should be matched against
    package_source: Option<SourceId>,

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
}

impl Query {
    /// Create a new query.
    ///
    /// This creates a "wildcard" query with no constraints. Use the various
    /// builder methods of this type to restrict which advisories match.
    ///
    /// Note that this differs from [`Query::default()`], which scopes the
    /// query to crates (i.e. [`Query::crate_scope`]).
    ///
    /// When in doubt, use [`Query::default()`].
    pub fn new() -> Self {
        Self {
            collection: None,
            package: None,
            version: None,
            severity: None,
            target_arch: None,
            target_os: None,
            year: None,
            withdrawn: None,
            informational: None,
            _package_scope: None,
        }
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

    /// Provide a package and use all of its attributes as part of the query
    pub fn package(mut self, package: &Package) -> Self {
        self.package_name = Some(package.name.clone());
        self.package_version = Some(package.version.clone());
        self.package_source = package.source.clone();
        self
    }

    /// Set package name to search for.
    pub fn package_name(mut self, name: package::Name) -> Self {
        self.package_name = Some(name);
        self
    }

    /// Set package version to search for
    pub fn package_version(mut self, version: Version) -> Self {
        self.package_version = Some(version);
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

        if let Some(package_name) = &self.package_name {
            if package_name != &advisory.metadata.package {
                return false;
            }
        }

        if let Some(package_version) = &self.package_version {
            if !advisory.versions.is_vulnerable(package_version) {
                return false;
            }
        }

        if let Some(package_source) = &self.package_source {
            let default_registry_url = SourceId::default().url().clone();
            let advisory_registry_url = advisory
                .metadata
                .registry
                .as_ref()
                .cloned()
                .unwrap_or(default_registry_url);

            if !package_source.is_registry() || package_source.url() != &advisory_registry_url {
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

impl Default for Query {
    fn default() -> Query {
        Query::crate_scope()
    }
}
