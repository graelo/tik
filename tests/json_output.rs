use assert_cmd::Command;
use predicates::prelude::*;

fn tik() -> Command {
    Command::cargo_bin("tik").unwrap()
}

#[test]
fn json_single_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    let output = tik()
        .arg("--json")
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(
        stdout.contains("\"token_count\": 2"),
        "expected token_count 2, got: {stdout}"
    );
    assert!(
        stdout.contains("\"file\": \""),
        "expected file key, got: {stdout}"
    );
    assert!(
        stdout.starts_with('['),
        "expected JSON array, got: {stdout}"
    );
    assert!(
        stdout.trim().ends_with(']'),
        "expected JSON array end, got: {stdout}"
    );
}

#[test]
fn json_multi_file() {
    let dir = tempfile::tempdir().unwrap();
    let path_a = dir.path().join("a.txt");
    let path_b = dir.path().join("b.txt");
    std::fs::write(&path_a, "hello world").unwrap();
    std::fs::write(&path_b, "the quick brown fox jumps over the lazy dog").unwrap();

    let output = tik()
        .arg("--json")
        .arg(path_a.to_str().unwrap())
        .arg(path_b.to_str().unwrap())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // Should have two entries
    assert_eq!(
        stdout.matches("\"token_count\"").count(),
        2,
        "expected 2 entries, got: {stdout}"
    );
    // Order matches argument order
    let pos_a = stdout.find("a.txt").expect("a.txt not found");
    let pos_b = stdout.find("b.txt").expect("b.txt not found");
    assert!(pos_a < pos_b, "expected a.txt before b.txt");
}

#[test]
fn json_stdin() {
    tik()
        .arg("--json")
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"file\": null"))
        .stdout(predicate::str::contains("\"token_count\": 2"));
}

#[test]
fn json_missing_file_errors_on_stderr() {
    let dir = tempfile::tempdir().unwrap();
    let good = dir.path().join("good.txt");
    std::fs::write(&good, "hello world").unwrap();
    let bad = dir.path().join("nonexistent.txt");

    let output = tik()
        .arg("--json")
        .arg(good.to_str().unwrap())
        .arg(bad.to_str().unwrap())
        .assert()
        .failure()
        .get_output()
        .clone();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Stdout has valid JSON with the successful file only
    assert!(
        stdout.contains("\"token_count\": 2"),
        "good file should appear in JSON"
    );
    assert!(
        !stdout.contains("nonexistent"),
        "bad file should not appear in JSON stdout"
    );

    // Stderr has the error
    assert!(stderr.contains("No such file"), "error should be on stderr");
}

#[test]
fn json_empty_array_for_all_binary() {
    let dir = tempfile::tempdir().unwrap();
    let bin_a = dir.path().join("a.bin");
    let bin_b = dir.path().join("b.bin");
    std::fs::write(&bin_a, b"\x00\x01\x02").unwrap();
    std::fs::write(&bin_b, b"\xff\xfe\x00").unwrap();

    tik()
        .arg("--json")
        .arg(bin_a.to_str().unwrap())
        .arg(bin_b.to_str().unwrap())
        .assert()
        .success()
        .stdout("[]\n");
}
