use std::fs::File;
use std::io::Write;
use std::path::Path;

use convert_case::{Case, Casing};
use openapiv3::{ArrayType, OpenAPI, Schema, SchemaKind};
use tera::{Context, Tera};
use url::Host;

use ln_core::fs;
use ln_core::OutputOptions;
pub use ln_mir::build_struct;

use crate::{MirSpec, };

pub fn code_sample(path: &Path) -> Option<String> {
    match path.read_dir() {
        Ok(read_dir) => {
            let mut examples = read_dir
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>();
            examples.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            let first = examples.into_iter().next();
            match first {
                None => None,
                Some(first) => {
                    let code = fs::read_to_string(&first).unwrap();
                    Some(code)
                }
            }
        }
        Err(_) => None,
    }
}