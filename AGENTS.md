# AGENTS.md

This file contains instructions for coding agents working in this repository.

- Repository: <https://github.com/graelo/tik>
- Prefer `gh` for GitHub operations.
- Do not mention an agent or assistant in issues, pull requests, comments, or
  commit messages.
- Do not expose private local information, including machine-specific paths.

## Project

`tik` is a command-line tool that counts BPE tokens in text files or stdin. It
is a binary-only crate and uses `clap` for argument parsing and `tiktoken` for
encoding and tokenization.

Rust 1.95 or later is required. The crate uses edition 2024.

## Architecture

- `src/main.rs`: CLI definition, command dispatch, file/stdin processing, and
  exit-status handling.
- `src/encoding.rs`: encoding and model resolution, including environment
  variable fallbacks.
- `src/count.rs`: UTF-8 and binary-file handling plus token counting.
- `src/output.rs`: plain-text and JSON output formatting.
- `tests/`: integration coverage for file discovery, encoding selection, stdin,
  and output behavior.
- `ci/test_full.sh`: MSRV check, locked builds, Nextest runs, and the release
  binary smoke test.

There is no `src/lib.rs` or public library API. Keep the crate binary-only
unless a task explicitly calls for a library target.

## Verification

The `Makefile` is the canonical definition of local verification tasks. **Read
it before choosing or running verification commands**; do not duplicate its
command implementations here. `make help` lists every target.

The primary targets are:

- `make check`: pre-push gate (formatting, linting, and tests).
- `make check-all`: pre-PR gate (adds dependency, commit-message, Markdown,
  manpage, and GitHub Actions security checks).
- `make fix`: formats code and applies Clippy fixes.
- `make md`: lints Markdown against `rumdl.toml`.
- `make man`: lints the `tik(1)` roff manpage.
- `make ci-security`: runs the Poutine and Zizmor GitHub Actions scans.

The check targets mirror the GitHub workflows and use locked dependency
resolution where applicable. They assume their external tools (for example
`cargo-nextest`, `cargo-deny`, `cargo-pants`, `convco`, `poutine`, `zizmor`,
`rumdl`, `mandoc`, and `cargo-llvm-cov`) are already installed locally.

For focused Rust tests, use `cargo nextest run <test_name>` or
`cargo nextest run <module::tests::name>`. The complete CI test sequence is
implemented in `ci/test_full.sh`; its Nextest CI profile is configured in
`.config/nextest.toml`.

## Documentation and releases

Keep user-facing documentation in sync with behavior:

- Update `README.md` and `man/tik.1` when changing CLI flags, defaults,
  environment variables, output formats, or examples.
- Lint Markdown with `make md` and the manpage with `make man`.
- For a release version bump, update `Cargo.toml`, `Cargo.lock`, the versioned
  section and comparison links in `CHANGELOG.md`, and the manpage `.TH` header.
  Create a `vX.Y.Z` tag; the release workflow derives artifact and GitHub
  Release versions from it.
- Commit messages must follow `.convco` Conventional Commit rules. Use
  `make commits` to check them.

`Cargo.toml`, `Cargo.lock`, `deny.toml`, and the GitHub workflows define the
release and supply-chain constraints. Preserve `--locked` behavior in Cargo
commands that resolve dependencies.
