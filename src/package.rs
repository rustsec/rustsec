//! Rust packages enumerated in `Cargo.lock`

pub mod checksum;
pub mod name;
pub mod source;

pub use self::{checksum::Checksum, name::Name, source::SourceId};
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

    /// Source identifier for the package
    pub source: Option<SourceId>,

    /// Checksum for this package
    pub checksum: Option<Checksum>,

    /// Dependencies of the package
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,

    /// Replace directive
    pub replace: Option<Dependency>,
}
