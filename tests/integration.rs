#[cfg(test)]
mod integration {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn calling_leis_without_args() {
        let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
        cmd.arg("/Users/mac/workspace/leis-municipais/resources/integration_tests/leis");
        cmd.assert().stdout(predicate::str::contains("orgânica"));
    }
}
