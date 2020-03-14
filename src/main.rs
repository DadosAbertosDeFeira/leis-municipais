use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("{:?}", parse_html_to_lei());
}

#[derive(Debug, PartialEq)]
struct Lei {
    titulo: String,
    // categoria: String,
    resumo: String,
}

fn parse_html_to_lei() -> Lei {
    let file_name = "resources/LeisMunicipais-com-br-Lei-Complementar-122-2019.html";
    let file = File::open(file_name).unwrap(); // TODO: handle error here
    let mut transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut dest = String::new();
    transcoded.read_to_string(&mut dest).unwrap(); // TODO: handle error here

    let titulo_regex = Regex::new(r"<h2>(?P<titulo>(.*))</h2>").unwrap();
    let resumo_regex = Regex::new(r"</h2><br>(?P<resumo>(.*))<br><br><img").unwrap();

    let captures = titulo_regex.captures(&dest).unwrap();
    let captures_resumo = resumo_regex.captures(&dest).unwrap();
    // println!("{:?}", &captures["titulo"]);

    Lei {
        titulo: captures["titulo"].to_string(),
        resumo: captures_resumo["resumo"].to_string(),
    }
}

// TODO: make path dynamic
// * remove tags from extract text

#[cfg(test)]
mod test {
    use crate::{parse_html_to_lei, Lei};

    #[test]
    fn should_read_html_and_print_it() {
        assert_eq!(
            parse_html_to_lei(),
            Lei {
                titulo: "LEI COMPLEMENTAR Nº 122, DE 22 DE FEVEREIRO DE 2019".to_string(),
                resumo: "Altera as disposições da Lei Complementar Nº <a class=\"link_law\" data-id=\"1592745\" data-original-title=\" Data da Norma: 10.04.2002 - ALTERA O REGIME DE PREVIDÊNCIA SOCIAL PRÓPRIO DO MUNICÍPIO DE FEIRA DE SANTANA E DÁ OUTRAS PROVIDÊNCIAS.\" data-toggle=\"tooltip\" href=\"https://leismunicipais.com.brhttp://www2.leismunicipais.com.br/a/ba/f/feira-de-santana/lei-complementar/2002/1/11/lei-complementar-n-11-2002-altera-o-regime-de-previdencia-social-proprio-do-municipio-de-feira-de-santana-e-da-outras-providencias\" rel=\"tooltip\">11</a>/2002 que trata do modo de concessão de pensão por morte, em concordância a Lei Federal de nº 13.135 de 17/06/2015 e Nota Técnica nº 11/2015/CGNAL/DRPSP/SPPS, de 14/08/2015, e dá outras providências.".to_string()
            }
        );
    }
}
