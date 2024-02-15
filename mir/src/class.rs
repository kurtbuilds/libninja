use std::fmt::{Debug, Formatter};
use crate::{Doc, Function, Ident, Visibility};

pub struct Class<T> {
    pub name: Ident,
    pub doc: Option<Doc>,
    /// `code` is for Python, where we need code like this:
    /// class Account(BaseModel):
    ///     class Config:
    ///         this_is_a_config_for_pydantic = True
    pub code: Option<String>,
    pub instance_fields: Vec<Field<T>>,
    pub static_fields: Vec<Field<T>>,
    pub constructors: Vec<Function<T>>,
    /// Use `class_methods` in Rust.
    pub class_methods: Vec<Function<T>>,
    pub static_methods: Vec<Function<T>>,
    pub public: bool,

    pub lifetimes: Vec<String>,
    pub decorators: Vec<T>,
    pub superclasses: Vec<T>,
}

#[derive(Debug, Default)]
pub struct Field<T> {
    pub name: Ident,
    pub ty: T,
    pub default: Option<T>,
    pub vis: Visibility,
    pub doc: Option<Doc>,
    pub optional: bool,
    pub decorators: Vec<T>,
}


impl Debug for Class<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let public = self.public;
        write!(f, "Class {{ name: {name:?}, \
        doc: {doc:?}, \
        instance_fields: todo!, \
        static_fields: todo!, \
        constructors: todo!, \
        class_methods: todo!, \
        static_methods: todo!, \
        public: {public}, \
        lifetimes: todo!, \
        superclasses: todo! }}",
               name = self.name,
               doc = self.doc,
        )
    }
}

impl<T> Default for Class<T> {
    fn default() -> Self {
        Self {
            name: Ident("".to_string()),
            code: None,
            doc: None,
            instance_fields: vec![],
            static_fields: vec![],
            constructors: vec![],
            class_methods: vec![],
            static_methods: vec![],
            public: false,
            lifetimes: vec![],
            decorators: vec![],
            superclasses: vec![],
        }
    }
}
