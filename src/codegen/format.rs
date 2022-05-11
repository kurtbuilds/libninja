use anyhow::Result;
use proc_macro2::TokenStream;


pub fn format_code(code: TokenStream) -> Result<String> {
    let code = code.to_string();
    let syntax_tree = syn::parse_file(&code).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    Ok(code)
}


#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use super::*;

    fn codegen_example() -> TokenStream {
        quote! {
            use tokio;

            pub async fn main() {
                println!("Hello, world!");
            }
        }
    }

    #[test]
    fn test_codegen() {
        let code = codegen_example();
        let code = format_code(code).unwrap();
        assert_eq!(code, r#"
use tokio;
pub async fn main() {
    println!("Hello, world!");
}
"#.trim_start());
    }
}