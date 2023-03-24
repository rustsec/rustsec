use std::{path::Path, io::{Cursor, Write}};

use fs_err::remove_dir_all;

use crate::Error;

pub fn fetch(url: &str, destination: &Path) -> Result<(), Error> {
    // TODO: lock the dir to protect from concurrent accesses?
    let response = ureq::get(url)
    // .set("if-none-match", etag); // TODO: etag
    .call()?;

    // Not modified - we have the latest version already (determined by the etag)
    if response.status() == 304 {
        return Ok(());
    }

    let new_etag = response.header("etag").map(String::from);
    let mut compressed_data: Vec<u8> = Vec::new();
    response.into_reader().read_to_end(&mut compressed_data)?;
    let mut archive = zip::read::ZipArchive::new(Cursor::new(compressed_data)).map_err(|e| format_err!(crate::ErrorKind::Parse, e))?;

    // Extract to a temporary folder so that we don't trample all over the existing DB
    // until extraction completes successfully.
    // The deterministic path ensures that we clean up the data from a previously aborted run
    // next time this function is invoked.
    let mut tempdir = destination.to_path_buf();
    tempdir.set_extension("part");
    create_temp_dir(&tempdir)?;

    archive.extract(&tempdir).map_err(|e| format_err!(crate::ErrorKind::Io, e))?;

    if let Some(tag) = new_etag {
        write_etag(&tempdir, &tag)?;
    }

    remove_dir_all(destination)?;
    std::fs::rename(tempdir, destination)?;

    Ok(())
}


// We don't use a dedicated facility for temporary directories here because
// std::fs::rename doesn't work across mount points,
// so we just put dirs next to each other instead to guarantee that they're on the same filesystem.
fn create_temp_dir(path: &Path) -> Result<(), std::io::Error> {
    // Handle the tempdir already existing after being interrupted earlier
    match remove_dir_all(&path) {
        Ok(_) => (), // cleaned up after a previous interrupted run
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (), // previous run wasn't interrupted, nothing to delete
        Err(e) => return Err(e.into()), // an actual error has occurred
    }
    std::fs::create_dir_all(&path)
}

fn write_etag(dir: &Path, tag: &str) -> Result<(), std::io::Error> {
    let mut etag_file_path = dir.to_owned();
    etag_file_path.push("etag");
    let mut file = std::fs::File::create(&etag_file_path)?;
    file.write_all(tag.as_bytes())
}