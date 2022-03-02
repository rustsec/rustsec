mod doc_target_info;
mod enums;
mod rustc_target_info;

use std::{
    collections::HashSet,
    env::args_os,
};

use doc_target_info::DocTargetsInfo;
use enums::*;

const FIELDS_WITH_ENUMS: [&'static str; 5] = [
    "target_arch",
    "target_os",
    "target_env",
    "target_endian",
    "target_pointer_width",
];

fn main() {
    let file = args_os().nth(1).expect(
        "No path to .md file specified!\n
Please download a local copy of
https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md
and pass it as an argument to this program.",
    );
    let doc_content = std::fs::read_to_string(file).unwrap();
    let doc_info = doc_target_info::parse_file(&doc_content);
    let triples = rustc_target_info::target_triples();

    ensure_rustc_and_docs_agree(&triples, &doc_info);

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
        let doc_data = &doc_info[triple];
        if doc_data.notes != "" {
            println!("/// {}", doc_data.notes);
        }
        println!(
            "pub const {}: Platform = Platform {{
    target_triple: \"{}\",",
            to_const_variable_name(triple),
            triple
        );
        for key in FIELDS_WITH_ENUMS.iter() {
            let value = enumify_value(key, &info[*key]);
            println!("    {}: {},", key, value);
        }
        println!("    tier: {},", tier_to_enum_variant(doc_data.tier));
        println!("}};\n")
    }
}

#[must_use]
fn to_const_variable_name(input: &str) -> String {
    input.to_ascii_uppercase().replace("-", "_")
}

fn ensure_rustc_and_docs_agree(
    rustc_triples: &[String],
    doc_triples: &DocTargetsInfo,
) {
    // Verify that all target triples known to the compiler are documented
    // and that all documented triples are recognized by rustc
    let rustc_triples: HashSet<String> = rustc_triples.iter().cloned().collect();
    let doc_triples: HashSet<String> = doc_triples.keys().cloned().collect();
    for triple in rustc_triples.union(&doc_triples) {
        match (rustc_triples.get(triple), doc_triples.get(triple)) {
            (Some(_), None) => {
                eprintln!("Error: Target triple '{}' is known to the compiler, but is not present in the documentation.
Please make sure your markdown file up to date. It should be available from
https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md", triple);
                std::process::exit(1);
            }
            (None, Some(_)) => {
                eprintln!("Error: Target triple '{}' is documented, but is not recognized by the compiler.
Please make sure your Rust compiler version is up to date.", triple);
                std::process::exit(1);
            }
            (Some(_), Some(_)) => (), // present in both, nothing to complain about
            (None, None) => unreachable!(),
        }
    }
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
