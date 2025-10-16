use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_gen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("treeedbgen-souffle-crystal")?;
    let tmp = NamedTempFile::new()?;
    cmd.arg("-o").arg(tmp.path());
    cmd.arg("--prefix=crystal").arg("--printsize");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let mut cmd = Command::cargo_bin("treeedb-crystal")?;
    cmd.arg("tests/crystal/hello-world.cr");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let mut souffle = Command::new("souffle");
    souffle.arg(tmp.path());
    souffle
        .assert()
        .success()
        .stdout(predicate::str::contains("crystal_node\t"))
        .stderr(predicate::str::is_empty());
    Ok(())
}
