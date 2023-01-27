use proc_macro::{Delimiter, TokenStream, TokenTree};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

fn opening(delim: Delimiter) -> &'static str {
    match delim {
        Delimiter::Parenthesis => "(",
        Delimiter::Brace => "{{",
        Delimiter::Bracket => "[",
        Delimiter::None => panic!("Delimiter::None"),
    }
}

fn closing(delim: Delimiter) -> &'static str {
    match delim {
        Delimiter::Parenthesis => ")",
        Delimiter::Brace => "}}",
        Delimiter::Bracket => "]",
        Delimiter::None => panic!("Delimiter::None"),
    }
}


/// Use this to create a binding for the given ident.
/// E.g. if we encounter #foo while tokenizing, get the idx of foo, returning it as the literal string r#"{idx}"#
/// If foo is not already captured, then we push to captured, returning that new idx.
fn interpolation_binding(ident: &str, captured: &mut Vec<String>, escape: bool) -> String {
    let interpolation_idx = match captured.iter().position(|name| name == ident) {
        None => {
            let idx = captured.len();
            captured.push(ident.to_owned());
            idx
        }
        Some(idx) => idx,
    };
    format!("{{{}{}}}", interpolation_idx, if escape { ":?" } else { "" })
}


/// Call this after we encounter a # in tokenization.
pub fn pull_interpolation(toks: &mut impl Iterator<Item=TokenTree>, captured: &mut Vec<String>, escape: bool) -> String {
    let ident = match toks.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        other => panic!("Expected ident after #, got {:?}", other),
    };
    interpolation_binding(&ident, captured, escape)
}


fn body_recurse(body: TokenStream, captured: &mut Vec<String>, lines: &mut Vec<String>, indent: usize) {
    let mut toks = body.into_iter().peekable();
    loop {
        match toks.next() {
            // match # punct
            Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
                lines
                    .last_mut()
                    .unwrap()
                    .push_str(pull_interpolation(&mut toks, captured, false).as_str());
                match toks.peek() {
                    Some(TokenTree::Punct(punct))
                    if ['#', '=', ':'].contains(&punct.as_char()) => {
                        lines.last_mut().unwrap().push(' ');
                    }
                    _ => {}
                }
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == ';' => {
                lines.push(" ".repeat(indent));
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                lines.last_mut().unwrap().push('.');
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
                lines.last_mut().unwrap().push('!');
            }
            Some(TokenTree::Group(group)) => {
                let n_lines = lines.len();
                lines.last_mut().unwrap().push_str(opening(group.delimiter()));
                let group_indent = indent + 4;
                if group.stream().to_string().contains(';') {
                    lines.push(" ".repeat(group_indent));
                }
                body_recurse(group.stream(), captured, lines, group_indent);
                let multiline = lines.len() > n_lines;
                if multiline {
                    lines.last_mut().unwrap().truncate(indent);
                }
                lines.last_mut().unwrap().push_str(closing(group.delimiter()));
                if multiline {
                    lines.push(" ".repeat(indent));
                }
            }
            Some(TokenTree::Ident(ident)) => {
                lines.last_mut().unwrap().push_str(&ident.to_string());
                match toks.peek() {
                    Some(TokenTree::Punct(punct)) if ['.', ';', ','].contains(&punct.as_char()) => {}
                    Some(TokenTree::Group(g)) if g.delimiter() != Delimiter::Brace => {}
                    Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace && !g.to_string().contains(';') => {}
                    None => {}
                    _ => {
                        lines.last_mut().unwrap().push(' ');
                    }
                }
            }
            Some(TokenTree::Punct(punct)) => {
                lines.last_mut().unwrap().push(punct.as_char());
                match toks.peek() {
                    Some(TokenTree::Punct(punct))
                    if ['>', '<', '=', '*'].contains(&punct.as_char()) => {}
                    Some(TokenTree::Group(_)) => {}
                    None => {}
                    _ => {
                        lines.last_mut().unwrap().push(' ');
                    }
                }
            }
            Some(TokenTree::Literal(literal)) => {
                lines.last_mut().unwrap().push_str(&literal.to_string());
            }
            None => {
                break;
            }
        }
    }
}

pub fn body_callable(body: TokenStream) -> TokenStream2 {
    let mut captured = vec![];
    let mut lines = vec!["".to_string()];

    body_recurse(body, &mut captured, &mut lines, 0);

    let lines = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let captured = captured
        .into_iter()
        .map(|name| Ident::new(&name, proc_macro2::Span::call_site()));

    // The line filtering is a hack for a smarter format action than format!
    quote! {
        format!(#lines, #( #captured ),*)
            .lines().filter(|line| !line.trim().is_empty()).collect::<Vec<_>>().join("\n")
    }
}
