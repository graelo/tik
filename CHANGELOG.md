# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- CI installs Rust with the `actions-rust-lang/setup-rust-toolchain` action
  pinned to the immutable `v2.0.0` tag, replacing the untagged
  `dtolnay/rust-toolchain` branch pin that triggered zizmor's
  `ref-version-mismatch` alerts; cargo invocations now deny warnings via
  the action's `build-warnings` default

## [0.2.0] - 2026-10-03

### Added

- Add a canonical `Makefile` for local formatting, linting, testing, audits,
  documentation checks, and release tasks
- Add a `tik(1)` manpage covering the CLI, environment variables, and output
  formats
- Add `AGENTS.md` with repository architecture, verification, and release
  guidance

### Changed

- Document the Makefile-based verification workflow in the README
- Add rumdl configuration with aligned Markdown table checking
- Include hidden commit sections in release changelog and align the release
  workflow with the playbook reference

### Removed

- Drop Windows support: remove the Windows targets from the CI platform
  matrix and stop publishing Windows release binaries. The `tiktoken` 4.x
  dependency overflows the 1 MiB main-thread stack on Windows when
  tokenizing non-ASCII text, and upstream does not test on Windows

## [0.1.3] - 2026-06-02

### Changed

- Dependency minor and patch updates (via Renovate)
- Align CI workflows with playbook v1.1 conventions and matrix job conventions

### Fixed

- Drop `cargo test --doc` from CI (binary-only crate has no lib target)

## [0.1.2] - 2026-05-14

### Added

- Windows release binaries (`x86_64-pc-windows-msvc`,
  `aarch64-pc-windows-msvc`) alongside the existing macOS and Linux musl
  targets

### Changed

- Bump Minimum Supported Rust Version (MSRV) to 1.95
- Release builds now use `cargo build --locked` for reproducibility
- Clarify dual licensing (MIT OR Apache-2.0) in README and align badges
  with the project's documentation style
- Dependency patch updates (via Renovate)

### Security

- Tighten CI security baseline with `--locked` builds and zizmor cleanup
- Adopt CI playbook conventions: explicit concurrency, least-privilege
  per-job permissions, and refreshed pinned action SHAs
- Restore poutine workflow analysis by removing the stale `.poutine.yml`
  and switching to the supported config layout
- Renovate now waits 3 days (`minimumReleaseAge`) before opening dependency
  PRs to reduce exposure to compromised upstream releases

## [0.1.1] - 2026-04-17

### Changed

- Linux release binaries are now statically linked against musl
  (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`), so they run on
  old distros without glibc version constraints
- Switch dependency updates from Dependabot to Renovate (runs Fridays)

### Security

- Harden GitHub Actions workflows: pin third-party actions to commit SHAs,
  scope per-job permissions with least privilege, move secrets from action
  inputs to `env` blocks, and scope release/renovate secrets to dedicated
  GitHub Environments
- Replace long-lived PATs with short-lived GitHub App tokens for release
  automation (Homebrew tap bump, Renovate)
- Add SLSA build provenance attestation to release artifacts
- Add zizmor and poutine for workflow and CI/CD supply-chain static analysis,
  extracted into reusable workflows
- Remove cache from release workflow to prevent cache poisoning
- Replace `ncipollo/release-action` with the built-in `gh release create` to
  reduce the supply chain surface

### Removed

- Drop `cargo outdated` from CI (superseded by Renovate)

## [0.1.0] - 2026-03-23

### Added

- Initial release: count LLM tokens in text files using tiktoken encodings
- Support for `cl100k_base`, `p50k_base`, `p50k_edit`, `r50k_base`, and
  `o200k_base` encodings, with model-name prefix resolution
- `--json` flag for machine-readable output
- `generate-completion` subcommand for shell completion scripts (bash, zsh,
  fish, PowerShell)
- Read from files or stdin

[Unreleased]: https://github.com/graelo/tik/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/graelo/tik/compare/v0.1.3...v0.2.0
