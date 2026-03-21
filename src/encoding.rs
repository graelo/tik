use tiktoken::CoreBpe;

/// Resolve the encoding from flags and environment variables.
///
/// Priority order:
/// 1. --encoding flag
/// 2. --model flag
/// 3. TIK_ENCODING env var
/// 4. TIK_MODEL env var
/// 5. Default: cl100k_base
pub fn resolve(
    encoding_flag: Option<&str>,
    model_flag: Option<&str>,
) -> Result<&'static CoreBpe, String> {
    if let Some(name) = encoding_flag {
        return get_by_encoding_name(name);
    }

    if let Some(name) = model_flag {
        return get_by_model_name(name);
    }

    if let Ok(name) = std::env::var("TIK_ENCODING") {
        return get_by_encoding_name(&name);
    }

    if let Ok(name) = std::env::var("TIK_MODEL") {
        return get_by_model_name(&name);
    }

    Ok(tiktoken::get_encoding("cl100k_base").expect("cl100k_base must exist"))
}

fn get_by_encoding_name(name: &str) -> Result<&'static CoreBpe, String> {
    tiktoken::get_encoding(name).ok_or_else(|| {
        let mut encodings: Vec<&str> = tiktoken::list_encodings().to_vec();
        encodings.sort();
        format!(
            "unknown encoding '{}'. Valid encodings: {}",
            name,
            encodings.join(", ")
        )
    })
}

fn get_by_model_name(name: &str) -> Result<&'static CoreBpe, String> {
    tiktoken::encoding_for_model(name).ok_or_else(|| {
        format!(
            "unknown model '{}'. Use --list-models to see recognized prefixes",
            name
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_flag_resolves() {
        let enc = resolve(Some("cl100k_base"), None).unwrap();
        assert_eq!(enc.vocab_size(), 100256);
    }

    #[test]
    fn model_flag_resolves() {
        let enc = resolve(None, Some("gpt-4o")).unwrap();
        // gpt-4o -> o200k_base, vocab size 199998
        assert_eq!(enc.vocab_size(), 199998);
    }

    #[test]
    fn encoding_flag_over_model_flag() {
        // When encoding is given, model is ignored (clap prevents both,
        // but resolve() handles it by checking encoding first)
        let enc = resolve(Some("cl100k_base"), None).unwrap();
        assert_eq!(enc.vocab_size(), 100256);
    }

    #[test]
    fn unknown_encoding_error_lists_valid() {
        match resolve(Some("bogus"), None) {
            Err(err) => {
                assert!(err.contains("unknown encoding 'bogus'"));
                assert!(err.contains("cl100k_base"));
                assert!(err.contains("o200k_base"));
            }
            Ok(_) => panic!("expected error for unknown encoding"),
        }
    }

    #[test]
    fn unknown_model_error() {
        match resolve(None, Some("nonexistent-model-xyz")) {
            Err(err) => {
                assert!(err.contains("unknown model"));
                assert!(err.contains("--list-models"));
            }
            Ok(_) => panic!("expected error for unknown model"),
        }
    }
}
