use std::fs;
use std::path::{Path, PathBuf};

/// Look for the first file satisfying a condition.
pub fn first<F>(directory: &Path, condition: F) -> Option<PathBuf> where F: Fn(&Path) -> bool {
    macro_rules! ok(
        ($result:expr) => (
            match $result {
                Ok(ok) => ok,
                Err(_) => return None,
            }
        );
    );

    if !ok!(fs::metadata(directory)).is_dir() {
        return None;
    }

    for entry in ok!(fs::read_dir(&directory)) {
        let entry = ok!(entry);
        if ok!(fs::metadata(entry.path())).is_dir() {
            continue;
        }
        if condition(&entry.path()) {
            return Some(entry.path());
        }
    }

    None
}

/// Look for the first file with a particular extension.
pub fn with_extension(directory: &Path, extension: &str) -> Option<PathBuf> {
    use std::ascii::AsciiExt;

    macro_rules! ok(
        ($option:expr) => (
            match $option {
                Some(some) => some,
                None => return false,
            }
        );
    );

    first(directory, |path| -> bool {
        ok!(ok!(path.extension()).to_str()).to_ascii_lowercase() == extension
    })
}
