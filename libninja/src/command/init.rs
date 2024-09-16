use anyhow::Result;
use clap::Parser;
use convert_case::{Case, Casing};
use std::env::set_current_dir;
use std::process::Command;

#[derive(Parser, Debug)]
pub struct Init {
    /// The "service" name. E.g. if we want to generate a library for the Stripe API, this would be "Stripe".
    name: String,
}

impl Init {
    pub fn run(self) -> Result<()> {
        let name = self.name.to_case(Case::Snake);
        Command::new("cargo").arg("new").arg(&name).output()?;
        set_current_dir(&name)?;
        Command::new("cargo").arg("add").arg("httpclient").output()?;
        Command::new("cargo").args(["add", "serde", "-F", "derive"]).output()?;
        Ok(())
    }
}
