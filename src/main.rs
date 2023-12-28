mod date_folder;

use std::thread::current;
use std::{env, os};

use delta_compressor::constants::binary_prefixes::*;
use delta_compressor::{self, command_line_args};

/// Delta files chop the changes into blocks of this size.
const DEFAULT_BLOCK_SIZE: usize = 4 * KB;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = command_line_args::parse_arguments(args);

    // If no working_dir was set, try and get the current directory.
    if config.working_dir == None {
        let current_dir = env::current_dir().expect("Failed to get the current directory!");
        config.working_dir = Some(current_dir);
    }

    // Validate that the current directory is not an empty.
    let Some(path) = &config.working_dir else {
        return;
    };
    let os_path = path.as_os_str();

    if os_path.len() != 0 {
        return;
    }
    panic!("Current directory path could not be determined!");
}

/*
fn to_hex_string(buf: &[u8]) -> String {
    buf.iter().map(|byte| format!("{:02x}", byte)).collect()
}
fn hash_file(file: &str) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(b"hello world");

    // read hash digest and consume hasher
    let result = hasher.finalize();
    String::from("TEST")
}
*/

/*

    delta_compressor::test();


    // Get curernt directory
    let current_dir = env::current_dir().expect("Failed to get the current directory");

    let p = PathBuf::from("TEST");

    // Look for folders with YYYY-MM-DD
    let name_reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    let mut dated_folders = Vec::<(String, PathBuf)>::new();

    // Iterate over everything in the directory
    let folder = std::fs::read_dir(current_dir)
        .expect("Failed to enumerate directory")
        // Unwrap
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|dir_entry| dir_entry.path())
        // Make sure that it's a folder
        .filter(|path| match fs::metadata(path) {
            // Log failure to read path metadata
            Err(msg) => {
                eprintln!("{}", msg);
                false
            }
            // Metadata found, is it a folder?
            Ok(metadata) => metadata.is_dir(),
        })
        .filter_map(|dir_path| {
            if let Some(file_name_str) = dir_path.file_name() {
                if let Some()
                .unwrap_or(None).to_str()
                if name_reg.is_match(file_name_str) {

                }
            }
            false
        })
        .map(|dir| dir);

    for f in folder {
        // Log errors
        if let Err(msg) = f {
            eprintln!("{}", msg);
            continue;
        }

        // Get the path of each entry
        // if it's a folder with a name YYYY-MM-DD store it for later.
        let dir_entry = f.unwrap();
        let path_buf = dir_entry.path();
        let file_name = path_buf.file_name();
        if file_name == None {
            continue;
        }
        let file_name = file_name.unwrap().to_str();
        if file_name == None {
            continue;
        }

        let file_name = file_name.unwrap();

        if name_reg.captur(&file_name) {
            dated_folders.push((file_name.to_owned(), path_buf));
        }
    }

    for f in dated_folders {
        println!("{}: {:?}", f.0, f.1);
    }

    /*
    let mut disk2 = BlockReader::new("disk2.img").unwrap();
    let mut disk3 = BlockReader::new("disk3.img").unwrap();

    loop {
        disk2.read()
    }
    */
}
     */
