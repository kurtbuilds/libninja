use std::fs::File;

use anyhow::Result;
use hir::Language;
use libninja::{generate_library, rust};
use ln_core::extractor::{extract_api_operations, extract_inputs, extract_spec};
use ln_core::{LibraryConfig, LibraryOptions, OutputOptions};
use openapiv3::OpenAPI;
use pretty_assertions::assert_eq;

const BASIC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/basic.yaml");
const RECURLY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/recurly.yaml");

const EXAMPLE: &str = include_str!("link_create_token.rs");

#[test]
pub fn test_required_args() {
    let yaml = File::open(BASIC).unwrap();
    let spec: OpenAPI = serde_yaml::from_reader(yaml).unwrap();
    let (operation, path) = spec.get_operation("linkTokenCreate").unwrap();
    let inputs = extract_inputs(&operation, path, &spec).unwrap();
    assert_eq!(inputs[8].name, "user_token");
    assert_eq!(inputs[8].optional, true);
}

#[test]
fn test_generate_example() -> Result<()> {
    let yaml = File::open(BASIC).unwrap();
    let spec: OpenAPI = serde_yaml::from_reader(yaml).unwrap();
    // let operation = spec.get_operation("linkTokenCreate").unwrap();

    let opt = LibraryOptions {
        package_name: "plaid".to_string(),
        service_name: "Plaid".to_string(),
        package_version: "0.1.0".to_string(),
        language: Language::Rust,
        build_examples: false,
        config: Default::default(),
    };
    let operations = extract_api_operations(&spec).unwrap();
    let operation = operations
        .iter()
        .find(|o| o.name == "linkTokenCreate")
        .unwrap();

    let spec = extract_spec(&spec).unwrap();
    let example = rust::generate_example(&operation, &opt, &spec)?;
    assert_eq!(example, EXAMPLE);
    Ok(())
}

#[test]
pub fn test_build_full_library_recurly() -> Result<()> {
    let yaml = File::open(RECURLY).unwrap();
    let temp = tempfile::tempdir()?;

    let spec: OpenAPI = serde_yaml::from_reader(yaml).unwrap();
    let opts = OutputOptions {
        library_options: LibraryOptions::new("Recurly", Language::Rust),
        qualified_github_repo: "libninja".to_string(),
        dest_path: temp.path().to_path_buf(),
    };
    generate_library(spec, opts)
}
