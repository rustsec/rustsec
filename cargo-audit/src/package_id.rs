// ! Package identifiers.
// !
// ! Adapted from Cargo's `package_id_spec.rs`:
// !
// ! <https://github.com/rust-lang/cargo/blob/master/src/cargo/core/package_id_spec.rs>
// !
// ! Copyright (c) 2014 The Rust Project Developers
// ! Licensed under the same terms as the `cargo-lock` crate: Apache 2.0 + MIT

use rustsec::{Error, ErrorKind, package::Name};
use serde::{Serialize, Deserialize};
use url::Url;
use semver::Version;
use std::str::FromStr;

/// Contains the information to identify a target package provided on cli by user
#[derive(Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct PackageIdSpec {
    /// The name of the target package provided on cli
    pub name: Name,
    
    /// The version of the target package provided on cli
    pub version: Option<Version>,
    
    /// The url of the target package provided on cli
    pub url: Option<Url>,
}

impl PackageIdSpec {
    /// Parses a spec string and returns a `PackageIdSpec` if the string was valid.
    ///
    /// # Examples
    /// Some examples of valid strings
    ///
    /// ```
    /// use cargo_audit::package_id::PackageIdSpec;
    ///
    /// let specs = vec![
    ///     "https://crates.io/foo",
    ///     "https://crates.io/foo#1.2.3",
    ///     "https://crates.io/foo#bar:1.2.3",
    ///     "https://crates.io/foo#bar@1.2.3",
    ///     "foo",
    ///     "foo:1.2.3",
    ///     "foo@1.2.3",
    /// ];
    /// for spec in specs {
    ///     assert!(PackageIdSpec::parse(spec).is_ok());
    /// }
    /// ```
    pub fn parse(spec: &str) -> rustsec::Result<PackageIdSpec> {
        if spec.contains("://") {
            if let Ok(url) = Url::parse(spec).map_err(|s| format!("Invalid url `{}`: {}", spec, s)) {
                return PackageIdSpec::from_url(url);
            }
        } else if spec.contains('/') || spec.contains('\\') {
            let abs = std::env::current_dir().unwrap_or_default().join(spec);
            if abs.exists() {
                let maybe_url = Url::from_file_path(abs)
                    .map_or_else(|_| "a file:// URL".to_string(), |url| url.to_string());
                return Err(Error::new(
                    ErrorKind::PackageIdSpec,
                    &format!("package ID specification `{}` looks like a file path, maybe try {}", spec, maybe_url)
                ));
            }
        }
        let mut parts = spec.splitn(2, [':', '@']);
        let name = parts.next().unwrap();
        let version = match parts.next() {
            Some(version) => Some(Self::to_semver(version)?),
            None => None,
        };

        Ok(PackageIdSpec {
            name: Name::from_str(name)?,
            version,
            url: None,
        })
    }

    /// Tries to convert a valid `Url` to a `PackageIdSpec`.
    fn from_url(mut url: Url) -> rustsec::Result<PackageIdSpec> {
        if url.query().is_some() {
            return Err(Error::new(ErrorKind::PackageIdSpec, &format!("cannot have a query string in a pkgid: {}", url)))
        }
        let frag = url.fragment().map(|s| s.to_owned());
        url.set_fragment(None);
        let (name, version) = {
            let mut path = url
                .path_segments()
                .ok_or_else(|| Error::new(ErrorKind::PackageIdSpec, &format!("pkgid urls must have a path: {}", url)))?;
            let path_name = path.next_back().ok_or_else(|| {
                Error::new(
                    ErrorKind::PackageIdSpec,
                    &format!("pkgid urls must have at least one path component: {}", url)
                )
            })?;
            match frag {
                Some(fragment) => {
                    let mut parts = fragment.splitn(2, [':', '@']);
                    let name_or_version = parts.next().unwrap();
                    match parts.next() {
                        Some(part) => {
                            let version = Self::to_semver(part)?;
                            (name_or_version.to_string(), Some(version))
                        }
                        None => {
                            if name_or_version.chars().next().unwrap().is_alphabetic() {
                                (name_or_version.to_string(), None)
                            } else {
                                let version = Self::to_semver(name_or_version)?;
                                (path_name.to_string(), Some(version))
                            }
                        }
                    }
                }
                None => (path_name.to_string(), None),
            }
        };

        Ok(PackageIdSpec {
            name: Name::from_str(name.as_str())?,
            version,
            url: Some(url),
        })
    }

    fn to_semver(version: &str) -> rustsec::Result<Version> {
        match Version::parse(version.trim()) {
            Ok(v) => Ok(v),
            Err(..) => Err(
                Error::new(
                    ErrorKind::Version,
                    &format!("cannot parse '{}' as a semver", version)
                )
            ),
        }
    }
}