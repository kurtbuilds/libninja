use openapiv3 as oa;

use hir::Record;
use libninja_core::extract_spec;
use mir::Ty;

#[test]
fn test_post_translate() {
    let s = include_str!("../../test_specs/deepl.yaml");
    let mut openapi: oa::OpenAPI = serde_yaml::from_str(s).unwrap();
    openapi.paths.paths.retain(|k, _| k == "/translate");

    let hir = extract_spec(&openapi).unwrap();
    let op = hir.get_operation("translateText").unwrap();
    let Ty::Model(name) = &op.ret else {
        panic!("Expected model type");
    };
    let Record::Struct(s) = hir.get_record(name).unwrap() else {
        panic!("Expected struct");
    };
    let z = &s.fields["translations"];
    let Ty::Array(ty) = &z.ty else {
        panic!("Expected array");
    };
    assert!(
        matches!(ty.as_ref(), Ty::Model(m) if m == "Translation"),
        "{:?}",
        ty
    );
    let p = op
        .parameters
        .iter()
        .find(|p| p.name == "target_lang")
        .unwrap();
    assert!(matches!(&p.ty, Ty::Model(m) if m == "TargetLanguageText"));
}
