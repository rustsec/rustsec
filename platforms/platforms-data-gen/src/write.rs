use std::io::Write;
use std::io::Result;

use crate::doc_target_info::DocTargetInfo;
use crate::enums::enumify_value;
use crate::rustc_target_info::RustcTargetInfo;

pub(crate) const FIELDS_WITH_ENUMS: [&'static str; 5] = [
    "target_arch",
    "target_os",
    "target_env",
    "target_endian",
    "target_pointer_width",
];

#[must_use]
pub(crate) fn write_target_struct<W: Write>(triple: &str, rustc_info: &RustcTargetInfo, doc_info: &DocTargetInfo, out: &mut W) -> Result<()> {
    if doc_info.notes != "" {
        writeln!(out, "/// {}", doc_info.notes)?;
    }
    writeln!(out, 
        "pub const {}: Platform = Platform {{
    target_triple: \"{}\",",
        to_const_variable_name(triple),
        triple
    )?;
    for key in FIELDS_WITH_ENUMS.iter() {
        let value = enumify_value(key, &rustc_info[*key]);
        writeln!(out, "    {}: {},", key, value)?;
    }
    writeln!(out, "    tier: {},", tier_to_enum_variant(doc_info.tier))?;
    writeln!(out, "}};\n")?;
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
