#![allow(non_snake_case)]
#![allow(unused)]

use std::path::{Path, PathBuf};
use anyhow::Result;
use openapi_client_generator::{generate_library, generate_library_at_path, GenerateLibrary};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Gen {
        #[clap(short, long, value_parser)]
        name: String,

        #[clap(value_parser)]
        spec: String,

        #[clap(short, long, value_parser, default_value_t = String::from("src"))]
        output_dir: String,

        #[clap(short, long, value_parser)]
        lib_rs: Option<String>,

        #[clap(short, long, value_parser)]
        model_rs: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Gen { name, spec, output_dir, lib_rs, model_rs } => {
            generate_library_at_path(Path::new(spec),GenerateLibrary {
                name: name.to_string(),
                dest_path: output_dir.into(),
                lib_rs_path: lib_rs.as_ref().map(PathBuf::from),
                model_rs_path: model_rs.as_ref().map(PathBuf::from),
            })?;
        }
    }
    Ok(())
}
