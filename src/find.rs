use std::fs;
use std::path::{Path, PathBuf};

/// A decision with respect to a file.
pub enum Verdict {
    /// Accept and keep searching.
    AcceptMore,
    /// Accept and stop searching.
    AcceptStop,
    /// Reject and keep searching.
    RejectMore,
    /// Reject and stop searching.
    RejectStop,
}

/// Look for all files satisfying a condition.
pub fn all<F>(directory: &Path, condition: F) -> Vec<PathBuf> where F: Fn(&Path) -> bool {
    some(directory, |path| {
        if condition(path) { Verdict::AcceptMore } else { Verdict::RejectMore }
    })
}

/// Look for the first file satisfying a condition.
pub fn first<F>(directory: &Path, condition: F) -> Option<PathBuf> where F: Fn(&Path) -> bool {
    some(directory, |path| {
        if condition(path) { Verdict::AcceptStop } else { Verdict::RejectMore }
    }).pop()
}

/// Look for the first file with a particular extension.
pub fn first_with_extension(directory: &Path, extension: &str) -> Option<PathBuf> {
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

/// Look for some files satisfying a condition.
pub fn some<F>(directory: &Path, condition: F) -> Vec<PathBuf> where F: Fn(&Path) -> Verdict {
    macro_rules! ok(
        ($result:expr) => (
            match $result {
                Ok(ok) => ok,
                Err(_) => return Vec::new(),
            }
        );
    );

    let mut paths = Vec::new();

    for entry in ok!(fs::read_dir(&directory)) {
        let entry = ok!(entry);
        if ok!(fs::metadata(entry.path())).is_dir() {
            continue;
        }
        let path = entry.path();
        match condition(&path) {
            Verdict::AcceptMore => paths.push(path),
            Verdict::AcceptStop => {
                paths.push(path);
                break;
            },
            Verdict::RejectMore => {},
            Verdict::RejectStop => break,
        }
    }

    paths
}
