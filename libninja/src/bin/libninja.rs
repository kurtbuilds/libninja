#![allow(non_snake_case)]
#![allow(unused)]

use anyhow::Result;
use convert_case::{Case, Casing};
use clap::{Args, Parser, Subcommand};
use ln_core::{OutputConfig, PackageConfig};
use hir::Language;
use libninja::rust::generate_rust_library;
use std::path::Path;
use libninja::command::*;

fn warn_if_not_found(command: &str) {
    if std::process::Command::new(command)
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn().is_err()
    {
        eprintln!("Warning: {} not found. Some commands may fail.", command);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[clap(short, long, global = true)]
    verbose: bool,
}

#[derive(Args, Debug)]
pub struct Foobar {
    pub foo: String,
    pub bar: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Gen(Generate),
    /// OpenAPI specs can be split into multiple files. This command takes a path to the spec root,
    /// and examines all files in its parent directory to coalesce the spec into one single file.
    /// `gen` will not work if the spec is split into multiple files, so use this step first if the
    /// spec is split.
    Coalesce(Resolve),
    /// Analyze the OpenAPI spec
    Meta(Meta),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_ez::set_global_default_stderr();

    match cli.command {
        Command::Gen(generate) => {
            use Language::*;
            match generate.language {
                Rust => {
                },
                Python => {
                    warn_if_not_found("pdm");
                    warn_if_not_found("black");
                },
                Typescript => {
                    warn_if_not_found("pnpm");
                    warn_if_not_found("prettier")
                },
                Golang => {
                    warn_if_not_found("gofmt");
                },
            }
            generate.run()
        },
        Command::Coalesce(resolve) => resolve.run(),
        Command::Meta(meta) => meta.run(),
    }
}
