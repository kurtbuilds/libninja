use std::path::Path;
use std::io::Write;
use std::io;
use std::fs::File;
pub use std::fs::{create_dir_all, remove_dir_all, read_to_string};
use crate::{fs, TEMPLATE_DIR};

pub fn write_file(path: &Path, text: &str) -> anyhow::Result<()> {
    let mut f = open(path)?;
    f.write_all(text.as_bytes())?;
    println!("{}: Wrote file.", path.display());
    Ok(())
}

pub fn open(path: &Path) -> io::Result<File> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub fn indent(s: &str, n: usize) -> String {
    let mut t = String::new();
    for line in s.trim().lines() {
        t.push_str(line);
        t.push('\n');
        for _ in 0..n {
            t.push(' ');
        }
    }
    t.trim_end().to_string()
}

fn copy_files_recursive(
    dest_path: &Path,
    dir: &include_dir::Dir,
    project_template: &str,
    ignore: &[&str],
) {
    for dir in dir.dirs() {
        let path = dir.path().strip_prefix(project_template).unwrap();
        if ignore.contains(&path.to_str().unwrap()) {
            continue;
        }
        copy_files_recursive(dest_path, dir, project_template, ignore);
    }
    for file in dir.files() {
        if file.path().extension().unwrap_or_default() == "j2" {
            continue;
        }
        let path = file.path().strip_prefix(project_template).unwrap();
        if ignore.contains(&path.to_str().unwrap()) {
            continue;
        }
        let path = dest_path.join(path);
        // Skip if the file already exists.
        if path.exists() {
            continue;
        }
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write_file(&path, file.contents_utf8().unwrap()).unwrap();
    }
}

/// Copy static files to the destination path.
pub fn copy_builtin_files(dest_path: &Path, project_template: &str, ignore: &[&str]) -> anyhow::Result<()> {
    copy_files_recursive(
        dest_path,
        TEMPLATE_DIR.get_dir(project_template).unwrap(),
        project_template,
        ignore,
    );
    Ok(())
}
