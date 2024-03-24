use crate::{Class, Doc, Enum, Function, Import};

pub struct File<T> {
    pub imports: Vec<Import>,
    pub doc: Option<Doc>,
    /// Code that is before function and class declarations
    pub declaration: Option<T>,
    pub classes: Vec<Class<T>>,
    pub enums: Vec<Enum<T>>,
    pub functions: Vec<Function<T>>,
    /// Code that follows after the function and class declarations
    pub code: Option<T>,
    pub package: Option<String>,
}

impl<T> Default for File<T> where T: Default {
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
