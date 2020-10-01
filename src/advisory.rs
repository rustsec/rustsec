//! Security advisories in the RustSec database

pub mod affected;
pub mod category;
pub mod date;
pub mod id;
pub mod informational;
pub mod keyword;
pub mod linter;
pub mod metadata;
pub mod versions;

mod parser;

pub use self::{
    affected::Affected, category::Category, date::Date, id::Id, informational::Informational,
    keyword::Keyword, linter::Linter, metadata::Metadata, versions::Versions,
};
pub use cvss::Severity;

use crate::{
    error::{Error, ErrorKind},
    fs,
};
use serde::{Deserialize, Serialize};
use std::{path::Path, str::FromStr};

/// RustSec Security Advisories
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Advisory {
    /// The `[advisory]` section of a RustSec advisory
    #[serde(rename = "advisory")]
    pub metadata: Metadata,

    /// The (optional) `[affected]` section of a RustSec advisory
    pub affected: Option<Affected>,

    /// Versions related to this advisory which are patched or unaffected.
    pub versions: Versions,
}

impl Advisory {
    /// Load an advisory from a `RUSTSEC-20XX-NNNN.toml` file
    pub fn load_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();

        // TODO(tarcieri): deprecate and remove legacy TOML-based advisory format
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => {
                // Legacy TOML-based advisory format
                fs::read_to_string(path)
                    .map_err(|e| {
                        format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e)
                    })?
                    .parse()
            }
            Some("md") => {
                // New V3 Markdown-based advisory format
                Self::parse_v3(path)
            }
            _ => fail!(
                ErrorKind::Repo,
                "unexpected file extension: {}",
                path.display()
            ),
        }
    }

    /// Parse a V3 advisory at the given path
    // TODO(tarcieri): make V3 advisory format the default
    pub fn parse_v3(path: &Path) -> Result<Self, Error> {
        let advisory_data = fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?;

        Self::parse_v3_string(&advisory_data)
            .map_err(|e| format_err!(ErrorKind::Parse, "error parsing {}: {}", path.display(), e))
    }

    /// Parse a V3 advisory from a string
    // TODO(tarcieri): make V3 advisory format the default
    pub fn parse_v3_string(advisory_data: &str) -> Result<Self, Error> {
        let parts = parser::Parts::parse(&advisory_data)?;

        let mut advisory: Self = toml::from_str(&parts.front_matter)?;

        if advisory.metadata.title != "" || advisory.metadata.description != "" {
            fail!(
                ErrorKind::Parse,
                "Markdown advisories MUST have empty title/description"
            );
        }

        advisory.metadata.title = parts.title.to_owned();
        advisory.metadata.description = parts.description.to_owned();
        Ok(advisory)
    }

    /// Get the severity of this advisory if it has a CVSS v3 associated
    pub fn severity(&self) -> Option<Severity> {
        self.metadata.cvss.as_ref().map(|cvss| cvss.severity())
    }
}

impl FromStr for Advisory {
    type Err = Error;

    fn from_str(advisory_data: &str) -> Result<Self, Error> {
        // TODO(tarcieri): make V3 advisory format the default
        if advisory_data.starts_with("```toml") {
            return Self::parse_v3_string(advisory_data);
        }

        let advisory: Self = toml::from_str(advisory_data)?;

        if advisory.metadata.title == "" || advisory.metadata.description == "" {
            fail!(
                ErrorKind::Parse,
                "missing title and/or description in advisory:\n\n{}",
                advisory_data
            )
        }

        Ok(advisory)
    }
}
