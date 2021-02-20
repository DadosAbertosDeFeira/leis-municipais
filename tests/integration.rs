use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn should_parser_folder_and_write_leis_to_file_as_json() {
    let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
    let file_name = "leis_em_json.json";
    cmd.arg("resources/integration_tests/leis");
    cmd.arg(file_name);

    cmd.assert()
        .stdout(predicate::str::contains("complementar | 2"));

    let actual_content =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let received_leis: serde_json::Value = serde_json::from_str(&actual_content).unwrap();
    let expected_content = fs::read_to_string("resources/integration_tests/leis.json")
        .expect("Something went wrong reading the file");
    let expected_leis: serde_json::Value = serde_json::from_str(&expected_content).unwrap();

    match (received_leis, expected_leis) {
        (serde_json::Value::Array(received_leis), serde_json::Value::Array(expected_leis)) => {
            received_leis.into_iter().for_each(|lei| {
                assert!(expected_leis.contains(&lei));
            })
        }
        _ => println!("unexpected type"),
    }
}

#[test]
fn should_not_parse_when_filename_to_write_is_not_json() {
    let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
    let file_name = "leis_em_json.anything";
    cmd.arg("resources/integration_tests/leis");
    cmd.arg(file_name);

    cmd.assert().stdout(predicate::str::contains(
        "Nome de arquivo não permitido. Extensão do arquivo deve ser .json",
    ));
}
