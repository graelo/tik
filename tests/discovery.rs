use assert_cmd::Command;
use predicates::prelude::*;

fn tik() -> Command {
    Command::cargo_bin("tik").unwrap()
}

#[test]
fn list_encodings_outputs_sorted() {
    let output = tik()
        .arg("--list-encodings")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty(), "should have at least one encoding");

    // Verify sorted
    let mut sorted = lines.clone();
    sorted.sort();
    assert_eq!(lines, sorted, "encodings must be sorted alphabetically");

    // Verify core encodings are present
    let required = ["cl100k_base", "gpt2", "llama3", "o200k_base", "r50k_base"];
    for enc in &required {
        assert!(lines.contains(enc), "missing required encoding: {enc}");
    }
}

#[test]
fn list_models_outputs_tab_separated_sorted() {
    let output = tik()
        .arg("--list-models")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty(), "should have model prefix mappings");

    // Each line must be tab-separated
    for line in &lines {
        assert!(line.contains('\t'), "line should be tab-separated: {line}");
    }

    // Verify sorted by prefix (first column)
    let prefixes: Vec<&str> = lines
        .iter()
        .map(|l| l.split('\t').next().unwrap())
        .collect();
    let mut sorted = prefixes.clone();
    sorted.sort();
    assert_eq!(prefixes, sorted, "prefixes must be sorted alphabetically");
}

#[test]
fn list_models_ignores_file_args() {
    tik()
        .args(["--list-models", "nonexistent.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cl100k_base"));
}

#[test]
fn list_encodings_ignores_file_args() {
    // --list-encodings should work even if files are given
    tik()
        .args(["--list-encodings", "nonexistent.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cl100k_base"));
}
