use std::path::PathBuf;

use openapiv3::{OpenAPI, Schema};
use pretty_assertions::assert_eq;
use serde_yaml::from_str;

use hir::{HirSpec, Record};
use libninja::rust::lower_hir::create_struct;
use ln_core::extractor::{extract_schema, extract_without_treeshake};
use ln_core::{Config, ConfigFlags};
use mir_rust::format_code;

fn formatted_code(record: &Record, spec: &HirSpec) -> String {
    let config = Config {
        package_name: "test".to_string(),
        service_name: "service".to_string(),
        language: hir::Language::Rust,
        package_version: "latest".to_string(),
        config: ConfigFlags::default(),
        dest: PathBuf::new(),
        derives: vec![],
    };
    let code = create_struct(&record, &config, spec);
    format_code(code)
}

#[test]
fn test_transaction() {
    let mut spec = OpenAPI::default();
    let s = &mut spec.schemas;
    s.insert("TransactionBase", Schema::new_object());
    s.insert("TransactionCode", Schema::new_string());
    s.insert("PersonalFinanceCategory", Schema::new_string());
    s.insert("TransactionCounterparty", Schema::new_string());

    let mut hir = extract_without_treeshake(&spec).unwrap();
    dbg!(&hir.schemas);
    let schema = include_str!("transaction.yaml");
    let schema: Schema = from_str(schema).unwrap();
    extract_schema("Transaction", &schema, &spec, &mut hir);
    let record = hir.get_record("Transaction").unwrap();
    let code = formatted_code(record, &hir);
    println!("{}", code);
    assert_eq!(code, include_str!("transaction.rs"));
}

#[test]
fn test_nullable_doesnt_deref() {
    let mut spec = OpenAPI::default();
    spec.schemas.insert("RecipientBACS", Schema::new_object());

    let mut hir = HirSpec::default();
    let schema = include_str!("restriction_bacs.yaml");
    let schema: Schema = from_str(schema).unwrap();
    let name = "PaymentInitiationOptionalRestrictionBacs";
    extract_schema(name, &schema, &spec, &mut hir);
    let record = hir.get_record(name).unwrap();
    let code = formatted_code(record, &hir);
    assert_eq!(code, include_str!("restriction_bacs.rs"));
}
