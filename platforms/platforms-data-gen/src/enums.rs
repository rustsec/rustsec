use std::collections::BTreeSet;

use crate::rustc_target_info::RustcTargetsInfo;

pub(crate) fn enum_variant_names(key: &str, info: &RustcTargetsInfo) -> Vec<String> {
    distinct_values(key, info)
        .iter()
        .map(|v| to_enum_variant_name(v))
        .collect()
}

fn distinct_values(key: &str, info: &RustcTargetsInfo) -> BTreeSet<String> {
    info.iter().map(|t| &t[key]).cloned().collect()
}

pub(crate) fn enumify_value(key: &str, value: &str) -> String {
    format!("{}::{}", to_enum_name(key), to_enum_variant_name(value))
}

pub(crate) fn to_enum_name(key: &str) -> &'static str {
    match key {
        "target_arch" => "Arch",
        "target_os" => "OS",
        "target_env" => "Env",
        "tier" => "Tier",
        "target_endian" => "Endian",
        "target_pointer_width" => "Bits",
        _ => unreachable!("unknown enum name: {}", key),
    }
}

fn to_enum_variant_name(value: &str) -> String {
    // TODO: there are some exceptions but this is the general trend
    // TODO: list of exceptions as of platforms v2.0 obtained via
    // `rg --only-matching --no-filename --no-line-number '    [A-Z0-9][A-Za-z0-9]*,' | grep -v ' [A-Z][a-z0-9]\+,'`
    // is:
    // AArch64,
    // AsmJs,
    // PowerPc,
    // PowerPc64,
    // RiscV,
    // S390X,
    // ThumbV6,
    // ThumbV7,
    // UClibc,
    // FreeBSD,
    // MacOS,
    // NetBSD,
    // OpenBSD,
    // TvOS,
    // VxWorks,
    let mut name = value.to_string();
    make_ascii_titlecase(&mut name);
    name
}

fn make_ascii_titlecase(s: &mut str) {
    // based on https://stackoverflow.com/a/53571882
    s.make_ascii_lowercase();
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}
