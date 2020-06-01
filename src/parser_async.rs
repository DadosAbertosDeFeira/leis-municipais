use crate::error::{CapturedOkOrUnexpected, Error};
use crate::parser::Lei;
use async_std::io;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use html_sanitizer::TagParser;
use regex::Regex;
use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref TITULO_REGEX: Regex = Regex::new("<h2>(?P<titulo>(.*))</h2>").unwrap();
    static ref RESUMO_REGEX: Regex = Regex::new("</h2><br>(?P<resumo>(.*))<br><br><img").unwrap();
    static ref TEXTO_REGEX: Regex = Regex::new("><br><br><br>(?P<texto>(.*))<p><img").unwrap();
    static ref DOCUMENTO_REGEX: Regex =
        Regex::new("btn-default\" href=\"(?P<documento>(.*))\" title").unwrap();
}

pub async fn parse_html_to_lei(file_name: &str, categoria: String) -> Result<Lei, Error> {
    let dest = read_file(file_name).await?;

    let captures_titulo = TITULO_REGEX
        .captures(&dest)
        .ok_or_unexpected("Título", file_name)?;
    let captures_resumo = RESUMO_REGEX
        .captures(&dest)
        .ok_or_unexpected("Resumo", file_name)?;
    let captures_texto = TEXTO_REGEX
        .captures(&dest)
        .ok_or_unexpected("Texto", file_name)?;
    let documento = DOCUMENTO_REGEX
        .captures(&dest)
        .map(|captures_documento| captures_documento["documento"].to_string());

    Ok(Lei {
        titulo: clean_html_to_text(&captures_titulo["titulo"]),
        resumo: clean_html_to_text(&captures_resumo["resumo"]),
        texto: clean_html_to_text(&captures_texto["texto"]),
        documento,
        categoria,
    })
}

fn clean_html_to_text(capture: &str) -> String {
    let mut tag_parser = TagParser::new(&mut capture.as_bytes());
    tag_parser.walk(|tag| {
        if tag.name == "br" {
            tag.rewrite_as("\n".to_string());
        } else {
            tag.ignore_self();
        }
    })
}

async fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut dest = String::new();
    transcoded
        .read_to_string(&mut dest)
        .expect("O conteúdo do arquivo não é UTF-8 válido");
    Ok(dest)
}
