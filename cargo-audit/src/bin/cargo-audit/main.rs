//! Main entry point for `cargo audit`

#![forbid(unsafe_code)]

use std::process::ExitCode;

use cargo_audit::error::Error;

fn main() -> Result<ExitCode, Error> {
    cargo_audit::application::boot()
}
