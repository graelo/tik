/// Print a single token count (single-file or stdin mode).
pub(crate) fn print_single(count: usize) {
    println!("{count}");
}

/// Print a filename and count in multi-file mode (tab-separated).
pub(crate) fn print_multi(path: &std::path::Path, count: usize) {
    println!("{}\t{count}", path.display());
}

/// Print results as a JSON array.
///
/// Each entry is `{"file": "path", "token_count": N}` or `{"file": null, "token_count": N}` for
/// stdin.
pub(crate) fn print_json(results: &[(Option<&std::path::Path>, usize)]) {
    print!("[");
    for (i, (path, count)) in results.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        match path {
            Some(p) => {
                let escaped = p
                    .display()
                    .to_string()
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"");
                print!("\n  {{\"file\": \"{escaped}\", \"token_count\": {count}}}");
            }
            None => {
                print!("\n  {{\"file\": null, \"token_count\": {count}}}");
            }
        }
    }
    if results.is_empty() {
        println!("]");
    } else {
        println!("\n]");
    }
}

/// Print all available encoding names, sorted alphabetically, one per line.
pub(crate) fn print_list_encodings() {
    let mut names: Vec<&str> = tiktoken::list_encodings().to_vec();
    names.sort();
    for name in names {
        println!("{name}");
    }
}

/// Print model prefix → encoding mappings, sorted by prefix.
pub(crate) fn print_list_models() {
    let prefixes: &[(&str, &str)] = &[
        ("Codestral", "mistral_v3"),
        ("DeepSeek", "deepseek_v3"),
        ("Llama-", "llama3"),
        ("Meta-Llama-", "llama3"),
        ("Mistral", "mistral_v3"),
        ("Mixtral", "mistral_v3"),
        ("Pixtral", "mistral_v3"),
        ("Qwen", "qwen2"),
        ("ada", "r50k_base"),
        ("babbage", "r50k_base"),
        ("chatgpt-4o", "o200k_base"),
        ("code-cushman", "p50k_base"),
        ("code-davinci", "p50k_base"),
        ("codestral", "mistral_v3"),
        ("curie", "r50k_base"),
        ("davinci", "r50k_base"),
        ("deepseek", "deepseek_v3"),
        ("gpt-3.5", "cl100k_base"),
        ("gpt-4", "cl100k_base"),
        ("gpt-4.1", "o200k_base"),
        ("gpt-4o", "o200k_base"),
        ("llama-", "llama3"),
        ("llama3", "llama3"),
        ("llama4", "llama3"),
        ("mistral", "mistral_v3"),
        ("mixtral", "mistral_v3"),
        ("o1", "o200k_base"),
        ("o3", "o200k_base"),
        ("o4-mini", "o200k_base"),
        ("pixtral", "mistral_v3"),
        ("qwen", "qwen2"),
        ("text-ada", "r50k_base"),
        ("text-babbage", "r50k_base"),
        ("text-curie", "r50k_base"),
        ("text-davinci-001", "r50k_base"),
        ("text-davinci-002", "p50k_base"),
        ("text-davinci-003", "p50k_base"),
        ("text-embedding-3", "cl100k_base"),
        ("text-embedding-ada", "cl100k_base"),
    ];

    for (prefix, encoding) in prefixes {
        println!("{prefix}\t{encoding}");
    }
}
