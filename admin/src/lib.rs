//! `rustsec-admin` CLI application
//!
//! Administrative tool for the RustSec Advisory Database

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

pub mod application;
pub mod assigner;
pub mod commands;
pub mod config;
pub mod error;
pub mod linter;
pub mod list_versions;
pub mod lock;
pub mod osv_export;
pub mod prelude;
pub mod synchronizer;
pub mod web;

use std::{collections::BTreeMap as Map, error::Error as StdError};

use tame_index::{SparseIndex, index::AsyncRemoteSparseIndex};

/// Get an async crates.io index
pub fn crates_index() -> Result<AsyncRemoteSparseIndex, tame_index::Error> {
    Ok(AsyncRemoteSparseIndex::new(
        SparseIndex::new(tame_index::IndexLocation::new(
            tame_index::IndexUrl::crates_io(None, None, None)?,
        ))?,
        tame_index::external::reqwest::ClientBuilder::new()
            .build()
            .map_err(tame_index::Error::from)?,
    ))
}

/// Displays the error and also follows the chain of the `.source` fields,
/// printing any errors that caused the top-level error.
///
/// This is required to properly present some `gix` errors to the user:
/// <https://github.com/rustsec/rustsec/issues/1029#issuecomment-1777487808>
pub fn display_err_with_source(error: &impl StdError) -> String {
    display_error_chain::DisplayErrorChain::new(error).to_string()
}
