use crate::{Doc, Function, Ident, Visibility};

#[derive(Debug, Default)]
pub struct Enum<T, E> {
    pub name: Ident,
    pub doc: Option<Doc>,
    pub variants: Vec<Variant<E>>,
    pub vis: Visibility,
    pub methods: Vec<Function<T>>,
    pub extra: E,
}

#[derive(Debug)]
pub struct Variant<E> {
    pub ident: Ident,
    pub doc: Option<Doc>,
    // in rust, value is like enum { Error = 0 }
    pub value: Option<String>,
    pub extra: E,
}
