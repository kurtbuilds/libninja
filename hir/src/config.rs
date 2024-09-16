use convert_case::{Case, Casing};
use mir::{Ident, Literal};
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub name: String,
    pub dest: PathBuf,
    pub derives: Vec<String>,
    pub build_examples: bool,
    pub ormlite: bool,
}

impl Config {
    pub fn user_agent(&self) -> Literal<String> {
        Literal(format!(
            "{}",
            self.name // self.package_version
        ))
    }

    pub fn client_name(&self) -> Ident {
        Ident(format!("{}Client", self.name))
    }

    pub fn async_client_name(&self) -> String {
        format!("Async{}Client", self.name)
    }

    pub fn authenticator_name(&self) -> String {
        format!("{}Auth", self.name)
    }

    pub fn env_var(&self, name: &str) -> Literal<String> {
        Literal(format!(
            "{}_{}",
            self.name.to_case(Case::ScreamingSnake),
            name.to_case(Case::ScreamingSnake)
        ))
    }

    pub fn src(&self) -> PathBuf {
        self.dest.join("src")
    }
}
