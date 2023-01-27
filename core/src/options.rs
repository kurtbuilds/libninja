use std::path::PathBuf;
use convert_case::{Case, Casing};
use crate::mir::{Literal, Name};

#[derive(Debug, Clone, Default)]
pub struct LibraryConfig {
    /// Only for Rust. Adds ormlite::TableMeta flags to the code.
    pub ormlite: bool,
}



#[derive(Debug, Clone)]
pub struct LibraryOptions {
    // e.g. petstore-api
    pub package_name: String,
    // eg PetStore
    pub service_name: String,

    pub generator: crate::Language,

    pub package_version: String,

    pub config: LibraryConfig,
}

impl LibraryOptions {
    pub fn new(service_name: &str, language: crate::Language) -> Self {
        Self {
            package_name: service_name.to_case(Case::Snake),
            service_name: service_name.to_string(),
            generator: crate::Language::Rust,
            package_version: "0.1.0".to_string(),
            config: Default::default(),
        }
    }

    pub fn user_agent(&self) -> Literal<String> {
        Literal::from(format!(
            "{}/{}/{}",
            self.package_name,
            self.generator.to_string(),
            self.package_version
        ))
    }

    pub fn client_name(&self) -> Name {
        Name(format!("{} Client", self.service_name))
    }

    pub fn async_client_name(&self) -> Name {
        Name(format!("Async {} Client", self.service_name))
    }

    pub fn authenticator_name(&self) -> Name {
        Name(format!("{} Authentication", self.service_name))
    }

    pub fn bare_client_name(&self) -> Name {
        Name::new("Client")
    }

    pub fn env_var(&self, name: &str) -> Literal<String> {
        Literal(format!(
            "{}_{}",
            self.service_name.to_case(Case::ScreamingSnake),
            name.to_case(Case::ScreamingSnake)
        ), false)
    }
}

pub struct OutputOptions {
    pub library_options: LibraryOptions,

    // eg libninjacom/petstore-rs
    pub qualified_github_repo: String,

    pub dest_path: PathBuf,
}

impl OutputOptions {
    pub fn user_agent(&self) -> String {
        format!(
            "{}/{}/{}",
            self.library_options.package_name,
            self.library_options.generator.to_string(),
            self.library_options.package_version
        )
    }

    pub fn client_name(&self) -> Name {
        self.library_options.client_name()
    }

    pub fn async_client_name(&self) -> Name {
        self.library_options.async_client_name()
    }
}
