use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

fn tokky() -> Command {
    Command::cargo_bin("tokky").unwrap()
}

#[test]
fn single_file_default_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn single_file_with_encoding_flag() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .args(["-e", "o200k_base", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn single_file_with_model_flag() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .args(["-m", "gpt-4o", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn single_file_not_found() {
    tokky()
        .arg("/tmp/tokky_test_nonexistent_xyz")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn single_file_binary() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("binary.bin");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(&[0x00, 0x01, 0x02]).unwrap();

    tokky()
        .arg(path.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Binary file"));
}

#[test]
fn single_file_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("empty.txt");
    std::fs::write(&path, "").unwrap();

    tokky()
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout("0\n");
}
