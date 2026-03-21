use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

fn tik() -> Command {
    Command::cargo_bin("tik").unwrap()
}

#[test]
fn two_valid_files() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    std::fs::write(&a, "hello").unwrap();
    std::fs::write(&b, "world").unwrap();

    let output = tik()
        .arg(a.to_str().unwrap())
        .arg(b.to_str().unwrap())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains('\t'));
    assert!(lines[1].contains('\t'));
}

#[test]
fn binary_file_silently_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let text = dir.path().join("text.txt");
    let bin = dir.path().join("binary.bin");
    std::fs::write(&text, "hello world").unwrap();
    let mut f = std::fs::File::create(&bin).unwrap();
    f.write_all(&[0x00, 0x01, 0x02]).unwrap();

    let output = tik()
        .arg(text.to_str().unwrap())
        .arg(bin.to_str().unwrap())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "binary file should be silently skipped");
    assert!(lines[0].starts_with(text.to_str().unwrap()));
}

#[test]
fn partial_failure_missing_file() {
    let dir = tempfile::tempdir().unwrap();
    let valid = dir.path().join("valid.txt");
    std::fs::write(&valid, "hello world").unwrap();

    tik()
        .arg(valid.to_str().unwrap())
        .arg("/tmp/tik_test_nonexistent_xyz")
        .assert()
        .failure()
        .stdout(predicate::str::contains(valid.to_str().unwrap()))
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn output_order_matches_argument_order() {
    let dir = tempfile::tempdir().unwrap();
    let z = dir.path().join("z.txt");
    let a = dir.path().join("a.txt");
    std::fs::write(&z, "first").unwrap();
    std::fs::write(&a, "second").unwrap();

    let output = tik()
        .arg(z.to_str().unwrap())
        .arg(a.to_str().unwrap())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].starts_with(z.to_str().unwrap()));
    assert!(lines[1].starts_with(a.to_str().unwrap()));
}
