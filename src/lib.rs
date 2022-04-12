#![allow(non_snake_case)]
#![allow(unused)]
use proc_macro2::TokenStream;
use anyhow::Result;

pub mod handwritten;
pub mod codegen;


pub fn format_code(code: TokenStream) -> Result<String> {
    let code = code.to_string();
    let syntax_tree = syn::parse_file(&code).map_err(|e| {
        println!("{:?}", e.span());
    }).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    Ok(code)
}
