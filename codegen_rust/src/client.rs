#![allow(non_snake_case)]

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use crate::extras::Extras;
use hir::{qualified_env_var, AuthLocation, AuthStrategy, HirSpec, Language, Oauth2Auth, ServerStrategy};
use hir::{Config, Operation};
use libninja_macro::rfunction;
use mir::{import, Class, Field, File, Function, Ident, Item, Visibility};
use mir_rust::{ToRustCode, ToRustIdent, ToRustType};

/// Generates the client code for a given OpenAPI specification.
pub fn make_lib_rs(spec: &HirSpec, extras: &Extras, cfg: &Config) -> File<TokenStream> {
    let struct_Client = struct_Client(spec, &cfg);
    let impl_Client = impl_Client(spec, &cfg);

    let client_name = struct_Client.name.clone();

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
            let struct_ServiceAuthentication = struct_Authentication(spec, &cfg);
            let impl_ServiceAuthentication = impl_Authentication(spec, &cfg);
            quote! {
                #struct_ServiceAuthentication
                #impl_ServiceAuthentication
            }
        })
        .unwrap_or_default();
    let static_shared_http_client = static_shared_http_client(spec, cfg);
    let oauth = spec
        .security
        .iter()
        .filter_map(|s| match s {
            AuthStrategy::OAuth2(auth) => Some(auth),
            _ => None,
        })
        .next();
    let shared_oauth2_flow = oauth
        .map(|auth| shared_oauth2_flow(auth, spec, cfg))
        .unwrap_or_default();
    File {
        attributes: vec![],
        doc: None,
        imports: vec![
            import!(std::sync, OnceLock),
            import!(std::borrow, Cow),
            import!(httpclient, Client),
        ],
        items: vec![
            Item::Block(base64_import),
            Item::Block(serde),
            Item::Block(static_shared_http_client),
            Item::Block(shared_oauth2_flow),
            Item::Block(fluent_request),
            Item::Class(struct_Client),
            Item::Block(impl_Client),
            Item::Block(security),
        ],
    }
}

fn server_url(spec: &HirSpec, opt: &Config) -> TokenStream {
    match spec.server_strategy() {
        ServerStrategy::Single(url) => quote!(#url),
        ServerStrategy::Env => {
            let var = qualified_env_var(&opt.name, "env");
            let error = format!("Missing environment variable {}", var);
            quote!(std::env::var(#var).expect(#error).as_str())
        }
        ServerStrategy::BaseUrl => {
            let var = qualified_env_var(&opt.name, "base_url");
            let error = format!("Missing environment variable {}", var);
            quote!(std::env::var(#var).expect(#error).as_str())
        }
    }
}

fn build_Client_from_env(spec: &HirSpec, opt: &Config) -> Function<TokenStream> {
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
    rfunction!(pub from_env() -> Self).body(body)
}

fn build_Client_with_auth(_spec: &HirSpec, opt: &Config) -> Function<TokenStream> {
    let auth_struct = opt.authenticator_name().to_rust_struct();
    rfunction!(pub with_auth(authentication: #auth_struct) -> Self {
        Self {
            client: shared_http_client(),
            authentication
        }
    })
}

fn build_Client_new_with(_spec: &HirSpec, opt: &Config) -> Function<TokenStream> {
    let auth_struct = opt.authenticator_name().to_rust_struct();
    let body = quote! {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    };
    rfunction!(pub new_with(client: Client, authentication: #auth_struct)).body(body)
}

pub fn struct_Client(spec: &HirSpec, opt: &Config) -> Class<TokenStream> {
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

    let mut methods = vec![build_Client_from_env(spec, opt)];
    if spec.has_security() {
        methods.push(build_Client_with_auth(spec, opt));
    } else {
        methods.push(rfunction!(pub new() -> Self {
            Self {
                client: shared_http_client()
            }
        }));
    }
    if spec.has_security() {
        methods.push(build_Client_new_with(spec, opt));
    }
    Class {
        name: opt.client_name(),
        fields: instance_fields,
        methods,
        vis: Visibility::Public,
        imports: vec![],
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

pub fn authenticate_variant(req: &AuthStrategy, opt: &Config) -> TokenStream {
    let auth_struct = opt.authenticator_name().to_rust_struct();

    match req {
        AuthStrategy::Token(req) => {
            let variant_name = req.name.to_rust_struct();
            let fields = req
                .fields
                .iter()
                .map(|field| {
                    let field = field.name.to_rust_ident();
                    quote! { #field }
                })
                .collect::<Vec<_>>();

            let set_values = req
                .fields
                .iter()
                .map(|sec_field| {
                    let field = sec_field.name.to_rust_ident();
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

pub fn build_Client_authenticate(spec: &HirSpec, opt: &Config) -> TokenStream {
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

pub fn impl_Client(spec: &HirSpec, opt: &Config) -> TokenStream {
    let client_struct_name = opt.client_name();
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

pub fn struct_Authentication(mir_spec: &HirSpec, opt: &Config) -> TokenStream {
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

fn build_Authentication_from_env(spec: &HirSpec, name: &str) -> TokenStream {
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
                    let field = Ident(f.name.to_case(Case::Snake));
                    let env_var = qualified_env_var(name, &f.name);
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
            let variant_name = Ident(strat.name.to_case(Case::Pascal));
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
            let access = qualified_env_var(name, "access_token");
            let refresh = qualified_env_var(name, "refresh_token");
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

pub fn impl_Authentication(spec: &HirSpec, opt: &Config) -> TokenStream {
    let auth_struct_name = opt.authenticator_name().to_rust_struct();
    let from_env = build_Authentication_from_env(spec, &opt.name);
    let oauth2 = spec
        .oauth2_auth()
        .map(|_oauth| {
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

fn static_shared_http_client(spec: &HirSpec, opt: &Config) -> TokenStream {
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

fn shared_oauth2_flow(auth: &Oauth2Auth, _spec: &HirSpec, cfg: &Config) -> TokenStream {
    let service_name = cfg.name.as_str();

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
