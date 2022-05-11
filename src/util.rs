use std::fs::File;
use std::io;

pub fn open<P: AsRef<std::path::Path>>(path: P) -> io::Result<File> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
}