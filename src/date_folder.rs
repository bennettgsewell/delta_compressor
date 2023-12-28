pub mod date;
use date::*;
use std::{io, path::PathBuf, fs};

/// Represents a folder with the naming convention YYYY-MM-DD
pub struct Folder {
    path: PathBuf,
    date: Date,
}

/// Gets all of the Folders in a path.
/// Folders are a struct that represents a dated folder e.g. YYYY-MM-DD
pub fn get_all_folders(path: &PathBuf) -> io::Result<()> {
    let dir = fs::read_dir(path)?;

    // Iterate over all the entries in this directory.
    for entry in dir {
        let entry = entry?;
        let entry_path = entry.path();

        let metadata = fs::metadata(&entry_path)?;

        // Skip anything that's not a directory.
        if !metadata.is_dir() {
            continue;
        }

        let entry_file_name = entry_path.file_name().expect("Failed to read file name!");

        // Attempt to parse the filename into UTF-8, if this doesn't work it's probably not a valid YYYY-MM-DD folder anyways.
        let entry_file_name = match entry_file_name.to_str() {
            Some(file_name_str) => file_name_str,
            None => continue,
        };

        let date = Date::from_str(entry_file_name);
    }

    Ok(())
}
