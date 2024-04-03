use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use openapiv3::OpenAPI;
use pretty_assertions::assert_eq;

use hir::{HirSpec, Language};
use libninja::{generate_library, rust};
use ln_core::extractor::{extract_api_operations, extract_inputs, extract_spec};
use ln_core::{OutputConfig, PackageConfig};

const BASIC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/basic.yaml");
const RECURLY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/recurly.yaml");
const KEYWORDS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/keywords.yaml");

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

    let opt = PackageConfig {
        package_name: "plaid".to_string(),
        service_name: "Plaid".to_string(),
        language: Language::Rust,
        package_version: "0.1.0".to_string(),
        config: Default::default(),
        dest: PathBuf::from_str("..").unwrap(),
        derives: vec![],
    };
    let mut result = HirSpec::default();
    extract_api_operations(&spec, &mut result).unwrap();
    let operation = result
        .operations
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
    let opts = OutputConfig {
        dest_path: temp.path().to_path_buf(),
        build_examples: false,
        package_name: "recurly".to_string(),
        service_name: "Recurly".to_string(),
        language: Language::Rust,
        config: Default::default(),
        github_repo: Some("libninjacom/recurly".to_string()),
        version: None,
        derive: vec![],
    };
    generate_library(spec, opts)
}

#[test]
pub fn test_keywords_schema() -> Result<()> {
    let yaml = File::open(KEYWORDS).unwrap();
    let temp = tempfile::tempdir()?;

    let spec: OpenAPI = serde_yaml::from_reader(yaml).unwrap();
    let opts = OutputConfig {
        dest_path: temp.path().to_path_buf(),
        build_examples: false,
        package_name: "keywords".to_string(),
        service_name: "Keywords".to_string(),
        language: Language::Rust,
        config: Default::default(),
        github_repo: Some("libninjacom/test".to_string()),
        version: None,
        derive: vec![],
    };
    generate_library(spec, opts)
}
