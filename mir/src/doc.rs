use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, Default)]
pub struct Doc(pub String);

impl Into<Doc> for &str {
    fn into(self) -> Doc {
        Doc(self.to_string())
    }
}

pub enum DocFormat {
    Markdown,
    Rst,
}

impl quote::ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc = self.0.trim();
        tokens.extend(quote!(#[doc = #doc]));
    }
}
