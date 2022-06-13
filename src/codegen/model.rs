use convert_case::Case;
use convert_case::Casing;
use openapiv3::{OpenAPI, ReferenceOr, Schema};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

use crate::codegen::util::ToIdent;
use crate::codegen::util::ToToken;

pub fn generate_model_rs(spec: &OpenAPI) -> TokenStream {
    let all_struct_Schema = all_struct_Schema(spec);

    quote! {
        #all_struct_Schema
    }
}


pub fn all_struct_Schema(spec: &OpenAPI) -> TokenStream {
    let schemas = spec.components.as_ref().unwrap().schemas.iter().map(|(k, schema)| {
        let schema = schema.resolve(spec);
        struct_Schema(k, schema, spec)
    });
    quote! {
        #(#schemas)*
    }
}

pub fn struct_Schema_object(name: &str, struct_schema: &Schema, spec: &OpenAPI) -> TokenStream {
    let fields = struct_schema.properties().unwrap().iter().map(|(k, v)| {
        let k = k.to_string();
        let prop_schema = v.resolve(spec);

        let field_type = match v {
            ReferenceOr::Reference { ref reference } => {
                let name = reference.rsplit('/').next().unwrap();
                let field_type = name.to_struct_name();
                if prop_schema.schema_data.nullable {
                    quote! { Option<#field_type> }
                } else {
                    quote! { #field_type }
                }
            }
            ReferenceOr::Item(schema) => schema.to_token(spec),
        };
        let serde_attr = if k.is_restricted() {
            let serde_line = quote! {
                #[serde(rename = #k)]
            };
            serde_line
        } else {
            TokenStream::new()
        };
        let doc_attr = if let Some(doc) = &prop_schema.schema_data.description {
            quote! {
                #[doc = #doc]
            }
        } else {
            TokenStream::new()
        };
        let field = k.to_ident();
        quote! {
            #serde_attr
            #doc_attr
            pub #field: #field_type,
        }
    });
    let name = name.to_struct_name();
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #name {
            #(#fields)*
        }
    }
}

pub fn struct_Schema_newtype(name: &str, schema: &Schema, spec: &OpenAPI) -> TokenStream {
    let field_type = schema.to_token(spec);
    let name = syn::Ident::new(name, Span::call_site());
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #name(pub #field_type);
    }
}

pub fn struct_Schema(name: &str, schema: &Schema, spec: &OpenAPI) -> TokenStream {
    match schema.properties() {
        Some(properties) => struct_Schema_object(name, schema, spec),
        None => struct_Schema_newtype(name, schema, spec),
    }
}
