use std::path::Path;

use walkdir::{DirEntry, WalkDir};

mod format_file;
use format_file::format_file;

fn is_gql_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|file_name| file_name.ends_with(".gql") || file_name.ends_with(".graphql"))
        .unwrap_or(false)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Expected path");
    }

    let mut is_quiet = false;

    for arg in args.iter() {
        if arg == "--quiet" || arg == "-q" {
            is_quiet = true;
        }
    }

    let root_path = Path::new(&args[1]);
    if !root_path.exists() {
        panic!("Path doesn't exists");
    }
    let folder = WalkDir::new(root_path).into_iter();

    for entry in folder.filter_map(|e| e.ok()) {
        if !is_gql_file(&entry) {
            continue;
        }
        format_file(entry.path());
        if !is_quiet {
            println!("Formatted: {}", entry.path().display());
        }
    }
}
