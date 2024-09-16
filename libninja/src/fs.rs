use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

pub fn open(path: &Path) -> io::Result<File> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}
