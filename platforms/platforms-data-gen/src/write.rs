use std::io::Result;
use std::io::Write;

use crate::doc_target_info::DocTargetInfo;
use crate::enums::*;
use crate::rustc_target_info::{RustcTargetInfo, RustcTargetsInfo};

pub(crate) const FIELDS_WITH_ENUMS: [&'static str; 5] = [
    "target_arch",
    "target_os",
    "target_env",
    "target_endian",
    "target_pointer_width",
];

#[must_use]
pub(crate) fn write_target_struct<W: Write>(
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
pub const {}: Platform = Platform {{
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
pub(crate) fn write_enum<W: Write>(key: &str, info: &RustcTargetsInfo, out: &mut W) -> Result<()> {
    write_enum_definition(key, info, out)?;
    write_enum_string_conversions(key, info, out)?;
    Ok(())
}

/// Accepts the key from the `rustc` output and generates an enum definition from it
#[must_use]
pub(crate) fn write_enum_definition<W: Write>(
    key: &str,
    info: &RustcTargetsInfo,
    out: &mut W,
) -> Result<()> {
    writeln!(out, "pub enum {} {{", to_enum_name(key))?;
    for variant_name in enum_variant_names(key, info) {
        writeln!(out, "    {variant_name},")?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

#[must_use]
pub(crate) fn write_enum_string_conversions<W: Write>(
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
    /// String representing this {enum_name} which matches `#[cfg({key})]`
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
        writeln!(out, "            {raw_string} => \"{variant}\",")?;
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
    input.to_ascii_uppercase().replace("-", "_")
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
