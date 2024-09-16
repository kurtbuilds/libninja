use std::fmt::Formatter;

/// Specifically represents a parameter in Location::Query. We need special treatment for repeated keys.
pub enum ParamKey {
    Key(String),
    RepeatedKey(String),
}

impl std::fmt::Display for ParamKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamKey::Key(s) => write!(f, "\"{}\"", s),
            ParamKey::RepeatedKey(s) => write!(f, "\"{}[]\"", s),
        }
    }
}
