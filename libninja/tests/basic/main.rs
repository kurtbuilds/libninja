use std::path::PathBuf;
use std::str::FromStr;

use codegen_rust::generate_example;
use hir::Config;
use libninja::{default, extractor::extract_spec};
use mir_rust::assert_code_eq;
use openapiv3::OpenAPI;

#[test]
fn test_generate_example() {
    let spec: OpenAPI = serde_yaml::from_str(include_str!("../../../test_specs/basic.yaml")).unwrap();

    let config = Config {
        name: "Plaid".to_string(),
        dest: PathBuf::from_str("..").unwrap(),
        ..default()
    };
    let hir = extract_spec(&spec).unwrap();
    let op = hir.get_operation("linkTokenCreate").unwrap();
    let example = generate_example(&op, &config, &hir).unwrap();
    assert_code_eq!(example, include_str!("link_create_token.rs"));
}

#[test]
pub fn test_build_full_library_recurly() {
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(std::io::stdout)
        .init();
    let spec = include_str!("../../../test_specs/recurly.yaml");
    let spec: OpenAPI = serde_yaml::from_str(spec).unwrap();

    let temp = tempfile::tempdir().unwrap();

    let spec = extract_spec(&spec).unwrap();
    let config = Config {
        name: "Recurly".to_string(),
        dest: temp.path().to_path_buf(),
        ..default()
    };
    codegen_rust::generate_rust_library(spec, config).unwrap();
}
