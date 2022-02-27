mod rustc_target_info;
mod enums;
mod doc_parser;

use enums::*;

const FIELDS_WITH_ENUMS: [&'static str; 5] = ["target_arch", "target_os", "target_env", "target_endian", "target_pointer_width"];

fn main() {
    let triples = rustc_target_info::target_triples();
    let targets_info = rustc_target_info::targets_info(&triples);

    for key in FIELDS_WITH_ENUMS.iter() {
        println!("pub enum {} {{", to_enum_name(key));
        for variant_name in enum_variant_names(key, &targets_info) {
            println!("    {},", variant_name);
        }
        println!("}}\n");
    }
    // Print list of platforms with all the data about them
    for (triple, info) in triples.iter().zip(targets_info) {
        println!("pub const {}: Platform = Platform {{
    target_triple: \"{}\",", to_const_variable_name(triple), triple);
    for key in FIELDS_WITH_ENUMS.iter() {
        let value = enumify_value(key, &info[*key]);
        println!("    {}: {},", key, value);
    }
    println!("}};\n")
    }
}

fn to_const_variable_name(input: &str) -> String {
    input.to_ascii_uppercase().replace("-", "_")
}