use std::fs::File;
use std::io::Read;
use encoding_rs_io::DecodeReaderBytesBuilder;
use encoding_rs::WINDOWS_1252;

fn main() {
    parse_html();
}

fn parse_html() {
    let file_name = "resources/LeisMunicipais-com-br-Decreto-1-1984.html";
    let file = File::open(file_name).unwrap(); // TODO: handle error here
    let mut transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut dest = String::new();
    transcoded.read_to_string(&mut dest).unwrap(); // TODO: handle error here
    println!("{:?}", dest);
}

// TODO: make path dynamic

#[cfg(test)]
mod test {

    #[test]
    fn should_read_html_and_print_it() {
        assert_eq!(1, 1);
    }
}
