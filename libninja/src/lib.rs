#![allow(non_snake_case)]
#![allow(deprecated)]
#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::process::ExitCode;

pub use ::openapiv3::OpenAPI;
use anyhow::{anyhow, Result};
pub use openapiv3;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use extractor::add_operation_models;
pub use options::{LibraryConfig, LibraryOptions, OutputOptions};
pub use repo::*;

use crate::extractor::{extract_api_operations, extract_spec};
use crate::mir::MirSpec;
pub use crate::lang::Language;
use crate::util::open;

pub mod mir;
pub mod custom;
pub mod extractor;
pub mod options;
pub mod rust;
pub mod sourcegen;
pub mod util;
pub mod command;
mod modify;
pub mod repo;
mod lang;


static TEMPLATE_DIR: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/template");

pub fn read_spec(path: impl AsRef<Path>, service_name: &str) -> Result<OpenAPI> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|_| anyhow!("{:?}: File not found.", path))?;
    let value: serde_yaml::Value = match path.extension().unwrap_or_default().to_str().unwrap() {
        "yaml" => serde_yaml::from_reader(file)?,
        "json" => serde_json::from_reader(file)?,
        _ => panic!("Unknown file extension"),
    };
    let spec = modify::modify_spec(value, service_name)?;
    Ok(spec)
}

pub fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
    match opts.library_options.generator {
        Language::Rust => rust::generate_rust_library(spec, opts),
        Language::Python => python::generate_python_library(spec, opts),
        Language::Typescript => typescript::generate_typescript_library(spec, opts),
        Language::Golang => go::generate_go_library(spec, opts),
    }
}

pub fn generate_library_using_spec_at_path(path: &Path, opts: OutputOptions) -> Result<()> {
    let spec = read_spec(path, &opts.library_options.service_name)?;
    generate_library(spec, opts)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Examples {
    pub rust: String,
    pub python: String,
    pub python_async: String,
    pub typescript: String,
    pub go: String,
}

pub fn generate_examples(
    spec: OpenAPI,
    mut opt: LibraryOptions,
) -> Result<HashMap<String, Examples>> {
    let mut map = HashMap::new();
    let spec = extract_spec(&spec, &opt)?;

    for operation in &spec.operations {
        let rust = {
            let generator = Language::Rust;
            let opt = LibraryOptions { generator, ..opt.clone() };
            let spec = add_operation_models(generator, spec.clone())?;
            rust::generate_example(operation, &opt, &spec)?
        };
        let python = {
            let opt = LibraryOptions { generator: Language::Python, ..opt.clone() };
            python::example::generate_sync_example(operation, &opt, &spec)?
        };
        let python_async = {
            let opt = LibraryOptions { generator: Language::Python, ..opt.clone() };
            python::example::generate_async_example(operation, &opt, &spec)?
        };
        let typescript = {
            let generator = Language::Rust;
            let opt = LibraryOptions { generator, ..opt.clone() };
            let spec = add_operation_models(generator, spec.clone())?;
            typescript::example::generate_example(operation, &opt, &spec)?
        };
        let go = {
            let opt = LibraryOptions { generator: Language::Golang, ..opt.clone() };
            go::example::generate_example(operation, &opt, &spec)?
        };
        let examples = Examples {
            rust,
            python,
            python_async,
            typescript,
            go,
        };
        map.insert(operation.name.0.clone(), examples);
    }
    Ok(map)
}
