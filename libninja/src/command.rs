mod generate;
mod resolve;

use anyhow::anyhow;
pub use generate::*;
pub use resolve::*;

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
