use std::collections::{BTreeSet, HashSet};
use clap::builder::Str;
use convert_case::{Case, Casing};
use openapiv3::{OpenAPI, ReferenceOr, Schema};
use openapiv3::{SchemaKind, StringType, Type};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use tracing_ez::{info, span};

use ln_mir::{Field, File, Ident, Import, import, Name, Visibility};
use ln_mir as model;

use ln_core::{extractor, hir, MirSpec};
use ln_core::hir::{DateSerialization, MirField, NewType, Record, StrEnum, Struct, Ty, TypeAlias};
use ln_core::hir::AuthLocation::Token;
use ln_core::extractor::schema_ref_to_ty;
use ln_core::LibraryConfig;
use crate::rust::codegen;
use crate::rust::codegen::{ToRustCode};
use crate::rust::codegen::ToRustIdent;
use crate::rust::codegen::ToRustType;

pub trait FieldExt {
    fn decorators(&self, name: &Name, config: &LibraryConfig) -> Vec<TokenStream>;
}

impl FieldExt for MirField {
    fn decorators(&self, name: &Name, config: &LibraryConfig) -> Vec<TokenStream> {
        let mut decorators = Vec::new();
        let rust_ident = name.to_rust_ident();
        if rust_ident.0 != name.0 {
            let name = &name.0;
            if self.flatten {
                decorators.push(quote! {
                    #[serde(flatten)]
                });
            } else {
                decorators.push(quote! {
                    #[serde(rename = #name)]
                });
            }
            if config.ormlite {
                decorators.push(quote! {
                    #[ormlite(column = #name)]
                });
            }
        }
        if self.optional {
            decorators.push(quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
            });
        }
        if self.ty.inner_model().is_some() && config.ormlite {
            decorators.push(quote! {
                #[ormlite(experimental_encode_as_json)]
            });
        }
        match self.ty {
            Ty::Integer { null_as_zero } => {
                if null_as_zero {
                    decorators.push(quote! {
                        #[serde(with = "crate::serde::option_i64_null_as_zero")]
                    });
                }
            }
            Ty::Date { serialization } => {
                match serialization {
                    DateSerialization::Iso8601 => {}
                    DateSerialization::Integer => {
                        decorators.push(quote! {
                            #[serde(with = "crate::serde::option_chrono_naive_date_as_int")]
                        });
                    }
                }
            }
            Ty::Currency { serialization: hir::CurrencySerialization::String } => {
                decorators.push(quote! {
                    #[serde(with = "rust_decimal::serde::str")]
                });
            },
            _ => {}
        }
        decorators
    }
}

pub trait StructExt {
    fn implements_default(&self) -> bool;
    fn derive_default(&self) -> TokenStream;
    fn model_fields<'a>(&'a self, config: &'a LibraryConfig) -> Box<dyn Iterator<Item=model::Field<TokenStream>> + 'a>;
    fn ref_target(&self) -> Option<RefTarget>;
}
impl StructExt for Struct {

    fn implements_default(&self) -> bool {
        self.fields.iter().all(|(_, f)| f.optional || f.ty.implements_default())
    }

    fn derive_default(&self) -> TokenStream {
        if self.implements_default() {
            quote! { , Default }
        } else {
            TokenStream::new()
        }
    }

    fn model_fields<'a>(&'a self, config: &'a LibraryConfig) -> Box<dyn Iterator<Item=model::Field<TokenStream>> + 'a> {
        Box::new(self.fields.iter().map(|(name, field)| {
            let decorators = field.decorators(name, config);
            let ty = field.ty.to_rust_type();
            let mut optional = field.optional;
            match field.ty {
                Ty::Integer { null_as_zero: true } => {
                    optional = true;
                }
                Ty::Date { serialization: DateSerialization::Integer } => {
                    optional = true;
                }
                _ => {}
            }
            model::Field {
                name: name.clone(),
                ty,
                visibility: Visibility::Public,
                decorators,
                optional,
                ..model::Field::default()
            }
        }))
    }

    fn ref_target(&self) -> Option<RefTarget> {
        self.fields.iter().find(|(_, f)| f.flatten && !f.optional).map(|(name, f)| {
            RefTarget {
                name: name.clone(),
                ty: f.ty.clone(),
            }
        })
    }
}

pub trait RecordExt {
    fn imports(&self, path: &str) -> Option<Import>;
}

impl RecordExt for Record {
    fn imports(&self, path: &str) -> Option<Import> {
        let names = self.fields()
            .flat_map(|f| f.ty.inner_model())
            .map(|name| name.to_rust_struct().0)
            .collect::<BTreeSet<_>>();
        if !names.is_empty() {
            Some(Import::new(path, names.into_iter().collect::<Vec<_>>()))
        } else {
            None
        }
    }
}

/// Generate a model.rs file that just imports from dependents.
pub fn generate_model_rs(spec: &MirSpec, config: &LibraryConfig) -> File<TokenStream> {
    let imports = spec.schemas.keys().map(|name| {
        Import::new(&name.to_filename(), vec!["*"]).public()
    }).collect();
    let code = spec.schemas.keys().map(|name| {
        let name = Ident::new(&name.to_filename());
        quote! {
            mod #name;
        }
    }).collect();
    File {
        imports,
        code: Some(code),
        ..File::default()
    }
}

/// Generate the file for a single struct.
pub fn generate_single_model_file(name: &str, record: &Record, spec: &MirSpec, config: &LibraryConfig) -> File<TokenStream> {
    let mut imports = vec![
        import!("serde", Serialize, Deserialize),
    ];
    if let Some(import) = record.imports("super") {
        imports.push(import);
    }
    if config.ormlite {
        imports.push(import!("ormlite", TableMeta, IntoArguments));
    }
    File {
        code: Some(create_struct(record, config)),
        imports,
        ..File::default()
    }
}

pub struct RefTarget {
    name: Name,
    ty: hir::Ty,
}

pub fn create_sumtype_struct(schema: &Struct, config: &LibraryConfig) -> TokenStream {
    let default = schema.derive_default();
    let ormlite = if config.ormlite { quote! { , TableMeta, IntoArguments } } else { TokenStream::new() };

    let name = schema.name.to_rust_struct();
    let fields = schema.model_fields(config).map(ToRustCode::to_rust_code);
    let deref = schema.ref_target().map(|t| {
        let target = t.name.to_rust_ident();
        let ty = t.ty.to_rust_type();
        quote! {
            impl std::ops::Deref for #name {
                type Target = #ty;
                fn deref(&self) -> &Self::Target {
                    &self.#target
                }
            }
            impl std::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#target
                }
            }
        }
    }).unwrap_or_default();

    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize #default #ormlite)]
        pub struct #name {
            #(#fields)*
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }
        #deref
    }
}


fn create_enum_struct(e: &StrEnum) -> TokenStream {
    let enums = e.variants.iter().filter(|s| !s.is_empty()).map(|s| {
        let original_name = s.to_string();
        let mut s = original_name.clone();
        if !s.is_empty() && s.chars().next().unwrap().is_numeric() {
            s = format!("{}{}", e.name.0, s);
        }
        let name = Name::new(&s).to_rust_struct();
        let serde_attr = codegen::serde_rename(&original_name, &name.to_string());
        quote! {
            #serde_attr
            #name
        }
    });
    let name = e.name.to_rust_struct();
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub enum #name {
            #(#enums,)*
        }
    }
}


pub fn create_newtype_struct(schema: &NewType) -> TokenStream {
    let name = schema.name.to_rust_struct();
    let fields = schema.fields.iter().map(|f| {
        f.ty.to_rust_type()
    });
    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct #name(#(pub #fields),*);
    }
}

pub fn create_typealias(name: &Name, schema: &MirField) -> TokenStream {
    let name = name.to_rust_struct();
    let mut ty = schema.ty.to_rust_type();
    if schema.optional {
        ty = quote! { Option<#ty> };
    }
    quote! {
        pub type #name = #ty;
    }
}

pub fn create_struct(record: &Record, config: &LibraryConfig) -> TokenStream {
    match record {
        Record::Struct(s) => create_sumtype_struct(s, config),
        Record::NewType(nt) => create_newtype_struct(nt),
        Record::Enum(en) => create_enum_struct(en),
        Record::TypeAlias(name, field) => create_typealias(name, field),
    }
}

#[cfg(test)]
mod tests {
    use ln_core::hir::{MirField, Name, Ty};
    use crate::rust::format::format_code;
    use super::*;

    #[test]
    fn test_struct_newtype() {
        let name = Name::new("NewType");
        let schema = NewType {
            name,
            fields: vec![MirField {
                ty: Ty::String,
                ..MirField::default()
            }],
        };
        let code = create_newtype_struct(&schema);
        let code = format_code(code).unwrap();
        assert_eq!(&code, "
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewType(pub String);
".trim());
    }
}