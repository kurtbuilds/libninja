use std::fmt::{Display, Formatter};

use quote::TokenStreamExt;

/// Localized string
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Ident(pub String);

impl Ident {
    pub fn new(s: &'static str) -> Self {
        Ident(s.into())
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl quote::ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(proc_macro2::Ident::new(
            &self.0,
            proc_macro2::Span::call_site(),
        ))
    }
}

impl From<Ident> for proc_macro2::TokenStream {
    fn from(val: Ident) -> Self {
        let mut tok = proc_macro2::TokenStream::new();
        tok.append(proc_macro2::Ident::new(
            &val.0,
            proc_macro2::Span::call_site(),
        ));
        tok
    }
}
