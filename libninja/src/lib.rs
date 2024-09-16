use anyhow::Context;
pub use openapiv3;
pub use ::openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};

use hir::Language;

mod command;
mod config;
mod extractor;
mod fs;
mod rust;
