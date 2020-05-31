use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
#[ignore]
fn should_parser_folder_and_write_leis_to_file_as_json() {
    let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
    cmd.arg("resources/integration_tests/leis");

    cmd.assert()
        .stdout(predicate::str::contains("complementar | 2"));

    let actual_content =
        fs::read_to_string("leis.json").expect("Something went wrong reading the file");
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
