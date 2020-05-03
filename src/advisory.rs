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

pub use self::{
    affected::Affected, category::Category, date::Date, id::Id, informational::Informational,
    keyword::Keyword, linter::Linter, metadata::Metadata, versions::Versions,
};
pub use cvss::Severity;

use crate::error::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr};

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

    /// Parse a V3 advisory from a string
    pub fn parse_v3(path: &Path) -> Result<Self, Error> {
        let advisory_data = fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?;

        if !advisory_data.starts_with("```toml") {
            fail!(
                ErrorKind::Parse,
                "unexpected start of V3 advisory: {}",
                path.display()
            )
        }

        let toml_end = advisory_data.find("\n```").ok_or_else(|| {
            format_err!(
                ErrorKind::Parse,
                "couldn't find end of TOML front matter in advisory: {}",
                path.display()
            )
        })?;

        let front_matter = advisory_data[7..toml_end].trim_start().trim_end();
        let mut advisory: Self = toml::from_str(front_matter)?;

        if advisory.metadata.title != "" || advisory.metadata.description != "" {
            fail!(
                ErrorKind::Parse,
                "Markdown advisories MUST have empty title/description: {}",
                path.display()
            )
        }

        let markdown = advisory_data[(toml_end + 4)..].trim_start();

        if !markdown.starts_with("# ") {
            fail!(
                ErrorKind::Parse,
                "Expected # header after TOML front matter in: {}",
                path.display()
            );
        }

        let next_newline = markdown.find('\n').ok_or_else(|| {
            format_err!(
                ErrorKind::Parse,
                "no Markdown body (i.e. description) found: {}",
                path.display()
            )
        })?;

        advisory.metadata.title = markdown[2..next_newline].trim_end().to_owned();
        advisory.metadata.description = markdown[(next_newline + 1)..]
            .trim_start()
            .trim_end()
            .to_owned();

        Ok(advisory)
    }

    /// Get the severity of this advisory if it has a CVSS v3 associated
    pub fn severity(&self) -> Option<Severity> {
        self.metadata.cvss.as_ref().map(|cvss| cvss.severity())
    }
}

impl FromStr for Advisory {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        let advisory: Self = toml::from_str(toml_string)?;

        if advisory.metadata.title == "" || advisory.metadata.description == "" {
            fail!(
                ErrorKind::Parse,
                "missing title and/or description in advisory:\n\n{}",
                toml_string
            )
        }

        Ok(advisory)
    }
}
