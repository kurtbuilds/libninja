use convert_case::{Case, Casing};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, StatusCode};
use proc_macro2::TokenStream;
use quote::quote;
use crate::codegen::util;
use crate::codegen::util::ToToken;
use crate::codegen::util::ToIdent;


/// Generates the client code for a given OpenAPI specification.
pub fn generate_lib_rs(spec: &OpenAPI, name: &str) -> TokenStream {
    let struct_ServiceClient = struct_ServiceClient(name);
    let struct_ServiceAuthentication = struct_ServiceAuthentication(name, spec);
    let impl_ServiceClient = impl_ServiceClient(name, spec);
    let impl_ServiceAuthentication = impl_ServiceAuthentication(name, spec);
    let impl_Authenticatable = impl_Authenticatable(name, spec);

    quote! {
        #struct_ServiceClient
        #impl_ServiceClient
        #struct_ServiceAuthentication
        #impl_ServiceAuthentication
        #impl_Authenticatable
    }
}

pub fn service_auth_struct_name(service_name: &str) -> syn::Ident {
    quote::format_ident!("{}Authentication", service_name)
}

pub fn service_client_struct_name(service_name: &str) -> syn::Ident {
    quote::format_ident!("{}Client", service_name)
}

pub fn struct_ServiceClient(service_name: &str) -> TokenStream {
    let auth_struct_name = service_auth_struct_name(service_name);
    let client_struct_name = service_client_struct_name(service_name);

    quote! {
        pub struct #client_struct_name {
            client: httpclient::Client,
            authentication: Option<#auth_struct_name>,
        }
    }
}


pub fn impl_ServiceClient_paths(spec: &OpenAPI) -> impl Iterator<Item=TokenStream> + '_ {
    spec.paths.iter()
        // .filter(|(path, _)| path.as_str() == "/item/get")
        .filter_map(move |(path, item)| {
            let item = item.as_item().unwrap();
            let operation = item.post.as_ref().unwrap();
            let name = operation.operation_id.as_ref().unwrap().to_case(Case::Snake);
            let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
            let request_body = operation.request_body.as_ref().unwrap()
                .as_item().unwrap();
            let params = request_body.content.iter()
                .filter(|(key, _)| key.as_str() == "application/json")
                .map(|(name, media_type)| {
                    let body_schema = media_type.schema
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .resolve(spec)
                        .unwrap();
                    let props = body_schema.properties().unwrap();
                    props.iter()
                })
                .flatten()
                .filter(|(k, v)| !["client_id", "secret"].contains(&k.as_str()))
                .map(|(k, v)| {
                    let prop_schema = v
                        .as_ref()
                        .resolve(spec)
                        .unwrap();
                    (k, prop_schema)
                })
                .collect::<Vec<_>>();
            let fn_args = params.iter().map(|(k, prop_schema)| {
                // let k = k.to_case(Case::Snake);
                let k = k.to_ident();
                let tok = prop_schema.to_token(spec);
                quote!(#k: #tok)
            })
                .collect::<Vec<_>>();
            let json_fields = params.iter().map(|(k, prop_schema)| {
                let iden = k.to_ident();
                quote!(#k: #iden)
            })
                .collect::<Vec<_>>();
            let mut doc_pieces = vec![];
            if let Some(summary) = operation.summary.as_ref() {
                if !summary.is_empty() {
                    doc_pieces.push(summary.clone());
                }
            }
            if let Some(description) = operation.description.as_ref() {
                if !description.is_empty() {
                    if doc_pieces.len() > 0 && description == &doc_pieces[0] {} else {
                        doc_pieces.push(description.clone());
                    }
                }
            }
            if let Some(external_docs) = operation.external_docs.as_ref() {
                doc_pieces.push(format!("See full Plaid docs at <https://plaid.com/docs{}>", external_docs.url));
            }
            let docstring = doc_pieces.join("\n\n");
            let response_success = operation.responses.responses
                .get(&StatusCode::Code(200))
                .unwrap()
                .as_item()
                .unwrap();
            let response_success_schema_name = match response_success.content.get("application/json") {
                Some(r) => r.schema.as_ref().unwrap().as_ref().to_token(spec),
                None => return None,
            };
            Some(quote! {
                #[doc = #docstring]
                pub async fn #name(&self, #(#fn_args),*) -> anyhow::Result<#response_success_schema_name> {
                    {
                         let res = self.client.post("/item/get")
                            .json(json!({
                                #(#json_fields),*
                            }))
                            .authenticate(&self.authentication)
                            .send()
                            .await
                            .unwrap()
                            .error_for_status();
                        match res {
                            Ok(res) => res
                                .json()
                                .await
                                .map_err(|e| anyhow::anyhow!("{:?}", e)),
                            Err(res) => {
                                let text = res
                                    .text()
                                    .await
                                    .map_err(|e| anyhow::anyhow!("{:?}", e));
                                Err(anyhow::anyhow!("{:?}", text))
                            }
                        }
                    }
                }
            })
        })
}


pub fn impl_ServiceClient(service_name: &str, spec: &OpenAPI) -> TokenStream {
    let client_struct_name = service_client_struct_name(service_name);
    let auth_struct_name = service_auth_struct_name(service_name);
    let path_fns = impl_ServiceClient_paths(spec);

    quote! {
        impl #client_struct_name {
            pub fn new(url: &str) -> Self {
                let client = httpclient::Client::new(Some(url.to_string()));
                let authentication = None;
                Self {
                    client,
                    authentication,
                }
            }

            pub fn with_authentication(mut self, authentication: #auth_struct_name) -> Self {
                self.authentication = Some(authentication);
                self
            }

            pub fn with_middleware<M: httpclient::Middleware + 'static>(mut self, middleware: M) -> Self {
                self.client = self.client.with_middleware(middleware);
                self
            }

            #(#path_fns)*
        }
    }
}

pub fn struct_ServiceAuthentication(service_name: &str, spec: &OpenAPI) -> TokenStream {
    let auth_struct_name = service_auth_struct_name(service_name);

    let variants = spec.security.as_ref().unwrap().iter().map(|security| {
        let args = security.iter().map(|(k, scopes)| {
            k.to_ident()
        });
        let (name, scopes) = security.iter().next().unwrap();
        let variant_name = syn::Ident::new(&name.to_case(Case::Pascal), proc_macro2::Span::call_site());
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

pub fn impl_ServiceAuthentication_from_env(service_name: &str, spec: &OpenAPI) -> TokenStream {
    let first_variant = spec.security.as_ref().unwrap().iter().next().unwrap();
    let fields = first_variant.iter().map(|(k, scopes)| {
        let field = syn::Ident::new(&k.to_case(Case::Snake), proc_macro2::Span::call_site());
        let env_var = if k.to_lowercase().starts_with(&service_name.to_lowercase()) {
            k.to_case(Case::ScreamingSnake)
        } else {
            format!("{}_{}", service_name.to_case(Case::ScreamingSnake), k.to_case(Case::ScreamingSnake))
        };
        let expect = format!("Environment variable {} is not set.", env_var);
        quote! {
            #field: std::env::var(#env_var).expect(#expect)
        }
    });
    let (name, scopes) = first_variant.iter().next().unwrap();
    let variant_name = syn::Ident::new(&name.to_case(Case::Pascal), proc_macro2::Span::call_site());
    quote! {
        pub fn from_env() -> Self {
            Self::#variant_name {
                #(#fields),*
            }
        }
    }
}


pub fn impl_ServiceAuthentication(service_name: &str, spec: &OpenAPI) -> TokenStream {
    let auth_struct_name = service_auth_struct_name(service_name);
    let from_env = impl_ServiceAuthentication_from_env(service_name, spec);

    quote! {
        impl #auth_struct_name {
            #from_env
        }
    }
}

pub fn impl_Authenticatable(service_name: &str, spec: &OpenAPI) -> TokenStream {
    let auth_struct_name = service_auth_struct_name(service_name);
    quote! {
        trait Authenticatable {
            fn authenticate(self, authenticator: &Option<#auth_struct_name>) -> Self;
        }

        impl<'a> Authenticatable for RequestBuilder<'a> {
            fn authenticate(self, authenticator: &Option<#auth_struct_name>) -> Self {
                if let Some(authenticator) = authenticator {
                    match authenticator {
                        PlaidAuthentication::ClientId { client_id, secret, ..} => {
                            self.push_json(json!({
                                "client_id": client_id,
                                "secret": secret,
                            }))
                        }
                    }
                } else {
                    self
                }
            }
        }
    }
}
