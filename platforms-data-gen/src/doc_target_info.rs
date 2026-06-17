//! Parses the contents of the Markdown file
//! https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md
//! to extract platform tiers and notes.
//!
//! There is extra information contained there like std support that we currently do not parse;
//! it might be added in the future.

use std::collections::HashMap;

use comrak::nodes::NodeValue;
use comrak::{format_commonmark, parse_document, Arena, Options};

/// Information about a given target triple extracted from tier documentation located at
/// https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocTargetInfo {
    pub tier: u8,
    pub notes: String,
}

#[must_use]
pub fn parse_file(input: &str) -> HashMap<String, DocTargetInfo> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.footnotes = true;
    let root = parse_document(&arena, input, &options);

    let mut result = HashMap::<String, DocTargetInfo>::new();
    let mut current_tier = None;
    let mut found_table = false;

    for node in root.children() {
        match node.data.borrow().value {
            // Headings delineate the support tiers
            NodeValue::Heading(_) => {
                let Some(inner) = node.first_child().map(|n| n.data.borrow()) else {
                    continue;
                };

                let NodeValue::Text(s) = &inner.value else {
                    continue;
                };

                let Some(rest) = s.trim().strip_prefix("Tier ") else {
                    continue;
                };

                let Some(digit) = rest.chars().next().and_then(|c| c.to_digit(10)) else {
                    continue;
                };

                current_tier = Some(digit as u8);
            }

            // Locate and parse the tables describing architectures and tiers
            NodeValue::Table(_) => {
                let Some(tier) = current_tier else { continue };

                let mut rows = node.children();
                let Some(header) = rows.next() else {
                    continue;
                };

                let (mut target_column, mut notes_column) = (None, None);
                for (i, cell) in header.children().enumerate() {
                    let Some(inner) = cell.first_child().map(|n| n.data.borrow()) else {
                        continue;
                    };

                    let NodeValue::Text(s) = &inner.value else {
                        continue;
                    };

                    if s.trim() == "target" {
                        target_column = Some(i);
                    } else if s.trim() == "notes" {
                        notes_column = Some(i);
                    }
                }

                let (Some(target_column), Some(notes_column)) = (target_column, notes_column)
                else {
                    continue;
                };

                for row in rows {
                    let (mut target, mut notes) = (None, None);
                    for (i, cell) in row.children().enumerate() {
                        if i == target_column {
                            target = Some(cell);
                        } else if i == notes_column {
                            notes = Some(cell);
                        }
                    }

                    let (Some(mut target_nodes), Some(notes_nodes)) = (target, notes) else {
                        eprintln!("warning: skipping a row in tier {tier} table;");
                        eprintln!("         expected to find `target` and `notes` columns");
                        continue;
                    };

                    let target = loop {
                        let data = target_nodes.data.borrow();
                        match &data.value {
                            NodeValue::Code(inner) => break Some(inner.literal.clone()),
                            _ => match target_nodes.first_child() {
                                Some(n) => {
                                    drop(data);
                                    target_nodes = n;
                                }
                                None => break None,
                            },
                        }
                    };

                    let Some(target) = target else {
                        eprintln!("warning: skipping a row in tier {tier} table;");
                        eprintln!("         target name not found");
                        continue;
                    };

                    let mut notes = String::new();
                    for node in notes_nodes.children() {
                        if matches!(
                            node.data.borrow().value,
                            NodeValue::SoftBreak
                                | NodeValue::LineBreak
                                | NodeValue::FootnoteReference(_)
                        ) {
                            continue;
                        }

                        if let Err(error) = format_commonmark(node, &options, &mut notes) {
                            eprintln!(
                                "warning: failed to format notes for {target} in tier {tier}"
                            );
                            eprintln!("         {error}");
                            continue;
                        }

                        notes.truncate(notes.trim_end_matches(['\n', '\r']).len());
                    }

                    // The same target triple can appear several times in the documentation.
                    // For example, `i686-pc-windows-msvc` is both tier 1 and tier 3,
                    // with the tier 3 version being for Windows XP only.
                    // To deal with that, we only keep the first (highest) tier encountered.
                    notes.truncate(notes.trim_end().len());
                    let notes = notes.replace("\\_", "_").replace("\\-", "-");
                    result
                        .entry(target)
                        .or_insert(DocTargetInfo { tier, notes });
                }

                found_table = true;
            }
            _ => {}
        }
    }

    // Make sure the format hasn't changed drastically, and that we can still extract data
    assert!(
        found_table,
        "no `target | ... | notes` table found in the input"
    );

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = "
blah blah

## Some random header

## Tier 1 with Host Tools

target | notes
-------|-------
`aarch64-unknown-linux-gnu` | ARM64 Linux (kernel 4.2, glibc 2.17+) [^missing-stack-probes]
`i686-pc-windows-gnu` | 32-bit MinGW (Windows 7+)
`i686-pc-windows-msvc` | 32-bit MSVC (Windows 7+)

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
`i686-pc-windows-msvc` | * |  | 32-bit Windows XP support

blah blah I guess

[^missing-stack-probes]: blah
";

    #[test]
    fn test_doc_parser() {
        let result = parse_file(SAMPLE_DATA);
        assert_ne!(result, HashMap::new());
        assert_eq!(result["aarch64-unknown-linux-gnu"].tier, 1);
        assert_eq!(
            &result["aarch64-unknown-linux-gnu"].notes,
            "ARM64 Linux (kernel 4.2, glibc 2.17+)"
        );

        assert_eq!(result["i686-pc-windows-gnu"].tier, 1);
        assert_eq!(
            &result["i686-pc-windows-gnu"].notes,
            "32-bit MinGW (Windows 7+)"
        );

        assert_eq!(result["i686-pc-windows-msvc"].tier, 1);
        assert_eq!(
            &result["i686-pc-windows-msvc"].notes,
            "32-bit MSVC (Windows 7+)"
        );

        // Targets written as Markdown links should be parsed too, using the code span as the triple.
        assert_eq!(result["aarch64-apple-ios-sim"].tier, 2);
        assert_eq!(
            &result["aarch64-apple-ios-sim"].notes,
            "Apple iOS Simulator on ARM64"
        );
        assert_eq!(result["aarch64-kmc-solid_asp3"].tier, 3);
        assert_eq!(
            &result["aarch64-kmc-solid_asp3"].notes,
            "ARM64 SOLID with TOPPERS/ASP3"
        );
    }
}
