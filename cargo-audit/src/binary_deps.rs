//! Extracts dependencies from binary files, using one of two ways:
//! 1. Recovers the dependency list embedded by `cargo auditable` (using `auditable-info`)
//! 2. Failing that, recovers as many crates as possible from panic messages (using `quitters`)

use std::{path::Path, str::FromStr};

use cargo_lock::{Lockfile, Package};
use rustsec::{Error, ErrorKind};

/// Load the dependency tree from a binary file
pub fn load_deps_from_binary(binary_path: &Path) -> rustsec::Result<Lockfile> {
    // Read the input
    let stuff = if binary_path == Path::new("-") {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        auditable_info::audit_info_from_reader(&mut handle, Default::default())
    } else {
        auditable_info::audit_info_from_file(binary_path, Default::default())
    };

    // The error handling boilerplate is in here instead of the `rustsec` crate because as of this writing
    // the public APIs of the crates involved are still somewhat unstable,
    // and this way we don't expose the error types in any public APIs
    use auditable_info::Error::*; // otherwise rustfmt makes the matches multiline and unreadable
    match stuff {
        Ok(json_struct) => Ok(cargo_lock::Lockfile::try_from(&json_struct)?),
        Err(e) => match e {
            NoAuditData => Err(Error::new(ErrorKind::NotFound, &e.to_string())),
            Io(_) => Err(Error::new(ErrorKind::Io, &e.to_string())),
            // Everything else is just Parse, but we enumerate them explicitly in case variant list changes
            InputLimitExceeded => Err(Error::new(ErrorKind::Parse, &e.to_string())),
            OutputLimitExceeded => Err(Error::new(ErrorKind::Parse, &e.to_string())),
            BinaryParsing(_) => Err(Error::new(ErrorKind::Parse, &e.to_string())),
            Decompression(_) => Err(Error::new(ErrorKind::Parse, &e.to_string())),
            Json(_) => Err(Error::new(ErrorKind::Parse, &e.to_string())),
            Utf8(_) => Err(Error::new(ErrorKind::Parse, &e.to_string())),
        },
    }
}

fn deps_from_panic_messages(binary_path: &Path, data: &[u8]) -> Option<Lockfile> {
    let deps = quitters::versions(data);
    if !deps.is_empty() {
        let mut packages: Vec<Package> = deps.into_iter().map(to_package).collect();
        let root_package = fake_root_package(binary_path, &packages);
        packages.push(root_package.clone());
        Some(Lockfile {
            version: cargo_lock::ResolveVersion::V2,
            packages,
            root: Some(root_package),
            metadata: Default::default(),
            patch: Default::default(),
        })
    } else {
        None
    }
}

fn fake_root_package(binary_path: &Path, other_packages: &[Package]) -> Package {
        // .file_name() can only return error if the name ends in /..,
        // which is a directory and would have errored out earlier
        let filename = binary_path.file_name().unwrap().to_string_lossy();
        // make up a version for the root package so that we have something to show in the tree view
        let fake_version = cargo_lock::Version::parse("0.0.0").unwrap();
        Package {
            // we shamelessly rely on the fact that cargo-lock crate doesn't actually run any checks here
            name: cargo_lock::Name::from_str(&filename).unwrap(),
            version: fake_version,
            source: None,
            checksum: None,
            dependencies: other_packages.iter().map(|p| p.into()).collect(),
            replace: None,
        }
}

// matches https://docs.rs/cargo-lock/8.0.2/src/cargo_lock/package/source.rs.html#19
// to signal crates.io to the `cargo-lock` crate
const CRATES_IO_INDEX: &str = "https://github.com/rust-lang/crates.io-index";

fn to_package(quitter: (&str, cargo_lock::Version)) -> Package {
    Package {
        // The `quitters` crate already ensures the name is valid, so we can just `.unwrap()` here
        name: cargo_lock::Name::from_str(quitter.0).unwrap(),
        version: quitter.1,
        // we can't know the exact registry, but by default `cargo audit` will
        // only scan crates from crates.io, so assume they're from there
        source: Some(cargo_lock::package::SourceId::from_url(CRATES_IO_INDEX).unwrap()),
        checksum: None,
        dependencies: Vec::new(),
        replace: None,
    }
}
