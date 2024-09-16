pub mod client;
mod example;
mod extras;
mod model;
pub mod request;
mod serde;

use crate::client::make_lib_rs;
use crate::example::write_examples_folder;
use crate::extras::calculate_extras;
use crate::model::write_model_module;
use crate::request::write_request_module;
use crate::serde::write_serde_module;
use anyhow::Result;
use hir::Config;
use hir::HirSpec;
use mir_rust::{format_code, ToRustCode};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub type Modified = HashSet<PathBuf>;

pub fn generate_rust_library(spec: HirSpec, cfg: Config) -> Result<()> {
    let src = cfg.dest.join("src");
    let extras = calculate_extras(&spec);

    let mut m: Modified = HashSet::new();
    fs::create_dir_all(&src)?;

    write_model_module(&spec, &cfg, &mut m)?;
    write_request_module(&spec, &cfg, &mut m)?;

    let file = make_lib_rs(&spec, &extras, &cfg);
    write_rust(&src.join("lib.rs"), file, &mut m)?;

    write_serde_module(&extras, &src, &mut m)?;

    // let spec = add_operation_models(opts.language, spec)?;

    if cfg.build_examples {
        write_examples_folder(&spec, &cfg, &mut m)?;
    }
    remove_old_files(&cfg.dest, &m)?;
    Ok(())
}

fn remove_old_files(dest: &Path, modified: &HashSet<PathBuf>) -> Result<()> {
    let mut to_delete: Vec<_> = fs::read_dir(dest.join("examples"))?
        .chain(fs::read_dir(dest.join("src"))?)
        .flat_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|e| e.ends_with(".rs"))
        .filter(|e| {
            fs::read_to_string(&e)
                .map(|content| !content.contains("libninja: static"))
                .unwrap_or(false)
        })
        .collect();
    to_delete.retain(|f| !modified.contains(f));
    for file in to_delete {
        fs::remove_file(file)?;
    }
    Ok(())
}

fn write_rust(
    path: &Path,
    code: impl ToRustCode,
    modified: &mut HashSet<PathBuf>,
) -> std::io::Result<()> {
    modified.insert(path.to_path_buf());
    let code = format_code(code.to_rust_code());
    let mut content = fs::read_to_string(path).unwrap_or_default();
    if content.contains("libninja: static") {
        return Ok(());
    } else if content.contains("libninja: after") {
        let (static_content, _gen) = content.split_once("libninja: after").unwrap();
        content.truncate(static_content.len() + "libninja: after".len());
        content.push('\n');
        content.push_str(&code);
    } else {
        content = code;
    }
    hir::write_file(path, &content)
}
