use mir::{Class, Function};
use proc_macro2::{TokenStream, Span};
use quote::quote;
use crate::{ToRustCode, ToRustIdent};

impl ToRustCode for Class<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let vis = self.public.then(|| quote!(pub)).unwrap_or_default();
        let fields = self.instance_fields.iter().map(|f| {
            let name = &f.name.to_rust_ident();
            let ty = &f.ty;
            let public = f.vis.to_rust_code();
            quote! { #public #name: #ty }
        });
        let instance_methods = self.instance_methods.into_iter().map(|m|
            codegen_function(m, quote! { self , })
        );
        let mut_self_instance_methods = self.mut_self_instance_methods.into_iter().map(|m| {
            codegen_function(m, quote! { mut self , })
        });
        let class_methods = self.class_methods.into_iter().map(|m| {
            codegen_function(m, TokenStream::new())
        });

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
            impl #lifetimes #name #lifetimes{
                #(#instance_methods)*
                #(#mut_self_instance_methods)*
                #(#class_methods)*
            }
        }
    }
}

pub fn codegen_function(func: Function<TokenStream>, self_arg: TokenStream) -> TokenStream {
    let name = func.name;
    let args = func.args.into_iter().map(|a| a.to_rust_code());
    let ret = func.ret;
    let async_ = func.async_.then(|| quote!(async)).unwrap_or_default();
    let vis = func.public.then(|| quote!(pub)).unwrap_or_default();
    let body = &func.body;
    quote! {
        #vis #async_ fn #name(#self_arg #(#args),*) -> #ret {
            #body
        }
    }
}

