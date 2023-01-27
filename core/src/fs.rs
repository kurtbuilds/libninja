use std::path::Path;
use std::io::Write;
use std::io;
use std::fs::File;

pub fn write_file(path: &Path, text: &str) -> anyhow::Result<()> {
    let mut f = open(path)?;
    f.write_all(text.as_bytes())?;
    println!("{}: Wrote file.", path.display());
    Ok(())
}

pub fn open<P: AsRef<std::path::Path>>(path: P) -> io::Result<File> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
}
