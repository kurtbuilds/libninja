/// Models that represent code
/// Things like, Parameters, Functions, Fields, Class, etc.
///

use core::default::Default;
use core::fmt::{Debug, Formatter};
use core::option::Option;
use core::option::Option::None;
use quote::TokenStreamExt;

pub use function::{ArgIdent, FnArg, FnArgTreatment, Function, build_struct, build_dict};
use hir::Doc;

mod function;
mod r#macro;

/// Localized string
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Ident(pub String);

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Crate,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}


#[derive(Debug, Default)]
pub struct Field<T> {
    pub name: String,
    pub ty: T,
    pub default: Option<T>,
    pub visibility: Visibility,
    pub doc: Option<Doc>,
    pub optional: bool,
    pub decorators: Vec<T>,
}

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
    pub instance_methods: Vec<Function<T>>,
    pub constructors: Vec<Function<T>>,
    pub class_methods: Vec<Function<T>>,
    pub static_methods: Vec<Function<T>>,
    pub public: bool,

    pub mut_self_instance_methods: Vec<Function<T>>,
    pub lifetimes: Vec<String>,
    pub decorators: Vec<T>,
    pub superclasses: Vec<T>,
}

pub struct File<T> {
    pub doc: Option<Doc>,
    /// Code that is before function and class declarations
    pub declaration: Option<T>,
    pub classes: Vec<Class<T>>,
    pub functions: Vec<Function<T>>,
    /// Code that follows after the function and class declarations
    pub code: Option<T>,
    pub imports: Vec<Import>,
    pub package: Option<String>,
}

pub struct Import {
    /// Path that we're importing from
    /// e.g. plaid.model in `from plaid.model import ...`
    pub path: String,
    /// Specific items that are imported
    /// e.g. `Account` in `from plaid.model import Account`
    pub imports: Vec<ImportItem>,
    /// If a wildcard import and if we want to alias, then alias
    pub alias: Option<String>,
    pub vis: Visibility,
}

pub struct ImportItem {
    /// This might not conform to standard ident rules for the language, so its a string, not an ident.
    pub name: String,
    pub alias: Option<String>,
}

pub struct Literal<T>(pub T);

pub struct Grave(String);

pub struct FString(String);

impl Visibility {
    pub fn public(&self) -> bool {
        match self {
            Visibility::Public => true,
            Visibility::Crate => false,
            Visibility::Private => false,
        }
    }
}

impl<T> From<T> for Ident where T: AsRef<str> {
    fn from(s: T) -> Self {
        Self::new(s.as_ref())
    }
}

impl Ident {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl<T> Default for Class<T> {
    fn default() -> Self {
        Self {
            name: Ident::new(""),
            code: None,
            doc: None,
            instance_fields: vec![],
            static_fields: vec![],
            instance_methods: vec![],
            constructors: vec![],
            class_methods: vec![],
            static_methods: vec![],
            public: false,
            mut_self_instance_methods: vec![],
            lifetimes: vec![],
            decorators: vec![],
            superclasses: vec![],
        }
    }
}

impl Debug for Class<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let public = self.public;
        write!(f, "Class {{ name: {name:?}, \
        doc: {doc:?}, \
        instance_fields: todo!, \
        static_fields: todo!, \
        instance_methods: todo!, \
        constructors: todo!, \
        class_methods: todo!, \
        static_methods: todo!, \
        public: {public}, \
        mut_self_instance_methods: todo!, \
        lifetimes: todo!, \
        superclasses: todo! }}",
               name = self.name,
               doc = self.doc,
        )
    }
}

impl ImportItem {
    pub fn alias(name: &str, alias: &str) -> Self {
        Self { name: name.to_string(), alias: Some(alias.to_string()) }
    }
}

impl From<&String> for ImportItem {
    fn from(s: &String) -> Self {
        Self { name: s.clone(), alias: None }
    }
}

impl From<String> for ImportItem {
    fn from(s: String) -> Self {
        Self { name: s, alias: None }
    }
}

impl From<&str> for ImportItem {
    fn from(s: &str) -> Self {
        Self { name: s.to_string(), alias: None }
    }
}

impl From<Ident> for ImportItem {
    fn from(s: Ident) -> Self {
        Self { name: s.0, alias: None }
    }
}


impl Import {
    pub fn package(path: &str) -> Self {
        Self {
            path: path.to_string(),
            imports: vec![],
            alias: None,
            vis: Visibility::Private,
        }
    }

    pub fn new(path: &str, imports: impl IntoIterator<Item=impl Into<ImportItem>>) -> Self {
        Self {
            path: path.to_string(),
            imports: imports
                .into_iter()
                .map(|s| s.into())
                .collect(),
            alias: None,
            vis: Visibility::Private,
        }
    }

    pub fn alias(path: &str, alias: &str) -> Self {
        Self {
            path: path.to_string(),
            imports: Vec::new(),
            alias: Some(alias.to_string()),
            vis: Visibility::Private,
        }
    }

    pub fn public(mut self) -> Self {
        self.vis = Visibility::Public;
        self
    }
}

impl<T> Default for File<T>
    where
        T: Default,
{
    fn default() -> Self {
        Self {
            doc: None,
            declaration: None,
            classes: vec![],
            functions: vec![],
            code: None,
            imports: vec![],
            package: None,
        }
    }
}

pub fn literal(s: impl Into<String>) -> Literal<String> {
    Literal(s.into())
}

pub fn grave(s: &str) -> Literal<Grave> {
    Literal(Grave(s.to_string()))
}

pub fn f_string(s: &str) -> Literal<FString> {
    Literal(FString(s.to_string()))
}

// impl From<String> for Literal<String> {
//     fn from(s: String) -> Self {
//         Self(s, false)
//     }
// }
//
// impl From<Ident> for Literal<String> {
//     fn from(s: Ident) -> Self {
//         Self(s.0, false)
//     }
// }

impl quote::ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(proc_macro2::Ident::new(&self.0, proc_macro2::Span::call_site()))
    }
}

impl From<Ident> for proc_macro2::TokenStream {
    fn from(val: Ident) -> Self {
        let mut tok = proc_macro2::TokenStream::new();
        tok.append(proc_macro2::Ident::new(&val.0, proc_macro2::Span::call_site()));
        tok
    }
}