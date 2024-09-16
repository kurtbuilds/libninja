use crate::{rust, Language};
use anyhow::{anyhow, Result};
use clap::Args;
use convert_case::{Case, Casing};
use openapiv3::{OpenAPI, VersionedOpenAPI};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct Generate {
    /// Service name.
    #[clap(short, long = "lang", default_value = "rust")]
    pub language: Language,

    /// Toggle whether to generate examples.
    /// Defaults to true
    #[clap(long, default_value = "true")]
    examples: bool,

    #[clap(short, long)]
    output_dir: Option<String>,

    /// List of additional namespaced traits to derive on generated structs.
    #[clap(long)]
    derive: Vec<String>,

    /// The "service" name. E.g. if we want to generate a library for the Stripe API, this would be "Stripe".
    name: String,

    /// Path to the OpenAPI spec file.
    spec_filepath: String,
}

impl Generate {
    pub fn run(self) -> Result<()> {
        let package_name = self
            .package_name
            .unwrap_or_else(|| self.name.to_lowercase());

        let path = PathBuf::from(self.spec_filepath);
        let output_dir = self.output_dir.unwrap_or_else(|| ".".to_string());
        let spec = read_spec(&path)?;
        generate_library(
            spec,
            Config {
                dest_path: PathBuf::from(output_dir),
                config: build_config(&self.config),
                language: self.language,
                build_examples: self.examples.unwrap_or(true),
                package_name,
                service_name: self.name.to_case(Case::Pascal),
                github_repo: self.repo,
                version: self.version,
                derive: self.derive,
            },
        )
    }
}

pub fn read_spec(path: &Path) -> Result<OpenAPI> {
    let file = File::open(path).map_err(|_| anyhow!("{:?}: File not found.", path))?;
    let ext = path
        .extension()
        .map(|s| s.to_str().expect("Extension isn't utf8"))
        .unwrap_or_else(|| "yaml");
    let openapi: VersionedOpenAPI = match ext {
        "yaml" => serde_yaml::from_reader(file)?,
        "json" => serde_json::from_reader(file)?,
        _ => panic!("Unknown file extension"),
    };
    let openapi = openapi.upgrade();
    Ok(openapi)
}

pub fn generate_library(spec: OpenAPI, opts: Config) -> Result<()> {
    let spec = extract_spec(&spec)?;
    match opts.language {
        Language::Rust => rust::generate_rust_library(spec, opts),
        // Language::Python => python::generate_library(spec, opts),
        // Language::Typescript => typescript::generate_library(spec, opts),
        // Language::Golang => go::generate_library(spec, opts),
    }
}
