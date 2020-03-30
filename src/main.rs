use crate::parser::{parse_html_to_lei, ParserError};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use walkdir::{DirEntry, WalkDir};

mod parser;

fn main() -> Result<(), ParserError> {
    let args: Vec<String> = env::args().collect();
    let folder_path = &args[1]; // TODO: error handler

    let walker = WalkDir::new(folder_path).into_iter();

    let mut directories = HashMap::new();
    let mut current_directory = String::new();
    let mut leis = Vec::new();
    for entry in walker.filter_entry(|entry| is_not_hidden(entry)).skip(1) {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            current_directory = entry.file_name().to_os_string().into_string().unwrap();

            directories.insert(current_directory.clone(), 0);
        } else if entry.file_type().is_file()
            && entry.file_name().to_string_lossy().ends_with(".html")
        {
            let lei = parse_html_to_lei(
                entry.path().to_str().unwrap(), // TODO: handle error
                current_directory.to_string(),
            )?;
            // TODO: não usar vector para armazenar as leis
            leis.push(lei);

            *directories.get_mut(&current_directory).unwrap() += 1;
        }
    }

    // TODO: escrever em formato de tabela igual no futiba
    for (directory, files_number) in &directories {
        println!("diretorio {}: {} arquivos lidos", directory, files_number);
    }

    let leis_file = File::create("leis.json").expect("Unable to create file");
    serde_json::to_writer(leis_file, &leis).expect("Unable to write data");

    Ok(())
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| entry.depth() == 0 || !s.starts_with('.'))
}
