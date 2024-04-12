#![allow(unused)]

pub use extractor::extract_spec;
pub use fs::*;
pub use options::*;
pub use template::*;

pub mod extractor;
pub mod fs;
mod options;
pub mod sanitize;
mod template;
pub mod util;
