use crate::parser::{parse_html_to_lei, Lei};
use crate::parser_executor::Folder;
use jwalk::{ClientState, DirEntry, WalkDir, WalkDirGeneric};
use std::collections::HashMap;

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

pub fn parse_on_directory_jwalk(directory_path: &str) -> (HashMap<String, Folder>, Vec<Lei>) {
    let mut directories = HashMap::new();
    let mut current_folder = String::new();
    let mut leis = Vec::new();

    for entry in WalkDir::new(directory_path).sort(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(".html") {
            let entry = entry.path();
            let file_path = entry.to_str().expect("file path not found");
            // directories.get_mut(&current_folder).unwrap().total += 1;

            let lei =
                unwrap_or_print_err!(parse_html_to_lei(file_path, current_folder.to_string()));

            leis.push(lei);
        }
        // directories.get_mut(&current_folder).unwrap().parsed += 1;
    }
    (directories, leis)
}
