use crate::{Ident, Visibility};

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
    pub feature: Option<String>
}

pub struct ImportItem {
    /// This might not conform to standard ident rules for the language, so its a string, not an ident.
    pub name: String,
    pub alias: Option<String>,
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

