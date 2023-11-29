use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use clap::Args;
use crate::read_spec;
use ln_core::child_schemas::ChildSchemas;

#[derive(Args, Debug)]
pub struct Meta {
    service_name: String,
    spec_filepath: String,
}

impl Meta {
    pub fn run(self) -> Result<()> {
        let path = PathBuf::from(self.spec_filepath);
        let spec = read_spec(path, &self.service_name)?;
        let mut schema_lookup = HashMap::new();
        spec.add_child_schemas(&mut schema_lookup);
        for (name, schema) in schema_lookup {
            println!("{}", name);
        }
        // println!("{}", serde_json::to_string_pretty(&spec)?);
        Ok(())
    }
}

