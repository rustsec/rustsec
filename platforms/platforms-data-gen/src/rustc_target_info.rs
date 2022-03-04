//! Invokes `rustc --print=cfg` to get info about a given target and parses its output.

use std::{io::BufRead, process::Child};

pub(crate) type RustcTargetInfo = std::collections::HashMap<String, String>;
pub(crate) type RustcTargetsInfo = Vec<RustcTargetInfo>;
pub(crate) type TargetTriple = String;

/// Returns a list of all known target triples.
/// Obtains it by invoking `rustc --print=target-list`
pub(crate) fn target_triples() -> Vec<TargetTriple> {
    std::process::Command::new("rustc")
        .arg("--print=target-list")
        .output()
        .expect("Failed to invoke rustc; make sure it's in $PATH")
        .stdout
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

/// Returns detailed information about all targets.
/// Since it invokes `rustc` for every target, it may take a while.
pub(crate) fn targets_info(triples: &[TargetTriple]) -> RustcTargetsInfo {
    // Spawn all queries at once to make use of all available cores.
    // No it's not premature optimization, it lets me iterate faster okay?
    let child_processes: Vec<Child> = triples
        .iter()
        .map(|t| spawn_rustc_target_info_query(t))
        .collect();
    child_processes
        .into_iter()
        .map(|c| {
            let output = c.wait_with_output().unwrap();
            assert_eq!(output.status.code(), Some(0));
            parse_rustc_target_info(&output.stdout)
        })
        .collect()
}

fn spawn_rustc_target_info_query(target_triple: &str) -> Child {
    std::process::Command::new("rustc")
        .arg("--print=cfg")
        .arg(format!("--target={}", target_triple)) //not being parsed by the shell, so not a vulnerability
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to invoke rustc; make sure it's in $PATH")
}

fn parse_rustc_target_info(rustc_output: &[u8]) -> RustcTargetInfo {
    // Decoupled from `rustc_target_info` to allow unit testing
    rustc_output
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            // rustc outputs some free-standing values as well as key-value pairs
            // we're only interested in the pairs, which are separated by '=' and the value is quoted
            if line.contains("=") {
                let key = line.split("=").nth(0).unwrap();
                let mut value: String = line.split("=").skip(1).collect();
                // strip first and last chars of the quoted value. Verify that they're quotes
                assert!(value.pop().unwrap() == '"');
                assert!(value.remove(0) == '"');
                Some((key.to_owned(), value))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rustc_parser_linux() {
        let rustc_output = br#"debug_assertions
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="fxsr"
target_feature="sse"
target_feature="sse2"
target_os="linux"
target_pointer_width="64"
target_vendor="unknown"
unix
"#;
        let result = parse_rustc_target_info(rustc_output);
        assert_eq!(result.get("target_arch").unwrap(), "x86_64");
        assert_eq!(result.get("target_endian").unwrap(), "little");
        assert_eq!(result.get("target_pointer_width").unwrap(), "64");
        assert_eq!(result.get("target_vendor").unwrap(), "unknown");
    }
}
