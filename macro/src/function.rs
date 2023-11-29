use proc_macro::{Delimiter, Ident, TokenStream, TokenTree};
use proc_macro2::{Ident as Ident2, TokenStream as TokenStream2};
use std::iter::Peekable;
use crate::body::{pull_interpolation};
use quote::quote;

pub struct Tags {
    pub asyn: bool,
    pub public: bool,
    pub fn_name: TokenStream2,
}

/// Capture $(async)? $(pub)? fn_name
pub fn parse_intro(toks: &mut impl Iterator<Item=TokenTree>) -> Tags {
    let mut asyn = false;
    let mut public = false;
    let mut captured = vec![];
    let fn_name = loop {
        let next = toks
            .next()
            .expect("Unexpectedly reached end of token stream in function! macro");
        match next {
            TokenTree::Ident(ident) if ident.to_string() == "async" => {
                asyn = true;
            }
            TokenTree::Ident(ident) if ident.to_string() == "pub" => {
                public = true;
            }
            TokenTree::Ident(ident) => {
                break ident.to_string();
            }
            TokenTree::Punct(punct) if punct.as_char() == '#' => {
                break pull_interpolation(toks, &mut captured, false);
            }
            _ => panic!(
                "Expected one of: async, pub, or the function's name. Got: {:?}",
                next
            ),
        }
    };
    let fn_name = if captured.is_empty() {
        quote!( ::hir::Ident(#fn_name.to_string()) )
    } else {
        let captured = captured
            .into_iter()
            .map(|name| Ident2::new(&name, proc_macro2::Span::call_site()));

        quote!( ::hir::Ident(format!("{}", #( #captured ),*)) )
    };
    Tags {
        asyn,
        public,
        fn_name,
    }
}

pub struct Arg {
    pub name: String,
    pub arg_type: TokenStream2,
    pub default: TokenStream2,
}

pub fn parse_args(arg_toks: TokenStream) -> Vec<Arg> {
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

        let arg_type = match arg_toks.next() {
            // Matches a ident arg_type, e.g.
            // str
            // Dict[str, str]
            // requests.PreparedRequest
            Some(TokenTree::Ident(ident)) => {
                parse_type(ident, &mut arg_toks)
            }
            // Matches a arg_type binding, e.g.
            // let arg_type = "int";
            // function!(add(a: #arg_type, b: #arg_type) {})
            Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
                let mut captured = vec![];
                let placeholder = pull_interpolation(&mut arg_toks, &mut captured, false);

                let captured = captured
                    .into_iter()
                    .map(|name| Ident2::new(&name, proc_macro2::Span::call_site()));

                quote! {
                    format!(#placeholder, #( #captured ),*)
                }
            }
            other => panic!("Expected an argument type. Got: {:?}", other),
        };

        let default = match arg_toks.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                arg_toks.next(); // we peaked, so we need to consume the =
                match arg_toks.next() {
                    Some(TokenTree::Literal(lit)) => {
                        let lit = lit.to_string();
                        quote!(Some(#lit.to_string()))
                    }
                    Some(other) => panic!("Expected a default value. Got: {:?}", other),
                    None => panic!("Expected a default value. Got: end of token stream"),
                }
            }
            Some(TokenTree::Punct(punct)) if [',', ')'].contains(&punct.as_char()) => {
                quote!(None)
            }
            Some(other) => panic!("Expected one of: , or ) or =. Got: {:?}", other),
            None => quote!(None),
        };

        args.push(Arg {
            name: arg_name,
            arg_type,
            default,
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


/// Matches a ident arg_type, e.g.
/// str
/// Dict[str, str]
/// requests.PreparedRequest
pub fn parse_type(ident: Ident, toks: &mut Peekable<impl Iterator<Item=TokenTree>>) -> TokenStream2 {
    let mut ident = ident.to_string();
    // Matches path-ed types, e.g. requests.PreparedRequest
    while matches!(toks.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '.') {
        ident += &toks.next().unwrap().to_string();
        if !matches!(toks.peek(), Some(TokenTree::Ident(_))) {
            panic!("Expected an identifier after a dot. Got: {:?}", toks.peek());
        }
        ident += &toks.next().unwrap().to_string();
    }
    // Matches python generics, e.g. Dict[str, Any]
    if matches!(toks.peek(), Some(TokenTree::Group(group))
                    if matches!(group.delimiter(), Delimiter::Bracket)) {
        ident += &toks.next().unwrap().to_string();
    }
    quote! {
        #ident.to_string()
    }
}

pub fn parse_return(toks: &mut Peekable<impl Iterator<Item=TokenTree>>) -> TokenStream2 {
    loop {
        match toks.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '-' => {
                toks.next();
                match toks.next() {
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                        match toks.next() {
                            Some(TokenTree::Ident(ident)) => {
                                break parse_type(ident, toks);
                            }
                            Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
                                let mut captured = vec![];
                                let placeholder = pull_interpolation(toks, &mut captured, false);

                                let captured = captured
                                    .into_iter()
                                    .map(|name| Ident2::new(&name, proc_macro2::Span::call_site()));
                                break quote!(format!(#placeholder, #( #captured ),*));
                            }
                            next => panic!("Expected the return type. Got: {:?}", next),
                        }
                    }
                    next => panic!("Expected ->. Got: {:?}", next),
                }
            }
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                break quote!("".to_string());
            }
            next => panic!("Expected -> or {{. Got: {:?}", next),
        }
    }
}



