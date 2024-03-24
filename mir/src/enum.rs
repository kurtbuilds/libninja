use crate::{Doc, Function, Ident, Visibility};

#[derive(Debug, Default)]
pub struct Enum<T> {
    pub name: Ident,
    pub doc: Option<Doc>,
    pub variants: Vec<Variant>,
    pub vis: Visibility,
    /// Attributes in Rust
    pub decorators: Vec<T>,
    pub methods: Vec<Function<T>>,
}

#[derive(Debug)]
pub struct Variant {
    pub name: Ident,
    pub doc: Option<Doc>,
}
