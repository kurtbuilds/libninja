use crate::{Doc, Function, Ident, Visibility};

pub struct Enum<T> {
    pub name: Ident,
    pub doc: Option<Doc>,
    pub variants: Vec<Variant<T>>,
    pub vis: Visibility,
    pub methods: Vec<Function<T>>,
    pub attributes: Vec<T>,
}

pub struct Variant<T> {
    pub ident: Ident,
    pub doc: Option<Doc>,
    // in rust, value is like enum { Error = 0 }
    pub value: Option<T>,
    pub attributes: Vec<T>,
}
