use std::fmt::{Debug, Formatter};

use crate::{Doc, Ident};

/// Localized
pub enum ArgIdent {
    Ident(String),
    // parallel to Ident
    Unpack(Vec<String>),
}

impl ArgIdent {
    pub fn force_string(&self) -> String {
        match self {
            ArgIdent::Ident(s) => s.clone(),
            ArgIdent::Unpack(_) => panic!("cannot force unpacked arg name to string"),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ArgIdent::Ident(s) => s.is_empty(),
            ArgIdent::Unpack(v) => v.is_empty(),
        }
    }

    pub fn unwrap_ident(self) -> Ident {
        match self {
            ArgIdent::Ident(s) => Ident(s),
            ArgIdent::Unpack(_) => panic!("cannot unwrap unpacked arg name"),
        }
    }
}

impl From<String> for ArgIdent {
    fn from(s: String) -> Self {
        ArgIdent::Ident(s)
    }
}

impl From<&str> for ArgIdent {
    fn from(s: &str) -> Self {
        ArgIdent::Ident(s.to_string())
    }
}


impl From<Ident> for ArgIdent {
    fn from(ident: Ident) -> Self {
        ArgIdent::Ident(ident.0)
    }
}


// // IR form. Therefore it's localized
// pub struct FnArg<T> {
//     pub name: ArgIdent,
//     pub ty: T,
//     // T is a String (for Rust, TokenStream)
//     pub default: Option<String>,
//     pub treatment: Option<FnArgTreatment>,
// }

pub enum FnArg2<T> {
    /// fn foo(a: i32)
    Basic {
        name: Ident,
        ty: T,
        default: Option<T>,
    },
    /// For typescript
    /// function foo({foo, bar}: FooProps)
    Unpack {
        names: Vec<Ident>,
        ty: T,
    },
    /// For rust
    /// fn foo(&self)
    SelfArg { mutable: bool, reference: bool },
    /// For python
    /// def foo(**kwargs)
    Kwargs {
        name: Ident,
        ty: T,
    },
    /// For python
    /// def foo(*args)
    Variadic {
        name: Ident,
        ty: T,
    },
}

impl<T> FnArg2<T> {
    pub fn name(&self) -> Option<&Ident> {
        let name = match self {
            FnArg2::Basic { name, .. } => name,
            FnArg2::Unpack { .. } => return None,
            FnArg2::SelfArg { .. } => return None,
            FnArg2::Kwargs { name, .. } => name,
            FnArg2::Variadic { name, .. } => name,
        };
        Some(name)
    }

    pub fn ty(&self) -> Option<&T> {
        let ty = match self {
            FnArg2::Basic { ty, .. } => ty,
            FnArg2::Unpack { ty, .. } => ty,
            FnArg2::SelfArg { .. } => return None,
            FnArg2::Kwargs { ty, .. } => ty,
            FnArg2::Variadic { ty, .. } => ty,
        };
        Some(ty)
    }
}

// impl<T> FnArg<T> {
//     pub fn new(name: String, ty: T) -> Self {
//         FnArg {
//             name: ArgIdent::Ident(name),
//             ty,
//             default: None,
//             treatment: None,
//         }
//     }
//
//     pub fn from_ident(name: Ident, ty: T) -> Self {
//         FnArg {
//             name: ArgIdent::Ident(name.0),
//             ty,
//             default: None,
//             treatment: None,
//         }
//     }
// }

// impl FnArg<String> {
//     /// Used by python for dividing required vs optional args
//     pub fn empty_variadic() -> Self {
//         FnArg {
//             name: ArgIdent::Ident("".to_string()),
//             ty: "".to_string(),
//             default: None,
//             treatment: Some(FnArgTreatment::Variadic),
//         }
//     }
// }

// pub enum FnArgTreatment {
//     /// python: **kwargs
//     Kwargs,
//     /// python: *args
//     /// golang: ...opt
//     Variadic,
// }

pub struct Function<T> {
    pub name: Ident,
    pub args: Vec<FnArg2<T>>,
    /// This *is* localized to the programming language.
    pub ret: T,
    pub body: T,
    pub doc: Option<Doc>,
    pub async_: bool,
    pub public: bool,
    /// #[...] in Rust
    /// @... in Python
    pub annotations: Vec<String>,
    pub generic: Vec<String>,
}

impl<T> Debug for Function<T>
    where
        T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function {{ name: {name:?}, args: debug print not impl, ret: {ret:?}, body: {body:?}, doc: {doc:?}, async_: {async_}, public: {public}, annotations: debug print not impl }}",
               name = self.name,
               ret = self.ret,
               body = self.body,
               doc = self.doc,
               async_ = self.async_,
               public = self.public
        )
    }
}

impl<T> Default for Function<T>
    where
        T: Default,
{
    fn default() -> Self {
        Self {
            name: Ident::new(""),
            args: vec![],
            ret: T::default(),
            body: T::default(),
            doc: None,
            async_: false,
            public: false,
            annotations: vec![],
            generic: vec![],
        }
    }
}

impl std::fmt::Display for ArgIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgIdent::Ident(name) => write!(f, "{}", name),
            ArgIdent::Unpack(vec) => write!(f, "{}", build_struct(vec.iter())),
        }
    }
}

/// Build something wrapped in braces, { A, B, C }
pub fn build_struct(mut s: impl Iterator<Item=impl AsRef<str>>) -> String {
    let mut r = String::from("{");
    let mut t = s.next();
    while let Some(u) = &t {
        r.push_str(u.as_ref());
        t = s.next();
        if t.is_some() {
            r.push_str(", ");
        }
    }
    r.push('}');
    r
}

/// Build keys wrapped in braces, e.g. {"a": 1, "b": 2}
pub fn build_dict<'a>(s: impl Iterator<Item=(&'a str, &'a str)>) -> String {
    build_struct(s.map(|(k, v)| format!("\"{}\": {}", k, v)))
}