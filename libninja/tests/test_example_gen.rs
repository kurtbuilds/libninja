use hir::{Config, Language};
use libninja::default;
use libninja::extractor::{add_operation_models, extract_spec};
use mir_rust::assert_code_eq;
use openapiv3::OpenAPI;

#[test]
fn test_example_generation_with_refs() {
    let s = include_str!("static/plaid_processor.yaml");
    let spec: OpenAPI = serde_yaml::from_str(s).unwrap();
    let spec = extract_spec(&spec).unwrap();
    let spec = add_operation_models(Language::Rust, spec).unwrap();

    let op = spec.operations.iter().next().unwrap();
    let opt = Config {
        name: "Plaid".to_string(),
        ..default()
    };
    let example = codegen_rust::generate_example(op, &opt, &spec).unwrap();
    assert_code_eq!(example, include_str!("static/plaid_processor_expected.rs"));
}

#[test]
fn test_example_generation_with_refs2() {
    let s = include_str!("static/plaid_watchlist.yaml");
    let spec: OpenAPI = serde_yaml::from_str(s).unwrap();
    let spec = extract_spec(&spec).unwrap();
    let spec = add_operation_models(Language::Rust, spec).unwrap();

    let op = spec.operations.iter().next().unwrap();
    let opt = Config {
        name: "Plaid".to_string(),
        ..default()
    };
    let example = codegen_rust::generate_example(op, &opt, &spec).unwrap();
    assert_code_eq!(example, include_str!("static/plaid_watchlist_expected.rs"));
}
