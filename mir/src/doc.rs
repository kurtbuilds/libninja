#[derive(Debug, Clone)]
pub struct Doc(pub String);

impl Into<Doc> for &str {
    fn into(self) -> Doc {
        Doc(self.to_string())
    }
}

pub enum DocFormat {
    Markdown,
    Rst,
}