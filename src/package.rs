//! Rust packages enumerated in `Cargo.lock`

use semver::Version;
use serde::{de, ser, Deserialize, Serialize};
use std::fmt;

/// Information about a Rust package (as sourced from `Cargo.lock`)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Package {
    /// Name of a crate
    pub name: Name,

    /// Crate version (using `semver`)
    pub version: Version,

    /// Source of the crate
    pub source: Option<String>,

    /// Dependencies of this crate
    #[serde(
        default,
        serialize_with = "serialize_dependencies",
        deserialize_with = "deserialize_dependencies"
    )]
    pub dependencies: Vec<Package>,
}

/// Serialize a package in `[[package.dependencies]]`
pub(crate) fn serialize_dependencies<S>(
    dependencies: &[Package],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    dependencies
        .iter()
        .map(|dependency| {
            let mut dependency_string = format!("{} {}", &dependency.name, &dependency.version);

            if let Some(source) = &dependency.source {
                dependency_string = format!("{} ({})", dependency_string, source);
            }

            dependency_string
        })
        .collect::<Vec<_>>()
        .serialize(serializer)
}

/// Parse a package in `[[package.dependencies]]`
pub(crate) fn deserialize_dependencies<'de, D>(deserializer: D) -> Result<Vec<Package>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let dependencies = Vec::<String>::deserialize(deserializer)?;
    let mut result = vec![];

    for dependency_string in &dependencies {
        let mut parts = dependency_string.split_whitespace();
        let name_str = parts
            .next()
            .ok_or_else(|| de::Error::custom("empty dependency string"))?;

        let version_str = parts.next().ok_or_else(|| {
            de::Error::custom(format!(
                "missing version for dependency: {}",
                &dependency_string
            ))
        })?;

        let source = parts
            .next()
            .map(|s| {
                if s.len() < 2 || !s.starts_with('(') || !s.ends_with(')') {
                    Err(de::Error::custom(format!(
                        "malformed source in dependency: {}",
                        &dependency_string,
                    )))
                } else {
                    Ok(s[1..(s.len() - 1)].to_owned())
                }
            })
            .transpose()?;

        if parts.next().is_some() {
            Err(de::Error::custom(format!(
                "malformed dependency: {}",
                dependency_string,
            )))?;
        }

        result.push(Package {
            name: name_str.into(),
            version: version_str.parse().map_err(de::Error::custom)?,
            source,
            dependencies: vec![],
        });
    }

    Ok(result)
}

/// Name of a Rust `[[package]]`
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Name(String);

impl Name {
    /// Get string reference to this package name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<Name> for Name {
    fn as_ref(&self) -> &Name {
        self
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<&'a str> for Name {
    fn from(string: &'a str) -> Name {
        Name(string.into())
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}
