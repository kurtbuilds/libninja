mod function;
mod body;
mod rfunction;

use crate::body::body_callable;
use crate::function::{parse_args, parse_intro, parse_return, Arg, Tags};
use proc_macro::Delimiter;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, };

/// Define a function where the body is a string. The fn interface definition is reminiscent of Python,
/// but because it creates a mir::Function, it will compile down into whatever language we target.
/// The body has to be valid code for the target language though. We don't have a MIR for the AST -
/// nor would making one make sense (languages don't have mutually compatible ASTs)
#[proc_macro]
pub fn function(item: TokenStream) -> TokenStream {
    let mut toks = item.into_iter().peekable();

    let Tags { is_async: asyn, vis, fn_name } = parse_intro(&mut toks);
    // 2. Argument groups
    let args = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            parse_args(group.stream()).into_iter().map(|arg| {
                let Arg { name, arg_type, default } = arg;
                quote! {
                    ::mir::Arg::Basic {
                        name: ::mir::Ident::new(#name),
                        ty: #arg_type,
                        default: #default,
                    }
                }
            }).collect::<Vec<_>>()
        },
        None => Vec::new(),
        _ => panic!("Expected a group of arguments"),
    };

    let ret = parse_return(&mut toks);

    // 4. Body
    let body = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            body_callable(group.stream())
        }
        None => quote! { Default::default() },
        other => panic!("Expected a function body. Got: {:?}", other),
    };
    
    quote! {
        ::mir::Function {
            name: #fn_name,
            is_async: #asyn,
            vis: #vis,
            args: vec![#(#args),*],
            ret: #ret,
            body: #body,
            ..::mir::Function::default()
        }
    }.into()
}


/// like function, but for Rust
#[proc_macro]
pub fn rfunction(item: TokenStream) -> TokenStream {
    let mut toks = item.into_iter().peekable();

    let Tags { is_async, vis, fn_name } = parse_intro(&mut toks);
    // 2. Argument groups
    let args = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            rfunction::parse_args2(group.stream()).into_iter().map(|arg| {
                let Arg { name, arg_type, default } = arg;
                quote! {
            ::mir::Arg::Basic {
                name: ::mir::Ident(#name.to_string()),
                ty: #arg_type,
                default: #default,
            }
        }
            }).collect::<Vec<_>>()
        },
        None => Vec::new(),
        _ => panic!("Expected a group of arguments"),
    };

    let ret = rfunction::parse_return2(&mut toks);

    let body = match toks.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            let toks = TokenStream2::from(group.stream());
            let toks = quote! {
                ::quote::quote!(#toks)
            };
            toks
        }
        None => quote::quote!(Default::default()),
        other => panic!("Expected function body. Got: {:?}", other),
    };

    quote! {
        ::mir::Function {
            name: #fn_name,
            is_async: #is_async,
            vis: #vis,
            args: vec![#(#args),*],
            ret: #ret,
            body: #body,
            ..::mir::Function::default()
        }
    }.into()
}

#[proc_macro]
pub fn body(body: TokenStream) -> TokenStream {
    body_callable(body).into()
}