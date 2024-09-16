mod example;
pub mod request;

use hir::Config;
use hir::{HirSpec, Language};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use proc_macro2::TokenStream;
use anyhow::Result;
use mir_rust::format_code;

pub fn generate_rust_library(spec: HirSpec, config: Config) -> Result<()> {
    let src_path = opts.dest_path.join("src");

    // Prepare the HIR Spec.
    let extras = calculate_extras(&spec);

    // if src doesn't exist that's fine
    let _ = fs::remove_dir_all(&src_path);
    fs::create_dir_all(&src_path)?;

    // If there's nothing in cargo.toml, you want to prompt for it here.
    // Then pass it back in.
    // But you only need it if you're generating the README and/or Cargo.toml
    let mut context = HashMap::<String, String>::new();
    if !opts.dest_path.join("README.md").exists() || !opts.dest_path.join("Cargo.toml").exists() {
        if let Some(github_repo) = &opts.github_repo {
            context.insert("github_repo".to_string(), github_repo.to_string());
        } else {
            println!(
                "Because this is a first-time generation, please provide additional information."
            );
            print!("Please provide a Github repo name (e.g. libninja/plaid-rs): ");
            let github_repo: String = read!("{}\n");
            context.insert("github_repo".to_string(), github_repo);
        }
    }
    let version = cargo_toml::update_cargo_toml(&extras, &opts, &context)?;
    let build_examples = opts.build_examples;
    let opts = Config {
        package_name: opts.package_name,
        service_name: opts.service_name,
        language: opts.language,
        package_version: version,
        config: opts.config,
        dest: opts.dest_path,
        derives: opts.derive,
    };
    write_model_module(&spec, &opts)?;
    write_request_module(&spec, &opts)?;
    write_lib_rs(&spec, &extras, &opts)?;
    write_serde_module_if_needed(&extras, &opts.dest)?;

    let spec = add_operation_models(opts.language, spec)?;

    if build_examples {
        write_examples(&spec, &opts)?;
    }

    let tera = prepare_templates();
    let mut template_context = create_context(&opts, &spec);
    template_context.insert(
        "client_docs_url",
        &format!("https://docs.rs/{}", opts.package_name),
    );
    if let Some(github_repo) = context.get("github_repo") {
        template_context.insert("github_repo", github_repo);
    }
    copy_builtin_files(&opts.dest, &opts.language.to_string(), &["src"])?;
    copy_builtin_templates(&opts, &tera, &template_context)?;
    copy_from_target_templates(&opts.dest)?;
    Ok(())
}


pub fn write_rust(path: &Path, tokens: TokenStream) -> Result<()> {
    let code = format_code(tokens);
    let existing_content = fs::read_to_string(path).unwrap_or_default();
    if existing_content.st
    
}