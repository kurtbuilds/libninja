use std::hash::Hash;
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
use ::mir::{Visibility, Import, File};
use ln_core::fs;
use hir::{HirSpec, IntegerSerialization, DateSerialization};

use crate::{add_operation_models, extract_spec, LibraryOptions, OutputOptions, util};
use crate::rust::client::build_Client_authenticate;
pub use crate::rust::codegen::generate_example;
use crate::rust::codegen::{codegen_function, sanitize_filename, ToRustCode};
use crate::rust::io::write_rust_file_to_path;
use crate::rust::lower_mir::{generate_model_rs, generate_single_model_file};
use crate::rust::request::{build_request_struct, build_request_struct_builder_methods, build_url, generate_request_model_rs};

pub mod client;
pub mod codegen;
pub mod format;
pub mod lower_mir;
pub mod request;
mod io;
mod serde;

#[derive(Debug)]
pub struct Extras {
    null_as_zero: bool,
    option_i64_str: bool,
    date_serialization: bool,
    currency: bool,
    integer_date_serialization: bool,
    basic_auth: bool
}

impl Extras {
    pub fn needs_serde(&self) -> bool {
        self.null_as_zero || self.date_serialization
    }
}

pub fn calculate_extras(spec: &HirSpec) -> Extras {
    use hir::Ty;
    let mut null_as_zero = false;
    let mut date_serialization = false;
    let mut currency = false;
    let mut integer_date_serialization = false;
    let mut option_i64_str = false;
    for (_, record) in &spec.schemas {
        for field in record.fields() {
            match &field.ty {
                Ty::Integer { serialization: IntegerSerialization::NullAsZero } => {
                    null_as_zero = true;
                }
                Ty::Integer { serialization: IntegerSerialization::String } => {
                    option_i64_str = true;
                }
                Ty::Date { serialization: DateSerialization::Integer } => {
                    integer_date_serialization = true;
                    date_serialization = true;
                }
                Ty::DateTime => {
                    date_serialization = true;
                }
                Ty::Currency { .. } => {
                    currency = true;
                }
                _ => {}
            }
        }
    }
    let basic_auth = spec.security.iter().any(|f| f.fields.iter().any(|f| matches!(f.location, hir::AuthLocation::Basic)));
    Extras {
        null_as_zero,
        date_serialization,
        integer_date_serialization,
        currency,
        option_i64_str,
        basic_auth,
    }
}


pub fn generate_rust_library(spec: OpenAPI, opts: OutputOptions) -> Result<()> {
    let config = &opts.library_options.config;
    let src_path = opts.dest_path.join("src");
    // Ignore failure
    let _ = fs::remove_dir_all(&src_path);
    fs::create_dir_all(&src_path)?;

    // Prepare the MIR Spec.
    let hir_spec = extract_spec(&spec)?;
    let hir_spec = add_operation_models(opts.library_options.language, hir_spec)?;
    let extras = calculate_extras(&hir_spec);

    write_model_module(&hir_spec, &opts)?;

    write_request_module(&hir_spec, &opts)?;

    write_lib_rs(&hir_spec, &extras, &spec, &opts)?;

    write_serde_module_if_needed(&extras, &opts)?;

    let tera = prepare_templates();
    let mut context = create_context(&opts, &hir_spec);

    if opts.library_options.build_examples {
        let example = write_examples(&hir_spec, &opts)?;
        context.insert("code_sample", &example);
    } else {
        context.insert("code_sample", "// Examples were skipped. Run libninja with `--examples true` flag to create them.");
    }

    context.insert("client_docs_url", &format!("https://docs.rs/{}", opts.library_options.package_name));

    copy_files(&opts.dest_path, &opts.library_options.language.to_string(), &["src"])?;
    copy_templates(&opts, &tera, &context)?;

    bump_version_and_update_deps(&extras, &opts)?;

    Ok(())
}

fn write_model_module(mir_spec: &HirSpec, opts: &OutputOptions) -> Result<()> {
    let config = &opts.library_options.config;
    let src_path = opts.dest_path.join("src");

    let model_rs = generate_model_rs(mir_spec, config);
    write_rust_file_to_path(&src_path.join("model.rs"), model_rs)?;
    fs::create_dir_all(src_path.join("model"))?;
    for (name, record) in &mir_spec.schemas {
        let file = generate_single_model_file(name, record, mir_spec, config);
        let name = sanitize_filename(name);
        write_rust_file_to_path(&src_path.join("model").join(name).with_extension("rs"), file)?;
    }
    Ok(())
}

/// Generates the client code for a given OpenAPI specification.
fn write_lib_rs(mir_spec: &HirSpec, extras: &Extras, spec: &OpenAPI, opts: &OutputOptions) -> Result<()> {
    let src_path = opts.dest_path.join("src");
    let name = &opts.library_options.service_name;
    let mut struct_Client = client::struct_Client(mir_spec, &opts.library_options);
    let impl_Client = client::impl_Client(mir_spec, spec, &opts.library_options);

    let client_name = struct_Client.name.clone();
    let template_path = opts.dest_path.join("template").join("src").join("lib.rs");
    dbg!(&template_path);
    let lib_rs_template = if template_path.exists() {
        fs::read_to_string(template_path)?
    } else {
        let s = get_template_file("rust/src/lib.rs");
        formatdoc!(
            r#"
            //! [`{client}`](struct.{client}.html) is the main entry point for this library.
            //!
            //! Library created with [`libninja`](https://www.libninja.com).
            {s}
            "#,
            client = client_name.0
        )
    };
    let template_has_from_env = lib_rs_template.contains("from_env");

    if template_has_from_env {
        struct_Client.class_methods.retain(|m| m.name.0 != "from_env");
    }
    let struct_Client = struct_Client.to_rust_code();

    let serde = extras.needs_serde().then(|| {
        quote! {
            mod serde;
        }
    }).unwrap_or_default();

    let fluent_request = quote! {
        pub struct FluentRequest<'a, T> {
            pub(crate) client: &'a #client_name,
            pub params: T,
        }
    };
    let base64_import = extras.basic_auth.then(|| {
        quote! {
            use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
        }
    }).unwrap_or_default();

    let security = mir_spec.has_security().then(|| {
        let struct_ServiceAuthentication = client::struct_Authentication(mir_spec, &opts.library_options);
        let impl_ServiceAuthentication = (!template_has_from_env).then(|| {
            client::impl_Authentication(mir_spec, spec, &opts.library_options)
        }).unwrap_or_default();

        quote! {
            #struct_ServiceAuthentication
            #impl_ServiceAuthentication
        }
    }).unwrap_or_default();

    let code = quote! {
        #base64_import
        #serde
        #fluent_request
        #struct_Client
        #impl_Client
        #security
    };
    io::write_rust_to_path(&src_path.join("lib.rs"), code, &lib_rs_template)?;
    Ok(())
}


fn write_request_module(spec: &HirSpec, opts: &OutputOptions) -> Result<()> {
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
        let method = syn::Ident::new(&operation.method, proc_macro2::Span::call_site());
        let struct_names = request_structs.iter().map(|s| s.name.to_string()).collect::<Vec<_>>();
        let request_structs = request_structs.into_iter().map(|s| s.to_rust_code()).collect::<Vec<_>>();
        let url = build_url(&operation);
        modules.push(fname.clone());
        let mut import = Import::new(&fname, struct_names);
        import.vis = Visibility::Public;
        imports.push(import);
        let builder_methods = build_request_struct_builder_methods(&operation);
        let builder_methods = builder_methods
            .into_iter()
            .map(|s| codegen_function(s, quote! { mut self , }));
        let file = quote! {
            use crate::#client_name;
            #(#request_structs)*

            impl FluentRequest<'_, #struct_name> {
                #(#builder_methods)*
            }

            impl<'a> ::std::future::IntoFuture for FluentRequest<'a, #struct_name> {
                type Output = httpclient::InMemoryResult<#response>;
                type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;

                fn into_future(self) -> Self::IntoFuture {
                    Box::pin(async {
                        let url = #url;
                        let mut r = self.client.client.#method(url);
                        r = r.set_query(self.params);
                        r = self.client.authenticate(r);
                        let res = r.await?;
                        res.json().map_err(Into::into)
                    })
                }
            }
        };
        let template = "\
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;";
        io::write_rust_to_path(&src_path.join(format!("request/{}.rs", fname)), file, template)?;
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
                version: Some("1.33".to_string()),
                features: vec!["serde-with-str".to_string()],
                ..cargo_toml::DependencyDetail::default()
            }));
        manifest.dependencies.entry("rust_decimal_macros".to_string())
            .or_insert(cargo_toml::Dependency::Simple("1.33".to_string()));
    }
    if extras.date_serialization {
        manifest.dependencies.entry("chrono".to_string())
            .or_insert(cargo_toml::Dependency::Detailed(cargo_toml::DependencyDetail {
                version: Some("0.4.23".to_string()),
                features: vec!["serde".to_string()],
                default_features: true,
                ..cargo_toml::DependencyDetail::default()
            }));
    }
    if opts.library_options.config.ormlite {
        manifest.dependencies.entry("ormlite".to_string())
            .or_insert(cargo_toml::Dependency::Detailed(cargo_toml::DependencyDetail {
                version: Some("0.16.0".to_string()),
                features: vec!["decimal".to_string()],
                ..cargo_toml::DependencyDetail::default()
            }));
    }
    if extras.basic_auth {
        manifest.dependencies.entry("base64".to_string())
            .or_insert(cargo_toml::Dependency::Simple("0.21.0".to_string()));
    }
    // delete any examples that no longer exist
    manifest.example.retain(|e| {
        let Some(p) = &e.path else { return true; };
        opts.dest_path.join("examples").join(p).exists()
    });
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

fn write_examples(spec: &HirSpec, opts: &OutputOptions) -> Result<String> {
    let example_path = opts.dest_path.join("examples");

    let _ = fs::remove_dir_all(&example_path);
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

    let date_as_int = if extras.integer_date_serialization {
        serde::option_chrono_naive_date_as_int_module()
    } else {
        TokenStream::new()
    };

    let int_as_str = if extras.option_i64_str {
        serde::option_i64_str_module()
    } else {
        TokenStream::new()
    };

    let code = quote! {
        pub use ::serde::*;
        #null_as_zero
        #date_as_int
        #int_as_str
    };
    let code = format_code(code).unwrap();
    fs::write_file(&src_path, &code)
}
