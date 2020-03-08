#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;
use std::io::Read;
use unhtml::FromHtml;

fn main() {
    println!("{:?}", parse_html_to_lei());
}

#[derive(FromHtml, Debug, PartialEq)]
struct Lei {
    #[html(selector = "h2", attr = "inner")]
    titulo: String,
    // categoria: String,
}

fn parse_html_to_lei() -> Lei {
    let file_name = "resources/LeisMunicipais-com-br-Decreto-1-1984.html";
    let file = File::open(file_name).unwrap(); // TODO: handle error here
    let mut transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut dest = String::new();
    transcoded.read_to_string(&mut dest).unwrap(); // TODO: handle error here

    Lei::from_html(&dest).unwrap()
}

// TODO: make path dynamic

#[cfg(test)]
mod test {
    use crate::{parse_html_to_lei, Lei};

    #[test]
    fn should_read_html_and_print_it() {
        assert_eq!(
            parse_html_to_lei(),
            Lei {
                titulo: "DECRETO Nº 1/84, de 05 de janeiro de 1984".to_string()
            }
        );
    }
}
