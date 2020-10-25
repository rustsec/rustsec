//! Security advisories in the RustSec database

pub mod affected;
pub mod category;
pub mod date;
pub mod id;
pub mod informational;
pub mod keyword;
pub mod linter;
pub mod metadata;
pub mod parser;
pub mod versions;

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
    /// One-liner description of a vulnerability
    #[serde(default)]
    pub title: String,

    /// Extended description of a vulnerability
    #[serde(default)]
    pub description: String,

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

        let advisory_data = fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?;

        advisory_data
            .parse()
            .map_err(|e| format_err!(ErrorKind::Parse, "error parsing {}: {}", path.display(), e))
    }

    /// Get the description of this advisory as HTML rendered from Markdown
    #[cfg(feature = "markdown")]
    pub fn description_html(&self) -> String {
        comrak::markdown_to_html(&self.description, &Default::default())
    }

    /// Get the severity of this advisory if it has a CVSS v3 associated
    pub fn severity(&self) -> Option<Severity> {
        self.metadata.cvss.as_ref().map(|cvss| cvss.severity())
    }
}

impl FromStr for Advisory {
    type Err = Error;

    fn from_str(advisory_data: &str) -> Result<Self, Error> {
        let parts = parser::Parts::parse(&advisory_data)?;

        let mut advisory: Self = toml::from_str(&parts.front_matter)?;

        if advisory.title != "" {
            fail!(
                ErrorKind::Parse,
                "invalid `title` attribute in advisory TOML"
            );
        }

        if advisory.description != "" {
            fail!(
                ErrorKind::Parse,
                "invalid `description` attribute in advisory TOML"
            );
        }

        advisory.title = parts.title.to_owned();
        advisory.description = parts.description.to_owned();

        Ok(advisory)
    }
}
