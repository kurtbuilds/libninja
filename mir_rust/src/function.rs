use crate::{FluentBool, ToRustCode};
use mir::Arg;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct RustFunction {
    pub inner: mir::Function<TokenStream>,
    pub annotations: Vec<TokenStream>,
}

impl From<mir::Function<TokenStream>> for RustFunction {
    fn from(inner: mir::Function<TokenStream>) -> Self {
        Self {
            inner,
            annotations: vec![],
        }
    }
}

impl Deref for RustFunction {
    type Target = mir::Function<TokenStream>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RustFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ToRustCode for RustFunction {
    fn to_rust_code(self) -> TokenStream {
        let RustFunction {
            annotations,
            inner:
                mir::Function {
                    name,
                    args,
                    ret,
                    body,
                    doc,
                    is_async: async_,
                    vis,
                },
        } = self;
        let doc = doc.to_rust_code();
        let vis = vis.to_rust_code();
        let async_ = async_.to_value(|| quote!(async));
        let args = args.into_iter().map(|a| a.to_rust_code());
        let ret = (!ret.is_empty()).to_value(|| quote!( -> #ret));
        quote! {
            #(#[ #annotations ])*
            #doc
            #vis #async_ fn #name(#(#args),*) #ret {
                #body
            }
        }
    }
}

impl ToRustCode for Arg<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        match self {
            Arg::Basic { name, ty, default } => {
                if default.is_some() {
                    panic!("No default args in Rust")
                }
                quote!(#name: #ty)
            }
            Arg::Unpack { .. } => panic!("unpack args not yet supported in Rust"),
            Arg::SelfArg { reference, mutable } => {
                let mutability = mutable.then(|| quote!(mut)).unwrap_or_default();
                let reference = reference.then(|| quote!(&)).unwrap_or_default();
                quote!(#reference #mutability self)
            }
            Arg::Variadic { .. } | Arg::Kwargs { .. } => {
                panic!("No variadic or kwargs args in Rust")
            }
        }
    }
}
