use ln_core::fs;
use mir_rust::ToRustCode;
use mir_rust::{format_code, RustExtra};
use proc_macro2::TokenStream;
use std::path::Path;

pub fn write_rust_file_to_path(
    path: &Path,
    file: mir::File<TokenStream, RustExtra>,
) -> anyhow::Result<()> {
    let code = file.to_rust_code();
    write_rust_code_to_path(path, code)
}

pub fn write_rust_code_to_path(path: &Path, code: TokenStream) -> anyhow::Result<()> {
    write_rust_to_path(path, code, "")
}

pub fn write_rust_to_path(path: &Path, code: TokenStream) -> anyhow::Result<()> {
    let code = format_code(code);
    let mut f = fs::open(path)?;
    let mut s = template.to_string();
    if !s.is_empty() && !s.ends_with('\n') {
        s += "\n";
    }
    s += &code;
    fs::write_file(path, &s)
}
