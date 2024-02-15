use proc_macro2::TokenStream;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;
    use mir_rust::format_code;

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
        let code = format_code(code);
        assert_eq!(
            code,
            r#"
use tokio;
pub async fn main() {
    println!("Hello, world!");
}
"#.trim()
        );
    }
}
