use proc_macro2::TokenStream;
use quote::quote;
use mir::{Enum, Variant};
use crate::ToRustCode;

impl ToRustCode for Enum<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let Enum { name, doc, vis, decorators, variants, methods } = self;
        let vis = vis.to_rust_code();
        let doc = doc.to_rust_code();
        let variants = variants
            .into_iter()
            .map(|v| v.to_rust_code());
        let methods = methods
            .into_iter()
            .map(|m| m.to_rust_code());
        quote! {
            #doc
            #(#decorators)*
            #vis enum #name {
                #(#variants),*
            }
            impl #name {
                #(#methods)*
            }
        }
    }
}

impl ToRustCode for Variant {
    fn to_rust_code(self) -> TokenStream {
        let Variant { name, doc } = self;
        let doc = doc.to_rust_code();
        quote! {
            #doc
            #name
        }
    }
}