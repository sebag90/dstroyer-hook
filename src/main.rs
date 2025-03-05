extern crate walkdir;

use std::fs;
use std::io::{self};
use walkdir::WalkDir;

struct Colors;
impl Colors {
    const PURPLE: &'static str = "\u{001b}[95m";
    const END: &'static str = "\u{001b}[0m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn dstroy_hook(path: &str) -> io::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok())
    // skip files without permissions
    {
        if entry.file_name() == ".DS_Store" {
            let color_filename = color_string(Colors::PURPLE, &entry.path().display().to_string());
            println!("{}", color_filename);
            fs::remove_file(&entry.path())?;
        }
    }
    Ok(())
}

fn main() {
    dstroy_hook(".").unwrap();
}