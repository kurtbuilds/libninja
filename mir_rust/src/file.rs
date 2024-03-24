use mir::File;
use proc_macro2::TokenStream;
use crate::ToRustCode;
use quote::quote;

impl ToRustCode for File<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let File {
            imports,
            classes,
            enums,
            doc,
            functions,
            code,
            ..
        } = self;
        let imports = imports.into_iter().map(|i| i.to_rust_code());
        let doc = doc.to_rust_code();
        let functions = functions.into_iter().map(|f| f.to_rust_code());
        let classes = classes.into_iter().map(|c| c.to_rust_code());
        let enums = enums.into_iter().map(|c| c.to_rust_code());
        let code = code.unwrap_or_default();
        quote! {
            #doc
            #(#imports)*
            #(#functions)*
            #(#classes)*
            #(#enums)*
            #code
        }
    }
}

