use libninja::rust::mir::StructExt;
use libninja::rust::request;
use ln_core::{hir, hir::Record};
use openapiv3::{OpenAPI, ReferenceOr, Schema};

const LINK_TOKEN_CREATE: &str = include_str!("link_token_create.yaml");

fn record_for_schema(name: &str, schema: &str, spec: &OpenAPI) -> hir::Record {
    let schema = serde_yaml::from_str::<Schema>(schema).unwrap();
    let schema_ref = ReferenceOr::Item(schema);
    let mut record = ln_core::extractor::create_record(name, &schema_ref, spec);
    record.clear_docs();
    record
}

#[test]
fn test_link_token_create() {
    let mut spec = OpenAPI::default();
    spec.add_schema("UserAddress", Schema::new_object());
    spec.add_schema("UserIDNumber", Schema::new_string());
    let record = record_for_schema("LinkTokenCreateRequestUser", LINK_TOKEN_CREATE, &spec);
    let Record::Struct(struc) = record else {
        panic!("expected struct");
    };
    assert!(struc.implements_default());
}

#[test]
fn test_single_undersore_in_operation_paramerer_name() {
    request::assign_inputs_to_request(&[hir::Parameter {
        name: hir::Name("_".into()),
        ty: hir::Ty::default(),
        location: hir::Location::Query,
        optional: true,
        doc: None,
        example: None,
    }]);
}

#[test]
fn test_different_style_of_parameters_in_url() {
    let url = request::build_url(&hir::Operation {
        name: Default::default(),
        doc: None,
        parameters: vec![
            hir::Parameter {
                name: hir::Name("param1".into()),
                ty: Default::default(),
                location: hir::Location::Path,
                optional: false,
                doc: None,
                example: None,
            },
            hir::Parameter {
                name: hir::Name("param_2".into()),
                ty: Default::default(),
                location: hir::Location::Path,
                optional: false,
                doc: None,
                example: None,
            },
            hir::Parameter {
                name: hir::Name("param-3".into()),
                ty: Default::default(),
                location: hir::Location::Path,
                optional: false,
                doc: None,
                example: None,
            },
            hir::Parameter {
                name: hir::Name("param.4".into()),
                ty: Default::default(),
                location: hir::Location::Path,
                optional: false,
                doc: None,
                example: None,
            },
        ],
        ret: Default::default(),
        path: "/path/{param1}/{param_2}/{param-3}/{param.4}".to_string(),
        method: "GET".to_string(),
    });

    assert_eq!(
        url.to_string(),
        "& format ! (\
        \"/path/{param1}/{param2}/{param3}/{param4}\" \
    , param1 = self . param1 \
    , param2 = self . param2 \
    , param3 = self . param3 \
    , param4 = self . param4)"
            .to_string()
    );
}
