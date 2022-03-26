use proc_macro2::TokenStream;
use quote::quote;
// use anyhow::Result;
use openapi_client_generator::format_code;


fn codegen() -> TokenStream {
    quote! {
        use tokio;

        pub async fn main() {
            println!("Hello, world!");
        }
    }
}


fn main() {
    // let owned = String::from("foo");
    // let reference = &owned;
    // let u = ["bar"].contains(&reference.as_str());
    // println!("{}", u);
    // let s = serde_json::to_string(&ForceTrue::True)?;
    // println!("{}", s);
    let code = codegen();
    let code = format_code(code).unwrap();
    println!("{code}");
}