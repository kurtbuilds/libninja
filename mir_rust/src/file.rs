use crate::ToRustCode;
use mir::{File, Interface, Item};
use proc_macro2::TokenStream;
use quote::quote;
use std::mem::take;

impl ToRustCode for File<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let File {
            attributes: annotations,
            doc,
            mut imports,
            mut items,
            modules,
        } = self;
        for m in &mut items {
            let Item::Class(m) = m else {
                continue;
            };
            imports.extend(take(&mut m.imports));
        }
        let imports = imports.into_iter().map(|i| i.to_rust_code());
        let doc = doc.to_rust_code();
        let items = items.into_iter().map(|f| f.to_rust_code());
        let modules = modules.into_iter().map(|m| {
            let vis = m.vis.to_rust_code();
            let name = syn::parse_str::<syn::Ident>(&m.name).unwrap();
            quote! {
                #vis mod #name;
            }
        });
        quote! {
            #(#annotations)*
            #doc
            #(#imports)*
            #(#modules)*
            #(#items)*
        }
    }
}

impl ToRustCode for Item<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        match self {
            Item::Class(c) => c.to_rust_code(),
            Item::Enum(e) => e.to_rust_code(),
            Item::Interface(i) => i.to_rust_code(),
            Item::Fn(f) => f.to_rust_code(),
            Item::Block(b) => b,
        }
    }
}

impl ToRustCode for Interface<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        panic!("Rust does not support interfaces")
    }
}
