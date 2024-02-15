use core::default::Default;
use core::fmt::Formatter;

use quote::TokenStreamExt;

pub use class::*;
pub use doc::{Doc, DocFormat};
pub use function::{ArgIdent, build_dict, build_struct, FnArg2, Function};
pub use ident::*;
pub use import::*;
pub use ty::*;
pub use visibility::*;

mod doc;
mod function;
mod r#macro;
mod ty;
mod class;
mod import;
mod visibility;
mod ident;

pub struct Interface<T> {
    pub name: String,
    pub doc: Option<Doc>,
    pub fields: Vec<Field<T>>,
    pub public: bool,
    pub instance_methods: Vec<Function<T>>,
}

pub struct NewType<T> {
    pub name: String,
    pub doc: Option<String>,
    pub ty: T,
    pub public: bool,
}

pub struct File<T> {
    pub doc: Option<Doc>,
    /// Code that is before function and class declarations
    pub declaration: Option<T>,
    pub classes: Vec<Class<T>>,
    pub functions: Vec<Function<T>>,
    /// Code that follows after the function and class declarations
    pub code: Option<T>,
    pub imports: Vec<Import>,
    pub package: Option<String>,
}

pub struct Literal<T>(pub T);

pub struct Grave(String);

pub struct FString(String);

impl Import {
    pub fn package(path: &str) -> Self {
        Self {
            path: path.to_string(),
            imports: vec![],
            alias: None,
            vis: Visibility::Private,
            feature: None,
        }
    }

    pub fn new(path: &str, imports: impl IntoIterator<Item=impl Into<ImportItem>>) -> Self {
        Self {
            path: path.to_string(),
            imports: imports
                .into_iter()
                .map(|s| s.into())
                .collect(),
            alias: None,
            vis: Visibility::Private,
            feature: None,
        }
    }

    pub fn alias(path: &str, alias: &str) -> Self {
        Self {
            path: path.to_string(),
            imports: Vec::new(),
            alias: Some(alias.to_string()),
            vis: Visibility::Private,
            feature: None,
        }
    }

    pub fn public(mut self) -> Self {
        self.vis = Visibility::Public;
        self
    }
}

impl<T> Default for File<T>
    where
        T: Default,
{
    fn default() -> Self {
        Self {
            doc: None,
            declaration: None,
            classes: vec![],
            functions: vec![],
            code: None,
            imports: vec![],
            package: None,
        }
    }
}

pub fn literal(s: impl Into<String>) -> Literal<String> {
    Literal(s.into())
}

pub fn grave(s: &str) -> Literal<Grave> {
    Literal(Grave(s.to_string()))
}

pub fn f_string(s: &str) -> Literal<FString> {
    Literal(FString(s.to_string()))
}

impl quote::ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(proc_macro2::Ident::new(&self.0, proc_macro2::Span::call_site()))
    }
}

impl From<Ident> for proc_macro2::TokenStream {
    fn from(val: Ident) -> Self {
        let mut tok = proc_macro2::TokenStream::new();
        tok.append(proc_macro2::Ident::new(&val.0, proc_macro2::Span::call_site()));
        tok
    }
}

/// Specifically represents a parameter in Location::Query. We need special treatment for repeated keys.
pub enum ParamKey {
    Key(String),
    RepeatedKey(String),
}

impl std::fmt::Display for ParamKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamKey::Key(s) => write!(f, "\"{}\"", s),
            ParamKey::RepeatedKey(s) => write!(f, "\"{}[]\"", s),
        }
    }
}

