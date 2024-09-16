use crate::{FluentBool, ToRustCode};
use mir::Arg;
use mir::Function;
use proc_macro2::TokenStream;
use quote::quote;

impl ToRustCode for Function<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let Function {
            name,
            args,
            ret,
            body,
            doc,
            is_async,
            vis,
            attributes: annotations,
        } = self;
        let doc = doc.to_rust_code();
        let vis = vis.to_rust_code();
        let async_ = is_async.to_value(|| quote!(async));
        let args = args.into_iter().map(|a| a.to_rust_code());
        let ret = (!ret.is_empty()).to_value(|| quote!( -> #ret));
        quote! {
            #(#annotations)*
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
