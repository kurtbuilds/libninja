#[derive(Debug, Clone)]
pub struct Doc(pub String);

pub enum DocFormat {
    Markdown,
    Rst,
}