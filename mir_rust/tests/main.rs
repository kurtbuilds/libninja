use hir::{Config, HirField, HirSpec, Struct};
use mir::{import, Import, Ty};
use mir_rust::{assert_code_eq, bmap, default, make_class, ToRustIdent};

#[test]
fn test_to_ident() {
    let ident = "meta/root".to_rust_ident();
    assert_eq!(ident, "meta_root");
}

#[test]
fn test_to_ident1() {
    let ident = "get-phone-checks-v0.1".to_rust_ident();
    assert_eq!(ident, "get_phone_checks_v0_1");
}

#[test]
fn test_star() {
    let i = import!("super::*");
    assert_code_eq!(i, "use super::*;");
    let i = Import::new("super", vec!["*"]);
    assert_code_eq!(i, "use super::{*};");
}

#[test]
fn test_import() {
    let code = import!("plaid::model::LinkTokenCreateRequestUser");
    assert_code_eq!(code, "use plaid::model::LinkTokenCreateRequestUser;");
    let code = import!(plaid::model, LinkTokenCreateRequestUser, Foobar);
    assert_code_eq!(code, "use plaid::model::{LinkTokenCreateRequestUser, Foobar};");
    let code = Import::alias("plaid::model", "foobar");
    assert_code_eq!(code, "use plaid::model as foobar;");
    let code = Import::package("foo_bar");
    assert_code_eq!(code, "use foo_bar;");
}

#[test]
fn test_struct_sumtype_empty_derive() {
    let name = "SumType".to_string();
    let schema = Struct {
        name,
        fields: bmap![
            "field1" => HirField::new(Ty::String).nullable(),
            "field2" =>HirField::new(Ty::String),
        ],
        ..default()
    };
    let cfg = Config::default();
    let spec = HirSpec::default();
    let code = make_class(&schema, &cfg, &spec);
    assert_code_eq!(code, include_str!("static/struct_sumtype_empty_derive.rs"));
}

#[test]
fn test_struct_sumtype_nonempty_derive() {
    let name = "SumType".to_string();
    let derives = vec!["oasgen::OaSchema".to_string(), "example::Other".to_string()];
    let schema = Struct {
        name,
        fields: bmap![
            "field1" => HirField::new(Ty::String).nullable(),
            "field2" => HirField::new(Ty::String),
        ],
        ..default()
    };
    let spec = HirSpec::default();
    let mut cfg = Config::default();
    cfg.derives = derives;
    let code = make_class(&schema, &cfg, &spec);
    assert_code_eq!(code, include_str!("static/struct_sumtype_nonempty_derive.rs"));
}
