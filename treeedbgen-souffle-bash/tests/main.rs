use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_gen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("treeedbgen-souffle-bash")?;
    let tmp = NamedTempFile::new()?;
    cmd.arg("-o").arg(tmp.path());
    cmd.arg("--prefix=sh").arg("--printsize");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let mut cmd = Command::cargo_bin("treeedb-bash")?;
    cmd.arg("tests/bash/hello-world.sh");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let mut souffle = Command::new("souffle");
    souffle.arg(tmp.path());
    souffle
        .assert()
        .success()
        .stdout(predicate::str::contains("sh_node\t61"))
        .stderr(predicate::str::is_empty());
    Ok(())
}
