use crate::extras::Extras;
use crate::{write_rust, Modified};
use proc_macro2::TokenStream;
use quote::quote;
use std::io::Result;
use std::path::Path;
use std::str::FromStr;

pub fn write_serde_module(extras: &Extras, path: &Path, modified: &mut Modified) -> Result<()> {
    if !extras.needs_serde() {
        return Ok(());
    }
    let path = path.join("serde.rs");

    let null_as_zero = extras
        .null_as_zero
        .then(option_i64_null_as_zero_module)
        .unwrap_or_default();

    let date_as_int = extras
        .integer_date_serialization
        .then(option_chrono_naive_date_as_int_module)
        .unwrap_or_default();

    let int_as_str = extras
        .option_i64_str
        .then(option_i64_str_module)
        .unwrap_or_default();

    let code = quote! {
        pub use ::serde::*;
        #null_as_zero
        #date_as_int
        #int_as_str
    };
    write_rust(&path, code, modified)
}

pub fn option_i64_null_as_zero_module() -> TokenStream {
    TokenStream::from_str(include_str!("option_i64_null_as_zero.rs")).unwrap()
}

pub fn option_i64_str_module() -> TokenStream {
    TokenStream::from_str(include_str!("option_i64_str.rs")).unwrap()
}

pub fn option_chrono_naive_date_as_int_module() -> TokenStream {
    TokenStream::from_str(include_str!("option_chrono_naive_date_as_int.rs")).unwrap()
}
