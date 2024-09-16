#![allow(non_snake_case)]
#![allow(unused)]

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use hir::Language;
use libninja::command::*;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn warn_if_not_found(command: &str) {
    if std::process::Command::new(command)
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn()
        .is_err()
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

#[derive(Subcommand, Debug)]
pub enum Command {
    Gen(Generate),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().without_time())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target(env!("CARGO_BIN_NAME"), level)
                .with_target("libninja_mir", level)
                .with_target("libninja_hir", level)
                .with_target("ln_core", level)
                .with_target("ln_macro", level),
        )
        .init();

    match cli.command {
        Command::Gen(generate) => {
            use Language::*;
            match generate.language {
                Rust => {}
                // Python => {
                //     warn_if_not_found("pdm");
                //     warn_if_not_found("black");
                // },
                // Typescript => {
                //     warn_if_not_found("pnpm");
                //     warn_if_not_found("prettier")
                // },
                // Golang => {
                //     warn_if_not_found("gofmt");
                // },
            }
            generate.run()
        }
    }
}
