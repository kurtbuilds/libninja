use ocg::extractor::{extract_api_operations, extract_inputs, extract_spec};
use ocg::options::{LibraryConfig, LibraryOptions};
use ocg::{generate_library, OutputOptions, rust};
use ocg::lang::Language;
use openapiv3::OpenAPI;
use pretty_assertions::assert_eq;
use anyhow::Result;
use std::fs::File;

const BASIC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/basic.yaml");
const RECURLY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/spec/recurly.yaml");

const EXAMPLE: &str = include_str!("link_create_token.rs");

#[test]
pub fn test_required_args() {
    let yaml = File::open(BASIC).unwrap();
    let spec: OpenAPI = serde_yaml::from_reader(yaml).unwrap();
    let (operation, path) = spec.get_operation("linkTokenCreate").unwrap();
    let inputs = extract_inputs(&operation, path, &spec).unwrap();
    assert_eq!(inputs[8].name.0, "user_token");
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
        generator: Language::Rust,
        config: Default::default(),
    };
    let operations = extract_api_operations(&spec).unwrap();
    let operation = operations
        .iter()
        .find(|o| o.name.0 == "linkTokenCreate")
        .unwrap();

    let spec = extract_spec(&spec, &opt).unwrap();
    let example = rust::example::generate_example(&operation, &opt, &spec)?;
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
