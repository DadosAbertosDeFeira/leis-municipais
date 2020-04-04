use crate::error::Error;
use crate::parser::parse_html_to_lei;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

mod error;
mod parser;

struct Folder {
    total: i32,
    parsed: i32,
}

fn main() -> Result<(), Error> {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1]; // TODO: error handler

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
        } else if entry.file_type().is_file()
            && entry.file_name().to_string_lossy().ends_with(".html")
        {
            let file_path = entry.path().to_str().expect("file path not found");
            // let lei = skip_fail!(parse_html_to_lei(file_path, current_folder.to_string()));
            let lei = match parse_html_to_lei(file_path, current_folder.to_string()) {
                Ok(lei) => lei,
                Err(error) => {
                    println!("{}", error);
                    continue;
                }
            };

            // TODO: não usar vector para armazenar as leis
            leis.push(lei);
            directories.get_mut(&current_folder).unwrap().parsed += 1;
            directories.get_mut(&current_folder).unwrap().total += 1;
        }
    }

    // TODO: escrever em formato de tabela igual no futiba
    for (directory, folder) in &directories {
        println!(
            "diretorio {}: total {}, {} processados, {} com erros",
            directory,
            folder.total,
            folder.parsed,
            (folder.total - folder.parsed)
        );
    }

    let leis_file = File::create("leis.json").expect("Unable to create file");
    serde_json::to_writer(leis_file, &leis).expect("Unable to write data");

    println!("Tempo de execucao: {} segundos", now.elapsed().as_secs());
    Ok(())
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| entry.depth() == 0 || !s.starts_with('.'))
}
