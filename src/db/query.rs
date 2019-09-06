//! Queries against the RustSec database

use crate::{
    advisory::{Advisory, Severity},
    package,
};

/// Queries against the RustSec database
#[derive(Clone, Debug, Default)]
pub struct Query {
    /// Collection to query against
    pub(super) collection: Option<package::Collection>,

    /// Package name to search for
    pub(super) package: Option<package::Name>,

    /// Severity threshold (i.e. minimum severity)
    severity: Option<Severity>,

    /// Year associated with the advisory ID
    year: Option<u32>,

    /// Query for obsolete advisories
    obsolete: Option<bool>,

    /// Query for informational advisories
    informational: Option<bool>,
}

impl Query {
    /// Create a new query
    pub fn new() -> Self {
        Self::default()
    }

    /// Set collection to query against
    pub fn collection(mut self, collection: package::Collection) -> Self {
        self.collection = Some(collection);
        self
    }

    /// Set package name to search for
    pub fn package(mut self, package: impl Into<package::Name>) -> Self {
        self.package = Some(package.into());
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

    /// Query for vulnerabilities occurring in a specific year.
    pub fn year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }

    /// Query for obsolete vulnerabilities. By default they will be omitted
    /// from query results.
    pub fn obsolete(mut self, setting: bool) -> Self {
        self.obsolete = Some(setting);
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

        if let Some(query_severity) = self.severity {
            if let Some(advisory_severity) = advisory.severity() {
                if advisory_severity < query_severity {
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

        if let Some(obsolete) = self.obsolete {
            if obsolete != advisory.metadata.obsolete {
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
