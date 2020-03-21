#[cfg(test)]
mod integration {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn calling_leis_without_args() {
        let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
        cmd.arg("resources/integration_tests/leis");

        cmd.assert()
            .stdout(predicate::str::contains("diretório orgânica: 1 arquivos lidos"));
        cmd.assert()
            .stdout(predicate::str::contains("complementar: 2 arquivos lidos"));
    }
}
