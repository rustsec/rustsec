mod doc_target_info;
mod enums;
mod rustc_target_info;
mod write;
mod comments;
mod data;

use std::{collections::HashSet, env::args_os};

use doc_target_info::DocTargetsInfo;
use write::{write_enum, write_target_struct, FIELDS_WITH_ENUMS};

fn main() -> std::io::Result<()> {
    let file = args_os().nth(1).expect(
        "No path to .md file specified!\n
Please download a local copy of
https://github.com/rust-lang/rust/blob/master/src/doc/rustc/src/platform-support.md
and pass it as an argument to this program.",
    );
    let doc_content = std::fs::read_to_string(file)?;
    let doc_info = doc_target_info::parse_file(&doc_content);
    let triples = rustc_target_info::target_triples();

    ensure_rustc_and_docs_agree(&triples, &doc_info);

    // TODO: write to files instead of stdout
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let rustc_info = rustc_target_info::targets_info(&triples);

    for key in FIELDS_WITH_ENUMS.iter() {
        write_enum(key, &rustc_info, &mut stdout)?;
    }

    for (triple, info) in triples.iter().zip(rustc_info) {
        write_target_struct(&triple, &info, &doc_info[triple], &mut stdout)?;
    }
    Ok(())
}

fn ensure_rustc_and_docs_agree(rustc_triples: &[String], doc_triples: &DocTargetsInfo) {
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
