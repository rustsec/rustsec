//! Cargo.lock-related utilities

use crate::prelude::*;
use std::process::{exit, Command};

/// Run `cargo generate-lockfile`
pub fn generate() {
    let status = Command::new("cargo")
        .arg("generate-lockfile")
        .status()
        .unwrap_or_else(|e| {
            status_err!("couldn't run `cargo generate-lockfile`: {}", e);
            exit(1);
        });

    if !status.success() {
        if let Some(code) = status.code() {
            status_err!(
                "non-zero exit status running `cargo generate-lockfile`: {}",
                code
            );
        } else {
            status_err!("no exit status running `cargo generate-lockfile`!");
        }

        exit(1);
    }
}
