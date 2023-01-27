use anyhow::Result;
use proc_macro2::TokenStream;
use std::fs::File;
use std::io::Write;

pub fn format_code(code: TokenStream) -> Result<String> {
    let code = code.to_string();
    let syntax_tree = match syn::parse_file(&code) {
        Ok(syntax_tree) => syntax_tree,
        Err(e) => {
            println!("{}", code);
            return Err(anyhow::anyhow!(
                "Failed to parse generated code: {}",
                e
            ))
        }
    };
    let mut code = prettyplease::unparse(&syntax_tree);
    if code.ends_with('\n') {
        code.pop();
    }
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

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
