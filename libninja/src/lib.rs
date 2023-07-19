#![allow(non_snake_case)]
#![allow(deprecated)]
#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::process::ExitCode;

pub use ::openapiv3::OpenAPI;
use anyhow::{anyhow, Context, Result};
pub use openapiv3;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use lang::*;
use ln_core::{Language, LibraryConfig, LibraryOptions, MirSpec, OutputOptions};
use ln_core::extractor::{extract_api_operations, extract_spec};
use ln_core::extractor::add_operation_models;
use ln_core::fs::open;
pub use repo::*;

pub mod custom;
pub mod rust;
pub mod util;
pub mod command;
mod modify;
pub mod repo;
mod lang;

pub fn read_spec(path: impl AsRef<Path>, service_name: &str) -> Result<OpenAPI> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|_| anyhow!("{:?}: File not found.", path))?;
    let ext = path.extension().unwrap_or_default().to_str().expect("File must have a file extension.");
    let value: serde_yaml::Value = match ext {
        "yaml" => serde_yaml::from_reader(file)?,
        "json" => serde_json::from_reader(file)?,
        _ => panic!("Unknown file extension"),
    };
    let spec = modify::modify_spec(value, service_name)
        .context("Failed to deserialize OpenAPI spec.")?;
    Ok(spec)
}

pub fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
    match opts.library_options.language {
        Language::Rust => rust::generate_rust_library(spec, opts),
        Language::Python => python::generate_library(spec, opts),
        Language::Typescript => typescript::generate_library(spec, opts),
        Language::Golang => go::generate_library(spec, opts),
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
            let opt = LibraryOptions { language: generator, ..opt.clone() };
            let spec = add_operation_models(generator, spec.clone())?;
            rust::generate_example(operation, &opt, &spec)?
        };
        let python = {
            let opt = LibraryOptions { language: Language::Python, ..opt.clone() };
            python::generate_sync_example(operation, &opt, &spec)?
        };
        let python_async = {
            let opt = LibraryOptions { language: Language::Python, ..opt.clone() };
            python::generate_async_example(operation, &opt, &spec)?
        };
        let typescript = {
            let generator = Language::Rust;
            let opt = LibraryOptions { language: generator, ..opt.clone() };
            let spec = add_operation_models(generator, spec.clone())?;
            typescript::generate_example(operation, &opt, &spec)?
        };
        let go = {
            let opt = LibraryOptions { language: Language::Golang, ..opt.clone() };
            go::generate_example(operation, &opt, &spec)?
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
