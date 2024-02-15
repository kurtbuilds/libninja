use mir::{Import, ImportItem};
use crate::ToRustCode;
use proc_macro2::{TokenStream, Span};
use syn::Path;
use quote::quote;

impl ToRustCode for Import {
    fn to_rust_code(mut self) -> TokenStream {
        fn inner(import: Import) -> TokenStream {
            let Import {
                path,
                alias,
                imports,
                vis,
                feature,
            } = import;
            if path.ends_with('*') {
                let path = syn::parse_str::<Path>(&path[..path.len() - 3]).unwrap();
                return quote! { use #path::*; };
            }
            let path = syn::parse_str::<Path>(&path).expect(&format!("Failed to parse as syn::Path: {}", &path));
            let vis = vis.to_rust_code();
            if let Some(alias) = alias {
                let alias = syn::parse_str::<syn::Ident>(&alias).unwrap();
                quote!( #vis use #path as #alias; )
            } else if !imports.is_empty() {
                let imports = imports.into_iter().map(|i| i.to_rust_code());
                quote!( #vis use #path::{#(#imports),*}; )
            } else {
                quote! { #vis use #path; }
            }
        }
        let feature = std::mem::take(&mut self.feature).map(|f| {
            let f = syn::Ident::new(&f, Span::call_site());
            quote!(#[cfg(feature = #f)])
        }).unwrap_or_default();
        let import = inner(self);
        quote!(#feature #import)
    }
}

impl ToRustCode for ImportItem {
    fn to_rust_code(self) -> TokenStream {
        if let Some(alias) = self.alias {
            let alias = syn::Ident::new(&alias, Span::call_site());
            let path = syn::parse_str::<syn::Path>(&self.name).unwrap();
            quote! { #path as #alias }
        } else if &self.name == "*" {
            quote! { * }
        } else {
            let path = syn::parse_str::<syn::Path>(&self.name).unwrap();
            quote! { #path }
        }
    }
}



