use std::path::{Path, PathBuf};
use anyhow::Result;
use clap::{Args, ValueEnum};
use convert_case::{Case, Casing};
use crate::{generate_library_using_spec_at_path, OutputOptions, Language, LibraryOptions};
use crate::options::LibraryConfig;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Config {
    /// Only used by Rust. Adds ormlite::TableMeta flags to the code.
    Ormlite,
}

impl From<&[Config]> for LibraryConfig {
    fn from(value: &[Config]) -> Self {
        use Config::*;
        let mut config = LibraryConfig::default();
        for c in value {
            match c {
                Ormlite => config.ormlite = true,
            }
        }
        config
    }
}

#[derive(Args, Debug)]
pub struct Generate {
    /// Service name.
    name: String,
    #[clap(short, long = "lang")]
    pub language: Language,

    /// Path to the OpenAPI spec file.
    spec_filepath: String,

    /// The qualified github repo name, eg libninjacom/petstore-rs
    #[clap(short, long = "repo")]
    github_repo: String,

    #[clap(short, long)]
    output_dir: String,

    /// Package name. Defaults to the service name.
    #[clap(short, long = "package")]
    package_name: Option<String>,

    /// Set the version of the library. Defaults to 0.1.0.
    #[clap(long = "version")]
    version: Option<String>,

    /// Package name. Defaults to the service name.
    #[clap(short, long)]
    config: Vec<Config>,
}

impl Generate {
    pub fn run(self) -> Result<()> {
        let package_name = self.package_name.unwrap_or_else(|| self.name.to_lowercase());
        let version = self.version.unwrap_or_else(|| "0.1.0".to_string());

        generate_library_using_spec_at_path(
            &PathBuf::from(self.spec_filepath),
            OutputOptions {
                library_options: LibraryOptions {
                    package_name,
                    service_name: self.name.to_case(Case::Pascal),
                    package_version: version,
                    generator: self.language,
                    config: LibraryConfig::from(self.config.as_slice()),
                },
                qualified_github_repo: self.github_repo,
                dest_path: PathBuf::from(self.output_dir),
            },
        )

    }
}