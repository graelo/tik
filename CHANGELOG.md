# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
