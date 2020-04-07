use crate::parser::{parse_html_to_lei, Lei};
use std::collections::HashMap;
use walkdir::{DirEntry, WalkDir};

macro_rules! unwrap_or_print_err {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    };
}

pub struct Folder {
    pub total: i32,
    pub parsed: i32,
}

pub fn parse_on_directory(directory_path: &str) -> (HashMap<String, Folder>, Vec<Lei>) {
    let walker = WalkDir::new(directory_path).into_iter();
    let mut directories = HashMap::new();
    let mut current_folder = String::new();
    let mut leis = Vec::new();

    for entry in walker.filter_entry(|entry| is_not_hidden(entry)).skip(1) {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            current_folder = entry.file_name().to_os_string().into_string().unwrap();

            directories.insert(
                current_folder.clone(),
                Folder {
                    total: 0,
                    parsed: 0,
                },
            );
        } else if is_html_file(&entry) {
            let file_path = entry.path().to_str().expect("file path not found");
            directories.get_mut(&current_folder).unwrap().total += 1;

            let lei =
                unwrap_or_print_err!(parse_html_to_lei(file_path, current_folder.to_string()));

            // TODO: não usar vector para armazenar as leis
            leis.push(lei);
            directories.get_mut(&current_folder).unwrap().parsed += 1;
        }
    }
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
