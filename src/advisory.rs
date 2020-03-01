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
        dbg!(&path);
        fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?
            .parse()
    }

    /// Get the severity of this advisory if it has a CVSS v3 associated
    pub fn severity(&self) -> Option<Severity> {
        self.metadata.cvss.as_ref().map(|cvss| cvss.severity())
    }
}

impl FromStr for Advisory {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        Ok(toml::from_str(toml_string)?)
    }
}
