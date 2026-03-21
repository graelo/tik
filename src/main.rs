mod count;
mod encoding;
mod output;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

/// Count LLM tokens in text files.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// BPE encoding name (e.g. cl100k_base, o200k_base)
    #[arg(short, long, conflicts_with = "model")]
    encoding: Option<String>,

    /// Model name that resolves to an encoding (e.g. gpt-4o, llama-3.1-70b)
    #[arg(short, long, conflicts_with = "encoding")]
    model: Option<String>,

    /// Print available encodings and exit
    #[arg(long)]
    list_encodings: bool,

    /// Print model prefix → encoding mappings and exit
    #[arg(long)]
    list_models: bool,

    /// Files to tokenize. Reads stdin if omitted.
    files: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> ExitCode {
    if args.list_encodings {
        output::print_list_encodings();
        return ExitCode::SUCCESS;
    }

    if args.list_models {
        output::print_list_models();
        return ExitCode::SUCCESS;
    }

    let enc = match encoding::resolve(args.encoding.as_deref(), args.model.as_deref()) {
        Ok(enc) => enc,
        Err(msg) => {
            eprintln!("tik: {msg}");
            return ExitCode::FAILURE;
        }
    };

    if args.files.is_empty() {
        if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
            eprintln!("tik: no input. Usage: tik [OPTIONS] [FILE...]");
            return ExitCode::FAILURE;
        }
        match count::count_stdin(enc) {
            Ok(n) => {
                output::print_single(n);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("tik: {e}");
                ExitCode::FAILURE
            }
        }
    } else if args.files.len() == 1 {
        match count::count_file(&args.files[0], enc) {
            Ok(n) => {
                output::print_single(n);
                ExitCode::SUCCESS
            }
            Err(count::FileError::NotFound(p)) => {
                eprintln!("tik: {p}: No such file");
                ExitCode::FAILURE
            }
            Err(count::FileError::Binary(p)) => {
                eprintln!("tik: {p}: Binary file");
                ExitCode::FAILURE
            }
            Err(count::FileError::Read(p, reason)) => {
                eprintln!("tik: {p}: {reason}");
                ExitCode::FAILURE
            }
        }
    } else {
        let mut had_error = false;
        for file in &args.files {
            match count::count_file(file, enc) {
                Ok(n) => output::print_multi(file, n),
                Err(count::FileError::Binary(_)) => {}
                Err(count::FileError::NotFound(p)) => {
                    eprintln!("tik: {p}: No such file");
                    had_error = true;
                }
                Err(count::FileError::Read(p, reason)) => {
                    eprintln!("tik: {p}: {reason}");
                    had_error = true;
                }
            }
        }
        if had_error {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
