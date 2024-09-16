mod null_as_zero;
mod option_chrono_naive_date_as_int;
mod option_i64_null_as_zero;
mod option_i64_str;

use proc_macro2::TokenStream;
use std::str::FromStr;

pub fn option_i64_null_as_zero_module() -> TokenStream {
    TokenStream::from_str(include_str!("option_i64_null_as_zero.rs")).unwrap()
}

pub fn option_i64_str_module() -> TokenStream {
    TokenStream::from_str(include_str!("option_i64_str.rs")).unwrap()
}

pub fn option_chrono_naive_date_as_int_module() -> TokenStream {
    TokenStream::from_str(include_str!("option_chrono_naive_date_as_int.rs")).unwrap()
}
