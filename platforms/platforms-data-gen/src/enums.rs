use std::collections::BTreeSet;

use crate::rustc_target_info::RustcTargetsInfo;

#[derive(Debug, Clone)]
pub(crate) struct EnumData {
    pub name: &'static str,
    pub variant_names: Vec<String>
}

fn enum_definition(key: &str, info: &RustcTargetsInfo) -> EnumData {
    let name = enum_name(key);
    let variant_names: Vec<String> = distinct_values(key, info).iter().map(|v| enum_variant_name(v)).collect();
    EnumData {name, variant_names }
}

fn distinct_values(key: &str, info: &RustcTargetsInfo) -> BTreeSet<String> {
    info.iter().map(|t| &t[key]).cloned().collect()
}

fn enum_name(key: &str) -> &'static str {
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