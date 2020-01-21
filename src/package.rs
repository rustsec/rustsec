//! Rust packages enumerated in `Cargo.lock`

pub mod checksum;
pub mod name;
pub mod source;

pub use self::{checksum::Checksum, name::Name, source::Source};
pub use semver::Version;

use crate::dependency::Dependency;
use serde::{Deserialize, Serialize};

/// Information about a Rust package (as sourced from `Cargo.lock`)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Package {
    /// Name of the package
    pub name: Name,

    /// Version of the package
    pub version: Version,

    /// Source for the package
    pub source: Option<Source>,

    /// Checksum for this package
    pub checksum: Option<Checksum>,

    /// Dependencies of the package
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

impl From<Package> for Dependency {
    /// Get the [`Dependency`] requirement for this `[[package]]`
    fn from(pkg: Package) -> Dependency {
        Self {
            name: pkg.name.clone(),
            version: Some(pkg.version.clone()),
            source: pkg.source,
        }
    }
}
