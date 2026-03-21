use assert_cmd::Command;
use predicates::prelude::*;

fn tik() -> Command {
    Command::cargo_bin("tik").unwrap()
}

#[test]
fn encoding_flag_selects_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "日本語のテキストをトークン化する").unwrap();

    // o200k_base: 12 tokens (vs cl100k_base: 15, r50k_base: 18)
    tik()
        .args(["-e", "o200k_base", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout("12\n");
}

#[test]
fn model_flag_resolves_to_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "日本語のテキストをトークン化する").unwrap();

    // gpt-4o -> o200k_base: 12 tokens
    tik()
        .args(["-m", "gpt-4o", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout("12\n");
}

#[test]
fn both_flags_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tik()
        .args(["-e", "cl100k_base", "-m", "gpt-4o", path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn env_var_tik_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tik()
        .env("TIK_ENCODING", "o200k_base")
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn env_var_tik_model() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tik()
        .env("TIK_MODEL", "gpt-4o")
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn flag_overrides_env_var() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "日本語のテキストをトークン化する").unwrap();

    // cl100k_base via flag (15 tokens) overrides TIK_MODEL=gpt-4o (o200k_base: 12 tokens)
    tik()
        .env("TIK_MODEL", "gpt-4o")
        .args(["-e", "cl100k_base", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout("15\n");
}

#[test]
fn all_nine_encodings_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    let encodings = [
        "cl100k_base",
        "o200k_base",
        "p50k_base",
        "p50k_edit",
        "r50k_base",
        "llama3",
        "deepseek_v3",
        "qwen2",
        "mistral_v3",
    ];
    for enc in encodings {
        tik()
            .args(["-e", enc, path.to_str().unwrap()])
            .assert()
            .success()
            .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
    }
}

#[test]
fn model_prefixes_across_providers() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    let models = [
        "gpt-4o",        // OpenAI -> o200k_base
        "llama-3.1-70b", // Meta -> llama3
        "deepseek-v3",   // DeepSeek -> deepseek_v3
        "qwen2.5",       // Alibaba -> qwen2
        "mistral-large", // Mistral -> mistral_v3
    ];
    for model in models {
        tik()
            .args(["-m", model, path.to_str().unwrap()])
            .assert()
            .success()
            .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
    }
}

#[test]
fn tik_encoding_env_takes_priority_over_tik_model_env() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "日本語のテキストをトークン化する").unwrap();

    // Both env vars set — TIK_ENCODING should win
    // cl100k_base: 15 tokens vs o200k_base (gpt-4o): 12 tokens
    tik()
        .env("TIK_ENCODING", "cl100k_base")
        .env("TIK_MODEL", "gpt-4o")
        .arg(path.to_str().unwrap())
        .assert()
        .success()
        .stdout("15\n");
}

#[test]
fn invalid_tik_encoding_env_var() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tik()
        .env("TIK_ENCODING", "bogus_encoding")
        .arg(path.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown encoding"));
}

#[test]
fn invalid_tik_model_env_var() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tik()
        .env("TIK_MODEL", "nonexistent-model-xyz")
        .arg(path.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown model"));
}

#[test]
fn unknown_encoding_lists_valid_options() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, "hello world").unwrap();

    tik()
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

    tik()
        .args(["-m", "nonexistent-model-xyz", path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--list-models"));
}
