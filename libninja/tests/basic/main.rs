use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use openapiv3::OpenAPI;
use pretty_assertions::assert_eq;
use serde_yaml::from_str;

use hir::Language;
use libninja::generate_library;
use libninja::rust::generate_example;
use ln_core::{OutputConfig, PackageConfig};
use ln_core::extractor::extract_spec;

const EXAMPLE: &str = include_str!("link_create_token.rs");

const BASIC: &str = include_str!("../../../test_specs/basic.yaml");
const RECURLY: &str = include_str!("../../../test_specs/recurly.yaml");

#[test]
fn test_generate_example() {
    let spec: OpenAPI = from_str(BASIC).unwrap();

    let config = PackageConfig {
        package_name: "plaid".to_string(),
        service_name: "Plaid".to_string(),
        language: Language::Rust,
        package_version: "0.1.0".to_string(),
        config: Default::default(),
        dest: PathBuf::from_str("..").unwrap(),
        derives: vec![],
    };
    let hir = extract_spec(&spec).unwrap();
    let op = hir.get_operation("linkTokenCreate").unwrap();
    let example = generate_example(&op, &config, &hir).unwrap();
    assert_eq!(example, EXAMPLE);
}

#[test]
pub fn test_build_full_library_recurly() -> Result<()> {
    let spec: OpenAPI = from_str(RECURLY).unwrap();

    let temp = tempfile::tempdir()?;

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
