use std::{env, process::Command};

pub(crate) fn cargo_audit() -> Command {
    // A cached test executable may have been built for a different target directory.
    let executable = env::var_os("CARGO_BIN_EXE_cargo-audit")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_cargo-audit").into());
    Command::new(executable)
}
