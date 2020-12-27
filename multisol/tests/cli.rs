use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn contract_does_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("multisol")?;

    cmd.arg("test/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Contract does not exist"));

    Ok(())
}

#[test]
fn provided_path_is_directory() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("multisol")?;

    cmd.arg("datasets");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Provided path is a directory, not a contract"));

    Ok(())
}

#[test]
fn contract_file_sol_extension() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("multisol")?;

    cmd.arg("Cargo.toml");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Contract file does not have the \"sol\" extension",
    ));

    Ok(())
}
