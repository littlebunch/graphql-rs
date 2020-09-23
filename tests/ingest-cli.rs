use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
#[test]
fn csv_doesnot_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ingest-csv")?;
    cmd.arg("-p").arg("/tmp/file/doesnt/exist.csv");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
#[test]
fn arg_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ingest-csv")?;
    cmd.arg("-s");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Found argument '-s'"));
    Ok(())
}
#[test]
fn arg_required() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ingest-csv")?;
    cmd.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
    Ok(())
}
#[test]
fn arg_type_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ingest-csv")?;
    cmd.arg("-t").arg("NONE").arg("-p").arg("/tmp");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid input type"));
    Ok(())
}
