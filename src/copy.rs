//! Routines for copying.

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind, Result};

/// Copy a file along with those files that the file refers to.
pub fn with_references(source: &Path, destination: &Path) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    macro_rules! some(
        ($option:expr) => (match $option {
            Some(some) => some,
            None => return Err(Error::new(ErrorKind::Other, "something went wrong")),
        });
    );

    macro_rules! next(
        ($writer:expr, $line:expr) => ({
            try!($writer.write($line.as_bytes()));
            try!($writer.write(b"\n"));
            continue;
        });
    );

    let from = some!(source.parent());
    let into = some!(destination.parent());

    let mut source = try!(File::open(source));
    let reader = BufReader::new(&mut source);

    let mut destination = try!(File::create(destination));
    let mut writer = BufWriter::new(&mut destination);

    for line in reader.lines() {
        let line = try!(line);

        let i = match line.find('"') {
            Some(i) => i,
            _ => next!(writer, line),
        };
        let j = match line.rfind('"') {
            Some(j) => j,
            _ => next!(writer, line),
        };

        let (prefix, middle, suffix) = (&line[..i+1], &line[i+1..j], &line[j..]);

        let path = PathBuf::from(middle);
        let name = match path.file_name() {
            Some(name) => PathBuf::from(name),
            _ => next!(writer, line),
        };

        let (source, destination) = if path.is_relative() {
            (from.join(&name), into.join(&name))
        } else {
            (path, into.join(&name))
        };

        let metadata = match fs::metadata(&source) {
            Ok(metadata) => metadata,
            _ => next!(writer, line),
        };
        if metadata.is_dir() {
            next!(writer, line);
        }

        try!(fs::copy(&source, &destination));
        try!(writer.write(prefix.as_bytes()));
        try!(writer.write(some!(destination.to_str()).as_bytes()));
        try!(writer.write(suffix.as_bytes()));
        try!(writer.write(b"\n"));
    }

    Ok(())
}
