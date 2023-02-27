use proc_macro2::TokenStream;
use quote::quote;
use ln_core::hir::Ty;
use crate::rust::codegen::ident::ToRustIdent;

/// Use this to generate Rust code types.
pub trait ToRustType {
    fn to_rust_type(&self) -> TokenStream;
    fn to_reference_type(&self, specifier: TokenStream) -> TokenStream;
    fn is_reference_type(&self) -> bool;
    fn implements_default(&self) -> bool;
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
            Ty::Model(inner, ..) => {
                inner.to_rust_struct().into()
            }
            Ty::Unit => quote!(()),
            Ty::Any => quote!(serde_json::Value),
            Ty::Date { .. } => quote!(chrono::NaiveDate),
            Ty::DateTime { .. } => quote!(chrono::DateTime<chrono::Utc>),
            Ty::Currency { .. } => quote!(rust_decimal::Decimal),
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
            Ty::Model(inner, ..) => {
                inner.to_rust_struct().into()
            }
            Ty::Unit => quote!(()),
            Ty::Any => quote!(serde_json::Value),
            Ty::Date { .. } => quote!(chrono::NaiveDate),
            Ty::DateTime { .. } => quote!(chrono::DateTime<chrono::Utc>),
            Ty::Currency { .. } => quote!(rust_decimal::Decimal),
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


    fn implements_default(&self) -> bool {
        match self {
            Ty::String => true,
            Ty::Integer { .. } => true,
            Ty::Float => true,
            Ty::Boolean => true,
            Ty::Array(_) => true,
            Ty::Model(..) => false,
            Ty::Unit => true,
            Ty::Any => false,
            Ty::Date { .. } => false,
            Ty::DateTime => false,
            Ty::Currency { .. } => false,
        }
    }
}
