use proc_macro2::TokenStream;
use quote::quote;

fn codegen() -> TokenStream {
    quote! {
        use tokio;

        pub async fn main() {
            println!("Hello, world!");
        }
    }
}

fn main() {
    let z = codegen();
    println!("{}", z)
}