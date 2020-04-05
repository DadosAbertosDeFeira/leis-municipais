use crate::error::Error;
use crate::parser_executor::parse_on_directory;
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
