//! RustSec Advisory DB tool to assign ids

use std::{
    fs::{self, File},
    io::{BufRead, BufReader, LineWriter, Write},
    path::Path,
    process::exit,
};

use rustsec::{
    Advisory, Collection,
    advisory::{IdKind, Parts},
};

use crate::{Map, prelude::*};

/// What sort of output should be generated on stdout.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OutputMode {
    /// Normal human readable logging
    HumanReadable,
    /// Output designed for use in the github action that runs this in prod
    GithubAction,
}

/// assign ids to advisories in a particular repo_path
pub fn assign_ids(repo_path: &Path, output_mode: OutputMode) {
    let db = rustsec::Database::open(repo_path).unwrap_or_else(|e| {
        status_err!(
            "couldn't open advisory DB repo from {}: {}",
            repo_path.display(),
            e
        );
        exit(1);
    });

    let advisories = db.iter();

    // Ensure we're parsing some advisories
    if advisories.len() == 0 {
        status_err!("no advisories found!");
        exit(1);
    }

    if output_mode == OutputMode::HumanReadable {
        status_ok!(
            "Loaded",
            "{} security advisories (from {})",
            advisories.len(),
            repo_path.display()
        );
    }

    let mut highest_id = Map::new();

    for advisory in advisories {
        let advisory_clone = advisory.clone();
        let metadata = advisory_clone.metadata;
        let id = metadata.id;
        let year = metadata.date.year();

        if let IdKind::RustSec = id.kind() {
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

    let mut assignments = vec![];
    for collection in [Collection::Crates, Collection::Rust] {
        assign_ids_across_directory(
            collection,
            repo_path,
            &mut highest_id,
            output_mode,
            &mut assignments,
        )
        .unwrap_or_else(|error| {
            status_err!(
                "Error assigning ids for {collection} in {}: {error}",
                repo_path.display(),
            );
            exit(1);
        });
    }

    if output_mode != OutputMode::GithubAction {
        return;
    }

    let mut title = format!("Assigned {}", assignments.join(", "));
    let mut dropped = 0;
    while title.len() > 255 {
        dropped += 1;
        let new = title.rsplit_once(", ").unwrap().0;
        title = format!("{new} and {dropped} more");
    }

    println!("{title}");
}

///Assign ids to files with placeholder IDs within the directory defined by dir_path
fn assign_ids_across_directory(
    collection: Collection,
    repo_path: &Path,
    highest_ids: &mut Map<u32, u32>,
    output_mode: OutputMode,
    assignments: &mut Vec<String>,
) -> Result<(), rustsec::Error> {
    let dir_path = repo_path.join(collection.to_string());
    let Ok(collection_entry) = fs::read_dir(dir_path) else {
        return Ok(());
    };

    for dir_entry in collection_entry {
        let unwrapped_dir_entry = dir_entry?;
        let dir_name = unwrapped_dir_entry
            .file_name()
            .into_string()
            .map_err(|os_str| {
                rustsec::Error::new(
                    rustsec::ErrorKind::Parse,
                    format!("Couldn't parse directory name: {}", os_str.display()),
                )
            })?;

        let dir_path = unwrapped_dir_entry.path();
        let dir_path_clone = dir_path.clone();
        for advisory_entry in fs::read_dir(dir_path)? {
            let unwrapped_advisory = advisory_entry?;
            let advisory_path = unwrapped_advisory.path();
            if !Advisory::is_draft(&advisory_path) {
                continue;
            }

            let advisory_data = fs::read_to_string(&advisory_path).map_err(|e| {
                rustsec::Error::with_source(
                    rustsec::ErrorKind::Io,
                    format!("Couldn't open {}", advisory_path.display()),
                    e,
                )
            })?;

            let advisory_parts = Parts::parse(&advisory_data)?;
            let advisory: Advisory = toml::from_str(advisory_parts.front_matter).map_err(|e| {
                rustsec::Error::with_source(
                    rustsec::ErrorKind::Parse,
                    format!(
                        "Couldn't parse TOML front matter in {}",
                        advisory_path.display()
                    ),
                    e,
                )
            })?;

            let date = advisory.metadata.date;
            let year = date.year();
            let new_id = highest_ids.get(&year).cloned().unwrap_or_default() + 1;
            let year_str = year.to_string();
            let string_id = format!("RUSTSEC-{year_str}-{new_id:04}");
            let new_filename = format!("{string_id}.md");
            let new_path = dir_path_clone.join(new_filename);
            let original_file = File::open(&advisory_path)?;
            let reader = BufReader::new(original_file);
            let new_file = File::create(new_path)?;
            let mut writer = LineWriter::new(new_file);
            for line in reader.lines() {
                let current_line = line?;
                if current_line.trim() == "id = \"RUSTSEC-0000-0000\"" {
                    writer.write_all(format!("id = \"{string_id}\"\n").as_ref())?;
                } else {
                    let current_line_with_newline = format!("{current_line}\n");
                    writer.write_all(current_line_with_newline.as_ref())?;
                }
            }
            highest_ids.insert(year, new_id);
            fs::remove_file(&advisory_path)?;
            if output_mode == OutputMode::HumanReadable {
                status_ok!("Assignment", "Assigned {} to {}", string_id, dir_name);
            } else {
                assignments.push(format!("{string_id} to {dir_name}"))
            }
        }
    }

    Ok(())
}
