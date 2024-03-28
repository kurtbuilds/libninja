use openapiv3 as oa;

use hir::{HirSpec, Record};
use ln_core::extractor::extract_api_operations;
use mir::Ty;

#[test]
fn test_post_translate() {
    let mut hir = HirSpec::default();
    let s = include_str!("../../test_specs/deepl.yaml");
    let openapi: oa::OpenAPI = serde_yaml::from_str(s).unwrap();

    extract_api_operations(&openapi, &mut hir).unwrap();
    let Some(op) = hir.operations.iter().find(|o| o.name == "translateText") else {
        panic!("Operation not found");
    };
    let Ty::Model(name) = &op.ret else {
        panic!("Expected model type");
    };
    let Record::Struct(s) = hir.schemas.get(name).unwrap() else {
        panic!("Expected struct");
    };
    let z = &s.fields["translations"];
    let Ty::Array(ty) = &z.ty else {
        panic!("Expected array");
    };
    assert!(matches!(ty.as_ref(), Ty::Model(m) if m == "TranslationsItem"));
}
