use std::io::BufRead;

fn main() {
    let triples = target_triples();
    // I am tempted to parallelize this loop like you wouldn't believe
    for triple in triples {
        let info = rustc_target_info(&triple);
        println!("{{
    target_triple: \"{}\",
    target_arch: \"{}\",
    target_os: \"{}\",
    target_env: \"{}\"
}};
", &triple, info["target_arch"], info["target_os"], info["target_env"]);
    }
}

pub type RustcTargetInfo = std::collections::HashMap<String, String>;

fn target_triples() -> Vec<String> {
    std::process::Command::new("rustc")
        .arg("--print=target-list")
        .output()
        .expect("Failed to invoke rustc; make sure it's in $PATH")
        .stdout
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn rustc_target_info(target_triple: &str) -> RustcTargetInfo {
    parse_rustc_target_info(&std::process::Command::new("rustc")
        .arg("--print=cfg")
        .arg(format!("--target={}", target_triple)) //not being parsed by the shell, so not a vulnerability
        .output()
        .expect("Failed to invoke rustc; make sure it's in $PATH")
        .stdout)
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
