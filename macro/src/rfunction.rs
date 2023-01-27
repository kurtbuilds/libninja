use proc_macro::{Delimiter, TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use std::iter::Peekable;
use quote::quote;
use crate::Arg;

pub fn parse_args2(arg_toks: TokenStream) -> Vec<Arg> {
    let mut args = Vec::new();
    let mut arg_toks = arg_toks.into_iter().peekable();
    loop {
        let arg_name = match arg_toks.next() {
            Some(TokenTree::Ident(ident)) => ident.to_string(),
            Some(other) => panic!("Expected an argument name. Got: {:?}", other),
            None => break,
        };

        match arg_toks.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => (),
            Some(other) => panic!("Expected a colon. Got: {:?}", other),
            None => panic!("Expected a colon. Got: end of token stream"),
        }

        let arg_type = match arg_toks.peek() {
            Some(TokenTree::Ident(_)) => {
                let toks = vec![
                    arg_toks.next().unwrap()
                ];
                let toks = TokenStream2::from(TokenStream::from_iter(toks.into_iter()));
                quote! {
                    ::quote::quote!(#toks)
                }
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
                let toks = vec![
                    arg_toks.next().unwrap(),
                    arg_toks.next().expect("Expected an ident after #"),
                ];
                let toks = TokenStream2::from(TokenStream::from_iter(toks.into_iter()));
                quote! {
                    ::quote::quote!(#toks)
                }
            }
            other => panic!("Expected an argument type. Got: {:?}", other),
        };

        args.push(Arg {
            name: arg_name,
            arg_type,
            default: quote!(None),
        });

        match arg_toks.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => (),
            None => break,
            Some(other) => panic!(
                "Expected a comma or a closing parenthesis. Got: {:?}",
                other
            ),
        }
    }
    args
}

// for parsing rust function, retuns a TokenStream2 OF a TokenStream2
pub fn parse_return2(toks: &mut Peekable<impl Iterator<Item=TokenTree>>) -> TokenStream2 {
    let mut muncher = vec![];
    match toks.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '-' => {
            toks.next(); // skip the -
            match toks.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                    loop {
                        match toks.peek() {
                            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => break,
                            Some(_) => {
                                muncher.push(toks.next().unwrap())
                            }
                            None => panic!("Expected return type. Got end of stream."),
                        }
                    }
                    let toks = TokenStream2::from(TokenStream::from_iter(muncher.into_iter()));
                    quote!(::quote::quote!(#toks))
                }
                next => panic!("Expected ->. Got: {:?}", next),
            }
        }
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            quote!(::proc_macro2::TokenStream::new())
        }
        next => panic!("Expected -> or {{. Got: {:?}", next),
    }
}
