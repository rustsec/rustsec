//! Parses the Markdown file
//! https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md
//! to extract platform tiers and notes.
//! 
//! There is extra information contained there like std support that we currently do not parse;
//! it might be added in the future.
//! 
//! The file must be downloaded and a local copy to it must be provided.

use std::path::Path;

use regex::Regex;

const TABLE_HEADER_REGEX: &'static str = r"target\s+\|.*\s+notes";

fn do_the_thing(file: &Path) {
    let contents = std::fs::read_to_string(file).unwrap();
    let parsed_contents = parse_file(&contents);
}

fn parse_file(raw: &str) {
    let table_header_regex = Regex::new(TABLE_HEADER_REGEX).unwrap();
    for section in raw.split("\n## Tier") {
        
    }
}

fn parse_table_line() {

}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = "
blah blah

## Tier 1 with Host Tools

target | notes
-------|-------
`aarch64-unknown-linux-gnu` | ARM64 Linux (kernel 4.2, glibc 2.17+) [^missing-stack-probes]
`i686-pc-windows-gnu` | 32-bit MinGW (Windows 7+)

## Tier 1

no useful data here

## Tier 2 with Host Tools

**NOTE:** Tier 2 targets currently do not build the `rust-docs` component.

target | notes
-------|-------
`aarch64-apple-darwin` | ARM64 macOS (11.0+, Big Sur+)
`aarch64-pc-windows-msvc` | ARM64 Windows MSVC

## Tier 2

The `std` column in the table below has the following meanings:

* ✓ indicates the full standard library is available.
* \\* indicates the target only supports [`no_std`] development.

[`no_std`]: https://rust-embedded.github.io/book/intro/no-std.html

**NOTE:** Tier 2 targets currently do not build the `rust-docs` component.

target | std | notes
-------|:---:|-------
`aarch64-apple-ios` | ✓ | ARM64 iOS
[`aarch64-apple-ios-sim`](platform-support/aarch64-apple-ios-sim.md) | ✓ | Apple iOS Simulator on ARM64
`aarch64-fuchsia` | ✓ | ARM64 Fuchsia

## Tier 3

The `std` column in the table below has the following meanings:

* ✓ indicates the full standard library is available.
* \\* indicates the target only supports [`no_std`] development.
* ? indicates the standard library support is unknown or a work-in-progress.

[`no_std`]: https://rust-embedded.github.io/book/intro/no-std.html

The `host` column indicates whether the codebase includes support for building
host tools.

target | std | host | notes
-------|:---:|:----:|-------
`aarch64-apple-ios-macabi` | ? |  | Apple Catalyst on ARM64
`aarch64-apple-tvos` | * |  | ARM64 tvOS
[`aarch64-kmc-solid_asp3`](platform-support/kmc-solid.md) | ✓ |  | ARM64 SOLID with TOPPERS/ASP3

blah blah I guess
";

    #[test]
    fn test_header_regex() {
        let table_header_regex = Regex::new(TABLE_HEADER_REGEX).unwrap();
        let found: Vec<&str> = table_header_regex.find_iter(SAMPLE_DATA).map(|m| m.as_str() ).collect();
        let expected = [
            "target | notes",
            "target | notes",
            "target | std | notes",
            "target | std | host | notes", 
        ];
        assert_eq!(found, expected);
    }
}