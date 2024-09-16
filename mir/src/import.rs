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
    pub feature: Option<String>,
}

impl Import {
    pub fn package(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            imports: vec![],
            alias: None,
            vis: Visibility::Private,
            feature: None,
        }
    }

    pub fn new(path: impl Into<String>, imports: impl IntoIterator<Item = impl Into<ImportItem>>) -> Self {
        Self {
            path: path.into(),
            imports: imports.into_iter().map(|s| s.into()).collect(),
            alias: None,
            vis: Visibility::Private,
            feature: None,
        }
    }

    pub fn alias(path: &str, alias: &str) -> Self {
        Self {
            path: path.to_string(),
            imports: Vec::new(),
            alias: Some(alias.to_string()),
            vis: Visibility::Private,
            feature: None,
        }
    }

    pub fn public(mut self) -> Self {
        self.vis = Visibility::Public;
        self
    }
}

pub struct ImportItem {
    /// This might not conform to standard ident rules for the language, so its a string, not an ident.
    pub name: String,
    pub alias: Option<String>,
}

impl ImportItem {
    pub fn alias(name: &str, alias: &str) -> Self {
        Self {
            name: name.to_string(),
            alias: Some(alias.to_string()),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("ImportItem name cannot be empty".to_string());
        }
        if self.name.chars().all(|c| c.is_digit(10)) {
            return Err("ImportItem name cannot be all digits".to_string());
        }
        Ok(())
    }
}

impl From<&String> for ImportItem {
    fn from(s: &String) -> Self {
        let r = Self {
            name: s.clone(),
            alias: None,
        };
        r.validate().unwrap();
        r
    }
}

impl From<String> for ImportItem {
    fn from(s: String) -> Self {
        let r = Self { name: s, alias: None };
        r.validate().unwrap();
        r
    }
}

impl From<&str> for ImportItem {
    fn from(s: &str) -> Self {
        let r = Self {
            name: s.to_string(),
            alias: None,
        };
        r.validate().unwrap();
        r
    }
}

impl From<Ident> for ImportItem {
    fn from(s: Ident) -> Self {
        Self { name: s.0, alias: None }
    }
}
