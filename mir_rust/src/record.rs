use crate::{class, derives_to_tokens, make_enum, ToRustIdent, ToRustType};
use hir::{Config, HirField, HirSpec, NewType, Record};
use mir::{
    Item, Ty,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn make_newtype(schema: &NewType, spec: &HirSpec, derives: &Vec<String>) -> Item<TokenStream> {
    let name = schema.name.to_rust_struct();
    let fields = schema.fields.iter().map(|f| f.ty.to_rust_type());
    let derives = derives_to_tokens(derives);
    let default = schema
        .fields
        .iter()
        .all(|f| f.ty.implements_default(spec))
        .then(|| {
            quote! { , Default }
        })
        .unwrap_or_default();
    Item::Block(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize #default #derives)]
        pub struct #name(#(pub #fields),*);
    })
}

pub fn make_typealias(name: &str, schema: &HirField) -> Item<TokenStream> {
    let name = name.to_rust_struct();
    let mut ty = schema.ty.to_rust_type();
    if schema.optional {
        ty = quote! { Option<#ty> };
    }
    Item::Block(quote! {
        pub type #name = #ty;
    })
}

pub fn make_item(record: &Record, config: &Config, hir: &HirSpec) -> Item<TokenStream> {
    match record {
        Record::Struct(s) => Item::Class(class::make_class(s, &config, hir)),
        Record::NewType(nt) => make_newtype(nt, hir, &config.derives),
        Record::Enum(en) => make_enum(en, &config.derives),
        Record::TypeAlias(name, field) => make_typealias(name, field),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{format_code, ToRustCode};
    #[test]
    fn test_struct_newtype() {
        let name = "NewType".to_string();
        let schema = NewType {
            name,
            fields: vec![HirField {
                ty: Ty::String,
                ..HirField::default()
            }],
            doc: None,
        };
        let code = make_newtype(&schema, &HirSpec::default(), &vec![]);
        let code = format_code(code.to_rust_code());
        assert_eq!(&code, include_str!("../tests/static/struct_newtype.rs"));
    }
}
