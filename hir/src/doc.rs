#[derive(Debug, Clone)]
pub struct Doc(pub String);

impl Doc {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub fn doc<S: Into<String>>(s: S) -> Option<Doc> {
    let s = s.into();
    if s.is_empty() {
        None
    } else {
        Some(Doc(s))
    }
}

pub enum DocFormat {
    Markdown,
    Rst,
}
