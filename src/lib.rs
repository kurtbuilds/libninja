#![allow(non_snake_case)]
#![allow(unused)]
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use proc_macro2::TokenStream;
use anyhow::Result;
pub use openapiv3::OpenAPI;
use tokio::io::AsyncWriteExt;
use crate::codegen::client::generate_lib_rs;
use crate::codegen::format::format_code;
use crate::codegen::model::generate_model_rs;
use crate::util::open;
use std::io::Write;

pub use openapiv3;

pub mod codegen;
mod util;

pub struct GenerateLibrary {
    pub name: String,
    pub dest_path: PathBuf,

    pub lib_rs_path: Option<PathBuf>,
    pub model_rs_path: Option<PathBuf>,
}


fn write_file(path: &Path, code: TokenStream, template: &str) -> Result<()> {
    let code = format_code(code)?;
    let mut f = open(&path)?;
    f.write(template.as_bytes())?;
    f.write("\n".as_bytes())?;
    f.write(code.as_bytes())?;
    Ok(())
}

pub fn read_spec(path: &Path) -> Result<OpenAPI> {
    let file = File::open(path)?;
    Ok(serde_yaml::from_reader(file)?)
}


pub fn generate_library(spec: OpenAPI, opts: GenerateLibrary) -> Result<()> {
    fs::create_dir_all(&opts.dest_path)?;


    let lib_rs_template = match opts.lib_rs_path {
        Some(path) => fs::read_to_string(path)?,
        None => include_str!("../template/lib.rs").to_string(),
    };
    let code = generate_lib_rs(&spec, &opts.name);
    write_file(&opts.dest_path.join("lib.rs"), code, &lib_rs_template)?;

    let lib_rs_template = match opts.model_rs_path {
        Some(path) => fs::read_to_string(path)?,
        None => include_str!("../template/model.rs").to_string(),
    };
    let code = generate_model_rs(&spec);
    write_file(&opts.dest_path.join("model.rs"), code, &lib_rs_template)?;

    Ok(())
}

pub fn generate_library_at_path(path: &Path, opts: GenerateLibrary) -> Result<()> {
    let spec: OpenAPI = {
        let file = File::open(path)?;
        serde_yaml::from_reader(file)?
    };
    generate_library(spec, opts)
}
