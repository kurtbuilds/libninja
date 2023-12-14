use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use clap::Args;
use crate::read_spec;
use ln_core::child_schemas::ChildSchemas;
use ln_core::extract_spec;
use ln_core::extractor::add_operation_models;
use hir::Language;
use crate::rust::calculate_extras;

#[derive(Args, Debug)]
pub struct Meta {
    service_name: String,
    spec_filepath: String,

    #[clap(short, long = "lang")]
    pub language: Option<Language>,

    #[clap(long)]
    pub repo: Option<String>,

    #[clap(short, long)]
    pub output: Option<String>,
}

impl Meta {
    pub fn run(self) -> Result<()> {
        let path = PathBuf::from(self.spec_filepath);
        let spec = read_spec(&path)?;
        let mut schema_lookup = HashMap::new();
        spec.add_child_schemas(&mut schema_lookup);
        for (name, schema) in schema_lookup {
            println!("{}", name);
        }
        let spec = extract_spec(&spec)?;
        let spec = add_operation_models(Language::Rust, spec)?;
        let extras = calculate_extras(&spec);
        println!("{:#?}", extras);
        // println!("{}", serde_json::to_string_pretty(&spec)?);
        Ok(())
    }
}

