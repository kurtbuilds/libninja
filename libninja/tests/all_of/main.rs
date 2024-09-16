use openapiv3::{OpenAPI, Schema};
use serde_yaml::from_str;

use hir::{Config, HirSpec};
use libninja::extractor::{extract_schema, extract_without_treeshake};
use mir_rust::{assert_code_eq, make_item};

#[test]
fn test_transaction() {
    let mut spec = OpenAPI::default();
    let cfg = Config::default();
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
    let code = make_item(record, &hir, &cfg);
    assert_code_eq!(code, include_str!("transaction.rs"));
}

#[test]
fn test_nullable_doesnt_deref() {
    let mut spec = OpenAPI::default();
    spec.schemas.insert("RecipientBACS", Schema::new_object());
    let cfg = Config::default();

    let mut hir = HirSpec::default();
    let schema = include_str!("restriction_bacs.yaml");
    let schema: Schema = from_str(schema).unwrap();
    let name = "PaymentInitiationOptionalRestrictionBacs";
    extract_schema(name, &schema, &spec, &mut hir);
    let record = hir.get_record(name).unwrap();
    let code = make_item(record, &hir, &cfg);
    assert_code_eq!(code, include_str!("restriction_bacs.rs"));
}
