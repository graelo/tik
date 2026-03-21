use std::fmt;
use std::path::Path;

use tiktoken::CoreBpe;

/// Errors that can occur when processing a file.
#[derive(Debug)]
pub(crate) enum FileError {
    NotFound(String),
    Binary(String),
    Read(String, String),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::NotFound(p) => write!(f, "{p}: No such file"),
            FileError::Binary(p) => write!(f, "{p}: Binary file"),
            FileError::Read(p, reason) => write!(f, "{p}: {reason}"),
        }
    }
}

/// Read a file and return its text content, detecting binary files.
pub(crate) fn read_text_file(path: &Path) -> Result<String, FileError> {
    let path_str = path.display().to_string();

    let content = std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => FileError::NotFound(path_str.clone()),
        std::io::ErrorKind::InvalidData => FileError::Binary(path_str.clone()),
        _ => FileError::Read(path_str.clone(), e.to_string()),
    })?;

    if content.contains('\0') {
        return Err(FileError::Binary(path_str));
    }

    Ok(content)
}

/// Count tokens in a file using the given encoding.
pub(crate) fn count_file(path: &Path, enc: &CoreBpe) -> Result<usize, FileError> {
    let text = read_text_file(path)?;
    Ok(enc.count(&text))
}

/// Count tokens from stdin using the given encoding.
pub(crate) fn count_stdin(enc: &CoreBpe) -> Result<usize, String> {
    let mut text = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut text)
        .map_err(|e| format!("failed to read stdin: {e}"))?;
    Ok(enc.count(&text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn read_utf8_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, "hello world").unwrap();
        let content = read_text_file(&path).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn read_binary_file_null_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("binary.bin");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(&[0x00, 0x01, 0x02]).unwrap();
        match read_text_file(&path) {
            Err(FileError::Binary(_)) => {}
            other => panic!("expected Binary error, got {other:?}"),
        }
    }

    #[test]
    fn read_nonexistent_file() {
        let path = Path::new("/tmp/tik_test_nonexistent_file_xyz");
        match read_text_file(path) {
            Err(FileError::NotFound(_)) => {}
            other => panic!("expected NotFound error, got {other:?}"),
        }
    }

    #[test]
    fn read_invalid_utf8_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("invalid.bin");
        // Invalid UTF-8: 0xFF is never valid in any position
        std::fs::write(&path, [0xFF, 0xFE, 0x68, 0x65, 0x6C, 0x6C, 0x6F]).unwrap();
        match read_text_file(&path) {
            Err(FileError::Binary(_)) => {}
            other => panic!("expected Binary error for invalid UTF-8, got {other:?}"),
        }
    }

    #[test]
    fn read_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.txt");
        std::fs::write(&path, "").unwrap();
        let content = read_text_file(&path).unwrap();
        assert_eq!(content, "");
    }
}
