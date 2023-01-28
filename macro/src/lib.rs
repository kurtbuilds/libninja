mod rfunction;
mod function;
mod body;

use proc_macro::{Delimiter, TokenStream, TokenTree};

use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use body::body_callable;
use function::{Arg, Tags};
use crate::function::{parse_intro, parse_args, parse_return} ;


#[proc_macro]
pub fn function(item: TokenStream) -> TokenStream {
    let mut toks = item.into_iter().peekable();

    let Tags { asyn, public, fn_name } = parse_intro(&mut toks);
    // 2. Argument groups
    let arg_toks = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => group,
        _ => panic!("Expected a group of arguments"),
    };
    let args = parse_args(arg_toks.stream()).into_iter().map(|arg| {
        let Arg { name, arg_type, default } = arg;
        quote! {
            ::ln_mir::FnArg {
                name: #name.into(),
                ty: #arg_type,
                default: #default,
                treatment: None,
            }
        }
    }).collect::<Vec<_>>();

    let ret = parse_return(&mut toks);

    // 4. Body
    let body = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            body_callable(group.stream())
        }
        other => panic!("Expected a function body. Got: {:?}", other),
    };

    quote! {
        ::ln_mir::Function {
            name: #fn_name,
            async_: #asyn,
            public: #public,
            args: vec![#(#args),*],
            ret: #ret,
            body: #body,
            ..::ln_mir::Function::default()
        }
    }.into()
}


#[proc_macro]
pub fn rfunction(item: TokenStream) -> TokenStream {
    let mut toks = item.into_iter().peekable();

    let Tags { asyn, public, fn_name } = function::parse_intro(&mut toks);
    // 2. Argument groups
    let arg_toks = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => group,
        _ => panic!("Expected a group of arguments"),
    };
    let args = rfunction::parse_args2(arg_toks.stream()).into_iter().map(|arg| {
        let Arg { name, arg_type, default } = arg;
        quote! {
            ::ln_mir::FnArg {
                name: #name.into(),
                ty: #arg_type,
                default: #default,
                treatment: None,
            }
        }
    }).collect::<Vec<_>>();

    let ret = rfunction::parse_return2(&mut toks);

    let body = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            let toks = TokenStream2::from(group.stream());
            let toks = quote! {
                ::quote::quote!(#toks)
            };
            toks
        }
        other => panic!("Expected function body. Got: {:?}", other),
    };

    quote! {
        ::ln_mir::Function {
            name: #fn_name,
            async_: #asyn,
            public: #public,
            args: vec![#(#args),*],
            ret: #ret,
            body: #body,
            ..::ln_mir::Function::default()
        }
    }.into()
}


#[proc_macro]
pub fn body(body: TokenStream) -> TokenStream {
    body_callable(body).into()
}
