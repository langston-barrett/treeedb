use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn gen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("treeedbgen-souffle-souffle")?;
    let tmp = NamedTempFile::new()?;
    cmd.arg("-o").arg(tmp.path());
    cmd.arg("--prefix=dl").arg("--printsize");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let mut cmd = Command::cargo_bin("treeedb-souffle")?;
    cmd.arg("tests/dl/hello-world.dl");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let mut souffle = Command::new("souffle");
    souffle.arg(tmp.path());
    souffle
        .assert()
        .success()
        .stdout(predicate::str::contains("dl_node\t12"))
        .stderr(predicate::str::is_empty());
    Ok(())
}
