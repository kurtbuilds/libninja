use convert_case::{Case, Casing};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};


pub trait ToToken {
    fn to_token(&self, spec: &OpenAPI) -> TokenStream;
}

impl ToToken for Schema {
    fn to_token(&self, spec: &OpenAPI) -> TokenStream {
        let z = match &self.schema_kind {
            SchemaKind::Type(openapiv3::Type::String(s)) => quote!(String),
            SchemaKind::Type(openapiv3::Type::Number(_)) => quote!(f64),
            SchemaKind::Type(openapiv3::Type::Integer(_)) => quote!(i64),
            SchemaKind::Type(openapiv3::Type::Boolean{}) => quote!(bool),
            SchemaKind::Type(openapiv3::Type::Object(o)) => {
                quote!(serde_json::Value)
            }
            SchemaKind::Type(openapiv3::Type::Array(a)) => {
                let inside = a.items
                    .as_ref()
                    .unwrap()
                    .unbox_ref()
                    .to_token(spec);
                quote! { Vec<#inside> }
            }
            SchemaKind::Any(..) => quote!(serde_json::Value),
            SchemaKind::AllOf{..} => quote!(serde_json::Value),
            _ => {
                println!("unimplemented: {:?}", self);
                unimplemented!()
            },
        };
        if self.schema_data.nullable {
            quote! { Option<#z> }
        } else {
            z
        }
    }
}


impl ToToken for ReferenceOr<&Schema> {
    fn to_token(&self, spec: &OpenAPI) -> TokenStream {
        match self {
            ReferenceOr::Reference{ .. } => {
                let name = self.get_struct_name().unwrap();
                syn::Ident::new(&name, Span::call_site()).to_token_stream()
            }
            ReferenceOr::Item(s) => s.to_token(spec),
        }
    }
}


pub trait ToIdent {
    fn to_struct_name(&self) -> syn::Ident;
    fn to_ident(&self) -> syn::Ident;
    fn is_restricted(&self) -> bool;
}

impl ToIdent for str {
    fn to_struct_name(&self) -> syn::Ident {
        let s = if self.is_restricted() {
            self.to_case(Case::Pascal) + "Struct"
        } else {
            self.to_case(Case::Pascal)
        };
        syn::Ident::new(&s, Span::call_site())
    }

    fn to_ident(&self) -> Ident {
        let s = if self.is_restricted() {
            self.to_case(Case::Snake) + "_"
        } else {
            self.to_case(Case::Snake)
        };
        syn::Ident::new(&s, Span::call_site())
    }

    fn is_restricted(&self) -> bool {
        ["type", "use"].contains(&self)
    }
}