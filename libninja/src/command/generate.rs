use std::path::{Path, PathBuf};
use std::process::Output;
use anyhow::Result;
use clap::{Args, ValueEnum};
use convert_case::{Case, Casing};
use tracing::debug;
use crate::{OutputConfig, Language, PackageConfig, read_spec, generate_library};
use ln_core::{ConfigFlags};

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Config {
    /// Only used by Rust. Adds ormlite::TableMeta flags to the code.
    Ormlite,
}

fn build_config(configs: &[Config]) -> ConfigFlags {
    let mut config = ConfigFlags::default();
    for c in configs {
        match c {
            Config::Ormlite => config.ormlite = true,
        }
    }
    config
}

#[derive(Args, Debug)]
pub struct Generate {
    /// Service name.
    #[clap(short, long = "lang")]
    pub language: Language,

    /// Toggle whether to generate examples.
    /// Defaults to true
    #[clap(long)]
    examples: Option<bool>,

    #[clap(short, long)]
    output_dir: Option<String>,

    #[clap(long)]
    version: Option<String>,

    /// config options
    #[clap(short, long)]
    config: Vec<Config>,

    /// Repo (e.g. libninjacom/plaid-rs)
    #[clap(long)]
    repo: Option<String>,

    /// Package name. Defaults to the service name.
    #[clap(short, long = "package")]
    package_name: Option<String>,

    /// The "service" name. E.g. if we want to generate a library for the Stripe API, this would be "Stripe".
    name: String,

    /// Path to the OpenAPI spec file.
    spec_filepath: String,
}

impl Generate {
    pub fn run(self) -> Result<()> {
        let package_name = self.package_name.unwrap_or_else(|| self.name.to_lowercase());

        let path = PathBuf::from(self.spec_filepath);
        let output_dir = self.output_dir.unwrap_or_else(|| ".".to_string());
        let spec = read_spec(&path)?;
        generate_library(spec, OutputConfig {
            dest_path: PathBuf::from(output_dir),
            config: build_config(&self.config),
            language: self.language,
            build_examples: self.examples.unwrap_or(true),
            package_name,
            service_name: self.name.to_case(Case::Pascal),
            github_repo: self.repo,
            version: self.version,
        })
    }
}
