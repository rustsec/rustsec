//! Utilities for generating enums from raw info about targets

use regex::{Captures, Regex};
use std::collections::BTreeSet;

use crate::rustc_target_info::RustcTargetsInfo;

#[must_use]
pub(crate) fn distinct_values(key: &str, info: &RustcTargetsInfo) -> BTreeSet<String> {
    info.iter().map(|t| &t[key]).cloned().collect()
}

#[must_use]
pub(crate) fn enumify_value(key: &str, value: &str) -> String {
    format!("{}::{}", to_enum_name(key), to_enum_variant_name(value))
}

#[must_use]
pub(crate) fn to_enum_name(key: &str) -> &'static str {
    match key {
        "target_arch" => "Arch",
        "target_os" => "OS",
        "target_env" => "Env",
        "tier" => "Tier",
        "target_endian" => "Endian",
        "target_pointer_width" => "PointerWidth",
        _ => unreachable!("unknown enum name: {}", key),
    }
}

#[must_use]
pub(crate) fn to_enum_variant_name(value: &str) -> String {
    let name = value.to_ascii_lowercase();
    match name.as_str() {
        "" => "None".to_owned(), // This is used for `Env` which is set to empty string by default
        // Numbers cannot be used as enum discriminants, so for `PointerWidth` enum we need this hack
        "16" => "U16".to_owned(),
        "32" => "U32".to_owned(),
        "64" => "U64".to_owned(),
        // list of exceptions to `Titlecase` enum naming from `platforms` v2.0, as gathered by
        // `rg --only-matching --no-filename --no-line-number '    [A-Z0-9][A-Za-z0-9]*,' | grep -v ' [A-Z][a-z0-9]\+,'`
        "aarch64" => "AArch64".to_owned(),
        "asmjs" => "AsmJs".to_owned(),
        "ios" => "iOS".to_owned(),
        "powerpc" => "PowerPc".to_owned(),
        "powerpc64" => "PowerPc64".to_owned(),
        "riscv" => "RiscV".to_owned(),
        "s390x" => "S390X".to_owned(),
        "thumbv6" => "ThumbV6".to_owned(),
        "thumbv7" => "ThumbV7".to_owned(),
        "uclibc" => "UClibc".to_owned(),
        "vxworks" => "VxWorks".to_owned(),
        // Things ending with "BSD", "OS" are handled below
        _ => {
            // Convert to `Titlecase` as per the Rust enum value convention
            let mut name = make_ascii_titlecase(&name);
            // Apply generalizable exceptions to `Titlecase`
            let len = name.len();
            if name.ends_with("os") {
                // exceptions in v2.0: `MacOS`, `TvOS`
                (&mut name[len - 2..]).make_ascii_uppercase();
            } else if name.ends_with("bsd") {
                // exceptions in v2.0: `FreeBSD`, `NetBSD`, `OpenBSD`
                (&mut name[len - 3..]).make_ascii_uppercase();
            }
            name
        }
    }
}

fn make_ascii_titlecase(s: &str) -> String {
    // don't bother to cache the regex. uppercase a lowercase letter after a _ and remove the _
    // this transforms `foo_bar` into `FooBar` but `x86_64` into `X86_64`
    let regex = Regex::new("(^|_)(?<letter>[a-z])").unwrap();
    regex
        .replace_all(s, |captures: &Captures| {
            captures
                .name("letter")
                .unwrap()
                .as_str()
                .to_ascii_uppercase()
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_naming() {
        assert_eq!(&to_enum_variant_name("foobar"), "Foobar");
        assert_eq!(&to_enum_variant_name("fooBar"), "Foobar");
        assert_eq!(&to_enum_variant_name("FOOBAR"), "Foobar");
        assert_eq!(&to_enum_variant_name("freebsd"), "FreeBSD");
        assert_eq!(&to_enum_variant_name("nonexistentbsd"), "NonexistentBSD");
        assert_eq!(&to_enum_variant_name("macos"), "MacOS");
        assert_eq!(&to_enum_variant_name("nonexistentos"), "NonexistentOS");
        assert_eq!(&to_enum_variant_name("riscv"), "RiscV");
        assert_eq!(&to_enum_variant_name("PoWeRpC"), "PowerPc");
        assert_eq!(&to_enum_variant_name("x86_64"), "X86_64");
        assert_eq!(&to_enum_variant_name("solid_asp3"), "SolidAsp3");
    }
}
