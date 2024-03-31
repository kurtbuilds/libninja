use std::collections::BTreeSet;

use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use hir::{HirField, HirSpec, NewType, Record, StrEnum, Struct};
use ln_core::{ConfigFlags, PackageConfig};
use mir::{Field, File, Ident, import, Import, Visibility};
use mir::{DateSerialization, DecimalSerialization, IntegerSerialization, Ty};
use mir_rust::{sanitize_filename, ToRustIdent};
use mir_rust::ToRustCode;

use crate::rust::codegen;
use crate::rust::codegen::ToRustType;

pub trait FieldExt {
    fn decorators(&self, name: &str, config: &ConfigFlags) -> Vec<TokenStream>;
}

impl FieldExt for HirField {
    fn decorators(&self, name: &str, config: &ConfigFlags) -> Vec<TokenStream> {
        let mut decorators = Vec::new();
        let rust_ident = name.to_rust_ident();
        if rust_ident.0 != name {
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
                    #[cfg_attr(feature = "ormlite", ormlite(column = #name))]
                });
            }
        }
        if self.optional {
            decorators.push(quote! {
                #[serde(default, skip_serializing_if = "Option::is_none")]
            });
        } else if self.ty.is_iterable() {
            decorators.push(quote! {
                #[serde(default, skip_serializing_if = "Vec::is_empty")]
            });
        } else if matches!(self.ty, Ty::Any(_)) {
            decorators.push(quote! {
                #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
            });
        }
        if self.ty.inner_model().is_some() && config.ormlite {
            decorators.push(quote! {
                #[cfg_attr(feature = "ormlite", ormlite(experimental_encode_as_json))]
            });
        }
        match self.ty {
            Ty::Integer { serialization } => match serialization {
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
            },
            Ty::Date { serialization } => match serialization {
                DateSerialization::Iso8601 => {}
                DateSerialization::Integer => {
                    decorators.push(quote! {
                        #[serde(with = "crate::serde::option_chrono_naive_date_as_int")]
                    });
                }
            },
            Ty::Currency {
                serialization: DecimalSerialization::String,
            } => {
                if self.optional {
                    decorators.push(quote! {
                        #[serde(with = "rust_decimal::serde::str_option")]
                    });
                } else {
                    decorators.push(quote! {
                        #[serde(with = "rust_decimal::serde::str")]
                    });
                }
            }
            _ => {}
        }
        decorators
    }
}

pub trait StructExt {
    fn implements_default(&self, spec: &HirSpec) -> bool;
    fn derive_default(&self, spec: &HirSpec) -> TokenStream;
    fn model_fields<'a>(
        &'a self,
        config: &'a ConfigFlags,
    ) -> Box<dyn Iterator<Item = Field<TokenStream>> + 'a>;
    fn ref_target(&self) -> Option<RefTarget>;
}

impl StructExt for Struct {
    fn implements_default(&self, spec: &HirSpec) -> bool {
        self.fields.values().all(|f| f.implements_default(spec))
    }

    fn derive_default(&self, spec: &HirSpec) -> TokenStream {
        if self.implements_default(spec) {
            quote! { , Default }
        } else {
            TokenStream::new()
        }
    }

    fn model_fields<'a>(
        &'a self,
        config: &'a ConfigFlags,
    ) -> Box<dyn Iterator<Item = Field<TokenStream>> + 'a> {
        Box::new(self.fields.iter().map(|(name, field)| {
            let decorators = field.decorators(name, config);
            let ty = field.ty.to_rust_type();
            let mut optional = field.optional;
            match field.ty {
                Ty::Integer {
                    serialization: IntegerSerialization::NullAsZero | IntegerSerialization::String,
                } => {
                    optional = true;
                }
                Ty::Date {
                    serialization: DateSerialization::Integer,
                } => {
                    optional = true;
                }
                _ => {}
            }
            Field {
                name: name.to_rust_ident(),
                ty,
                vis: Visibility::Public,
                decorators,
                optional,
                doc: field.doc.clone(),
                ..Field::default()
            }
        }))
    }

    fn ref_target(&self) -> Option<RefTarget> {
        self.fields
            .iter()
            .find(|(_, f)| f.flatten && !f.optional)
            .map(|(name, f)| RefTarget {
                name: name.clone(),
                ty: f.ty.clone(),
            })
    }
}

pub trait RecordExt {
    fn imports(&self, path: &str) -> Option<Import>;
}

impl RecordExt for Record {
    fn imports(&self, path: &str) -> Option<Import> {
        let names = self
            .fields()
            .flat_map(|f| f.ty.inner_model())
            .filter(|&name| name != self.name())
            .map(|name| name.to_rust_struct().0)
            .collect::<BTreeSet<_>>();
        if !names.is_empty() {
            Some(Import::new(path, names.into_iter().collect::<Vec<_>>()))
        } else {
            None
        }
    }
}

pub trait HirFieldExt {
    fn implements_default(&self, spec: &HirSpec) -> bool;
}

impl HirFieldExt for HirField {
    fn implements_default(&self, spec: &HirSpec) -> bool {
        self.optional || self.ty.implements_default(spec)
    }
}

/// Generate a model.rs file that just imports from dependents.
pub fn generate_model_rs(spec: &HirSpec, config: &ConfigFlags) -> File<TokenStream> {
    let it = spec.schemas.keys();
    let imports = it
        .clone()
        .map(|name: &String| {
            let fname = sanitize_filename(&name);
            Import::new(&fname, vec!["*"]).public()
        })
        .collect();
    let code = it
        .clone()
        .map(|name| {
            let name = Ident(sanitize_filename(name));
            quote! {
                mod #name;
            }
        })
        .collect();
    File {
        imports,
        code: Some(code),
        ..File::default()
    }
}

/// Generate the file for a single struct.
pub fn generate_single_model_file(
    name: &str,
    record: &Record,
    spec: &HirSpec,
    config: &PackageConfig,
) -> File<TokenStream> {
    let mut imports = vec![import!("serde", Serialize, Deserialize)];
    if let Some(import) = record.imports("super") {
        imports.push(import);
    }
    File {
        code: Some(create_struct(record, &config, spec)),
        imports,
        ..File::default()
    }
}

pub struct RefTarget {
    name: String,
    ty: Ty,
}

pub fn create_sumtype_struct(
    schema: &Struct,
    config: &ConfigFlags,
    spec: &HirSpec,
    derives: &Vec<String>,
) -> TokenStream {
    let default = schema.derive_default(spec);
    let derives = derives_to_tokens(derives);
    let ormlite = config.ormlite.then(|| quote! {
        #[cfg_attr(feature = "ormlite", derive(ormlite::TableMeta, ormlite::IntoArguments, ormlite::FromRow))]
    }).unwrap_or_default();
    let fake = config.fake && schema.fields.values().all(|f| f.ty.implements_dummy(spec));
    let dummy = fake
        .then(|| {
            quote! {
                #[cfg_attr(feature = "fake", derive(fake::Dummy))]
            }
        })
        .unwrap_or_default();

    let docs = schema.docs.clone().to_rust_code();

    let name = schema.name.to_rust_struct();
    let fields = schema.model_fields(config).map(ToRustCode::to_rust_code);
    let deref = schema
        .ref_target()
        .map(|t| {
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
        })
        .unwrap_or_default();

    quote! {
        #docs
        #ormlite
        #dummy
        #[derive(Debug, Clone, Serialize, Deserialize #default #derives)]
        pub struct #name {
            #(#fields,)*
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }
        #deref
    }
}

fn create_enum_struct(e: &StrEnum, derives: &Vec<String>) -> TokenStream {
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
    let derives = derives_to_tokens(derives);
    quote! {
        #[derive(Debug, Serialize, Deserialize #derives)]
        pub enum #name {
            #(#enums,)*
        }
    }
}

pub fn create_newtype_struct(
    schema: &NewType,
    spec: &HirSpec,
    derives: &Vec<String>,
) -> TokenStream {
    let name = schema.name.to_rust_struct();
    let fields = schema.fields.iter().map(|f| f.ty.to_rust_type());
    let derives = derives_to_tokens(derives);
    let default = schema
        .fields
        .iter()
        .all(|f| f.implements_default(spec))
        .then(|| {
            quote! { , Default }
        })
        .unwrap_or_default();
    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize #default #derives)]
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

pub fn create_struct(record: &Record, config: &PackageConfig, hir: &HirSpec) -> TokenStream {
    match record {
        Record::Struct(s) => create_sumtype_struct(s, &config.config, hir, &config.derives),
        Record::NewType(nt) => create_newtype_struct(nt, hir, &config.derives),
        Record::Enum(en) => create_enum_struct(en, &config.derives),
        Record::TypeAlias(name, field) => create_typealias(name, field),
    }
}

pub fn derives_to_tokens(derives: &Vec<String>) -> TokenStream {
    derives
        .iter()
        .map(|d| {
            if let Ok(d) = d.trim().parse::<TokenStream>() {
                quote! { , #d }
            } else {
                return TokenStream::new();
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use hir::HirField;
    use mir::Ty;
    use mir_rust::format_code;

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
            docs: None,
        };
        let code = create_newtype_struct(&schema, &HirSpec::default(), &vec![]);
        let code = format_code(code);
        assert_eq!(
            &code,
            "
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewType(pub String);
"
            .trim()
        );
    }

    #[test]
    fn test_struct_sumtype_empty_derive() {
        let name = "SumType".to_string();
        let schema = Struct {
            nullable: false,
            name,
            fields: vec![
                (
                    "field1".to_string(),
                    HirField {
                        ty: Ty::String,
                        optional: true,
                        ..HirField::default()
                    },
                ),
                (
                    "field2".to_string(),
                    HirField {
                        ty: Ty::String,
                        optional: false,
                        ..HirField::default()
                    },
                ),
            ]
            .into_iter()
            .collect(),
            docs: None,
        };
        let code = create_sumtype_struct(
            &schema,
            &ConfigFlags::default(),
            &HirSpec::default(),
            &vec![],
        );
        let code = format_code(code);
        println!("{}", code);
        assert_eq!(
            &code,
            r#"
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SumType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field1: Option<String>,
    pub field2: String,
}
impl std::fmt::Display for SumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
"#
            .trim()
        );
    }

    #[test]
    fn test_struct_sumtype_nonempty_derive() {
        let name = "SumType".to_string();
        let derives = vec!["oasgen::OaSchema".to_string(), "example::Other".to_string()];
        let schema = Struct {
            nullable: false,
            name,
            fields: vec![
                (
                    "field1".to_string(),
                    HirField {
                        ty: Ty::String,
                        optional: true,
                        ..HirField::default()
                    },
                ),
                (
                    "field2".to_string(),
                    HirField {
                        ty: Ty::String,
                        optional: false,
                        ..HirField::default()
                    },
                ),
            ]
            .into_iter()
            .collect(),
            docs: None,
        };
        let code = create_sumtype_struct(
            &schema,
            &ConfigFlags::default(),
            &HirSpec::default(),
            &derives,
        );
        let code = format_code(code);
        assert_eq!(
            &code,
            r#"
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Default,
    oasgen::OaSchema,
    example::Other
)]
pub struct SumType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field1: Option<String>,
    pub field2: String,
}
impl std::fmt::Display for SumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
"#
            .trim()
        );
    }
}
