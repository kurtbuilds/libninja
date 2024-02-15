use proc_macro2::TokenStream;
use quote::quote;
use mir::{FnArg2, Function};
use crate::{FluentBool, ToRustCode};

impl ToRustCode for Function<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let Function {
            name,
            args,
            body,
            doc,
            async_,
            annotations,
            ret,
            public,
            ..
        } = self;
        let annotations = annotations
            .into_iter()
            .map(|a| syn::parse_str::<syn::Expr>(&a).unwrap());
        let doc = doc.to_rust_code();

        let vis = public.to_value(|| quote!(pub));
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

impl ToRustCode for FnArg2<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        match self {
            FnArg2::Basic { name, ty, default } => {
                if default.is_some() {
                    panic!("No default args in Rust")
                }
                quote!(#name: #ty)
            }
            FnArg2::Unpack { .. } => panic!("unpack args not yet supported in Rust"),
            FnArg2::SelfArg { reference, mutable } => {
                let mutability = mutable.then(|| quote!(mut)).unwrap_or_default();
                let reference = reference.then(|| quote!(&)).unwrap_or_default();
                quote!(#reference #mutability self)
            }
            FnArg2::Variadic { .. } | FnArg2::Kwargs { .. } => panic!("No variadic or kwargs args in Rust"),
        }
    }
}
