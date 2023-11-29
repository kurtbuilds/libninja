use std::collections::BTreeSet;

use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use hir::{DateSerialization, DecimalSerialization, HirField, HirSpec, IntegerSerialization, NewType, Record, StrEnum, Struct, Ty, TypeAlias};
use ln_core::LibraryConfig;
use mir::{Field, File, Ident, Import, import, Visibility};

use crate::rust::codegen;
use crate::rust::codegen::{sanitize_filename, ToRustCode};
use crate::rust::codegen::ToRustIdent;
use crate::rust::codegen::ToRustType;

pub trait FieldExt {
    fn decorators(&self, name: &str, config: &LibraryConfig) -> Vec<TokenStream>;
}

impl FieldExt for HirField {
    fn decorators(&self, name: &str, config: &LibraryConfig) -> Vec<TokenStream> {
        let mut decorators = Vec::new();
        let rust_ident = name.to_rust_ident();
        if rust_ident.0 != name {
            let name = &name;
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
            Ty::Integer { serialization } => {
                match serialization {
                    IntegerSerialization::Simple => {}
                    IntegerSerialization::String => {
                        decorators.push(quote! {
                            #[serde(with = "crate::serde::option_i64_str")]
                        });
                    }
                    IntegerSerialization::NullAsZero => {
                        decorators.push(quote! {
                            #[serde(with = "crate::serde::option_i64_null_as_zero")]
                        });
                    }
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
            Ty::Currency { serialization: DecimalSerialization::String } => {
                if self.optional {
                    decorators.push(quote! {
                        #[serde(with = "rust_decimal::serde::str_option")]
                    });
                } else {
                    decorators.push(quote! {
                        #[serde(with = "rust_decimal::serde::str")]
                    });
                }
            },
            _ => {}
        }
        decorators
    }
}

pub trait StructExt {
    fn implements_default(&self) -> bool;
    fn derive_default(&self) -> TokenStream;
    fn model_fields<'a>(&'a self, config: &'a LibraryConfig) -> Box<dyn Iterator<Item=Field<TokenStream>> + 'a>;
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

    fn model_fields<'a>(&'a self, config: &'a LibraryConfig) -> Box<dyn Iterator<Item=Field<TokenStream>> + 'a> {
        Box::new(self.fields.iter().map(|(name, field)| {
            let decorators = field.decorators(name, config);
            let ty = field.ty.to_rust_type();
            let mut optional = field.optional;
            match field.ty {
                Ty::Integer { serialization: IntegerSerialization::NullAsZero | IntegerSerialization::String } => {
                    optional = true;
                }
                Ty::Date { serialization: DateSerialization::Integer } => {
                    optional = true;
                }
                _ => {}
            }
            Field {
                name: name.clone(),
                ty,
                visibility: Visibility::Public,
                decorators,
                optional,
                ..Field::default()
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
pub fn generate_model_rs(spec: &HirSpec, config: &LibraryConfig) -> File<TokenStream> {
    let imports = spec.schemas.keys().map(|name: &String| {
        let fname = sanitize_filename(&name);
        Import::new(&fname, vec!["*"]).public()
    }).collect();
    let code = spec.schemas.keys().map(|name| {
        let name = Ident(sanitize_filename(name));
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
pub fn generate_single_model_file(name: &str, record: &Record, spec: &HirSpec, config: &LibraryConfig) -> File<TokenStream> {
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
    name: String,
    ty: Ty,
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
            s = format!("{}{}", e.name, s);
        }
        let name = s.to_rust_struct();
        let serde_attr = codegen::serde_rename(&original_name, &name);
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

pub fn create_typealias(name: &str, schema: &HirField) -> TokenStream {
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
    use hir::{HirField, Ty};

    use crate::rust::format::format_code;

    use super::*;

    #[test]
    fn test_struct_newtype() {
        let name = "NewType".to_string();
        let schema = NewType {
            name,
            fields: vec![HirField {
                ty: Ty::String,
                ..HirField::default()
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