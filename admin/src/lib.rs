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
pub mod osv_export;
pub mod prelude;
pub mod web;

use std::collections::BTreeMap as Map;
