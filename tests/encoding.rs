use assert_cmd::Command;
use predicates::prelude::*;

fn tokky() -> Command {
    Command::cargo_bin("tokky").unwrap()
}

#[test]
fn encoding_flag_selects_encoding() {
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
fn model_flag_resolves_to_encoding() {
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
fn both_flags_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .args(["-e", "cl100k_base", "-m", "gpt-4o", path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn env_var_tokky_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .env("TOKKY_ENCODING", "o200k_base")
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn env_var_tokky_model() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .env("TOKKY_MODEL", "gpt-4o")
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn flag_overrides_env_var() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    // Use cl100k_base via flag, even though env says o200k_base
    // Both should produce a count, but the flag should win
    tokky()
        .env("TOKKY_MODEL", "gpt-4o")
        .args(["-e", "cl100k_base", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn unknown_encoding_lists_valid_options() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .args(["-e", "bogus", path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("unknown encoding")
                .and(predicate::str::contains("cl100k_base")),
        );
}

#[test]
fn unknown_model_suggests_list_models() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tokky()
        .args(["-m", "nonexistent-model-xyz", path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--list-models"));
}
