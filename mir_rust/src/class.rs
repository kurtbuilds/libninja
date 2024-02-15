use proc_macro2::{Span, TokenStream};
use quote::quote;

use mir::{Class, Field};

use crate::{FluentBool, ToRustCode, ToRustIdent};

impl ToRustCode for Class<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let vis = self.public.to_value(|| quote!(pub));
        let fields = self.instance_fields.into_iter().map(|f| f.to_rust_code());
        let class_methods = self.class_methods.into_iter().map(|m| m.to_rust_code());

        let doc = self.doc.to_rust_code();
        let lifetimes = if self.lifetimes.is_empty() {
            quote! {}
        } else {
            let lifetimes = self.lifetimes.iter().map(|l| {
                let name = syn::Lifetime::new(l, Span::call_site());
                quote! { # name }
            });
            quote! { < # ( # lifetimes), * > }
        };
        let decorator = self.decorators;
        let name = self.name;
        quote! {
            #doc
            #(
                #decorator
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
        let name = self.name.to_rust_ident();
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