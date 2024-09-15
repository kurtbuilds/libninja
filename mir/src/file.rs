use crate::{Class, Doc, Enum, Function, Import};

pub struct File<T, E> {
    pub imports: Vec<Import>,
    pub doc: Option<Doc>,
    /// Code that is before function and class declarations
    pub declaration: Option<T>,
    pub classes: Vec<Class<T>>,
    pub enums: Vec<Enum<T, E>>,
    pub functions: Vec<Function<T>>,
    /// Code that follows after the function and class declarations
    pub code: Option<T>,
    pub package: Option<String>,
}

impl<T, E> Default for File<T, E>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            doc: None,
            declaration: None,
            classes: vec![],
            enums: vec![],
            functions: vec![],
            code: None,
            imports: vec![],
            package: None,
        }
    }
}
