mod file;
mod class;
mod import;

use mir::{Function, Visibility, FnArg2, Doc, Field, Literal, ParamKey, Ident};
use proc_macro2::{TokenStream};
use quote::quote;
use regex::{Captures, Regex};
use convert_case::{Casing, Case};
pub use class::codegen_function;

/// Use this for codegen structs: Function, Class, etc.
pub trait ToRustCode {
    fn to_rust_code(self) -> TokenStream;
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
            generic,
        } = self;
        let annotations = annotations
            .into_iter()
            .map(|a| syn::parse_str::<syn::Expr>(&a).unwrap());
        let doc = doc.to_rust_code();
        let vis = public.then(|| quote!(pub)).unwrap_or_default();
        let async_ = async_.then(|| quote!(async)).unwrap_or_default();
        let args = args.into_iter().map(|a| a.to_rust_code());
        let ret = ret.is_empty().then(|| quote!(-> #ret)).unwrap_or_default();
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
        todo!()
    }
}

impl ToRustCode for Option<Doc> {
    fn to_rust_code(self) -> TokenStream {
        match self {
            None => TokenStream::new(),
            Some(Doc(doc)) => {
                let doc = doc.trim();
                quote!(#[doc = #doc])
            },
        }
    }
}
impl ToRustCode for Field<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let name = self.name.to_rust_ident();
        let ty = if self.optional {
            let ty = self.ty;
            quote! { Option<#ty> }
        } else {
            self.ty
        };
        let vis = self.vis.to_rust_code();
        let doc = self.doc.to_rust_code();
        let decorators = self.decorators;
        quote! {
            #doc
            #(
                #decorators
            )*
            #vis #name: #ty,
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


pub trait ToRustIdent {
    fn to_rust_struct(&self) -> Ident;
    fn to_rust_ident(&self) -> Ident;
}

impl ToRustIdent for String {
    fn to_rust_struct(&self) -> Ident {
        sanitize_struct(self)
    }

    fn to_rust_ident(&self) -> Ident {
        sanitize_ident(self)
    }
}

impl ToRustIdent for &str {
    fn to_rust_struct(&self) -> Ident {
        sanitize_struct(self)
    }

    fn to_rust_ident(&self) -> Ident {
        sanitize_ident(self)
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

fn sanitize(s: impl AsRef<str>) -> String {
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
    assert_valid_ident(&s, &original);
    Ident(s)
}

pub fn is_restricted(s: &str) -> bool {
    ["type", "use", "ref", "self", "match", "final"].contains(&s)
}

fn assert_valid_ident(s: &str, original: &str) {
    if s.chars().next().map(|c| c.is_numeric()).unwrap_or_default() {
        panic!("Numeric identifier: {}", original)
    }
    if s.contains('.') {
        panic!("Dot in identifier: {}", original)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename() {
        let s = "SdAddress.contractor1099";
        assert_eq!(String::from(s).to_rust_ident().0, "sd_address_contractor1099");
        assert_eq!(sanitize_filename(s), "sd_address_contractor1099");
    }
}