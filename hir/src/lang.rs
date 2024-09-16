use anyhow::Result;
use clap::ValueEnum;
use std::fmt::Display;

#[derive(Eq, PartialEq, Copy, Clone, Debug, ValueEnum)]
pub enum Language {
    Rust,
    // Python,
    // Typescript,
    // Golang,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Language::Rust => "rust",
            // Language::Python => "python",
            // Language::Typescript => "typescript",
            // Language::Golang => "go",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

impl std::str::FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rust" => Ok(Language::Rust),
            // "python" => Ok(Language::Python),
            // "typescript" => Ok(Language::Typescript),
            // "go" => Ok(Language::Golang),
            _ => Err(anyhow::anyhow!("Unknown generator: {}", s)),
        }
    }
}
