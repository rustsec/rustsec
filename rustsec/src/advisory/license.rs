use crate::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Type representing allowed licenses for advisory content
enum LicenseVariant {
    /// CC0-1.0
    CreativeCommonsZero10,
    /// CC-BY-4.0
    ///
    /// Note: For GitHub Security Advisories database,
    /// providing a link is [documented](https://docs.github.com/en/site-policy/github-terms/github-terms-for-additional-products-and-features#advisory-database)
    /// as fulfilling the attribution obligation for the CC-BY 4.0 license used.
    ///
    /// For advisories imported from a GitHub Security Advisory, we follow this by putting the
    /// original URL in the `url` filed of the RustSec advisory, as it assures the link will be
    /// visible to downstream users.
    CreativeCommonsCc40,
    /// Other SPDX requirement
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Opaque type representing allowed licenses for advisory content
pub struct License {
    inner: LicenseVariant,
}

impl Default for License {
    fn default() -> Self {
        Self {
            inner: LicenseVariant::CreativeCommonsZero10,
        }
    }
}

impl FromStr for License {
    type Err = Error;

    // Parse standard SPDX identifiers
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner: match s {
                "CC0-1.0" => LicenseVariant::CreativeCommonsZero10,
                "CC-BY-4.0" => LicenseVariant::CreativeCommonsCc40,
                l => LicenseVariant::Other(l.to_string()),
            },
        })
    }
}

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl AsRef<str> for License {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl License {
    /// Get license as an `&str` containing the SPDX identifier
    pub fn as_str(&self) -> &str {
        match self.inner {
            LicenseVariant::CreativeCommonsCc40 => "CC-BY-4.0",
            LicenseVariant::CreativeCommonsZero10 => "CC0-1.0",
            LicenseVariant::Other(ref l) => l,
        }
    }

    /// Is this an unknown license?
    pub fn is_other(&self) -> bool {
        matches!(self.inner, LicenseVariant::Other(_))
    }

    /// Is this a CC0-1.0 license?
    pub fn is_cc0_10(&self) -> bool {
        matches!(self.inner, LicenseVariant::CreativeCommonsZero10)
    }

    /// Is this a CC-BY-4.0 license?
    pub fn is_cc_by_40(&self) -> bool {
        matches!(self.inner, LicenseVariant::CreativeCommonsCc40)
    }
}
