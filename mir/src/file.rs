use crate::interface::Interface;
use crate::{Class, Doc, Enum, Function, Import};

pub enum Item<T> {
    Class(Class<T>),
    Enum(Enum<T>),
    Interface(Interface<T>),
    Fn(Function<T>),
    Block(T),
}

pub struct File<T> {
    pub attributes: Vec<T>,
    pub doc: Option<Doc>,
    pub imports: Vec<Import>,
    /// Code that is before function and class declarations
    pub items: Vec<Item<T>>,
}

impl<T> Default for File<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            attributes: vec![],
            doc: None,
            imports: vec![],
            items: vec![],
        }
    }
}
