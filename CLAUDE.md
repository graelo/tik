# tik Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-21

## Active Technologies

- Rust, 2024 edition (latest stable) + `clap` (derive API), `tiktoken` 3.1.2 (goliajp) (001-token-counter-cli)

## Project Structure

```text
src/
├── main.rs          # Entry point, CLI arg parsing
├── encoding.rs      # Encoding resolution logic
├── count.rs         # Token counting (file, stdin)
└── output.rs        # Output formatting

tests/
└── integration/
    ├── single_file.rs
    ├── multi_file.rs
    ├── stdin.rs
    ├── encoding.rs
    └── discovery.rs
```

## Commands

```bash
cargo nextest run
cargo +nightly clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
cargo build --release
cargo deny check
```

## Code Style

Rust 2024 edition: Follow standard conventions. No unsafe code.

## Recent Changes

- 001-token-counter-cli: Added Rust 2024 + clap + tiktoken
- 001-token-counter-cli: Add cargo-deny license compliance configuration

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
