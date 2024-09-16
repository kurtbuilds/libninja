mod function;
mod body;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ReturnType, Visibility};
use crate::body::body_callable;

/// Define a function where the body is a string. The fn interface definition is reminiscent of Python,
/// but because it creates a mir::Function, it will compile down into whatever language we target.
/// The body has to be valid code for the target language though. We don't have a MIR for the AST -
/// nor would making one make sense (languages don't have mutually compatible ASTs)
#[proc_macro]
pub fn function(item: TokenStream) -> TokenStream {

    let item = parse_macro_input!(item as function::FnHeader);
    let vis = match item.vis {
        Visibility::Public(_) => quote!(::mir::Visibility::Public),
        Visibility::Restricted(_) => quote!(::mir::Visibility::Private),
        Visibility::Inherited => quote!(::mir::Visibility::Private),
    };
    let is_async = if item.sig.asyncness.is_some() {
        quote!(true)
    } else {
        quote!(false)
    };
    let name = item.sig.ident.to_string();
    let ret = match item.sig.output {
        ReturnType::Default => TokenStream2::new(),
        ReturnType::Type(_, t) => t.to_token_stream(),
    };
    let args = item.sig.inputs.iter().map(|arg| {
        let (name, ty) = match arg {
            syn::FnArg::Receiver(_) => panic!("Self arguments are not supported"),
            syn::FnArg::Typed(pat) => {
                let name = match &*pat.pat {
                    syn::Pat::Ident(ident) => ident.ident.to_string(),
                    _ => panic!("Only simple identifiers are supported"),
                };
                let ty = pat.ty.to_token_stream();
                (name, ty)
            }
        };
        quote! {
            ::mir::Arg::Basic {
                name: Ident(#name.to_string()),
                ty: #ty,
                default: None,
            }
        }
    });

    quote! {
        ::mir::Function {
            name: Ident(#name.to_string()),
            is_async: #is_async,
            vis: #vis,
            args: vec![#(#args),*],
            ret: #ret,
            ..::mir::Function::default()
        }
    }.into()
}

#[proc_macro]
pub fn body(body: TokenStream) -> TokenStream {
    body_callable(body).into()
}