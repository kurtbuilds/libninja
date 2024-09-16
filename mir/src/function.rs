use crate::{Doc, Ident, Visibility};
use std::fmt::{Debug, Formatter};

pub enum Arg<T> {
    /// fn foo(a: i32)
    Basic {
        name: Ident,
        ty: T,
        default: Option<T>,
    },
    /// For typescript
    /// function foo({foo, bar}: FooProps)
    Unpack { names: Vec<Ident>, ty: T },
    /// rust: fn foo(&self)
    SelfArg { mutable: bool, reference: bool },
    /// python: def foo(**kwargs)
    Kwargs { name: Ident, ty: T },
    /// python: def foo(*args)
    Variadic { name: Ident, ty: T },
}

impl<T> Arg<T> {
    pub fn ident(&self) -> Option<&Ident> {
        let name = match self {
            Arg::Basic { name, .. } => name,
            Arg::Unpack { .. } => return None,
            Arg::SelfArg { .. } => return None,
            Arg::Kwargs { name, .. } => name,
            Arg::Variadic { name, .. } => name,
        };
        Some(name)
    }

    pub fn ty(&self) -> Option<&T> {
        let ty = match self {
            Arg::Basic { ty, .. } => ty,
            Arg::Unpack { ty, .. } => ty,
            Arg::SelfArg { .. } => return None,
            Arg::Kwargs { ty, .. } => ty,
            Arg::Variadic { ty, .. } => ty,
        };
        Some(ty)
    }
}

pub struct Function<T> {
    pub name: Ident,
    pub args: Vec<Arg<T>>,
    pub ret: T,
    pub body: T,
    pub doc: Option<Doc>,
    pub is_async: bool,
    pub vis: Visibility,
    pub attributes: Vec<T>,
}

impl<T> Function<T> {
    pub fn body(mut self, body: T) -> Self {
        self.body = body;
        self
    }
}

impl<T> Debug for Function<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("ret", &self.ret)
            .field("args", &"..")
            .field("body", &"..")
            .finish()
    }
}

impl<T> Default for Function<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            name: Ident("".to_string()),
            args: vec![],
            ret: T::default(),
            body: T::default(),
            doc: None,
            is_async: false,
            vis: Default::default(),
            attributes: vec![],
        }
    }
}

/// Build something wrapped in braces, { A, B, C }
pub fn build_struct(mut s: impl Iterator<Item = impl AsRef<str>>) -> String {
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
pub fn build_dict<'a>(s: impl Iterator<Item = (&'a str, &'a str)>) -> String {
    build_struct(s.map(|(k, v)| format!("\"{}\": {}", k, v)))
}
