use openapiv3::{OpenAPI, ReferenceOr, Schema};
use pretty_assertions::assert_eq;

/// Tests that the `allOf` keyword is handled correctly.
use ln_core::{LibraryConfig};
use hir::Record;

const TRANSACTION: &str = include_str!("transaction.yaml");
const TRANSACTION_RS: &str = include_str!("transaction.rs");

const RESTRICTION_BACS: &str = include_str!("restriction_bacs.yaml");
const RESTRICTION_BACS_RS: &str = include_str!("restriction_bacs.rs");


fn record_for_schema(name: &str, schema: &str, spec: &OpenAPI) -> Record {
    let schema = serde_yaml::from_str::<Schema>(schema).unwrap();
    let mut record = ln_core::extractor::create_record(name, &schema, spec);
    record.clear_docs();
    record
}

fn formatted_code(record: Record) -> String {
    let config = LibraryConfig::default();
    let code = libninja::rust::lower_mir::create_struct(&record, &config);
    libninja::rust::format::format_code(code).unwrap()
}

#[test]
fn test_transaction() {
    let mut spec = OpenAPI::default();
    spec.add_schema("TransactionBase", Schema::new_object());
    spec.add_schema("TransactionCode", Schema::new_string());
    spec.add_schema("PersonalFinanceCategory", Schema::new_string());
    spec.add_schema("TransactionCounterparty", Schema::new_string());

    let record = record_for_schema("Transaction", TRANSACTION, &spec);
    let code = formatted_code(record);
    println!("{}", code);
    assert_eq!(code, TRANSACTION_RS);
}

#[test]
fn test_nullable_doesnt_deref() {
    let mut spec = OpenAPI::default();
    spec.add_schema("RecipientBACS", Schema::new_object());

    let record = record_for_schema("PaymentInitiationOptionalRestrictionBacs", RESTRICTION_BACS, &spec);
    let code = formatted_code(record);
    assert_eq!(code, RESTRICTION_BACS_RS);
}