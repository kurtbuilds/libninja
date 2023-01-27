// use openapiv3::OpenAPI;
// use ocg::{Language, LibraryOptions};
// use ocg::extractor::add_operation_models;
//
// const OPENAPI: &str = include_str!("simplify_mir.yaml");
//
// #[test]
// fn test_simplify_mir() {
//     let spec: OpenAPI = serde_yaml::from_str(OPENAPI).unwrap();
//     let spec = ocg::extractor::extract_spec(&spec, &LibraryOptions::new("test", Language::Rust)).unwrap();
//     let spec = add_operation_models(Language::Rust, spec).unwrap();
// }
