use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use hir::{
    AuthLocation, AuthStrategy, HirSpec, Language, Operation, qualified_env_var
    , ServerStrategy,
};
use ln_core::PackageConfig;
use mir::{Class, Field, Visibility};
use mir::{FnArg2, Function, Ident};
use mir_rust::{ToRustCode, ToRustIdent};

use crate::rust::codegen::ToRustType;

pub fn server_url(spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    match spec.server_strategy() {
        ServerStrategy::Single(url) => quote!(#url),
        ServerStrategy::Env => {
            let var = qualified_env_var(&opt.service_name, "env");
            let error = format!("Missing environment variable {}", var);
            quote!(std::env::var(#var).expect(#error).as_str())
        }
        ServerStrategy::BaseUrl => {
            let var = qualified_env_var(&opt.service_name, "base_url");
            let error = format!("Missing environment variable {}", var);
            quote!(std::env::var(#var).expect(#error).as_str())
        }
    }
}

fn build_Client_from_env(spec: &HirSpec, opt: &PackageConfig) -> Function<TokenStream> {
    let auth_struct = opt.authenticator_name().to_rust_struct();
    let body = if spec.has_security() {
        let auth_struct = opt.authenticator_name().to_rust_struct();
        quote! {
            Self {
                client: shared_http_client(),
                authentication: #auth_struct::from_env(),
            }
        }
    } else {
        quote! {
            Self {
                client: shared_http_client()
            }
        }
    };

    Function {
        name: Ident::new("from_env"),
        vis: Visibility::Public,
        ret: quote!(Self),
        body,
        ..Function::default()
    }
}

fn build_Client_with_auth(spec: &HirSpec, opt: &PackageConfig) -> Function<TokenStream> {
    let auth_struct = opt.authenticator_name().to_rust_struct();
    let body = quote! {
        Self {
            client: shared_http_client(),
            authentication
        }
    };
    Function {
        name: Ident::new("with_auth"),
        vis: Visibility::Public,
        ret: quote!(Self),
        body,
        args: vec![FnArg2::Basic {
            name: Ident("authentication".to_string()),
            ty: quote!(#auth_struct),
            default: None,
        }],
        ..Function::default()
    }
}

fn build_Client_new_with(spec: &HirSpec, opt: &PackageConfig) -> Function<TokenStream> {
    let auth_struct = opt.authenticator_name().to_rust_struct();
    let body = quote! {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    };
    Function {
        name: Ident::new("new_with"),
        vis: Visibility::Public,
        ret: quote!(Self),
        body,
        args: vec![
            FnArg2::Basic {
                name: Ident("client".to_string()),
                ty: quote!(httpclient::Client),
                default: None,
            },
            FnArg2::Basic {
                name: Ident("authentication".to_string()),
                ty: quote!(#auth_struct),
                default: None,
            },
        ],
        ..Function::default()
    }
}

pub fn struct_Client(spec: &HirSpec, opt: &PackageConfig) -> Class<TokenStream> {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();

    let mut instance_fields = vec![Field {
        name: Ident::new("client"),
        ty: quote!(Cow<'static, httpclient::Client>),
        ..Field::default()
    }];
    if spec.has_security() {
        instance_fields.push(Field {
            name: Ident::new("authentication"),
            ty: quote!(#auth_struct_name),
            ..Field::default()
        });
    }

    let mut class_methods = vec![build_Client_from_env(spec, opt)];
    if spec.has_security() {
        class_methods.push(build_Client_with_auth(spec, opt));
    } else {
        class_methods.push(Function {
            name: Ident::new("new"),
            vis: Visibility::Public,
            ret: quote!(Self),
            body: quote! {
                Self {
                    client: shared_http_client()
                }
            },
            ..Function::default()
        });
    }
    class_methods.push(build_Client_new_with(spec, opt));
    Class {
        name: opt.client_name().to_rust_struct(),
        instance_fields,
        class_methods,
        vis: Visibility::Public,
        ..Class::default()
    }
}

pub fn build_api_client_method(operation: &Operation) -> TokenStream {
    let use_struct = operation.use_required_struct(Language::Rust);

    let fn_args = if use_struct {
        let arg_struct = operation.required_struct_name().to_rust_struct();
        vec![quote!(args: request::#arg_struct)]
    } else {
        operation
            .parameters
            .iter()
            .filter(|param| !param.optional)
            .map(|param| {
                let k = param.name.to_rust_ident();
                let arg_type = param.ty.to_reference_type(TokenStream::new());
                quote!(#k: #arg_type)
            })
            .collect()
    };

    let struct_field_values: Vec<TokenStream> = operation
        .parameters
        .iter()
        .map(|param| {
            let name = param.name.to_rust_ident();
            if param.optional {
                quote!(#name: None)
            } else if param.ty.is_reference_type() {
                let iterable = param.ty.is_iterable();
                let mut value = if iterable {
                    quote!(#name.iter().map(|&x| x.to_owned()).collect())
                } else {
                    quote!(#name.to_owned())
                };
                if use_struct {
                    value = quote!(args.#value)
                }
                quote!(#name: #value)
            } else if use_struct {
                quote!(#name: args.#name)
            } else {
                quote!(#name)
            }
        })
        .collect();

    let doc = operation.doc.clone().to_rust_code();
    let request_struct = operation.request_struct_name().to_rust_struct();
    let name = &operation.name.to_rust_ident();
    quote! {
        #doc
        pub fn #name(&self, #(#fn_args),*) -> FluentRequest<'_, request::#request_struct> {
            FluentRequest {
                client: self,
                params: request::#request_struct {
                    #(#struct_field_values,)*
                }
            }
        }
    }
}

pub fn impl_ServiceClient_paths(spec: &HirSpec) -> Vec<TokenStream> {
    let mut result = vec![];
    for operation in &spec.operations {
        result.push(build_api_client_method(operation));
    }
    result
}

pub fn authenticate_variant(req: &AuthStrategy, opt: &PackageConfig) -> TokenStream {
    let auth_struct = opt.authenticator_name().to_rust_struct();

    match req {
        AuthStrategy::Token(req) => {
            let variant_name = req.name.to_rust_struct();
            let fields = req
                .fields
                .iter()
                .map(|field| {
                    let field = syn::Ident::new(
                        &field.name.to_case(Case::Snake),
                        proc_macro2::Span::call_site(),
                    );
                    quote! { #field }
                })
                .collect::<Vec<_>>();

            let set_values = req
                .fields
                .iter()
                .map(|sec_field| {
                    let field = syn::Ident::new(
                        &sec_field.name.to_case(Case::Snake),
                        proc_macro2::Span::call_site(),
                    );
                    match &sec_field.location {
                        AuthLocation::Header { key } => quote! { r = r.header(#key, #field); },
                        AuthLocation::Basic => quote! { r = r.basic_auth(#field); },
                        AuthLocation::Bearer => quote! { r = r.bearer_auth(#field); },
                        AuthLocation::Token => quote! { r = r.token_auth(#field); },
                        AuthLocation::Query { key } => quote! { r = r.query(#key, #field); },
                        AuthLocation::Cookie { key } => quote! { r = r.cookie(#key, #field); },
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                #auth_struct::#variant_name { #(#fields,)*} => {
                    #(#set_values)*
                }
            }
        }
        AuthStrategy::OAuth2(_) => {
            quote! {
                #auth_struct::OAuth2 { middleware } => {
                    r.middlewares.insert(0, middleware.clone());
                }
            }
        }
        AuthStrategy::NoAuth => {
            quote! {
                #auth_struct::NoAuth => {}
            }
        }
    }
}

pub fn build_Client_authenticate(spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let authenticate_variant = spec
        .security
        .iter()
        .map(|req| authenticate_variant(req, opt))
        .collect::<Vec<_>>();

    quote! {
        pub(crate) fn authenticate<'a>(&self, mut r: httpclient::RequestBuilder<'a>) -> httpclient::RequestBuilder<'a> {
            match &self.authentication {
                #(#authenticate_variant,)*
            }
            r
        }
    }
}

pub fn impl_Client(spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let client_struct_name = opt.client_name().to_rust_struct();
    let path_fns = impl_ServiceClient_paths(spec);

    let security = spec.has_security();
    let authenticate = security
        .then(|| build_Client_authenticate(spec, opt))
        .unwrap_or_default();

    quote! {
        impl #client_struct_name {
            #authenticate
            #(#path_fns)*
        }
    }
}

pub fn struct_Authentication(mir_spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();

    let variants = mir_spec.security.iter().map(|strategy| match strategy {
        AuthStrategy::Token(strategy) => {
            let variant_name = strategy.name.to_rust_struct();
            let args = strategy.fields.iter().map(|f| f.name.to_rust_ident());
            quote! {
                #variant_name {
                    #(#args: String),*
                }
            }
        }
        AuthStrategy::OAuth2(_) => {
            quote! {
                OAuth2 { middleware: Arc<httpclient_oauth2::OAuth2> }
            }
        }
        AuthStrategy::NoAuth => {
            quote! {
                NoAuth
            }
        }
    });
    quote! {
        pub enum #auth_struct_name {
            #(#variants),*
        }
    }
}

fn build_Authentication_from_env(spec: &HirSpec, service_name: &str) -> TokenStream {
    let Some(strat) = spec.security.first() else {
        return TokenStream::new();
    };
    match strat {
        AuthStrategy::Token(strat) => {
            let fields = strat
                .fields
                .iter()
                .map(|f| {
                    let basic = matches!(f.location, AuthLocation::Basic);
                    let field = syn::Ident::new(
                        &f.name.to_case(Case::Snake),
                        proc_macro2::Span::call_site(),
                    );
                    let env_var = qualified_env_var(service_name, &f.name);
                    let expect = format!("Environment variable {} is not set.", env_var);
                    if basic {
                        quote! {
                            #field: {
                                let value = std::env::var(#env_var).expect(#expect);
                                STANDARD_NO_PAD.encode(value)
                            }
                        }
                    } else {
                        quote! {
                            #field: std::env::var(#env_var).expect(#expect)
                        }
                    }
                })
                .collect::<Vec<_>>();
            let variant_name = syn::Ident::new(
                &strat.name.to_case(Case::Pascal),
                proc_macro2::Span::call_site(),
            );
            quote! {
                pub fn from_env() -> Self {
                    Self::#variant_name {
                        #(#fields),*
                    }
                }
            }
        }
        AuthStrategy::NoAuth => {
            quote! {
                pub fn from_env() -> Self {
                    Self::NoAuth
                }
            }
        }
        AuthStrategy::OAuth2(_) => {
            let access = qualified_env_var(service_name, "access_token");
            let refresh = qualified_env_var(service_name, "refresh_token");
            quote! {
                pub fn from_env() -> Self {
                    let access = std::env::var(#access).unwrap();
                    let refresh = std::env::var(#refresh).unwrap();
                    let mw = shared_oauth2_flow().bearer_middleware(access, refresh);
                    Self::OAuth2 {
                        middleware: std::sync::Arc::new(mw),
                    }
                }
            }
        }
    }
}

pub fn impl_Authentication(spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();
    let from_env = build_Authentication_from_env(spec, &opt.service_name);
    let oauth2 = spec
        .oauth2_auth()
        .map(|oauth| {
            quote! {
                pub fn oauth2(access: String, refresh: String) -> Self {
                    let mw = shared_oauth2_flow().bearer_middleware(access, refresh);
                    Self::OAuth2 { middleware: Arc::new(mw) }
                }
            }
        })
        .unwrap_or_default();

    quote! {
        impl #auth_struct_name {
            #from_env
            #oauth2
        }
    }
}
