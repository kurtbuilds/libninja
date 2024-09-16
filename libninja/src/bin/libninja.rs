use anyhow::Result;
use clap::{Parser, Subcommand};
use libninja::command::*;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
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
        Command::Gen(generate) => generate.run(),
    }
}
