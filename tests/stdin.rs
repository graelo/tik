use assert_cmd::Command;
use predicates::prelude::*;

fn tik() -> Command {
    Command::cargo_bin("tik").unwrap()
}

#[test]
fn piped_input_produces_count() {
    tik()
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn empty_piped_input_returns_zero() {
    tik().write_stdin("").assert().success().stdout("0\n");
}

// TTY detection test: assert_cmd always provides a pipe (not a TTY),
// so is_terminal() returns false and tk reads stdin (returning 0).
// The TTY usage hint behavior must be verified manually:
//   $ tik    # should print usage hint to stderr and exit 1
#[test]
fn no_args_no_pipe_reads_empty_stdin() {
    // With assert_cmd, stdin is a pipe (not TTY), so tik reads it
    // and gets 0 tokens. This verifies the stdin-reading path.
    tik().assert().success().stdout("0\n");
}
