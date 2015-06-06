//! Auxiliary routines for testing.

use std::fs;
use std::path::Path;

pub mod find;
pub mod copy;

/// Check if a path exists and is not a directory.
pub fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn exists() {
        assert!(::exists(&Path::new("src/lib.rs")));
        assert!(!::exists(&Path::new("src")));
    }
}
