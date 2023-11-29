use openapiv3::OpenAPI;
use anyhow::{anyhow, Result};
use ln_core::{LibraryOptions, OutputOptions};
use hir::{HirSpec, Operation};

#[cfg(feature = "commercial")]
pub mod python {
    pub use ln_commercial::python::*;
}

#[cfg(not(feature = "commercial"))]
pub mod python {
    use super::*;

    pub fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    pub fn generate_sync_example(operation: &Operation, opt: &LibraryOptions, spec: &HirSpec) -> Result<String> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    pub fn generate_async_example(operation: &Operation, opt: &LibraryOptions, spec: &HirSpec) -> Result<String> {
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

    pub fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    pub fn generate_example(operation: &Operation, opt: &LibraryOptions, spec: &HirSpec) -> Result<String> {
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

    pub fn generate_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
        Err(anyhow!("Commercial features are not enabled"))
    }

    pub fn generate_example(operation: &Operation, opt: &LibraryOptions, spec: &HirSpec) -> Result<String> {
        Err(anyhow!("Commercial features are not enabled"))
    }
}