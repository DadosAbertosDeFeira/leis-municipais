#[cfg(test)]
mod integration {
    use assert_cmd::Command;
    use predicates::prelude::*; // Used for writing assertions

    #[test]
    fn calling_leis_without_args() {
        let mut cmd = Command::cargo_bin("leis-municipais").unwrap();
        cmd.assert().stdout(predicate::str::contains("mateus"));
    }
}
