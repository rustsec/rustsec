#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa_core::testing::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RUNNER: CmdRunner = CmdRunner::default();
}

#[test]
fn version_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner.arg("version").capture_stdout().run();
    cmd.stdout().expect_regex(r"\Arustsec-cli [\d\.\-]+\z");
}
