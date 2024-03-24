use convert_case::{Case, Casing};
use hir::Language;
use mir::{literal, Literal};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct ConfigFlags {
    /// Only for Rust. Adds ormlite::TableMeta flags to the code.
    pub ormlite: bool,
    /// Only for Rust (for now). Adds fake::Dummy flags to the code.
    pub fake: bool,
}

#[derive(Debug, Clone)]
pub struct PackageConfig {
    // e.g. petstore-api
    pub package_name: String,
    // eg PetStore
    pub service_name: String,

    pub language: Language,

    pub package_version: String,

    pub config: ConfigFlags,

    pub dest: PathBuf,

    pub derives: Vec<String>,
}

impl PackageConfig {
    pub fn user_agent(&self) -> Literal<String> {
        literal(format!(
            "{}/{}/{}",
            self.package_name,
            self.language.to_string(),
            self.package_version
        ))
    }

    pub fn client_name(&self) -> String {
        format!("{} Client", self.service_name)
    }

    pub fn async_client_name(&self) -> String {
        format!("Async {} Client", self.service_name)
    }

    pub fn authenticator_name(&self) -> String {
        format!("{} Auth", self.service_name)
    }

    pub fn env_var(&self, name: &str) -> Literal<String> {
        literal(format!(
            "{}_{}",
            self.service_name.to_case(Case::ScreamingSnake),
            name.to_case(Case::ScreamingSnake)
        ))
    }

    pub fn get_file_template(&self, path: &str) -> Option<String> {
        let path = self.dest.join("template").join(path);
        std::fs::read_to_string(path).ok()
    }
}

pub struct OutputConfig {
    pub dest_path: PathBuf,
    pub build_examples: bool,
    // e.g. petstore-api
    pub package_name: String,
    // eg PetStore
    pub service_name: String,

    pub language: Language,

    pub config: ConfigFlags,

    pub github_repo: Option<String>,

    pub version: Option<String>,

    pub derive: Vec<String>,
}
