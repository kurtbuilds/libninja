#![allow(non_snake_case)]
#![allow(deprecated)]
#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub use ::openapiv3::OpenAPI;
use anyhow::{anyhow, Context, Result};
pub use openapiv3;
use openapiv3::VersionedOpenAPI;
use serde::{Deserialize, Serialize};

use commercial::*;
use hir::Language;
use ln_core::{OutputConfig, PackageConfig};
use ln_core::extractor::add_operation_models;
use ln_core::extractor::extract_spec;

pub mod command;
mod commercial;
pub mod custom;
pub mod rust;

pub fn read_spec(path: &Path) -> Result<OpenAPI> {
    let file = File::open(path).map_err(|_| anyhow!("{:?}: File not found.", path))?;
    let ext = path
        .extension()
        .map(|s| s.to_str().expect("Extension isn't utf8"))
        .unwrap_or_else(|| "yaml");
    let openapi: VersionedOpenAPI = match ext {
        "yaml" => serde_yaml::from_reader(file)?,
        "json" => serde_json::from_reader(file)?,
        _ => panic!("Unknown file extension"),
    };
    let openapi = openapi.upgrade();
    Ok(openapi)
}

pub fn generate_library(spec: OpenAPI, opts: OutputConfig) -> Result<()> {
    match opts.language {
        Language::Rust => rust::generate_rust_library(spec, opts),
        Language::Python => python::generate_library(spec, opts),
        Language::Typescript => typescript::generate_library(spec, opts),
        Language::Golang => go::generate_library(spec, opts),
    }
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
    mut opt: PackageConfig,
) -> Result<HashMap<String, Examples>> {
    let mut map = HashMap::new();
    let spec = extract_spec(&spec)?;

    for operation in &spec.operations {
        let rust = {
            let generator = Language::Rust;
            let opt = PackageConfig {
                language: generator,
                ..opt.clone()
            };
            let spec = add_operation_models(generator, spec.clone())?;
            rust::generate_example(operation, &opt, &spec)?
        };
        let python = {
            let opt = PackageConfig {
                language: Language::Python,
                ..opt.clone()
            };
            python::generate_sync_example(operation, &opt, &spec)?
        };
        let python_async = {
            let opt = PackageConfig {
                language: Language::Python,
                ..opt.clone()
            };
            python::generate_async_example(operation, &opt, &spec)?
        };
        let typescript = {
            let generator = Language::Rust;
            let opt = PackageConfig {
                language: generator,
                ..opt.clone()
            };
            let spec = add_operation_models(generator, spec.clone())?;
            typescript::generate_example(operation, &opt, &spec)?
        };
        let go = {
            let opt = PackageConfig {
                language: Language::Golang,
                ..opt.clone()
            };
            go::generate_example(operation, &opt, &spec)?
        };
        let examples = Examples {
            rust,
            python,
            python_async,
            typescript,
            go,
        };
        map.insert(operation.name.clone(), examples);
    }
    Ok(map)
}
