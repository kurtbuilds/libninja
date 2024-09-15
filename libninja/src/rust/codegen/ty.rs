use proc_macro2::TokenStream;
use quote::quote;

use hir::HirSpec;
use mir::Ty;

use crate::rust::codegen::ToRustIdent;
use crate::rust::lower_hir::HirFieldExt;

/// Use this to generate Rust code types.
pub trait ToRustType {
    fn to_rust_type(&self) -> TokenStream;
    fn to_reference_type(&self, specifier: TokenStream) -> TokenStream;
    fn is_reference_type(&self) -> bool;
    fn implements_default(&self, spec: &HirSpec) -> bool;
    fn implements_dummy(&self, spec: &HirSpec) -> bool;
}

impl ToRustType for Ty {
    fn to_rust_type(&self) -> TokenStream {
        match self {
            Ty::String => quote!(String),
            Ty::Integer { .. } => quote!(i64),
            Ty::Float => quote!(f64),
            Ty::Boolean => quote!(bool),
            Ty::Array(inner) => {
                let inner = inner.to_rust_type();
                quote!(Vec<#inner>)
            }
            Ty::Model(inner, ..) => inner.to_rust_struct().into(),
            Ty::Unit => quote!(()),
            Ty::Any(_) => quote!(serde_json::Value),
            Ty::Date { .. } => quote!(chrono::NaiveDate),
            Ty::DateTime { .. } => quote!(chrono::DateTime<chrono::Utc>),
            Ty::Currency { .. } => quote!(rust_decimal::Decimal),
            Ty::HashMap(inner) => {
                let inner = inner.to_rust_type();
                quote!(std::collections::HashMap<String, #inner>)
            }
        }
    }

    fn to_reference_type(&self, specifier: TokenStream) -> TokenStream {
        match self {
            Ty::String => quote!(& #specifier str),
            Ty::Integer { .. } => quote!(i64),
            Ty::Float => quote!(f64),
            Ty::Boolean => quote!(bool),
            Ty::Array(inner) => {
                if inner.is_reference_type() {
                    let inner = inner.to_reference_type(specifier.clone());
                    quote! { & #specifier [#inner] }
                } else {
                    self.to_rust_type()
                }
            }
            Ty::Model(inner, ..) => inner.to_rust_struct().into(),
            Ty::Unit => quote!(()),
            Ty::Any(_) => quote!(serde_json::Value),
            Ty::Date { .. } => quote!(chrono::NaiveDate),
            Ty::DateTime { .. } => quote!(chrono::DateTime<chrono::Utc>),
            Ty::Currency { .. } => quote!(rust_decimal::Decimal),
            Ty::HashMap(inner) => {
                let inner = inner.to_rust_type();
                quote!(std::collections::HashMap<String, #inner>)
            }
        }
    }

    fn is_reference_type(&self) -> bool {
        match self {
            Ty::String => true,
            Ty::Array(inner) => inner.is_reference_type(),
            // Ty::Array(inner) => true,
            _ => false,
        }
    }

    fn implements_default(&self, spec: &HirSpec) -> bool {
        match self {
            Ty::String => true,
            Ty::Integer { .. } => true,
            Ty::Float => true,
            Ty::Boolean => true,
            Ty::Array(_) => true,
            Ty::Model(name) => {
                let model = spec.get_record(name.as_str()).expect("Model not found");
                model.fields().all(|f| f.implements_default(spec))
            }
            Ty::Unit => true,
            Ty::Any(_) => true,
            Ty::Date { .. } => true,
            Ty::DateTime => true,
            Ty::Currency { .. } => true,
            Ty::HashMap(_) => true,
        }
    }

    fn implements_dummy(&self, spec: &HirSpec) -> bool {
        match self {
            Ty::String => true,
            Ty::Integer { .. } => true,
            Ty::Float => true,
            Ty::Boolean => true,
            Ty::Array(inner) => inner.implements_dummy(spec),
            Ty::Model(name) => {
                let model = spec.get_record(name.as_str()).expect("Model not found");
                model.fields().all(|f| f.ty.implements_dummy(spec))
            }
            Ty::Unit => true,
            Ty::Any(_) => false,
            Ty::Date { .. } => true,
            Ty::DateTime => true,
            Ty::Currency { .. } => true,
            Ty::HashMap(inner) => inner.implements_dummy(spec),
        }
    }
}
