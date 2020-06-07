use crate::parser::{parse_html_to_lei, Lei};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use walkdir::{DirEntry, WalkDir};

pub fn parse_on_directory(directory_path: &str) -> HashMap<String, Vec<Option<Lei>>> {
    println!("test");
    let walker = WalkDir::new(directory_path).into_iter();
    let files = walker
        .filter_entry(|entry| is_not_hidden(entry))
        .skip(1)
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if is_html_file(&entry) {
                Some(entry)
            } else {
                None
            }
        })
        .collect::<Vec<DirEntry>>();

    files
        .par_iter()
        .map(|entry| {
            let file_folder = get_file_folder(&entry);
            let file_path = entry
                .path()
                .to_str()
                .expect("file path not found")
                .to_string();
            let lei_result = parse_html_to_lei(&file_path, file_folder.clone());
            match lei_result {
                Ok(lei) => (file_folder, Some(lei)),
                Err(e) => {
                    eprintln!("{}", e);
                    (file_folder, None)
                }
            }
        })
        .collect::<Vec<(String, Option<Lei>)>>()
        .into_iter()
        .into_group_map()
}

fn get_file_folder(entry: &DirEntry) -> String {
    entry
        .path()
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
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
