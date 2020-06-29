//! RustSec Advisory DB tool to assign ids

use crate::{error::ErrorKind, prelude::*, Map};
use rustsec::{advisory::id::Kind, collection::Collection, Advisory};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, LineWriter, Write},
    path::Path,
    process::exit,
};

/// assign ids to advisories in a particular repo_path
pub fn assign_ids(repo_path: &Path) {
    let repo = rustsec::Repository::open(repo_path).unwrap_or_else(|e| {
        status_err!(
            "couldn't open advisory DB repo from {}: {}",
            repo_path.display(),
            e
        );
        exit(1);
    });

    // Ensure Advisories.toml parses
    let db = rustsec::Database::load(&repo).unwrap();
    let advisories = db.iter();

    // Ensure we're parsing some advisories
    if advisories.len() == 0 {
        status_err!("no advisories found!");
        exit(1);
    }

    status_ok!(
        "Loaded",
        "{} security advisories (from {})",
        advisories.len(),
        repo_path.display()
    );

    let mut highest_id = Map::new();

    for advisory in advisories {
        let advisory_clone = advisory.clone();
        let metadata = advisory_clone.metadata;
        let id = metadata.id;
        let year = metadata.date.year();

        if let Kind::RUSTSEC = id.kind() {
            let id_num = id.numerical_part().unwrap();

            if let Some(&number) = highest_id.get(&year) {
                if number < id_num {
                    highest_id.insert(year, id_num);
                }
            } else {
                highest_id.insert(year, id_num);
            }
        }
    }

    let mut collection_strs = vec![];
    let crates_str = Collection::Crates.to_string();
    let rust_str = Collection::Rust.to_string();
    collection_strs.push(crates_str);
    collection_strs.push(rust_str);

    for collection_str in collection_strs {
        assign_ids_across_directory(collection_str, repo_path, &mut highest_id);
    }
}

///Assign ids to files with placeholder IDs within the directory defined by dir_path
fn assign_ids_across_directory(
    collection_str: String,
    repo_path: &Path,
    highest_ids: &mut Map<u32, u32>,
) {
    let dir_path = repo_path.join(collection_str);

    if let Ok(collection_entry) = fs::read_dir(dir_path) {
        for dir_entry in collection_entry {
            let unwrapped_dir_entry = dir_entry.unwrap();
            let dir_name = unwrapped_dir_entry.file_name().into_string().unwrap();
            let dir_path = unwrapped_dir_entry.path();
            let dir_path_clone = dir_path.clone();
            for advisory_entry in fs::read_dir(dir_path).unwrap() {
                let unwrapped_advisory = advisory_entry.unwrap();
                let advisory_path = unwrapped_advisory.path();
                let advisory_path_clone = advisory_path.clone();
                let advisory_path_for_reading = advisory_path.clone();
                let advisory_path_for_deleting = advisory_path.clone();
                let displayed_advisory_path = advisory_path.display();
                let advisory_filename = unwrapped_advisory.file_name();
                let advisory_filename_str = advisory_filename.into_string().unwrap();
                if advisory_filename_str.contains("RUSTSEC-0000-0000") {
                    let toml = fs::read_to_string(advisory_path_clone)
                        .map_err(|e| {
                            format_err!(
                                ErrorKind::Io,
                                "Couldn't open {}: {}",
                                displayed_advisory_path,
                                e
                            );
                        })
                        .unwrap();
                    let advisory = toml.parse::<Advisory>().unwrap();
                    let date = advisory.metadata.date;
                    let year = date.year();
                    let new_id = highest_ids.get(&year).unwrap() + 1;
                    let year_str = year.to_string();
                    let string_id = format!("RUSTSEC-{}-{:04}", year_str, new_id);
                    let new_filename = format!("{}.toml", string_id);
                    let new_path = dir_path_clone.join(new_filename);
                    let original_file = File::open(advisory_path_for_reading).unwrap();
                    let reader = BufReader::new(original_file);
                    let new_file = File::create(new_path).unwrap();
                    let mut writer = LineWriter::new(new_file);
                    for line in reader.lines() {
                        let current_line = line.unwrap();
                        if current_line.contains("id = ") {
                            writer
                                .write_all(format!("id = \"{}\"\n", string_id).as_ref())
                                .unwrap();
                        } else {
                            let current_line_with_newline = format!("{}\n", current_line);
                            writer
                                .write_all(current_line_with_newline.as_ref())
                                .unwrap();
                        }
                    }
                    highest_ids.insert(year, new_id);
                    fs::remove_file(advisory_path_for_deleting).unwrap();
                    status_ok!("Assignment", "Assigned {} to {}", string_id, dir_name);
                }
            }
        }
    }
}
