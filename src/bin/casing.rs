use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

fn main() {
    println!("{}", "clientId".to_case(Case::Snake));
}