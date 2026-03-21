use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

fn tik() -> Command {
    Command::cargo_bin("tik").unwrap()
}

#[test]
fn single_file_default_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    // "hello world" is 2 tokens with cl100k_base (default)
    tik()
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout("2\n");
}

#[test]
fn single_file_with_encoding_flag() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    // Use text that produces different token counts per encoding
    std::fs::write(&path, "日本語のテキストをトークン化する").unwrap();

    // o200k_base: 12 tokens for this text
    tik()
        .args(["-e", "o200k_base", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout("12\n");
}

#[test]
fn single_file_with_model_flag() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    // Use text that produces different token counts per encoding
    std::fs::write(&path, "日本語のテキストをトークン化する").unwrap();

    // gpt-4o -> o200k_base: 12 tokens
    tik()
        .args(["-m", "gpt-4o", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout("12\n");
}

#[test]
fn single_file_not_found() {
    tik()
        .arg("/tmp/tik_test_nonexistent_xyz")
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

    tik()
        .arg(path.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Binary file"));
}

#[test]
fn single_file_whitespace_only() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("spaces.txt");
    std::fs::write(&path, "   \n\t\n  ").unwrap();

    tik()
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn single_file_invalid_utf8() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("invalid.bin");
    std::fs::write(&path, [0xFF, 0xFE, 0x68, 0x65]).unwrap();

    tik()
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

    tik()
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout("0\n");
}
