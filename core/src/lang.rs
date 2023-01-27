use anyhow::Result;
use serde::{Deserialize, Serialize};
use clap::ValueEnum;

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug, ValueEnum)]
pub enum Language {
    Rust,
    Python,
    Typescript,
    Golang,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::Typescript => "typescript",
            Language::Golang => "go",
        }
        .to_string()
    }
}

impl std::str::FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rust" => Ok(Language::Rust),
            "python" => Ok(Language::Python),
            "typescript" => Ok(Language::Typescript),
            "go" => Ok(Language::Golang),
            _ => Err(anyhow::anyhow!("Unknown generator: {}", s)),
        }
    }
}
