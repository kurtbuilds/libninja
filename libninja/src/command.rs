mod generate;
mod resolve;
mod meta;

use anyhow::anyhow;
pub use generate::*;
pub use resolve::*;
pub use meta::*;

pub trait Success {
    fn ok(&self) -> anyhow::Result<()>;
}

impl Success for std::process::ExitStatus {
    fn ok(&self) -> anyhow::Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow!("Process exited with code: {:?}", self))
        }
    }
}
