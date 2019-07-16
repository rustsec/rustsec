//! Audit Cargo.lock files for crates containing security vulnerabilities

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate abscissa_core;

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
mod prelude;
