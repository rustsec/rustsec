use crate::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
/// Type representing licenses used for advisory content
#[non_exhaustive]
pub enum License {
    /// Creative Commons Zero v1.0 Universal
    /// SPDX identifier: CC0-1.0
    #[serde(rename = "CC0-1.0")]
    #[default]
    CcZero10,
    /// Creative Commons Attribution 4.0 International
    /// SPDX identifier: CC-BY-4.0
    ///
    /// Note: For GitHub Security Advisories database,
    /// providing a link is [documented](https://docs.github.com/en/site-policy/github-terms/github-terms-for-additional-products-and-features#advisory-database)
    /// as fulfilling the attribution obligation for the CC-BY 4.0 license used.
    ///
    /// For advisories imported from a GitHub Security Advisory, we follow this by putting the
    /// original URL in the `url` filed of the RustSec advisory, as it assures the link will be
    /// visible to downstream users.
    #[serde(rename = "CC-BY-4.0")]
    CcBy40,
    /// Other SPDX requirement
    Other(String),
}

impl FromStr for License {
    type Err = Error;

    // Parse standard SPDX identifiers
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CC0-1.0" => License::CcZero10,
            "CC-BY-4.0" => License::CcBy40,
            l => License::Other(l.to_string()),
        })
    }
}

impl License {
    /// Get license as an `&str` containing the SPDX identifier
    pub fn spdx(&self) -> &str {
        match self {
            License::CcBy40 => "CC-BY-4.0",
            License::CcZero10 => "CC0-1.0",
            License::Other(ref l) => l,
        }
    }
}
