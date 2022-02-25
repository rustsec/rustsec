use std::collections::BTreeSet;

use crate::rustc_target_info::RustcTargetsInfo;


pub(crate) fn enum_variant_names(key: &str, info: &RustcTargetsInfo) -> Vec<String> {
    distinct_values(key, info).iter().map(|v| enum_variant_name(v)).collect()
}

fn distinct_values(key: &str, info: &RustcTargetsInfo) -> BTreeSet<String> {
    info.iter().map(|t| &t[key]).cloned().collect()
}

pub(crate) fn enumify_value(key: &str, value: &str) -> String {
    format!("{}::{}", enum_name(key), enum_variant_name(value))
}

pub(crate) fn enum_name(key: &str) -> &'static str {
    match key {
        "target_arch" => "Arch",
        "target_os" => "OS",
        "target_env" => "Env",
        "tier" => "Tier",
        "target_endian" => "Endian",
        "target_pointer_width" => "Bits",
        _ => unreachable!(format!("unknown enum name: {}", key))
    }
}

fn enum_variant_name(value: &str) -> String {
    // TODO: there are some exceptions but this is the general trend
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