pub mod client;
mod example;
mod extras;
mod model;
pub mod request;
mod serde;

use anyhow::Result;
use client::make_lib_rs;
pub use example::generate_example;
use example::write_examples_folder;
use extras::calculate_extras;
use hir::Config;
use hir::HirSpec;
use mir_rust::{format_code, ToRustCode};
use model::write_model_module;
use request::write_request_module;
use serde::write_serde_module;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use std_ext::PathExt;

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
    let to_delete = walkdir::WalkDir::new(dest.join("src"))
        .into_iter()
        .chain(walkdir::WalkDir::new(dest.join("examples")).into_iter())
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.ext_str() == "rs")
        .filter(|e| !modified.contains(e))
        .filter(|p| {
            !fs::read_to_string(&p)
                .map(|content| content.contains("libninja: static"))
                .unwrap_or(false)
        });
    for e in to_delete {
        fs::remove_file(&e)?;
        eprintln!("{}: Remove unused file.", e.display());
    }
    Ok(())
}

fn write_rust(path: &Path, code: impl ToRustCode, modified: &mut HashSet<PathBuf>) -> std::io::Result<()> {
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
