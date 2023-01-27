use openapiv3::OpenAPI;
use anyhow::{anyhow, Result};
use ln_core::{mir, LibraryOptions, OutputOptions};

#[cfg(feature = "commercial")]
pub mod python {
    pub use ln_commercial::python::*;
}

#[cfg(not(feature = "commercial"))]
pub mod python {
    use super::*;

    fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    fn generate_sync_example(operation: &mir::Operation, opt: &LibraryOptions, spec: &mir::MirSpec) -> Result<String> {
        Err(anyhow!("Commercial features are not enabled"))
    }
}

#[cfg(feature = "commercial")]
pub mod go {
    pub use ln_commercial::go::*;
}

#[cfg(not(feature = "commercial"))]
pub mod go {
    use super::*;

    fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    fn generate_example(operation: &mir::Operation, opt: &LibraryOptions, spec: &mir::MirSpec) -> Result<String> {
        Err(anyhow!("Commercial features are not enabled"))
    }
}

#[cfg(feature = "commercial")]
pub mod typescript {
    pub use ln_commercial::typescript::*;
}

#[cfg(not(feature = "commercial"))]
pub mod typescript {
    use super::*;

    fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    fn generate_example(operation: &mir::Operation, opt: &LibraryOptions, spec: &mir::MirSpec) -> Result<String> {
        Err(anyhow!("Commercial features are not enabled"))
    }
}