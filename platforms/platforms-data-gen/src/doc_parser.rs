//! Parses the Markdown file
//! https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md
//! to extract platform tiers and notes.
//!
//! There is extra information contained there like std support that we currently do not parse;
//! it might be added in the future.
//!
//! The file must be downloaded and a local copy to it must be provided.

use std::{path::Path, collections::HashMap};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocTargetInfo {
    tier: u8,
    notes: String,
}

const TABLE_HEADER_REGEX: &'static str = r"target\s+\|.*\s+notes";

fn do_the_thing(file: &Path) -> HashMap<String, DocTargetInfo> {
    let contents = std::fs::read_to_string(file).unwrap();
    parse_file(&contents)
}

#[must_use]
fn parse_file(input: &str) -> HashMap<String, DocTargetInfo> {
    // compile the regex once outside the loop; not that it really matters
    let table_header_regex = Regex::new(TABLE_HEADER_REGEX).unwrap();
    assert!(table_header_regex.is_match(input));

    let mut result: HashMap<String, DocTargetInfo> = HashMap::new();

    // find the headers delineating the support tiers
    let section_headers = section_headers(input);
    let sections = sections(input);

    // Locate and parse the tables describing architectures and tiers
    for (section, header) in section_headers.iter().zip(sections) {
        // There are no Tier 1 platforms without host tools, so that header does not contain a table
        if let Some(table_header) = table_header_regex.find(section) {
            let after_table_header = &section[table_header.end()..];
            for table_row in after_table_header
                .lines()
                .skip(2) // skip the table header and the separator line
                .take_while(|l| l.contains("|")) // read until the end of the table
            {
                let (arch, notes) = parse_table_row(table_row);
                let target_info = DocTargetInfo {
                    tier: header_to_tier(header),
                    notes: notes.to_string(),
                };
                result.insert(arch.to_string(), target_info);
            }
        }
    }

    result
}

const SECTION_HEADER_REGEX: &'static str = r"## ?Tier \d";

#[must_use]
fn section_headers(input: &str) -> Vec<&str> {
    let section_header_regex = Regex::new(SECTION_HEADER_REGEX).unwrap();
    assert!(section_header_regex.is_match(input));
    section_header_regex.find_iter(input).map(|m| m.as_str()).collect()
}

#[must_use]
fn sections(input: &str) -> Vec<&str> {
    let section_header_regex = Regex::new(SECTION_HEADER_REGEX).unwrap();
    assert!(section_header_regex.is_match(input));
    section_header_regex.split(input).skip(1).collect()
}

/// Accepts a table line string and returns the tuple of `(arch, notes)`
#[must_use]
fn parse_table_row(line: &str) -> (&str, &str) {
    let arch = line.split('`').nth(1).unwrap();
    let notes = &line[(line.rfind('|').unwrap() + 1)..].trim_matches(|c: char| c.is_whitespace());
    (arch, notes)
}

#[must_use]
fn header_to_tier(header: &str) -> u8 {
    header.chars().last().unwrap().to_digit(10).unwrap().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = "
blah blah

## Some random header

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
    fn test_table_header_regex() {
        let table_header_regex = Regex::new(TABLE_HEADER_REGEX).unwrap();
        let found: Vec<&str> = table_header_regex
            .find_iter(SAMPLE_DATA)
            .map(|m| m.as_str())
            .collect();
        let expected = [
            "target | notes",
            "target | notes",
            "target | std | notes",
            "target | std | host | notes",
        ];
        assert_eq!(found, expected);
    }

    #[test]
    fn test_row_parser() {
        assert_eq!(
            parse_table_row("`i686-pc-windows-gnu` | 32-bit MinGW (Windows 7+)"),
            ("i686-pc-windows-gnu", "32-bit MinGW (Windows 7+)")
        );
        assert_eq!(
            parse_table_row("[`aarch64-kmc-solid_asp3`](platform-support/kmc-solid.md) | ✓ |  | ARM64 SOLID with TOPPERS/ASP3"),
            ("aarch64-kmc-solid_asp3", "ARM64 SOLID with TOPPERS/ASP3")
        );
    }

    #[test]
    fn test_section_parser() {
        assert_eq!(section_headers(SAMPLE_DATA).len(), 5);
        assert_eq!(sections(SAMPLE_DATA).len(), 5);
    }

    #[test]
    fn test_doc_parser() {
        let result = parse_file(SAMPLE_DATA);
        assert_ne!(result, HashMap::new());
        assert_eq!(result["aarch64-unknown-linux-gnu"].tier, 1);
        assert_eq!(&result["aarch64-unknown-linux-gnu"].notes, "ARM64 Linux (kernel 4.2, glibc 2.17+) [^missing-stack-probes]");

        assert_eq!(result["i686-pc-windows-gnu"].tier, 1);
        assert_eq!(&result["i686-pc-windows-gnu"].notes, "32-bit MinGW (Windows 7+)");
    }
}
