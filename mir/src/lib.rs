use core::fmt::Formatter;

pub use class::*;
pub use doc::{Doc, DocFormat};
pub use file::File;
pub use function::{ArgIdent, build_dict, build_struct, FnArg2, Function};
pub use ident::*;
pub use import::*;
pub use r#enum::*;
pub use ty::*;
pub use visibility::*;

mod class;
mod doc;
mod r#enum;
mod file;
mod function;
mod ident;
mod import;
mod r#macro;
mod ty;
mod visibility;

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

pub struct Literal<T>(pub T);

#[allow(unused)]
pub struct Grave(String);

#[allow(unused)]
pub struct FString(String);

pub fn literal(s: impl Into<String>) -> Literal<String> {
    Literal(s.into())
}

pub fn grave(s: &str) -> Literal<Grave> {
    Literal(Grave(s.to_string()))
}

pub fn f_string(s: &str) -> Literal<FString> {
    Literal(FString(s.to_string()))
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
