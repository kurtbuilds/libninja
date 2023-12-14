use convert_case::{Case, Casing};
use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use hir::{AuthLocation, AuthorizationStrategy, DocFormat, Location, Parameter, ServerStrategy, Doc, HirSpec, Language, Operation};
use mir::{field, Function, Ident};
use mir::{Class, Field, FnArg, Visibility};
use ln_core::PackageConfig;

use crate::rust::codegen::ToRustCode;
use crate::rust::codegen::ToRustIdent;
use crate::rust::codegen::ToRustType;

fn build_Client_from_env(spec: &HirSpec, opt: &PackageConfig) -> Function<TokenStream> {
    let declare_url = match spec.server_strategy() {
        ServerStrategy::Single(url) => quote! {
            .base_url(#url)
        },
        ServerStrategy::Env => {
            let var = opt.env_var("env").0;
            let error = format!("Missing environment variable {}", var);
            quote! {
                .base_url(std::env::var(#var).expect(#error).as_str())
            }
        }
        ServerStrategy::BaseUrl => {
            let var = opt.env_var("base_url").0;
            let error = format!("Missing environment variable {}", var);
            quote! {
                .base_url(std::env::var(#var).expect(#error).as_str())
            }
        }
    };

    let auth_struct = opt.authenticator_name().to_rust_struct();
    let body = if spec.has_security() {
        let auth_struct = opt.authenticator_name().to_rust_struct();
        quote! {
            Self {
                client: httpclient::Client::new()#declare_url,
                authentication: #auth_struct::from_env(),
            }
        }
    } else {
        quote! {
            Self {
                client: httpclient::Client::new()#declare_url
            }
        }
    };

    Function {
        name: Ident::new("from_env"),
        public: true,
        ret: quote!(Self),
        body,
        ..Function::default()
    }
}

pub fn struct_Client(mir_spec: &HirSpec, opt: &PackageConfig) -> Class<TokenStream> {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();

    let mut instance_fields = vec![
        field!(pub client: quote!(httpclient::Client)),
    ];
    if mir_spec.has_security() {
        instance_fields.push(field!(authentication: quote!(#auth_struct_name)));
    }

    let class_methods = vec![build_Client_from_env(mir_spec, opt)];
    Class {
        name: opt.client_name().to_rust_struct(),
        instance_fields,
        class_methods,
        public: true,
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

pub fn authenticate_variant(
    req: &AuthorizationStrategy,
    opt: &PackageConfig,
) -> TokenStream {
    let auth_struct = opt.authenticator_name().to_rust_struct();

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

pub fn build_Client_authenticate(mir_spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let authenticate_variant = mir_spec.security
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

fn build_new_fn(security: bool, opt: &PackageConfig) -> TokenStream {
    if security {
        let auth_struct_name = opt.authenticator_name().to_rust_struct();
        quote! {
            pub fn new(url: &str, authentication: #auth_struct_name) -> Self {
                let client = httpclient::Client::new()
                    .base_url(url);
                Self {
                    client,
                    authentication,
                }
            }
        }
    } else {
        quote! {
            pub fn new(url: &str) -> Self {
                let client = httpclient::Client::new()
                    .base_url(url);
                Self {
                    client
                }
            }
        }
    }
}

pub fn impl_Client(mir_spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let client_struct_name = opt.client_name().to_rust_struct();
    let path_fns = impl_ServiceClient_paths(mir_spec);

    let security = mir_spec.has_security();
    let new_fn = build_new_fn(security, opt);
    let authenticate = if security {
        build_Client_authenticate(mir_spec, opt)
    } else {
        TokenStream::new()
    };
    let with_authentication = if security {
        let auth_struct_name = opt.authenticator_name().to_rust_struct();
        quote! {
            pub fn with_authentication(mut self, authentication: #auth_struct_name) -> Self {
                self.authentication = authentication;
                self
            }
        }
    } else {
        TokenStream::new()
    };

    quote! {
        impl #client_struct_name {
            #new_fn
            #with_authentication
            #authenticate

            pub fn with_middleware<M: httpclient::Middleware + 'static>(mut self, middleware: M) -> Self {
                self.client = self.client.with_middleware(middleware);
                self
            }

            #(#path_fns)*
        }
    }
}

pub fn struct_Authentication(mir_spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();

    let variants = mir_spec.security.iter().map(|strategy| {
        let variant_name = strategy.name.to_rust_struct();
        let args = strategy.fields.iter().map(|f| f.name.to_rust_ident());
        quote! {
            #variant_name {
                #(#args: String),*
            }
        }
    });
    quote! {
        pub enum #auth_struct_name {
            #(#variants),*
        }
    }
}

fn build_Authentication_from_env(hir_spec: &HirSpec, service_name: &str) -> TokenStream {
    let first_variant = hir_spec.security.first()
        .unwrap();
    let fields = first_variant
        .fields
        .iter()
        .map(|f| {
            let basic = matches!(f.location, AuthLocation::Basic);
            let field =
                syn::Ident::new(&f.name.to_case(Case::Snake), proc_macro2::Span::call_site());
            let expect = format!("Environment variable {} is not set.", f.env_var);
            let env_var = &f.env_var_for_service(service_name);
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
        &first_variant.name.to_case(Case::Pascal),
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

pub fn impl_Authentication(mir_spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();
    let from_env = build_Authentication_from_env(mir_spec, &opt.service_name);

    quote! {
        impl #auth_struct_name {
            #from_env
        }
    }
}
