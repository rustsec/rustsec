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
    Checksum, Dependency, Error, Metadata, Name, Package, Patch, Result, SourceId, Version,
    metadata,
};
use serde::{Deserialize, Serialize, de, ser};
use std::{fmt, fmt::Write, str::FromStr};

impl<'de> Deserialize<'de> for Lockfile {
    fn deserialize<D: de::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        EncodableLockfile::deserialize(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

impl Serialize for Lockfile {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
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
    #[serde(default, skip_serializing_if = "EncodablePatch::is_empty")]
    pub(super) patch: EncodablePatch,
}

/// Unused patches need the same version-aware source decoding as packages.
#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct EncodablePatch {
    #[serde(default)]
    unused: Vec<EncodablePackage>,
}

impl EncodablePatch {
    fn is_empty(&self) -> bool {
        self.unused.is_empty()
    }
}

impl EncodableLockfile {
    /// Attempt to find a checksum for a package in a V1 lockfile
    fn find_checksum(&self, package: &Package) -> Option<Checksum> {
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

    fn try_from(raw_lockfile: EncodableLockfile) -> Result<Self> {
        let version = match raw_lockfile.version {
            Some(n) => n.try_into()?,
            None => ResolveVersion::detect(&raw_lockfile.package, &raw_lockfile.metadata)?,
        };

        let mut packages = Vec::with_capacity(raw_lockfile.package.len());

        for raw_package in &raw_lockfile.package {
            packages.push(if version == ResolveVersion::V1 {
                // In the V1 format, all dependencies are fully qualified with
                // their versions, but their checksums are stored in metadata.
                let mut pkg = raw_package.resolve(&[], version)?;
                pkg.checksum = raw_lockfile.find_checksum(&pkg);
                pkg
            } else {
                // In newer versions, we may need to look up dependency versions
                // from the other packages listed in the lockfile
                raw_package.resolve(&raw_lockfile.package, version)?
            })
        }

        Ok(Self {
            version,
            packages,
            root: raw_lockfile
                .root
                .as_ref()
                .map(|root| root.resolve(&[], version))
                .transpose()?,
            metadata: raw_lockfile.metadata,
            patch: Patch {
                unused: raw_lockfile
                    .patch
                    .unused
                    .iter()
                    .map(|package| {
                        package
                            .resolve(&[], version)
                            .map(|pkg| Dependency::from(&pkg))
                    })
                    .collect::<Result<_>>()?,
            },
        })
    }
}

impl From<&Lockfile> for EncodableLockfile {
    fn from(lockfile: &Lockfile) -> Self {
        let mut packages = Vec::with_capacity(lockfile.packages.len());
        let mut metadata = lockfile.metadata.clone();

        for package in &lockfile.packages {
            let mut raw_pkg = EncodablePackage::from_package(package, lockfile.version);
            let checksum_key = metadata::MetadataKey::for_checksum(&Dependency::from(package));

            if lockfile.version == ResolveVersion::V1 {
                // In the V1 format, we need to remove the checksum from
                // packages and add it to metadata
                if let Some(checksum) = raw_pkg.checksum.take() {
                    let value = checksum
                        .to_string()
                        .parse::<metadata::MetadataValue>()
                        .unwrap();
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

        Self {
            version,
            package: packages,
            root: lockfile
                .root
                .as_ref()
                .map(|root| EncodablePackage::from_package(root, lockfile.version)),
            metadata,
            patch: EncodablePatch {
                unused: lockfile
                    .patch
                    .unused
                    .iter()
                    .map(|dep| EncodablePackage {
                        name: dep.name.clone(),
                        version: dep.version.clone(),
                        source: dep
                            .source
                            .clone()
                            .and_then(|id| encodable_source_id(id, lockfile.version)),
                        checksum: None,
                        dependencies: vec![],
                        replace: None,
                    })
                    .collect(),
            },
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
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

        if let Some(value) = toml.get("version") {
            if let Some(version) = value.as_integer() {
                if version >= 3 {
                    writeln!(out, "version = {version}").unwrap();
                }
            }
        }

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
            out.push_str(&toml::to_string_pretty(&meta).unwrap());
        }

        // Trim redundant newlines
        while out.ends_with("\n\n") {
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
    writeln!(out, "name = {}", dep["name"]).unwrap();
    writeln!(out, "version = {}", dep["version"]).unwrap();

    if dep.contains_key("source") {
        writeln!(out, "source = {}", dep["source"]).unwrap();
    }
    if dep.contains_key("checksum") {
        writeln!(out, "checksum = {}", dep["checksum"]).unwrap();
    }

    if let Some(s) = dep.get("dependencies") {
        let slice = s.as_array().unwrap();

        if !slice.is_empty() {
            out.push_str("dependencies = [\n");

            for child in slice.iter() {
                writeln!(out, " {child},").unwrap();
            }

            out.push_str("]\n");
        }
    } else if dep.contains_key("replace") {
        writeln!(out, "replace = {}", dep["replace"]).unwrap();
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
    pub(super) source: Option<EncodableSourceId>,

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
    fn resolve(&self, packages: &[Self], version: ResolveVersion) -> Result<Package> {
        let mut dependencies = Vec::with_capacity(self.dependencies.len());

        for dep in &self.dependencies {
            dependencies.push(dep.resolve(packages, version)?);
        }

        Ok(Package {
            name: self.name.clone(),
            version: self.version.clone(),
            source: self
                .source
                .as_ref()
                .map(|s| s.resolve(version))
                .transpose()?,
            checksum: self.checksum.clone(),
            dependencies,
            replace: self
                .replace
                .as_ref()
                .map(|rep| rep.resolve(&[], version))
                .transpose()?,
        })
    }

    /// Prepare `ResolveVersion::V2` dependencies by removing ones which are unambiguous
    fn v2_deps(&mut self, packages: &[Package]) {
        for dependency in &mut self.dependencies {
            dependency.v2(packages);
        }
    }

    fn from_package(package: &Package, version: ResolveVersion) -> Self {
        Self {
            name: package.name.clone(),
            version: package.version.clone(),
            source: package
                .source
                .clone()
                .and_then(|id| encodable_source_id(id, version)),
            checksum: package.checksum.clone(),
            dependencies: package
                .dependencies
                .iter()
                .map(|dep| EncodableDependency::from_dependency(dep, version))
                .collect::<Vec<_>>(),
            replace: package
                .replace
                .as_ref()
                .map(|rep| EncodableDependency::from_dependency(rep, version)),
        }
    }
}

fn encodable_source_id(id: SourceId, version: ResolveVersion) -> Option<EncodableSourceId> {
    if id.is_path() {
        None
    } else {
        Some(EncodableSourceId(
            id.as_url(version >= ResolveVersion::V4).to_string(),
        ))
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
    pub(super) source: Option<EncodableSourceId>,
}

impl EncodableDependency {
    /// Resolve this dependency, which in the V2 format may be abbreviated to
    /// prevent merge conflicts
    fn resolve(
        &self,
        packages: &[EncodablePackage],
        version: ResolveVersion,
    ) -> Result<Dependency> {
        for pkg in packages {
            if pkg.name == self.name
                && (self.version.is_none() || self.version.as_ref() == Some(&pkg.version))
                && self.source.is_none()
            {
                return Ok(Dependency {
                    name: pkg.name.clone(),
                    version: pkg.version.clone(),
                    source: pkg
                        .source
                        .as_ref()
                        .map(|s| s.resolve(version))
                        .transpose()?,
                });
            }
        }

        let package_version = self
            .version
            .clone()
            .ok_or_else(|| Error::Parse(format!("couldn't resolve dependency: {}", self.name)))?;

        Ok(Dependency {
            name: self.name.clone(),
            version: package_version,
            source: self
                .source
                .as_ref()
                .map(|s| s.resolve(version))
                .transpose()?,
        })
    }

    /// Prepare `ResolveVersion::V2` dependencies by removing ones which are unambiguous
    fn v2(&mut self, packages: &[Package]) {
        let mut matching = vec![];

        for package in packages {
            if package.name == self.name {
                matching.push(package);
            }
        }

        if matching.len() == 1 {
            // Unambiguous match by name, no need to specify version and source
            self.version = None;
            self.source = None;
            return;
        }

        let Some(version) = self.version.as_ref() else {
            // Version was already removed. This is unexpected. Maybe this function
            // was already called before?
            return;
        };

        if matching
            .iter()
            .filter(|package| &package.version == version)
            .count()
            == 1
        {
            // Unambiguous match by name and version, no need to specify source
            self.source = None;
        }
    }

    fn from_dependency(dep: &Dependency, version: ResolveVersion) -> Self {
        Self {
            name: dep.name.clone(),
            version: Some(dep.version.clone()),
            source: dep
                .source
                .clone()
                .and_then(|id| encodable_source_id(id, version)),
        }
    }
}

/// Note: this only works for `ResolveVersion::V1` dependencies.
impl FromStr for EncodableDependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();

        let name = parts
            .next()
            .ok_or_else(|| Error::Parse("empty dependency string".to_owned()))?
            .parse()?;

        let version = parts.next().map(FromStr::from_str).transpose()?;

        let source = parts
            .next()
            .map(|s| {
                if s.len() < 2 || !s.starts_with('(') || !s.ends_with(')') {
                    Err(Error::Parse(format!("malformed source in dependency: {s}")))
                } else {
                    Ok(EncodableSourceId(s[1..(s.len() - 1)].to_owned()))
                }
            })
            .transpose()?;

        if parts.next().is_some() {
            return Err(Error::Parse(format!("malformed dependency: {s}")));
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
        write!(f, "{}", self.name)?;

        if let Some(version) = &self.version {
            write!(f, " {version}")?;
        }

        if let Some(source) = &self.source {
            write!(f, " ({})", source.0)?;
        }

        Ok(())
    }
}

/// Note: this only works for `ResolveVersion::V1` dependencies.
impl TryFrom<&EncodableDependency> for Dependency {
    type Error = Error;

    fn try_from(raw_dependency: &EncodableDependency) -> Result<Self> {
        raw_dependency.resolve(&[], ResolveVersion::V1)
    }
}

impl<'de> Deserialize<'de> for EncodableDependency {
    fn deserialize<D: de::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

impl Serialize for EncodableDependency {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

/// Keep the source URL intact until the lockfile version is known.
#[derive(Deserialize, Serialize, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
#[serde(transparent)]
pub(super) struct EncodableSourceId(String);

impl EncodableSourceId {
    fn resolve(&self, version: ResolveVersion) -> Result<SourceId> {
        SourceId::from_url_with_encoding(&self.0, version >= ResolveVersion::V4)
    }
}
