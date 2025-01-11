pub use class::make_class;
use convert_case::{Case, Casing};
use mir::parameter::ParamKey;
use mir::Literal;
use mir::{Doc, Ident, Visibility};
use proc_macro2::TokenStream;
use quote::quote;
use regex::{Captures, Regex};

pub mod class;
mod r#enum;
mod example;
mod file;
mod function;
mod ident;
mod import;
mod record;
mod ty;
mod util;

pub use example::to_rust_example_value;

pub use ident::ToRustIdent;
pub use r#enum::make_enum;
pub use record::make_item;
pub use ty::{CanDerive, ToRustType};
pub use util::*;

pub fn serde_rename2(value: &str, ident: &Ident) -> Option<TokenStream> {
    if ident.0 != value {
        Some(quote!(#[serde(rename = #value)]))
    } else {
        None
    }
}

pub fn derives_to_tokens(derives: &[String]) -> TokenStream {
    derives
        .iter()
        .map(|d| {
            if let Ok(d) = d.trim().parse::<TokenStream>() {
                quote! { , #d }
            } else {
                return TokenStream::new();
            }
        })
        .collect()
}

/// Use this for codegen structs: Function, Class, etc.
pub trait ToRustCode {
    fn to_rust_code(self) -> TokenStream;
}

pub trait FluentBool {
    fn to_value<T: Default>(self, f: impl FnOnce() -> T) -> T;
}

impl FluentBool for bool {
    fn to_value<T: Default>(self, f: impl FnOnce() -> T) -> T {
        if self {
            f()
        } else {
            Default::default()
        }
    }
}

impl ToRustCode for Visibility {
    fn to_rust_code(self) -> TokenStream {
        match self {
            Visibility::Public => quote!(pub),
            Visibility::Private => TokenStream::new(),
            Visibility::Crate => quote!(pub(crate)),
        }
    }
}

impl ToRustCode for Option<Doc> {
    fn to_rust_code(self) -> TokenStream {
        match self {
            None => TokenStream::new(),
            Some(Doc(doc)) => {
                let doc = doc.trim();
                quote!(#[doc = #doc])
            }
        }
    }
}

impl ToRustCode for Literal<String> {
    fn to_rust_code(self) -> TokenStream {
        let s = self.0;
        quote!(#s)
    }
}

impl ToRustCode for ParamKey {
    fn to_rust_code(self) -> TokenStream {
        match self {
            ParamKey::Key(s) => quote!(#s),
            ParamKey::RepeatedKey(mut s) => {
                s += "[]";
                quote!(#s)
            }
        }
    }
}

impl ToRustCode for TokenStream {
    fn to_rust_code(self) -> TokenStream {
        self
    }
}

pub fn sanitize_filename(s: &str) -> String {
    sanitize(s)
}

pub fn sanitize_ident(s: &str) -> Ident {
    Ident(sanitize(s))
}

fn rewrite_names(s: &str) -> String {
    // custom logic for Github openapi spec lol
    if s == "+1" {
        return "PlusOne".to_string();
    } else if s == "-1" {
        return "MinusOne".to_string();
    }
    s.replace('/', "_")
        .replace(['@', '\'', '+'], "")
        .replace(':', " ")
        .replace('.', "_")
}

pub fn sanitize(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    let original = s;
    let s = rewrite_names(s);
    let regex = Regex::new("[a-z]_[0-9]").unwrap();
    let mut s = s.to_case(Case::Snake);
    s = regex
        .replace_all(&s, |c: &Captures| {
            let mut c = c.get(0).unwrap().as_str().to_string();
            c.remove(1);
            c
        })
        .into();
    if is_restricted(&s) {
        s += "_"
    }
    if s.chars().next().unwrap().is_numeric() {
        s = format!("_{}", s)
    }
    assert_valid_ident(&s, original);
    s
}

fn sanitize_struct(s: impl AsRef<str>) -> Ident {
    let s = s.as_ref();
    let original = s;
    let s = rewrite_names(s);
    let mut s = s.to_case(Case::Pascal);
    if is_restricted(&s) {
        s += "Struct"
    }
    if s == "Self" {
        s += "_";
    }
    if s.chars().next().unwrap().is_numeric() {
        s = format!("_{}", s)
    }
    assert_valid_ident(&s, &original);
    Ident(s)
}

pub fn is_restricted(s: &str) -> bool {
    ["type", "use", "ref", "self", "match", "final"].contains(&s)
}

fn assert_valid_ident(s: &str, original: &str) {
    if s.contains('(') {
        panic!("Parentheses in identifier: {}", original)
    }
    if s.chars().next().map(|c| c.is_numeric()).unwrap_or_default() {
        panic!("Numeric identifier: {}", original)
    }
    if s.contains('.') {
        panic!("Dot in identifier: {}", original)
    }
    if s.is_empty() {
        panic!("Empty identifier: {}", original)
    }
}

pub fn format_code(code: TokenStream) -> String {
    let syntax_tree = match syn::parse2(code.clone()) {
        Ok(syntax_tree) => syntax_tree,
        Err(e) => {
            eprintln!("{}", code.to_string());
            panic!("Failed to parse generated code: {}", e);
        }
    };
    prettyplease::unparse(&syntax_tree)
}

pub fn format_string(code: &str) -> String {
    let syntax_tree = match syn::parse_str(code) {
        Ok(syntax_tree) => syntax_tree,
        Err(e) => {
            eprintln!("{}", code);
            panic!("Failed to parse generated code: {}", e);
        }
    };
    prettyplease::unparse(&syntax_tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ident::ToRustIdent;

    #[test]
    fn test_filename() {
        let s = "SdAddress.contractor1099";
        assert_eq!(String::from(s).to_rust_ident().0, "sd_address_contractor1099");
        assert_eq!(sanitize_filename(s), "sd_address_contractor1099");
    }
}
