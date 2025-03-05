extern crate walkdir;

use std::fs;
use std::io::{self};
use walkdir::WalkDir;


fn dstroy_hook(path: &str) -> io::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok())
    {
        if entry.file_name() == ".DS_Store" {
            fs::remove_file(&entry.path())?;
        }
    }
    Ok(())
}

fn main() {
    dstroy_hook(".").unwrap();
}