use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_gen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("treeedbgen-souffle-swift")?;
    let tmp = NamedTempFile::new()?;
    cmd.arg("-o").arg(tmp.path());
    cmd.arg("--prefix=swift").arg("--printsize");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    // TODO(lb): Test!
    // let mut cmd = Command::cargo_bin("treeedb-swift")?;
    // cmd.arg("tests/TODO");
    // cmd.assert()
    //     .success()
    //     .stdout(predicate::str::is_empty())
    //     .stderr(predicate::str::is_empty());

    // TODO(lb): Install Soufflé in CI
    // let mut souffle = Command::new("souffle");
    // souffle.arg(tmp.path());
    // souffle
    //     .assert()
    //     .success()
    //     .stdout(predicate::str::contains("swift_node\tTODO"))
    //     .stderr(predicate::str::is_empty());
    Ok(())
}
