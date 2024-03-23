use openapiv3::{OpenAPI, Schema};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

use hir::{HirSpec, Record};
use ln_core::extractor::extract_records;
/// Tests that the `allOf` keyword is handled correctly.
use ln_core::{ConfigFlags, PackageConfig};

const TRANSACTION: &str = include_str!("transaction.yaml");
const TRANSACTION_RS: &str = include_str!("transaction.rs");

const RESTRICTION_BACS: &str = include_str!("restriction_bacs.yaml");
const RESTRICTION_BACS_RS: &str = include_str!("restriction_bacs.rs");

fn record_for_schema(name: &str, schema: &str, spec: &OpenAPI) -> Record {
    let schema = serde_yaml::from_str::<Schema>(schema).unwrap();
    ln_core::extractor::create_record(name, &schema, spec)
}

fn formatted_code(record: Record, spec: &HirSpec) -> String {
    let config = PackageConfig {
        package_name: "test".to_string(),
        service_name: "service".to_string(),
        language: hir::Language::Rust,
        package_version: "latest".to_string(),
        config: ConfigFlags::default(),
        dest: PathBuf::new(),
        derives: vec![],
    };
    let code = libninja::rust::lower_hir::create_struct(&record, &config, spec);
    mir_rust::format_code(code)
}

#[test]
fn test_transaction() {
    let mut spec = OpenAPI::default();
    spec.schemas.insert("TransactionBase", Schema::new_object());
    spec.schemas.insert("TransactionCode", Schema::new_string());
    spec.schemas
        .insert("PersonalFinanceCategory", Schema::new_string());
    spec.schemas
        .insert("TransactionCounterparty", Schema::new_string());

    let mut result = HirSpec::default();
    extract_records(&spec, &mut result).unwrap();
    let record = record_for_schema("Transaction", TRANSACTION, &spec);
    let code = formatted_code(record, &result);
    println!("{}", code);
    assert_eq!(code, TRANSACTION_RS);
}

#[test]
fn test_nullable_doesnt_deref() {
    let mut spec = OpenAPI::default();
    spec.schemas.insert("RecipientBACS", Schema::new_object());

    let record = record_for_schema(
        "PaymentInitiationOptionalRestrictionBacs",
        RESTRICTION_BACS,
        &spec,
    );
    let code = formatted_code(record, &HirSpec::default());
    assert_eq!(code, RESTRICTION_BACS_RS);
}
