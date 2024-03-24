use mir::{Class, Field};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::ToRustCode;

impl ToRustCode for Class<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let Class {
            name,
            doc,
            code,
            instance_fields,
            static_fields,
            constructors,
            class_methods,
            static_methods,
            vis,
            lifetimes,
            decorators,
            superclasses
        } = self;
        assert!(superclasses.is_empty(), "superclasses not supported in Rust");
        assert!(static_fields.is_empty(), "static fields not supported in Rust");
        assert!(constructors.is_empty(), "constructors not supported in Rust");
        assert!(code.is_none(), "code in class body not supported in Rust");
        assert!(static_methods.is_empty(), "static methods not supported in Rust");

        let vis = vis.to_rust_code();
        let fields = instance_fields.into_iter().map(|f| f.to_rust_code());
        let class_methods = class_methods.into_iter().map(|m| m.to_rust_code());

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
            #(
                #decorators
            )*
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