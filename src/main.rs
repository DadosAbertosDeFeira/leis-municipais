use crate::error::Error;
use crate::parser::Lei;
use crate::parser_executor::{parse_on_directory, Folder};
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

    print_report(&directories);
    write_json_file(&leis);

    println!("Tempo de execucao: {} segundos", now.elapsed().as_secs());
    Ok(())
}

fn write_json_file(leis: &[Lei]) {
    let leis_file = File::create("leis.json").expect("Unable to create file");
    serde_json::to_writer(leis_file, &leis).expect("Unable to write data");
}

fn print_report(directories: &HashMap<String, Folder>) {
    for (directory, folder) in directories {
        println!(
            "diretorio {}: total {}, {} processados, {} com erros",
            directory,
            folder.total,
            folder.parsed,
            (folder.total - folder.parsed)
        );
    }
}

// diretorio Lei Promulgada: total 346, 345 processados, 1 com erros
// diretorio Emendas à Lei Orgânica: total 27, 27 processados, 0 com erros
// diretorio Regimento interno: total 1, 1 processados, 0 com erros
// diretorio Leis Ordinárias: total 3762, 3367 processados, 395 com erros
// diretorio Lei Orgânica: total 1, 0 processados, 1 com erros
// diretorio Resoluções: total 525, 523 processados, 2 com erros
// diretorio Leis Complementares: total 128, 124 processados, 4 com erros
// diretorio Decretos Legislativos: total 1431, 1428 processados, 3 com erros
// diretorio Decretos: total 218, 218 processados, 0 com erros
