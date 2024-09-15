use crate::{RustExtra, ToRustCode};
use mir::{Enum, Variant};
use proc_macro2::TokenStream;
use quote::quote;

impl ToRustCode for Enum<TokenStream, RustExtra> {
    fn to_rust_code(self) -> TokenStream {
        let Enum {
            name,
            doc,
            vis,
            variants,
            methods,
            extra,
        } = self;
        let vis = vis.to_rust_code();
        let doc = doc.to_rust_code();
        let variants = variants.into_iter().map(|v| v.to_rust_code());
        let methods = methods.into_iter().map(|m| m.to_rust_code());
        let attributes = extra.attributes;
        quote! {
            #doc
            #(#attributes)*
            #vis enum #name {
                #(#variants),*
            }
            impl #name {
                #(#methods)*
            }
        }
    }
}

impl ToRustCode for Variant<RustExtra> {
    fn to_rust_code(self) -> TokenStream {
        let Variant {
            ident,
            doc,
            value,
            extra,
        } = self;
        let doc = doc.to_rust_code();
        let attributes = extra.attributes;
        let value = value.map(|v| quote!(= #v)).unwrap_or_default();
        quote! {
            #doc
            #(#attributes)*
            #ident #value
        }
    }
}
