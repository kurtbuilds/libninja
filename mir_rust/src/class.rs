use mir::{Class, Field};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::ToRustCode;

pub struct RustClass {
    pub class: Class<TokenStream>,
    pub lifetimes: Vec<String>,
}

impl ToRustCode for RustClass {
    fn to_rust_code(self) -> TokenStream {
        let RustClass {
            class:
                Class {
                    vis,
                    name,
                    doc,
                    fields,
                    methods,
                    attributes,
                },
            lifetimes,
        } = self;
        let vis = vis.to_rust_code();
        let fields = fields.into_iter().map(|f| f.to_rust_code());
        let class_methods = methods.into_iter().map(|m| m.into().to_rust_code());

        let doc = doc.to_rust_code();
        let lifetimes = if lifetimes.is_empty() {
            quote! {}
        } else {
            let lifetimes = lifetimes.iter().map(|l| {
                let name = syn::Lifetime::new(l, Span::call_site());
                quote! { # name }
            });
            quote! { < # ( # lifetimes), * > }
        };
        quote! {
            #doc
            #(#attributes)*
            #vis struct #name #lifetimes {
                #(#fields,)*
            }
            impl #lifetimes #name #lifetimes {
                #(#class_methods)*
            }
        }
    }
}

impl ToRustCode for Field<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let name = self.name;
        let ty = if self.optional {
            let ty = self.ty;
            quote! { Option<#ty> }
        } else {
            self.ty
        };
        let vis = self.vis.to_rust_code();
        let doc = self.doc.to_rust_code();
        let decorators = self.decorators;
        quote! {
            #doc
            #(
                #decorators
            )*
            #vis #name: #ty
        }
    }
}
