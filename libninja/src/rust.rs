use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::thread::current;

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use indoc::formatdoc;
use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use text_io::read;
use tracing::debug;

use ::mir::{File, Import, Visibility};
use codegen::ToRustType;
use hir::{qualified_env_var, AuthStrategy, HirSpec, Location, Oauth2Auth, Parameter};
use ln_core::fs;
use ln_core::{
    copy_builtin_files, copy_builtin_templates, create_context, get_template_file,
    prepare_templates,
};
use mir::Ident;
use mir::{DateSerialization, IntegerSerialization};
use mir_rust::format_code;
use mir_rust::ToRustIdent;
use mir_rust::{sanitize_filename, ToRustCode};

use crate::rust::client::{build_Client_authenticate, server_url};
pub use crate::rust::codegen::generate_example;
use crate::rust::io::write_rust_file_to_path;
use crate::rust::lower_hir::{generate_model_rs, generate_single_model_file};
use crate::rust::request::{
    assign_inputs_to_request, build_request_struct, build_request_struct_builder_methods,
    build_url, generate_request_model_rs,
};
use crate::{add_operation_models, extract_spec, OutputConfig, PackageConfig};

mod cargo_toml;
pub mod client;
pub mod codegen;
pub mod format;
mod io;
pub mod lower_hir;
pub mod request;
mod serde;

#[derive(Debug)]
pub struct Extras {
    null_as_zero: bool,
    option_i64_str: bool,
    date_serialization: bool,
    currency: bool,
    integer_date_serialization: bool,
    basic_auth: bool,
    oauth2: bool,
}

impl Extras {
    pub fn needs_serde(&self) -> bool {
        self.null_as_zero || self.date_serialization
    }
}

pub fn calculate_extras(spec: &HirSpec) -> Extras {
    use mir::Ty;
    let mut null_as_zero = false;
    let mut date_serialization = false;
    let mut currency = false;
    let mut integer_date_serialization = false;
    let mut option_i64_str = false;
    for (_, record) in &spec.schemas {
        for field in record.fields() {
            match &field.ty {
                Ty::Integer {
                    serialization: IntegerSerialization::NullAsZero,
                } => {
                    null_as_zero = true;
                }
                Ty::Integer {
                    serialization: IntegerSerialization::String,
                } => {
                    option_i64_str = true;
                }
                Ty::Date {
                    serialization: DateSerialization::Integer,
                } => {
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
    let basic_auth = spec.has_basic_auth();
    let oauth2 = spec.oauth2_auth().is_some();
    Extras {
        null_as_zero,
        date_serialization,
        integer_date_serialization,
        currency,
        option_i64_str,
        basic_auth,
        oauth2,
    }
}

pub fn copy_from_target_templates(dest: &Path) -> Result<()> {
    let template_path = dest.join("template");
    if !template_path.exists() {
        return Ok(());
    }
    for path in ignore::Walk::new(&template_path) {
        let path: ignore::DirEntry = path?;
        let rel_path = path.path().strip_prefix(&template_path)?;
        if path
            .file_type()
            .expect(&format!("Failed to read file: {}", path.path().display()))
            .is_file()
        {
            let dest = dest.join(rel_path);
            if dest.exists() {
                continue;
            }
            fs::create_dir_all(dest.parent().unwrap())?;
            //copy the file
            std::fs::copy(&path.path(), &dest)?;
        }
    }
    Ok(())
}

pub fn generate_rust_library(spec: OpenAPI, opts: OutputConfig) -> Result<()> {
    let src_path = opts.dest_path.join("src");

    // Prepare the HIR Spec.
    let spec = extract_spec(&spec)?;
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
    let opts = PackageConfig {
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

fn write_file_with_template(
    mut file: File<TokenStream>,
    template: Option<String>,
    path: &Path,
) -> Result<()> {
    let Some(template) = template else {
        return write_rust_file_to_path(path, file);
    };
    // Write things in this order
    // file.doc
    // file.imports
    // template.imports
    // template.defs
    // file.defs
    // let template = syn::parse_file(&template)?;
    if template.contains("libninja: static") {
        debug!("Writing file from template/ as-is: {}", path.display());
        return fs::write_file(path, &template);
    }
    let doc = std::mem::take(&mut file.doc).to_rust_code();
    let imports = std::mem::take(&mut file.imports)
        .into_iter()
        .filter(|i| !template.contains(&i.path))
        .map(|i| i.to_rust_code());
    let pre = quote! {
        #doc
        #(#imports)*
    };
    let mut code = format_code(pre);
    code.push('\n');
    code += template.trim();
    code.push('\n');
    let after = file.to_rust_code();
    code += &format_code(after);
    fs::write_file(path, &code)
}

fn write_model_module(spec: &HirSpec, opts: &PackageConfig) -> Result<()> {
    let config = &opts.config;
    let src_path = opts.dest.join("src");

    let model_rs = generate_model_rs(spec, config);
    write_rust_file_to_path(&src_path.join("model.rs"), model_rs)?;
    fs::create_dir_all(src_path.join("model"))?;
    for (name, record) in &spec.schemas {
        let file = generate_single_model_file(name, record, spec, opts);
        let name = sanitize_filename(name);
        let dest = src_path.join("model").join(&name).with_extension("rs");
        write_file_with_template(
            file,
            opts.get_file_template(&format!("src/model/{}.rs", name)),
            &dest,
        )?;
    }
    Ok(())
}

fn static_shared_http_client(spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let url = server_url(spec, opt);
    quote! {
        static SHARED_HTTPCLIENT: OnceLock<httpclient::Client> = OnceLock::new();

        pub fn default_http_client() -> httpclient::Client {
            httpclient::Client::new()
                .base_url(#url)
        }

        /// Use this method if you want to add custom middleware to the httpclient.
        /// It must be called before any requests are made, otherwise it will have no effect.
        /// Example usage:
        ///
        /// ```
        /// init_http_client(default_http_client()
        ///     .with_middleware(..)
        /// );
        /// ```
        pub fn init_http_client(init: httpclient::Client) {
            let _ = SHARED_HTTPCLIENT.set(init);
        }

        fn shared_http_client() -> Cow<'static, httpclient::Client> {
            Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
        }
    }
}

fn shared_oauth2_flow(auth: &Oauth2Auth, spec: &HirSpec, opts: &PackageConfig) -> TokenStream {
    let service_name = opts.service_name.as_str();

    let client_id = qualified_env_var(service_name, "client id");
    let client_id_expect = format!("{} must be set", client_id);
    let client_secret = qualified_env_var(service_name, "client secret");
    let client_secret_expect = format!("{} must be set", client_secret);
    let redirect_uri = qualified_env_var(service_name, "redirect uri");
    let redirect_uri_expect = format!("{} must be set", redirect_uri);

    let init_endpoint = auth.auth_url.as_str();
    let exchange_endpoint = auth.exchange_url.as_str();
    let refresh_endpoint = auth.refresh_url.as_str();
    quote! {
        static SHARED_OAUTH2FLOW: OnceLock<httpclient_oauth2::OAuth2Flow> = OnceLock::new();

        pub fn init_oauth2_flow(init: httpclient_oauth2::OAuth2Flow) {
            let _ = SHARED_OAUTH2FLOW.set(init);
        }

        pub fn shared_oauth2_flow() -> &'static httpclient_oauth2::OAuth2Flow {
            SHARED_OAUTH2FLOW.get_or_init(|| httpclient_oauth2::OAuth2Flow {
                client_id: std::env::var(#client_id).expect(#client_id_expect),
                client_secret: std::env::var(#client_secret).expect(#client_secret_expect),
                init_endpoint: #init_endpoint.to_string(),
                exchange_endpoint: #exchange_endpoint.to_string(),
                refresh_endpoint: #refresh_endpoint.to_string(),
                redirect_uri: std::env::var(#redirect_uri).expect(#redirect_uri_expect),
            })
        }
    }
}

/// Generates the client code for a given OpenAPI specification.
fn write_lib_rs(spec: &HirSpec, extras: &Extras, opts: &PackageConfig) -> Result<()> {
    let src_path = opts.dest.join("src");
    let name = &opts.service_name;
    let mut struct_Client = client::struct_Client(spec, &opts);
    let impl_Client = client::impl_Client(spec, &opts);

    let client_name = struct_Client.name.clone();
    let lib_rs_template = opts.get_file_template("src/lib.rs").unwrap_or_else(|| {
        let s = get_template_file("rust/src/lib.rs");
        formatdoc!(
            r#"
            //! [`{client}`](struct.{client}.html) is the main entry point for this library.
            //!
            //! Library created with [`libninja`](https://www.libninja.com).
            "#,
            client = client_name.0
        ) + s
    });
    let template_has_from_env = lib_rs_template.contains("from_env");
    if template_has_from_env {
        struct_Client
            .class_methods
            .retain(|m| m.name.0 != "from_env");
    }
    let struct_Client = struct_Client.to_rust_code();

    let serde = extras
        .needs_serde()
        .then(|| {
            quote! {
                mod serde;
            }
        })
        .unwrap_or_default();

    let fluent_request = quote! {
        #[derive(Clone)]
        pub struct FluentRequest<'a, T> {
            pub(crate) client: &'a #client_name,
            pub params: T,
        }
    };
    let base64_import = extras
        .basic_auth
        .then(|| {
            quote! {
                use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
            }
        })
        .unwrap_or_default();

    let security = spec
        .has_security()
        .then(|| {
            let struct_ServiceAuthentication = client::struct_Authentication(spec, &opts);
            let impl_ServiceAuthentication = (!template_has_from_env)
                .then(|| client::impl_Authentication(spec, &opts))
                .unwrap_or_default();

            quote! {
                #struct_ServiceAuthentication
                #impl_ServiceAuthentication
            }
        })
        .unwrap_or_default();
    let static_shared_http_client = static_shared_http_client(spec, opts);
    let oauth = spec
        .security
        .iter()
        .filter_map(|s| match s {
            AuthStrategy::OAuth2(auth) => Some(auth),
            _ => None,
        })
        .next();
    let shared_oauth2_flow = oauth
        .map(|auth| shared_oauth2_flow(auth, spec, opts))
        .unwrap_or_default();

    let code = quote! {
        #base64_import
        #serde
        #static_shared_http_client
        #shared_oauth2_flow
        #fluent_request
        #struct_Client
        #impl_Client
        #security
    };
    io::write_rust_to_path(&src_path.join("lib.rs"), code, &lib_rs_template)?;
    Ok(())
}

fn write_request_module(spec: &HirSpec, opts: &PackageConfig) -> Result<()> {
    let src_path = opts.dest.join("src");
    let client_name = opts.client_name().to_rust_struct();
    let mut imports = vec![];
    fs::create_dir_all(src_path.join("request"))?;
    let mut modules = vec![];

    let authenticate = spec
        .has_security()
        .then(|| {
            quote! {
                r = self.client.authenticate(r);
            }
        })
        .unwrap_or_default();

    for operation in &spec.operations {
        let fname = operation.file_name();
        let request_structs = build_request_struct(operation, spec, &opts);
        let struct_name = request_structs[0].name.clone();
        let response = operation.ret.to_rust_type();
        let method = syn::Ident::new(&operation.method, proc_macro2::Span::call_site());
        let struct_names = request_structs
            .iter()
            .map(|s| s.name.to_string())
            .collect::<Vec<_>>();
        let request_structs = request_structs
            .into_iter()
            .map(|s| s.to_rust_code())
            .collect::<Vec<_>>();
        let url = build_url(&operation);
        modules.push(fname.clone());
        let mut import = Import::new(&fname, struct_names);
        import.vis = Visibility::Public;
        imports.push(import);
        let builder_methods = build_request_struct_builder_methods(&operation)
            .into_iter()
            .map(|s| s.to_rust_code());

        let assign_inputs = assign_inputs_to_request(&operation.parameters);

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
                    Box::pin(async move {
                        let url = #url;
                        let mut r = self.client.client.#method(url);
                        #assign_inputs
                        #authenticate
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
        io::write_rust_to_path(
            &src_path.join(format!("request/{}.rs", fname)),
            file,
            template,
        )?;
    }
    let file = File {
        imports,
        ..File::default()
    }
    .to_rust_code();
    let modules = modules
        .iter()
        .map(|m| format!("pub mod {};", m))
        .collect::<Vec<_>>()
        .join("\n");
    io::write_rust_to_path(&src_path.join("request.rs"), file, &modules)?;
    Ok(())
}

fn write_examples(spec: &HirSpec, opts: &PackageConfig) -> Result<()> {
    let example_path = opts.dest.join("examples");
    let _ = fs::remove_dir_all(&example_path);
    fs::create_dir_all(&example_path)?;
    for operation in &spec.operations {
        let mut source = generate_example(operation, &opts, spec)?;
        source.insert_str(0, "#![allow(unused_imports)]\n");
        fs::write_file(
            &example_path
                .join(operation.file_name())
                .with_extension("rs"),
            &source,
        )?;
    }
    Ok(())
}

fn write_serde_module_if_needed(extras: &Extras, dest: &Path) -> Result<()> {
    let src_path = dest.join("src").join("serde.rs");

    if !extras.needs_serde() {
        return Ok(());
    }

    let null_as_zero = extras
        .null_as_zero
        .then(serde::option_i64_null_as_zero_module)
        .unwrap_or_default();

    let date_as_int = extras
        .integer_date_serialization
        .then(serde::option_chrono_naive_date_as_int_module)
        .unwrap_or_default();

    let int_as_str = extras
        .option_i64_str
        .then(serde::option_i64_str_module)
        .unwrap_or_default();

    let code = quote! {
        pub use ::serde::*;
        #null_as_zero
        #date_as_int
        #int_as_str
    };
    let code = format_code(code);
    fs::write_file(&src_path, &code)
}
