#![allow(clippy::non_ascii_literal)]
#[macro_use]
extern crate prettytable;
use crate::error::Error;
use crate::parser::Lei;
use crate::parser_executor::{parse_on_directory, Folder};
use prettytable::Table;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::time::Instant;

mod error;
mod parser;
mod parser_executor;

fn main() -> Result<(), Error> {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1]; // TODO: error handler

    let (directories, leis) = parse_on_directory(directory_path);

    let total_files = directories
        .iter()
        .map(|(_, folder)| folder.total)
        .sum::<i32>();
    println!("\nTotal de arquivos: {}", total_files);
    print_report(&directories);
    write_json_file(&leis);

    println!("Tempo de execução: {} segundos", now.elapsed().as_secs());
    Ok(())
}

fn write_json_file(leis: &[Lei]) {
    let leis_file = File::create("leis.json").expect("Unable to create file");
    serde_json::to_writer_pretty(leis_file, &leis).expect("Unable to write data");
    println!(
        "\nArquivo salvo em: {}",
        format!("{}/leis.json", env::current_dir().unwrap().display())
    );
}

fn print_report(directories: &HashMap<String, Folder>) {
    let mut table = Table::new();
    table.set_titles(row!["Diretório", "Total", "Parseados", "Com erros",]);

    for (directory, folder) in directories {
        table.add_row(row![
            directory,
            folder.total,
            folder.parsed,
            (folder.total - folder.parsed)
        ]);
    }

    println!("\nResumo da execução:");
    table.printstd();
}
