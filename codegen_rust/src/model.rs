use crate::{write_rust, Modified};
use hir::{Config, HirSpec, Record};
use mir::{import, File, Ident, Import, Item};
use mir_rust::{make_item, sanitize_filename, ToRustCode, ToRustIdent};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use std::fs;

pub fn write_model_module(spec: &HirSpec, cfg: &Config, m: &mut Modified) -> anyhow::Result<()> {
    let src = cfg.src();
    fs::create_dir_all(src.join("model"))?;

    let model_rs = make_model_rs(spec, cfg);
    let path = src.join("model").join("mod.rs");
    write_rust(&path, model_rs.to_rust_code(), m)?;

    for (name, record) in &spec.schemas {
        let file = make_single_module(record, spec, cfg);
        let name = sanitize_filename(name);
        let dest = src.join("model").join(&name).with_extension("rs");
        write_rust(&dest, file.to_rust_code(), m)?;
    }
    Ok(())
}

/// Generate a model.rs file that just imports from dependents.
pub fn make_model_rs(spec: &HirSpec, _cfg: &Config) -> File<TokenStream> {
    let it = spec.schemas.keys();
    let imports = it
        .clone()
        .map(|name: &String| {
            let fname = sanitize_filename(&name);
            Import::new(&fname, vec!["*"]).public()
        })
        .collect();
    let items = it
        .clone()
        .map(|name| {
            let name = Ident(sanitize_filename(name));
            Item::Block(quote! {
                mod #name;
            })
        })
        .collect();
    File {
        imports,
        items,
        ..File::default()
    }
}

fn check_imports(r: &Record, path: &str) -> Option<Import> {
    let names = r
        .fields()
        .flat_map(|f| f.ty.inner_model())
        .filter(|&name| name != r.name())
        .map(|name| name.to_rust_struct().0)
        .collect::<BTreeSet<_>>();
    if !names.is_empty() {
        Some(Import::new(path, names.into_iter().collect::<Vec<_>>()))
    } else {
        None
    }
}

/// Generate the file for a single struct.
pub fn make_single_module(record: &Record, spec: &HirSpec, cfg: &Config) -> File<TokenStream> {
    let mut imports = vec![import!(serde, Serialize, Deserialize)];
    if let Some(import) = check_imports(record, "super") {
        imports.push(import);
    }
    File {
        items: vec![make_item(record, spec, &cfg)],
        imports,
        ..File::default()
    }
}
