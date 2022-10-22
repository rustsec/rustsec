//! Extracts dependencies from binary files, using one of two ways:
//! 1. Recovers the dependency list embedded by `cargo auditable` (using `auditable-info`)
//! 2. Failing that, recovers as many crates as possible from panic messages (using `quitters`)

use std::{path::Path, str::FromStr};

use cargo_lock::{Lockfile, Package};
use rustsec::{ErrorKind, Error};

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