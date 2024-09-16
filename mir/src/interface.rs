use crate::{Doc, Field, Function};

pub struct Interface<T> {
    pub name: String,
    pub doc: Option<Doc>,
    pub fields: Vec<Field<T>>,
    pub public: bool,
    pub instance_methods: Vec<Function<T>>,
}
