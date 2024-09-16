use hir::Language;
use libninja::rust;
use ln_core::extractor::add_operation_models;
use ln_core::{extract_spec, Config};
use openapiv3::OpenAPI;
use pretty_assertions::assert_eq;

#[test]
fn test_example_generation_with_refs() {
    let s = include_str!("files/plaid_processor.yaml");
    let spec: OpenAPI = serde_yaml::from_str(s).unwrap();
    let spec = extract_spec(&spec).unwrap();
    let spec = add_operation_models(Language::Rust, spec).unwrap();

    let op = spec.operations.iter().next().unwrap();
    let opt = Config {
        package_name: "plaid".to_string(),
        service_name: "Plaid".to_string(),
        language: Language::Rust,
        package_version: "1.0".to_string(),
        config: Default::default(),
        dest: Default::default(),
        derives: vec![],
    };
    let example = rust::generate_example(op, &opt, &spec).unwrap();
    assert_eq!(example, include_str!("files/plaid_processor_expected.rs"));
}

#[test]
fn test_example_generation_with_refs2() {
    let s = include_str!("files/plaid_watchlist.yaml");
    let spec: OpenAPI = serde_yaml::from_str(s).unwrap();
    let spec = extract_spec(&spec).unwrap();
    let spec = add_operation_models(Language::Rust, spec).unwrap();

    let op = spec.operations.iter().next().unwrap();
    let opt = Config {
        package_name: "plaid".to_string(),
        service_name: "Plaid".to_string(),
        language: Language::Rust,
        package_version: "1.0".to_string(),
        config: Default::default(),
        dest: Default::default(),
        derives: vec![],
    };
    let example = rust::generate_example(op, &opt, &spec).unwrap();
    assert_eq!(example, include_str!("files/plaid_watchlist_expected.rs"));
}
