use hir::HirSpec;
use mir_rust::class::implements_default;
use openapiv3::{OpenAPI, Schema};
use serde_yaml::from_str;

#[test]
fn test_link_token_create() {
    let mut spec = OpenAPI::default();
    let mut hir = HirSpec::default();

    spec.schemas.insert("UserName", Schema::new_string());
    spec.schemas.insert("UserAddress", Schema::new_object());
    spec.schemas.insert("UserIDNumber", Schema::new_string());

    let schema = include_str!("link_token_create.yaml");
    let schema: Schema = from_str(schema).unwrap();
    extract_schema("LinkTokenCreateRequestUser", &schema, &spec, &mut hir);
    let s = hir
        .get_record("LinkTokenCreateRequestUser")
        .unwrap()
        .as_struct()
        .unwrap();
    assert!(implements_default(s, &hir));
}
