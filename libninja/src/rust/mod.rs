use std::hash::Hash;

use convert_case::Casing;

use codegen_rust::file::client;
use ln_core::fs;
use ln_core::{copy_builtin_files, create_context, get_template_file, prepare_templates};
use mir_rust::ident::ToRustIdent;
use mir_rust::ty::ToRustType;
use mir_rust::{format_code, RustExtra};
use mir_rust::{sanitize_filename, ToRustCode};

pub use crate::rust::codegen::generate_example;
use crate::rust::io::write_rust_file_to_path;
use crate::{add_operation_models, extract_spec, Config, Config};
use codegen_rust::file::client::{build_Client_authenticate, server_url};
use codegen_rust::file::request::{
    assign_inputs_to_request, build_request_struct, build_request_struct_builder_methods,
    build_url, generate_request_model_rs,
};

mod cargo_toml;
pub mod codegen;
pub mod format;
mod io;
pub mod lower_hir;
mod serde;
