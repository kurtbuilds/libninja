use crate::{derives_to_tokens, serde_rename2, RustExtra, ToRustCode, ToRustIdent};
use mir::{Enum, Variant, Visibility};
use proc_macro2::TokenStream;
use quote::quote;

pub fn lower_enum(e: &hir::Enum, derives: &[String]) -> Enum<TokenStream, RustExtra> {
    let variants = e
        .variants
        .iter()
        .map(|s| {
            let ident = if let Some(a) = &s.alias {
                a.to_rust_struct()
            } else {
                let mut s = s.value.clone();
                if !s.is_empty() && s.chars().next().unwrap().is_numeric() {
                    s = format!("{}{}", e.name, s);
                }
                s.to_rust_struct()
            };
            let rename = serde_rename2(&s.value, &ident);
            Variant {
                ident,
                doc: None,
                value: None,
                extra: RustExtra {
                    attributes: rename.into_iter().collect(),
                },
            }
        })
        .collect();
    let derives = derives_to_tokens(derives);
    let derives = quote! { #[derive(Debug, Serialize, Deserialize #derives)] };
    Enum {
        name: e.name.to_rust_struct(),
        doc: e.doc.clone(),
        variants,
        vis: Visibility::Public,
        methods: Vec::new(),
        extra: RustExtra {
            attributes: vec![derives],
        },
    }
}

impl ToRustCode for Enum<TokenStream, RustExtra> {
    fn to_rust_code(self) -> TokenStream {
        let Enum {
            name,
            doc,
            vis,
            variants,
            methods,
            extra,
        } = self;
        let vis = vis.to_rust_code();
        let doc = doc.to_rust_code();
        let variants = variants.into_iter().map(|v| v.to_rust_code());
        let methods = methods.into_iter().map(|m| m.to_rust_code());
        let attributes = extra.attributes;
        quote! {
            #doc
            #(#attributes)*
            #vis enum #name {
                #(#variants),*
            }
            impl #name {
                #(#methods)*
            }
        }
    }
}

impl ToRustCode for Variant<RustExtra> {
    fn to_rust_code(self) -> TokenStream {
        let Variant {
            ident,
            doc,
            value,
            extra,
        } = self;
        let doc = doc.to_rust_code();
        let attributes = extra.attributes;
        let value = value.map(|v| quote!(= #v)).unwrap_or_default();
        quote! {
            #doc
            #(#attributes)*
            #ident #value
        }
    }
}
