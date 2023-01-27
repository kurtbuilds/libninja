use proc_macro2::TokenStream;
use quote::quote;
use ocg_macro::rfunction;
use ln_model::{File, Import};
use crate::{Language, LibraryOptions, mir};
use crate::mir::{MirSpec, Parameter};
use crate::rust::codegen;
use crate::rust::codegen::ident::ToRustIdent;
use crate::rust::codegen::{to_rust_example_value, ToRustCode};
use crate::rust::format::format_code;

pub trait ToRustExample {
    fn to_rust_example(&self, spec: &MirSpec) -> anyhow::Result<TokenStream>;
}

impl ToRustExample for Parameter {
    fn to_rust_example(&self, spec: &MirSpec) -> anyhow::Result<TokenStream> {
        codegen::to_rust_example_value(&self.ty, &self.name, spec, false)
    }
}


pub fn generate_example(operation: &mir::Operation, opt: &LibraryOptions, spec: &MirSpec) -> anyhow::Result<String> {
    let args = operation.function_args(Language::Rust);
    let declarations = args.iter().map(|p| {
        let ident = p.name.to_rust_ident();
        let value = to_rust_example_value(&p.ty, &p.name, spec, true)?;
        Ok(quote! {
            let #ident = #value;
        })
    }).collect::<anyhow::Result<Vec<_>, anyhow::Error>>()?;
    let fn_args = args.iter().map(|p| p.name.to_rust_ident());
    let optionals = operation.optional_args().into_iter().map(|p| {
        let ident = p.name.to_rust_ident();
        let value = to_rust_example_value(&p.ty, &p.name, spec, true)?;
        Ok(quote! {
            .#ident(#value)
        })
    }).collect::<anyhow::Result<Vec<_>, anyhow::Error>>()?;
    let qualified_client = format!("{}::{}", opt.package_name, opt.client_name().to_rust_struct());
    let mut imports = vec![
        Import::package(&qualified_client),
        Import::package(&format!("{}::model::*", opt.package_name)),
    ];
    if operation.use_required_struct(Language::Rust) {
        let struct_name = operation.required_struct_name().to_rust_struct().to_string();
        imports.push(Import::package(&format!("{}::request::{}", opt.package_name, struct_name)));
    }
    let operation = operation.name.to_rust_ident();
    let client = opt.client_name().to_rust_struct();
    let mut main = rfunction!(async main() {
        let client = #client::from_env();
        #(#declarations)*
        let response = client.#operation(#(#fn_args),*)
            #(#optionals)*
            .await
            .unwrap();
        println!("{:#?}", response);
    });
    main.annotations.push("tokio::main".to_string());

    let example = File {
        imports,
        functions: vec![main],
        ..File::default()
    };
    let code = example.to_rust_code();
    format_code(code)
}
