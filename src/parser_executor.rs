use crate::parser::{parse_html_to_lei, Lei};
use rayon::prelude::*;
use std::collections::HashMap;
use walkdir::{DirEntry, WalkDir};

pub struct Folder {
    pub total: i32,
    pub parsed: i32,
}

pub fn parse_on_directory(directory_path: &str) -> (HashMap<String, Folder>, Vec<Lei>) {
    let walker = WalkDir::new(directory_path).into_iter();
    let mut directories = HashMap::new();
    let files = walker
        .filter_entry(|entry| is_not_hidden(entry))
        .skip(1)
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if is_html_file(&entry) {
                Some(
                    entry
                        .path()
                        .to_str()
                        .expect("file path not found")
                        .to_string(),
                )
            } else {
                let current_folder = entry.file_name().to_os_string().into_string().unwrap();
                directories.insert(
                    current_folder,
                    Folder {
                        total: 0,
                        parsed: 0,
                    },
                );
                None
            }
        })
        .collect::<Vec<String>>();

    let leis = files
        .par_iter()
        .filter_map(|file_path| {
            let lei_result = parse_html_to_lei(file_path, "".to_string());
            match lei_result {
                Ok(lei) => Some(lei),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        })
        .collect();

    (directories, leis)
}

fn is_html_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(".html")
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| entry.depth() == 0 || !s.starts_with('.'))
}

// TODO: create test for files with different type than HTML
