use std::fs;
use std::path::{Path, PathBuf};

/// Look for the first file satisfying a condition.
pub fn first<F>(path: &Path, condition: F) -> Option<PathBuf> where F: Fn(&Path) -> bool {
    macro_rules! ok(
        ($result:expr) => (
            match $result {
                Ok(ok) => ok,
                Err(_) => return None,
            }
        );
    );

    if !ok!(fs::metadata(path)).is_dir() {
        return None;
    }

    for entry in ok!(fs::read_dir(&path)) {
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
pub fn extension(path: &Path, extension: &str) -> Option<PathBuf> {
    use std::ascii::AsciiExt;

    macro_rules! ok(
        ($option:expr) => (
            match $option {
                Some(some) => some,
                None => return false,
            }
        );
    );

    first(path, |path| -> bool {
        ok!(ok!(path.extension()).to_str()).to_ascii_lowercase() == extension
    })
}
