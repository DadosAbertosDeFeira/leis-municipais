use crate::parser::parse_html_to_lei;
use std::collections::HashMap;
use walkdir::{DirEntry, WalkDir};

mod parser;

fn main() {
    let lei = parse_html_to_lei(
        "resources/LeisMunicipais-com-br-Lei-Complementar-122-2019.html",
        "teste".to_string(),
    );
    // println!("{}", serde_json::to_string(&lei).unwrap()); // TODO: error handler

    // TODO: needs to be absolute path
    let walker = WalkDir::new("/Users/mac/Downloads/LeisMunicipais - Feira de Santana").into_iter();

    let mut directories = HashMap::new();
    let mut current_directory = String::new();
    for entry in walker.filter_entry(|entry| is_not_hidden(entry)).skip(1) {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            current_directory = entry.file_name().to_os_string().into_string().unwrap();

            directories.insert(current_directory.clone(), 0);
        } else if entry.file_type().is_file()
            && entry.file_name().to_string_lossy().ends_with(".html")
        {
            *directories.get_mut(&current_directory).unwrap() += 1;
        }
    }

    println!("{}", "mateus");
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with('.'))
        .unwrap_or(false)
}
