//! serde-based `Cargo.lock` parser/serializer
//!
//! Customized to allow pre/postprocessing to detect and serialize both
//! the V1 vs V2 formats and ensure the end-user is supplied a consistent
//! representation regardless of which version is in use.
//!
//! Parts adapted from upstream Cargo.
//! Cargo is primarily distributed under the terms of both the MIT license and
//! the Apache License (Version 2.0).

use super::{Lockfile, ResolveVersion};
use crate::{
    metadata, Checksum, Dependency, Error, ErrorKind, Metadata, Name, Package, Patch, SourceId,
    Version,
};
use serde::{de, ser, Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

impl<'de> Deserialize<'de> for Lockfile {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw_lockfile = EncodableLockfile::deserialize(deserializer)?;
        raw_lockfile.try_into().map_err(de::Error::custom)
    }
}

impl Serialize for Lockfile {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        EncodableLockfile::from(self).serialize(serializer)
    }
}

/// Serialization-oriented equivalent to [`Lockfile`]
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct EncodableLockfile {
    /// Lockfile version
    pub(super) version: Option<u32>,

    /// Packages in the lockfile
    #[serde(default)]
    pub(super) package: Vec<EncodablePackage>,

    /// Legacy root package (preserved for compatibility)
    pub(super) root: Option<EncodablePackage>,

    /// Metadata fields
    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub(super) metadata: Metadata,

    /// Patch section
    #[serde(default, skip_serializing_if = "Patch::is_empty")]
    pub(super) patch: Patch,
}

impl EncodableLockfile {
    /// Attempt to find a checksum for a package in a V1 lockfile
    pub fn find_checksum(&self, package: &Package) -> Option<Checksum> {
        for (key, value) in &self.metadata {
            if let Ok(dep) = key.checksum_dependency() {
                if dep.name == package.name && dep.version == package.version {
                    return value.checksum().ok();
                }
            }
        }

        None
    }
}

impl TryFrom<EncodableLockfile> for Lockfile {
    type Error = Error;

    fn try_from(raw_lockfile: EncodableLockfile) -> Result<Lockfile, Error> {
        let version = ResolveVersion::detect(&raw_lockfile.package, &raw_lockfile.metadata)?;
        let mut packages = Vec::with_capacity(raw_lockfile.package.len());

        for raw_package in &raw_lockfile.package {
            packages.push(if version == ResolveVersion::V1 {
                // In the V1 format, all dependencies are fully qualified with
                // their versions, but their checksums are stored in metadata.
                let mut pkg = Package::try_from(raw_package)?;
                pkg.checksum = raw_lockfile.find_checksum(&pkg);
                pkg
            } else {
                // In newer versions, we may need to look up dependency versions
                // from the other packages listed in the lockfile
                raw_package.resolve(&raw_lockfile.package)?
            })
        }

        Ok(Lockfile {
            version,
            packages,
            root: raw_lockfile
                .root
                .as_ref()
                .map(|root| root.try_into())
                .transpose()?,
            metadata: raw_lockfile.metadata,
            patch: raw_lockfile.patch,
        })
    }
}

impl From<&Lockfile> for EncodableLockfile {
    fn from(lockfile: &Lockfile) -> EncodableLockfile {
        let mut packages = Vec::with_capacity(lockfile.packages.len());
        let mut metadata = lockfile.metadata.clone();

        for package in &lockfile.packages {
            let mut raw_pkg = EncodablePackage::from(package);
            let checksum_key = metadata::Key::for_checksum(&Dependency::from(package));

            if lockfile.version == ResolveVersion::V1 {
                // In the V1 format, we need to remove the checksum from
                // packages and add it to metadata
                if let Some(checksum) = raw_pkg.checksum.take() {
                    let value = checksum.to_string().parse::<metadata::Value>().unwrap();
                    metadata.insert(checksum_key, value);
                }
            } else {
                // In newer versions, we need to remove the version/source from
                // unambiguous dependencies, and remove checksums from the
                // metadata table if present
                raw_pkg.v2_deps(&lockfile.packages);
                metadata.remove(&checksum_key);
            }

            packages.push(raw_pkg);
        }

        let version = if lockfile.version.is_explicit() {
            Some(lockfile.version.into())
        } else {
            None
        };

        EncodableLockfile {
            version,
            package: packages,
            root: lockfile.root.as_ref().map(|root| root.into()),
            metadata,
            patch: lockfile.patch.clone(),
        }
    }
}

impl ToString for EncodableLockfile {
    /// Adapted from `serialize_resolve` in upstream Cargo:
    /// <https://github.com/rust-lang/cargo/blob/0c70319/src/cargo/ops/lockfile.rs#L103-L174>
    fn to_string(&self) -> String {
        let toml = toml::Value::try_from(self).unwrap();
        let mut out = String::new();

        // At the start of the file we notify the reader that the file is generated.
        // Specifically Phabricator ignores files containing "@generated", so we use that.
        let marker_line = "# This file is automatically @generated by Cargo.";
        let extra_line = "# It is not intended for manual editing.";
        out.push_str(marker_line);
        out.push('\n');
        out.push_str(extra_line);
        out.push('\n');

        let deps = toml["package"].as_array().unwrap();
        for dep in deps {
            let dep = dep.as_table().unwrap();

            out.push_str("[[package]]\n");
            emit_package(dep, &mut out);
        }

        if let Some(patch) = toml.get("patch") {
            let list = patch["unused"].as_array().unwrap();
            for entry in list {
                out.push_str("[[patch.unused]]\n");
                emit_package(entry.as_table().unwrap(), &mut out);
                out.push('\n');
            }
        }

        if let Some(meta) = toml.get("metadata") {
            out.push_str("[metadata]\n");
            out.push_str(&meta.to_string());
        }

        // Trim redundant newlines
        if out.ends_with("\n\n") {
            out.pop();
        }

        out
    }
}

/// Emit a single package from a lockfile.
///
/// This method is adapted from the same-named method in upstream Cargo:
/// <https://github.com/rust-lang/cargo/blob/0c70319/src/cargo/ops/lockfile.rs#L194-L221>
fn emit_package(dep: &toml::value::Table, out: &mut String) {
    out.push_str(&format!("name = {}\n", &dep["name"]));
    out.push_str(&format!("version = {}\n", &dep["version"]));

    if dep.contains_key("source") {
        out.push_str(&format!("source = {}\n", &dep["source"]));
    }
    if dep.contains_key("checksum") {
        out.push_str(&format!("checksum = {}\n", &dep["checksum"]));
    }

    if let Some(s) = dep.get("dependencies") {
        let slice = s.as_array().unwrap();

        if !slice.is_empty() {
            out.push_str("dependencies = [\n");

            for child in slice.iter() {
                out.push_str(&format!(" {},\n", child));
            }

            out.push_str("]\n");
        }
    } else if dep.contains_key("replace") {
        out.push_str(&format!("replace = {}\n", &dep["replace"]));
    }

    out.push('\n');
}

/// Serialization-oriented equivalent to [`Package`]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct EncodablePackage {
    /// Package name
    pub(super) name: Name,

    /// Package version
    pub(super) version: Version,

    /// Source of a package
    pub(super) source: Option<SourceId>,

    /// Package checksum
    pub(super) checksum: Option<Checksum>,

    /// Package dependencies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) dependencies: Vec<EncodableDependency>,

    /// Replace directive
    pub(super) replace: Option<EncodableDependency>,
}

impl EncodablePackage {
    /// Resolve all of the dependencies of a package, which in the V2 format
    /// may be abbreviated to prevent merge conflicts
    fn resolve(&self, packages: &[EncodablePackage]) -> Result<Package, Error> {
        let mut dependencies = Vec::with_capacity(self.dependencies.len());

        for dep in &self.dependencies {
            dependencies.push(dep.resolve(packages)?);
        }

        Ok(Package {
            name: self.name.clone(),
            version: self.version.clone(),
            source: self.source.clone(),
            checksum: self.checksum.clone(),
            dependencies,
            replace: self
                .replace
                .as_ref()
                .map(|rep| rep.try_into())
                .transpose()?,
        })
    }

    /// Prepare `ResolveVersion::V2` dependencies by removing ones which are unambiguous
    fn v2_deps(&mut self, packages: &[Package]) {
        for dependency in &mut self.dependencies {
            dependency.v2(packages);
        }
    }
}

/// Note: this only works for `ResolveVersion::V1` dependencies.
impl TryFrom<&EncodablePackage> for Package {
    type Error = Error;

    fn try_from(raw_package: &EncodablePackage) -> Result<Package, Error> {
        raw_package.resolve(&[])
    }
}

impl From<&Package> for EncodablePackage {
    fn from(package: &Package) -> EncodablePackage {
        EncodablePackage {
            name: package.name.clone(),
            version: package.version.clone(),
            source: package.source.clone(),
            checksum: package.checksum.clone(),
            dependencies: package
                .dependencies
                .iter()
                .map(|dep| dep.into())
                .collect::<Vec<_>>(),
            replace: package.replace.as_ref().map(|rep| rep.into()),
        }
    }
}

/// Package dependencies
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct EncodableDependency {
    /// Name of the dependency
    pub(super) name: Name,

    /// Version of the dependency
    pub(super) version: Option<Version>,

    /// Source for the dependency
    pub(super) source: Option<SourceId>,
}

impl EncodableDependency {
    /// Resolve this dependency, which in the V2 format may be abbreviated to
    /// prevent merge conflicts
    pub fn resolve(&self, packages: &[EncodablePackage]) -> Result<Dependency, Error> {
        let mut version = None;
        let mut source = None;

        if let Some(v) = &self.version {
            version = Some(v.clone());
            source = self.source.clone();
        } else {
            for pkg in packages {
                if pkg.name == self.name {
                    if version.is_some() {
                        fail!(ErrorKind::Parse, "ambiguous dependency: {}", self.name);
                    }

                    version = Some(pkg.version.clone());
                    source = pkg.source.clone();
                }
            }
        };

        if version.is_none() {
            fail!(
                ErrorKind::Parse,
                "couldn't resolve dependency: {}",
                self.name
            );
        }

        Ok(Dependency {
            name: self.name.clone(),
            version: version.unwrap(),
            source,
        })
    }

    /// Prepare `ResolveVersion::V2` dependencies by removing ones which are unambiguous
    pub fn v2(&mut self, packages: &[Package]) {
        let mut matching = vec![];

        for package in packages {
            if package.name == self.name {
                matching.push(package);
            }
        }

        // TODO(tarcieri): better handle other cases?
        if matching.len() == 1 {
            self.version = None;
            self.source = None;
        }
    }
}

impl FromStr for EncodableDependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut parts = s.split_whitespace();

        let name = parts
            .next()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "empty dependency string"))?
            .parse()?;

        let version = parts.next().map(FromStr::from_str).transpose()?;

        let source = parts
            .next()
            .map(|s| {
                if s.len() < 2 || !s.starts_with('(') || !s.ends_with(')') {
                    Err(format_err!(
                        ErrorKind::Parse,
                        "malformed source in dependency: {}",
                        s
                    ))
                } else {
                    s[1..(s.len() - 1)].parse()
                }
            })
            .transpose()?;

        if parts.next().is_some() {
            fail!(ErrorKind::Parse, "malformed dependency: {}", s);
        }

        Ok(Self {
            name,
            version,
            source,
        })
    }
}

impl fmt::Display for EncodableDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)?;

        if let Some(version) = &self.version {
            write!(f, " {}", version)?;
        }

        if let Some(source) = &self.source {
            write!(f, " ({})", source)?;
        }

        Ok(())
    }
}

/// Note: this only works for `ResolveVersion::V1` dependencies.
impl TryFrom<&EncodableDependency> for Dependency {
    type Error = Error;

    fn try_from(raw_dependency: &EncodableDependency) -> Result<Dependency, Error> {
        raw_dependency.resolve(&[])
    }
}

impl From<&Dependency> for EncodableDependency {
    fn from(package: &Dependency) -> EncodableDependency {
        EncodableDependency {
            name: package.name.clone(),
            version: Some(package.version.clone()),
            source: package.source.clone(),
        }
    }
}

impl<'de> Deserialize<'de> for EncodableDependency {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Serialize for EncodableDependency {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}
