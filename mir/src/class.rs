use crate::{Doc, Function, Ident, Visibility};
use std::fmt::{Debug, Formatter};

pub struct Class<T> {
    pub vis: Visibility,
    pub name: Ident,
    pub doc: Option<Doc>,
    pub fields: Vec<Field<T>>,
    pub methods: Vec<Function<T>>,
    pub attributes: Vec<T>,
}

#[derive(Debug, Default)]
pub struct Field<T> {
    pub name: Ident,
    pub ty: T,
    pub default: Option<T>,
    pub vis: Visibility,
    pub doc: Option<Doc>,
    pub optional: bool,
    pub attributes: Vec<T>,
}

impl Debug for Class<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name)
            .field("doc", &self.doc)
            .field("instance_fields", &self.fields)
            .field("vis", &self.vis)
            .finish()
    }
}

impl<T> Default for Class<T> {
    fn default() -> Self {
        Self {
            name: Ident::empty(),
            doc: None,
            fields: vec![],
            vis: Visibility::Private,
            attributes: vec![],
            methods: vec![],
        }
    }
}
