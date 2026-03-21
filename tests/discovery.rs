use assert_cmd::Command;
use predicates::prelude::*;

fn tokky() -> Command {
    Command::cargo_bin("tokky").unwrap()
}

#[test]
fn list_encodings_outputs_nine_lines_sorted() {
    let output = tokky()
        .arg("--list-encodings")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 9);

    // Verify sorted
    let mut sorted = lines.clone();
    sorted.sort();
    assert_eq!(lines, sorted, "encodings must be sorted alphabetically");

    // Verify known encodings present
    assert!(lines.contains(&"cl100k_base"));
    assert!(lines.contains(&"o200k_base"));
    assert!(lines.contains(&"llama3"));
}

#[test]
fn list_models_outputs_tab_separated_sorted() {
    let output = tokky()
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
fn list_encodings_ignores_file_args() {
    // --list-encodings should work even if files are given
    tokky()
        .args(["--list-encodings", "nonexistent.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cl100k_base"));
}
