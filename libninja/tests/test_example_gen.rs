use openapiv3::OpenAPI;
use hir::Language;
use libninja::rust;
use ln_core::{extract_spec, PackageConfig};
use ln_core::extractor::add_operation_models;
use pretty_assertions::assert_eq;

const PLAID_PROCESSOR: &str = include_str!("plaid_processor/plaid.processor.yaml");

#[test]
fn test_example_generation_with_refs() {
    let spec: OpenAPI = serde_yaml::from_str(PLAID_PROCESSOR).unwrap();
    let spec = extract_spec(&spec).unwrap();
    let spec = add_operation_models(Language::Rust, spec).unwrap();

    let op = spec.operations.iter().next().unwrap();
    let opt = PackageConfig {
        package_name: "plaid".to_string(),
        service_name: "Plaid".to_string(),
        language: Language::Rust,
        package_version: "1.0".to_string(),
        config: Default::default(),
        dest: Default::default(),
    };
    let example = rust::generate_example(op, &opt, &spec).unwrap();
    assert_eq!(example, include_str!("plaid_processor/expected.rs"));
}