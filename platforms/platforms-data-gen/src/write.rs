//! Generates and writes contents of the auto-generated files

use std::io::Result;
use std::io::Write;

use crate::comments::Comments;
use crate::doc_target_info::DocTargetInfo;
use crate::doc_target_info::DocTargetsInfo;
use crate::enums::*;
use crate::rustc_target_info::{RustcTargetInfo, RustcTargetsInfo};
use crate::templates::Templates;

pub(crate) const FIELDS_WITH_ENUMS: [&'static str; 5] = [
    "target_arch",
    "target_os",
    "target_env",
    "target_endian",
    "target_pointer_width",
];

#[must_use]
pub(crate) fn write_targets_file<W: Write>(
    triples: &[String],
    rustc_info: &RustcTargetsInfo,
    doc_info: &DocTargetsInfo,
    out: &mut W,
) -> Result<()> {
    // write the header
    write!(
        out,
        "\
//! The list of targets.

// Note: this file is auto-generated. Do not edit it manually!
// If you need to referesh it, re-run the generator included in the source tree.

// Comments on targets are sourced from
// https://doc.rust-lang.org/nightly/rustc/platform-support.html
// and some of the more obscure targets do not have a comment on them
#![allow(missing_docs)]

use crate::{{
    platform::{{Platform, Tier}},
    target::{{"
    )?;
    // write the names of the enums we need to import
    for field in FIELDS_WITH_ENUMS.iter() {
        let name = to_enum_name(field);
        write!(out, " {name},")?;
    }
    writeln!(out, "}},\n}};\n")?;

    write_list_of_targets(triples, out)?;

    // write the actual targets
    for (triple, info) in triples.iter().zip(rustc_info) {
        write_target_struct(&triple, &info, &(doc_info[triple]), out)?;
    }
    Ok(())
}

#[must_use]
pub(crate) fn write_list_of_targets<W: Write>(triples: &[String], out: &mut W) -> Result<()> {
    writeln!(
        out,
        "
/// The list of all targets recognized by the Rust compiler
pub(crate) const ALL: &[Platform] = &["
    )?;
    for triple in triples {
        writeln!(out, "    {},", to_const_variable_name(triple))?;
    }
    writeln!(out, "];")?;
    Ok(())
}

#[must_use]
fn write_target_struct<W: Write>(
    triple: &str,
    rustc_info: &RustcTargetInfo,
    doc_info: &DocTargetInfo,
    out: &mut W,
) -> Result<()> {
    if doc_info.notes != "" {
        write!(out, "\n/// {}", doc_info.notes)?;
    }
    writeln!(
        out,
        "
pub(crate) const {}: Platform = Platform {{
    target_triple: \"{triple}\",",
        to_const_variable_name(triple),
    )?;
    for key in FIELDS_WITH_ENUMS.iter() {
        let value = enumify_value(key, &rustc_info[*key]);
        writeln!(out, "    {}: {},", key, value)?;
    }
    writeln!(out, "    tier: {},", tier_to_enum_variant(doc_info.tier))?;
    writeln!(out, "}};")?;
    Ok(())
}

/// Accepts the key from the `rustc` output and generates an enum from it,
/// including all `impl`s that depend on the info about available targets
#[must_use]
pub(crate) fn write_enum_file<W: Write>(
    key: &str,
    info: &RustcTargetsInfo,
    out: &mut W,
) -> Result<()> {
    let templates = Templates::new();
    out.write_all(templates.header(key).unwrap())?;
    write_enum_definition(key, info, out)?;
    write_enum_string_conversions(key, info, out)?;
    out.write_all(templates.footer(key).unwrap())?;
    Ok(())
}

/// Accepts the key from the `rustc` output and generates an enum definition from it
#[must_use]
fn write_enum_definition<W: Write>(key: &str, info: &RustcTargetsInfo, out: &mut W) -> Result<()> {
    writeln!(out, "pub enum {} {{", to_enum_name(key))?;
    let comments = Comments::new();
    let raw_strings = distinct_values(key, info);
    for raw_string in &raw_strings {
        if let Some(comment) = comments.enum_variant_comment(raw_string) {
            // if there is a manually added comment for this enum variant in our data, add it
            writeln!(out, "    /// `{raw_string}`: {comment}")?;
        } else {
            // otherwise just write out the raw value from rustc for reference
            writeln!(out, "    /// `{raw_string}`")?;
        }
        let enum_variant = to_enum_variant_name(raw_string);
        // deal with names like 'iOS' that violate Rust naming conventions
        if enum_variant.chars().nth(0).unwrap().is_lowercase() {
            writeln!(out, "    #[allow(non_camel_case_types)]")?;
        }
        writeln!(out, "    {enum_variant},\n")?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

#[must_use]
fn write_enum_string_conversions<W: Write>(
    key: &str,
    info: &RustcTargetsInfo,
    out: &mut W,
) -> Result<()> {
    let raw_strings = distinct_values(key, info);
    let enum_name = to_enum_name(key);

    // write as_str()
    writeln!(
        out,
        "
impl {enum_name} {{
    /// String representing this `{enum_name}` which matches `#[cfg({key})]`
    pub fn as_str(self) -> &'static str {{
        match self {{"
    )?;
    for raw_string in &raw_strings {
        let variant = enumify_value(key, &raw_string);
        //                       OS::Android => "android",
        writeln!(out, "            {variant} => \"{raw_string}\",")?;
    }
    writeln!(
        out,
        "        }}
    }}
}}"
    )?;

    // write `from_str()` impl
    writeln!(
        out,
        "
impl FromStr for {enum_name} {{
    type Err = Error;

    /// Create a new `{enum_name}` from the given string
    fn from_str(name: &str) -> Result<Self, Self::Err> {{
        let result = match name {{"
    )?;
    for raw_string in &raw_strings {
        let variant = enumify_value(key, &raw_string);
        //                            "android" => OS::Android,
        writeln!(out, "            \"{raw_string}\" => {variant},")?;
    }
    writeln!(
        out,
        "            _ => return Err(Error),
        }};

        Ok(result)
    }}
}}"
    )?;
    Ok(())
}

#[must_use]
fn to_const_variable_name(input: &str) -> String {
    input
        .to_ascii_uppercase()
        .replace("-", "_")
        .replace(".", "_")
}

#[must_use]
fn tier_to_enum_variant(tier: u8) -> &'static str {
    match tier {
        1 => "Tier::One",
        2 => "Tier::Two",
        3 => "Tier::Three",
        _ => unreachable!("Unknown tier: {}", tier),
    }
}
