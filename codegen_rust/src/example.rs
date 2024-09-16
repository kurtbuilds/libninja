use crate::write_rust;
use anyhow::Result;
use hir::{Config, HirSpec, Language, Operation};
use libninja_macro::rfunction;
use mir::{import, File, Function, Item};
use mir_rust::{to_rust_example_value, ToRustCode, ToRustIdent};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub fn write_examples_folder(spec: &HirSpec, config: &Config, modified: &mut HashSet<PathBuf>) -> Result<()> {
    let path = config.dest.join("examples");
    fs::create_dir_all(&path)?;
    for operation in &spec.operations {
        let file = generate_example(operation, &config, spec)?;
        let path = path.join(operation.file_name()).with_extension("rs");
        write_rust(&path, file.to_rust_code(), modified)?;
    }
    Ok(())
}

pub fn generate_example(operation: &Operation, cfg: &Config, spec: &HirSpec) -> Result<File<TokenStream>> {
    let args = operation.function_args(Language::Rust);
    let declarations = args
        .iter()
        .map(|p| {
            let ident = p.name.to_rust_ident();
            let value = to_rust_example_value(&p.ty, &p.name, spec, true);
            Ok(quote! {
                let #ident = #value;
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let fn_args = args.iter().map(|p| p.name.to_rust_ident());
    let optionals: Vec<TokenStream> = operation
        .optional_args()
        .into_iter()
        .map(|p| {
            let ident = p.name.to_rust_ident();
            let value = to_rust_example_value(&p.ty, &p.name, spec, true);
            Ok(quote! {
                .#ident(#value)
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut imports = vec![
        import!(format!("{}::model::*", cfg.package_name())),
        import!(cfg.package_name(), cfg.client_name()),
    ];
    if operation.use_required_struct(Language::Rust) {
        let struct_name = operation.required_struct_name();
        imports.push(import!(format!("{}::request", cfg.package_name()), struct_name));
    }
    let operation = operation.name.to_rust_ident();
    let client = cfg.client_name();
    let mut main: Function<TokenStream> = rfunction!(async main() {
       let client = #client::from_env();
        #(#declarations)*
        let response = client.#operation(#(#fn_args),*)
            #(#optionals)*
            .await
            .unwrap();
        println!("{:#?}", response);
    });
    main.attributes.push(quote!(#[tokio::main]));

    Ok(File {
        attributes: vec![quote! {#![allow(unused_imports)]}],
        imports,
        items: vec![Item::Fn(main)],
        ..File::default()
    })
}
