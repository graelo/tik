# tik

Count LLM tokens in text files.

[![CI](https://github.com/graelo/tik/actions/workflows/essentials.yml/badge.svg)](https://github.com/graelo/tik/actions/workflows/essentials.yml)
[![crates.io](https://img.shields.io/crates/v/tik.svg)](https://crates.io/crates/tik)
[![MSRV](https://img.shields.io/badge/MSRV-1.94-blue)](https://github.com/graelo/tik)
[![license](https://img.shields.io/crates/l/tik.svg)](LICENSE)

## Example

```console
$ tik src/main.rs
832

$ tik src/*.rs
src/main.rs	832
src/encoding.rs	614
src/count.rs	597
src/output.rs	312

$ cat prompt.txt | tik
1423

$ tik -m gpt-4o src/main.rs
791
```

## Synopsis

```text
tik [OPTIONS] [FILE...]
```

Counts BPE tokens for a given encoding. Reads from stdin when no files are
given. Binary files are silently skipped in multi-file mode.

Supports encodings for OpenAI, Meta (Llama), DeepSeek, Qwen, and Mistral
models.

## Options

| Flag                    | Description                                                              |
| ----------------------- | ------------------------------------------------------------------------ |
| `-e`, `--encoding NAME` | BPE encoding (`cl100k_base`, `o200k_base`, ...)                          |
| `-m`, `--model NAME`    | Model name that resolves to an encoding (`gpt-4o`, `llama-3.1-70b`, ...) |
| `--list-encodings`      | Print available encodings                                                |
| `--list-models`         | Print model-to-encoding mappings                                         |
| `-h`, `--help`          | Print help                                                               |
| `-V`, `--version`       | Print version                                                            |

`-e` and `-m` are mutually exclusive. Default encoding: `cl100k_base`.

## Environment

| Variable       | Description                           |
| -------------- | ------------------------------------- |
| `TIK_ENCODING` | Default encoding (overridden by `-e`) |
| `TIK_MODEL`    | Default model (overridden by `-m`)    |

Flag > env var > built-in default.

## Output

- **Single file / stdin** — bare integer, newline-terminated.
- **Multiple files** — tab-separated `<path>\t<count>`, one per line. Composes
  with `sort`, `awk`, `cut`.

## Installation

### From source

```console
cargo install --git https://github.com/graelo/tik
```

### Binary releases

Pre-built binaries for macOS (ARM, x86) and Linux (x86, ARM) are available on
the [releases page](https://github.com/graelo/tik/releases).

## License

MIT
