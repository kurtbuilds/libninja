use openapiv3::{OpenAPI, ReferenceOr, Schema};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use crate::codegen::util::ToToken;

pub fn all_struct_Schema(spec: &OpenAPI) -> TokenStream {
    let schemas = spec.components.as_ref().unwrap().schemas.iter().map(|(k, schema)| {
        let schema = schema
            .as_ref()
            .resolve(spec).unwrap();
        struct_Schema(k, schema, spec)
    });
    quote! {
        #(#schemas)*
    }
}

pub fn struct_Schema_object(name: &str, schema: &Schema, spec: &OpenAPI) -> TokenStream {
    let fields = schema.properties().unwrap().iter().map(|(k, v)| {
        let mut k = k.to_string();
        let prop_schema = v
            .as_ref()
            .resolve(spec)
            .unwrap();
        let mut field_type = match v {
            ReferenceOr::Reference { ref reference } => {
                let name = reference.rsplit('/').next().unwrap();
                syn::Ident::new(name, Span::call_site()).to_token_stream()
            },
            ReferenceOr::Item(schema) => schema.to_token(spec),
        };
        if !schema.required(&k) && !prop_schema.schema_data.nullable {
            field_type = quote! { Option<#field_type>};
        }
        let serde = if ["type", "use"].contains(&k.as_str()) {
            let serde = quote! {
                #[serde(rename = #k)]
            };
            k += "_";
            serde
        } else {
            TokenStream::new()
        };
        let field = syn::Ident::new(&k, Span::call_site());
        quote! {
            #serde
            pub #field: #field_type,
        }
    });
    let name = syn::Ident::new(name, Span::call_site());
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #name {
            #(#fields)*
        }
    }
}

pub fn struct_Schema_newtype(name: &str, schema: &Schema, spec: &OpenAPI) -> TokenStream {
    let field = syn::Ident::new("value", Span::call_site());
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