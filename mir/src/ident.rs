use std::fmt::Formatter;

/// Localized string
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Ident(pub String);

impl Ident {
    pub fn new(s: &'static str) -> Self {
        Ident(s.into())
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}