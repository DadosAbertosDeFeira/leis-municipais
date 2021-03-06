#![allow(clippy::non_ascii_literal)]
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate lazy_static;

use crate::parser::Lei;
use crate::parser_executor::parse_on_directory;
use prettytable::Table;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

mod error;
mod parser;
mod parser_executor;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1]; // TODO: error handler
    let file_name = &args[2]; // TODO: error handler

    let file_path = Path::new(file_name);
    if file_path.extension() == Some(OsStr::new("json")) {
        let leis_by_folder = parse_on_directory(directory_path);

        print_report(&leis_by_folder);
        write_json_file(&leis_by_folder, file_name);

        println!("Tempo de execução: {} segundos", now.elapsed().as_secs());
    } else {
        println!("Nome de arquivo não permitido. Extensão do arquivo deve ser .json");
    }
}

fn write_json_file(leis_by_folder: &HashMap<String, Vec<Option<Lei>>>, file_name: &str) {
    let leis = leis_by_folder
        .values()
        .flatten()
        .filter_map(Option::as_ref)
        .collect::<Vec<&Lei>>();
    let leis_file = File::create(file_name).expect("Unable to create file");
    serde_json::to_writer_pretty(leis_file, &leis).expect("Unable to write data");
    println!(
        "\nArquivo salvo em: {}",
        format!("{}/{}", env::current_dir().unwrap().display(), file_name)
    );
}

fn print_report(leis_by_folder: &HashMap<String, Vec<Option<Lei>>>) {
    let total_files = leis_by_folder.values().flatten().count();
    println!("\nTotal de arquivos: {}", total_files);

    let mut table = Table::new();
    table.set_titles(row!["Diretório", "Total", "Parseados", "Com erros",]);

    for (directory, files) in leis_by_folder {
        let not_parsed = files.len() - files.iter().flatten().count();
        let parsed = files.len() - not_parsed;
        table.add_row(row![directory, files.len(), parsed, not_parsed]);
    }

    println!("\nResumo da execução:");
    table.printstd();
}
