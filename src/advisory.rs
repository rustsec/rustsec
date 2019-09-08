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
    category::Category, date::Date, id::Id, keyword::Keyword, linter::Linter, metadata::Metadata,
    versions::Versions,
};
pub use cvss::Severity;

use self::affected::Affected;
use crate::error::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr};

/// RustSec Security Advisories
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Advisory {
    /// The `[advisory]` section of a RustSec advisory
    #[serde(rename = "advisory")]
    pub metadata: Metadata,

    /// Versions related to this advisory which are patched or unaffected.
    ///
    /// This maps to the `[versions]` section of an advisory, but we can't
    /// actually start using that until clients have all updated, so for
    /// backwards compatibility we still use `[advisory.patched_versions]`
    /// and `[advisory.unaffected_versions]`, but load them into this section.
    #[serde(default)]
    pub versions: Versions,

    /// The (optional) `[affected]` section of a RustSec advisory
    pub affected: Option<Affected>,
}

impl Advisory {
    /// Load an advisory from a `RUSTSEC-20XX-NNNN.toml` file
    pub fn load_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        fs::read_to_string(path)
            .map_err(|e| format_err!(ErrorKind::Io, "couldn't open {}: {}", path.display(), e))?
            .parse()
    }

    /// Get the severity of this advisory if it has a CVSS v3 associated
    pub fn severity(&self) -> Option<Severity> {
        self.metadata.cvss.as_ref().map(|cvss| cvss.severity())
    }

    /// Populate the new version fields from the legacy `patched_versions` and
    /// `unaffected_versions` fields
    // TODO(tarcieri): deprecate and remove the old version fields
    fn fixup_versions(&mut self) -> Result<(), Error> {
        macro_rules! populate_new_version_fields {
            ($advisory:expr, $old_field:ident, $new_field:ident) => {
                if $advisory.versions.$new_field != $advisory.metadata.$old_field {
                    if $advisory.versions.$new_field.is_empty() {
                        $advisory.versions.$new_field = $advisory.metadata.$old_field.clone();
                    } else if !$advisory.metadata.$old_field.is_empty() {
                        fail!(
                            ErrorKind::Parse,
                            "conflict between legacy `[advisory.{}]` \
                             and `[versions]`: '{:?}' vs '{:?}'",
                            stringify!($old_field),
                            self.metadata.$old_field,
                            self.versions.$new_field,
                        );
                    }
                }

                $advisory.metadata.$old_field = vec![];
            };
        }

        populate_new_version_fields!(self, patched_versions, patched);
        populate_new_version_fields!(self, patched_versions, patched);

        Ok(())
    }
}

impl FromStr for Advisory {
    type Err = Error;

    fn from_str(toml_string: &str) -> Result<Self, Error> {
        let mut advisory: Self = toml::from_str(toml_string)?;

        // TODO(tarcieri): deprecate and remove the old version fields
        advisory.fixup_versions()?;

        Ok(advisory)
    }
}
