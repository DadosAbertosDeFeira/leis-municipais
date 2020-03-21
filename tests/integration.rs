#[cfg(test)]
mod integration {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    #[test]
    fn should_parser_folder_and_write_leis_to_file_as_json() {
        let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
        cmd.arg("resources/integration_tests/leis");

        cmd.assert()
            .stdout(predicate::str::contains("diretório orgânica: 1 arquivos lidos"));
        cmd.assert()
            .stdout(predicate::str::contains("complementar: 2 arquivos lidos"));

        let actual_content = fs::read_to_string("leis.json").expect("Something went wrong reading the file");
        let expected_content = fs::read_to_string("resources/integration_tests/leis.json").expect("Something went wrong reading the file");
        assert_eq!(actual_content, expected_content);
    }
}
