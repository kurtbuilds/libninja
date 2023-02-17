use std::path::Path;
use std::thread::current;

use anyhow::Result;
use convert_case::{Case, Casing};
use indoc::formatdoc;
use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::quote;

use codegen::ToRustIdent;
use codegen::ToRustType;
use format::format_code;
use ln_core::{copy_files, copy_templates, create_context, get_template_file, prepare_templates};
use ln_mir::{Visibility, Import, File};
use ln_core::fs;

use crate::{add_operation_models, extract_spec, LibraryOptions, MirSpec, OutputOptions, util};
pub use crate::rust::codegen::generate_example;
use crate::rust::codegen::ToRustCode;
use crate::rust::io::write_rust_file_to_path;
use crate::rust::mir::{generate_model_rs, generate_single_model_file};
use crate::rust::request::{build_request_struct, generate_request_model_rs};

pub mod client;
pub mod codegen;
pub mod format;
pub mod mir;
pub mod request;
mod io;
mod serde;

pub struct Extras {
    null_as_zero: bool,
    date_serialization: bool,
    currency: bool,
}

impl Extras {
    pub fn needs_serde(&self) -> bool {
        self.null_as_zero || self.date_serialization
    }
}

pub fn calculate_extras(spec: &MirSpec) -> Extras {
    use ln_core::hir::Ty;
    let mut null_as_zero = false;
    let mut date_serialization = false;
    let mut currency = false;
    for (_, record) in &spec.schemas {
        for field in record.fields() {
            match &field.ty {
                Ty::Integer { null_as_zero: true } => {
                    null_as_zero = true;
                }
                Ty::Date { serialization: ln_core::hir::DateSerialization::Integer } => {
                    date_serialization = true;
                }
                Ty::Currency { .. } => {
                    currency = true;
                }
                _ => {}
            }
        }
    }
    Extras {
        null_as_zero,
        date_serialization,
        currency,
    }
}


pub fn generate_rust_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
    let config = &opts.library_options.config;
    let src_path = opts.dest_path.join("src");
    fs::remove_dir_all(&src_path)?;
    fs::create_dir_all(&src_path)?;

    // Prepare the MIR Spec.
    let mir_spec = extract_spec(&spec, &opts.library_options)?;
    let mir_spec = add_operation_models(opts.library_options.language, mir_spec)?;
    let extras = calculate_extras(&mir_spec);

    write_model_module(&mir_spec, &opts)?;

    write_request_module(&mir_spec, &opts)?;

    write_lib_rs(&mir_spec, &extras, &spec, &opts)?;

    write_serde_module_if_needed(&extras, &opts)?;

    let example = write_examples(&mir_spec, &opts)?;

    let tera = prepare_templates();
    let mut context = create_context(&opts, &mir_spec);
    context.insert("code_sample", &example);
    context.insert("client_docs_url", &format!("https://docs.rs/{}", opts.library_options.package_name));

    copy_files(&opts.dest_path, &opts.library_options.language.to_string(), &["src"])?;
    copy_templates(&opts, &tera, &context)?;

    bump_version_and_update_deps(&extras, &opts)?;

    Ok(())
}

fn write_model_module(mir_spec: &MirSpec, opts: &OutputOptions) -> Result<()> {
    let config = &opts.library_options.config;
    let src_path = opts.dest_path.join("src");

    let model_rs = generate_model_rs(mir_spec, config);
    write_rust_file_to_path(&src_path.join("model.rs"), model_rs)?;
    fs::create_dir_all(src_path.join("model"))?;
    for (name, record) in &mir_spec.schemas {
        let file = generate_single_model_file(name, record, mir_spec, config);
        let name = name.to_filename();
        write_rust_file_to_path(&src_path.join("model").join(name).with_extension("rs"), file)?;
    }
    Ok(())
}

/// Generates the client code for a given OpenAPI specification.
fn write_lib_rs(mir_spec: &MirSpec, extras: &Extras, spec: &OpenAPI, opts: &OutputOptions) -> Result<()> {
    let src_path = opts.dest_path.join("src");
    let name = &opts.library_options.service_name;
    let mut struct_Client = client::struct_Client(mir_spec, &opts.library_options);
    let impl_Client = client::impl_Client(mir_spec, spec, &opts.library_options);

    let security = if mir_spec.has_security() {
        let struct_ServiceAuthentication = client::struct_Authentication(mir_spec, &opts.library_options);
        let impl_ServiceAuthentication = client::impl_Authentication(mir_spec, spec, &opts.library_options);
        quote! {
            #struct_ServiceAuthentication
            #impl_ServiceAuthentication
        }
    } else {
        quote! {}
    };

    let client_name = struct_Client.name.to_string();
    let template_path = opts.dest_path.join("template").join("src").join("lib.rs");
    let lib_rs_template = if template_path.exists() {
        fs::read_to_string(template_path)?
    } else {
        let s = get_template_file("rust/lib.rs");
        formatdoc!(
            r#"
            //! [`{client}`](struct.{client}.html) is the main entry point for this library.
            //!
            //! Library created with [`libninja`](https://www.libninja.com).
            {s}"#,
            client = client_name
        )
    };
    let template_has_from_env = lib_rs_template.contains("from_env");

    if template_has_from_env {
        struct_Client.class_methods.retain(|m| m.name.0 != "from_env");
    }
    let struct_Client = struct_Client.to_rust_code();
    let serde = if extras.needs_serde() {
        quote! {
            mod serde
        }
    } else {
        TokenStream::new()
    };
    let code = quote! {
        #serde;
        #struct_Client
        #impl_Client
        #security
    };

    io::write_rust_to_path(&src_path.join("lib.rs"), code, &lib_rs_template)?;
    Ok(())
}


fn write_request_module(spec: &MirSpec, opts: &OutputOptions) -> Result<()> {
    let src_path = opts.dest_path.join("src");
    let client_name = opts.library_options.client_name().to_rust_struct();
    let mut imports = vec![];
    fs::create_dir_all(src_path.join("request"))?;
    let mut modules = vec![];
    for operation in &spec.operations {
        let fname = operation.file_name();
        let request_structs = build_request_struct(operation, spec, &opts.library_options);
        let struct_name = request_structs[0].name.clone();
        let response = operation.ret.to_rust_type();
        let struct_names = request_structs.iter().map(|s| s.name.to_string()).collect::<Vec<_>>();
        let request_structs = request_structs.into_iter().map(|s| s.to_rust_code()).collect::<Vec<_>>();
        modules.push(fname.clone());
        let mut import = Import::new(&fname, struct_names);
        import.vis = Visibility::Public;
        imports.push(import);
        let file = quote! {
            use crate::#client_name;
            #(#request_structs)*

            impl<'a> ::std::future::IntoFuture for #struct_name<'a> {
                type Output = httpclient::InMemoryResult<#response>;
                type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;

                fn into_future(self) -> Self::IntoFuture {
                    Box::pin(self.send())
                }
            }
        };
        io::write_rust_to_path(&src_path.join(format!("request/{}.rs", fname)), file, "use serde_json::json;
use crate::model::*;")?;
    }
    let file = File {
        imports,
        ..File::default()
    }.to_rust_code();
    let modules = modules.iter().map(|m| format!("pub mod {};", m)).collect::<Vec<_>>().join("\n");
    io::write_rust_to_path(&src_path.join("request.rs"), file, &modules)?;
    Ok(())
}

fn bump_version_and_update_deps(extras: &Extras, opts: &OutputOptions) -> anyhow::Result<()> {
    let cargo = opts.dest_path.join("Cargo.toml");

    let mut manifest = cargo_toml::Manifest::from_path(&cargo)?;
    let package = manifest.package.as_mut().expect("Cargo.toml does not have a package section. You might have set the output-dir to a workspace directory.");

    package.version = cargo_toml::Inheritable::Set(opts.library_options.package_version.clone());

    let template_manifest = cargo_toml::Manifest::from_str(get_template_file("rust/Cargo.toml.j2")).unwrap();
    bump_deps(&mut manifest, &template_manifest)?;
    if extras.currency {
        manifest.dependencies.entry("rust_decimal".to_string())
            .or_insert(cargo_toml::Dependency::Detailed(cargo_toml::DependencyDetail {
                version: Some("1.28.1".to_string()),
                features: vec!["serde".to_string()],
                ..cargo_toml::DependencyDetail::default()
            }));
    }
    if extras.date_serialization {
        manifest.dependencies.entry("chrono".to_string())
            .or_insert(cargo_toml::Dependency::Detailed(cargo_toml::DependencyDetail {
                version: Some("0.4.23".to_string()),
                features: vec!["serde".to_string()],
                ..cargo_toml::DependencyDetail::default()
            }));
    }

    let content = toml::to_string(&manifest).unwrap();
    fs::write_file(&cargo, &content)
}

fn bump_deps(current_manifest: &mut cargo_toml::Manifest, from_other: &cargo_toml::Manifest) -> Result<()> {
    for (name, other_dep) in &from_other.dependencies {
        let dep = current_manifest.dependencies.entry(name.clone()).or_insert_with(|| other_dep.clone());
        let current = semver::Version::parse(dep.req()).unwrap();
        let other = semver::Version::parse(other_dep.req()).unwrap();
        if current < other {
            dep.detail_mut().version = Some(other.to_string());
        }
    }
    Ok(())
}

fn write_examples(spec: &MirSpec, opts: &OutputOptions) -> Result<String> {
    let example_path = opts.dest_path.join("examples");

    fs::remove_dir_all(&example_path)?;
    fs::create_dir_all(&example_path)?;
    let mut first_example = None;
    for operation in &spec.operations {
        let mut source = generate_example(operation, &opts.library_options, spec)?;
        if first_example.is_none() {
            first_example = Some(source.clone());
        }
        source.insert_str(0, "#![allow(unused_imports)]\n");
        fs::write_file(&example_path.join(operation.file_name()).with_extension("rs"), &source)?;
    }
    first_example.ok_or_else(|| anyhow::anyhow!("No examples were generated."))
}

fn write_serde_module_if_needed(extras: &Extras, opts: &OutputOptions) -> Result<()> {
    let src_path = opts.dest_path.join("src").join("serde.rs");

    if !extras.needs_serde() {
        return Ok(());
    }

    let null_as_zero = if extras.null_as_zero {
        serde::option_i64_null_as_zero_module()
    } else {
        TokenStream::new()
    };

    let date_as_int = if extras.date_serialization {
        serde::option_chrono_naive_date_as_int_module()
    } else {
        TokenStream::new()
    };

    let code = quote! {
        #null_as_zero
        #date_as_int
    };
    let code = format_code(code).unwrap();
    fs::write_file(&src_path, &code)
}