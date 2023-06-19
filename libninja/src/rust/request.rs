use std::default::Default;

use anyhow::Result;
use convert_case::{Case, Casing};
use indoc::formatdoc;
use lazy_static::lazy_static;
use openapiv3::{ArrayType, OpenAPI, Operation, SchemaKind, Type};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex::{Captures, Regex};

use ln_core::extractor::{extract_response_success, schema_ref_to_ty, spec_defines_auth};
use ln_core::hir;
use ln_core::hir::{Location, Parameter, Ty};
use ln_core::LibraryOptions;
use ln_core::OutputOptions;
use ln_core::{extractor, Language};
use ln_mir::{doc, Doc, Ident, Name};
use ln_mir::{Class, Field, FnArg, Function, Visibility};

use crate::rust::codegen::ToRustCode;
use crate::rust::codegen::ToRustIdent;
use crate::rust::codegen::ToRustType;

lazy_static! {
    static ref BUILD_URL_RX: Regex = Regex::new(r"\{([_\-\.\w]+)\}").unwrap();
}

pub fn assign_inputs_to_request(inputs: &[Parameter]) -> TokenStream {
    let assigns = inputs
        .iter()
        .filter(|input| input.location != Location::Path)
        .map(|input| {
            let name = &input.name;
            let field = name.to_rust_ident();

            let mut assign = {
                let param_key = input.to_key().to_rust_code();
                let value_identifier = if input.ty.is_iterable() && input.location != Location::Body
                {
                    quote! { item }
                } else if input.optional {
                    quote! { unwrapped }
                } else {
                    quote! { self.#field }
                };
                match input.location {
                    Location::Path => panic!("Should be filtered."),
                    Location::Body => quote! {
                        r = r.json(json!({#param_key: #value_identifier}));
                    },
                    Location::Query => quote! {
                        r = r.query(#param_key, &#value_identifier.to_string());
                    },
                    Location::Header => quote! {
                        r = r.header(#param_key, &#value_identifier.to_string());
                    },
                    Location::Cookie => quote! {
                        r = r.cookie(#param_key, &#value_identifier.to_string());
                    },
                }
            };

            if input.ty.is_iterable() && input.location != Location::Body {
                let container = if input.optional {
                    quote! { unwrapped }
                } else {
                    quote! { self.#field }
                };
                assign = quote! {
                    for item in #container {
                        #assign
                    }
                };
            }

            if input.optional {
                assign = quote! {
                    if let Some(ref unwrapped) = self.#field {
                        #assign
                    }
                };
            }
            assign
        });
    quote! {
        #(#assigns)*
    }
}

pub fn build_url(operation: &hir::Operation) -> TokenStream {
    let inputs = operation
        .parameters
        .iter()
        .filter(|a| a.location == Location::Path)
        .collect::<Vec<_>>();
    if inputs.is_empty() {
        let path = &operation.path;
        quote! {
            #path
        }
    } else {
        let inputs = inputs.into_iter().map(|input| {
            let name = input.name.to_rust_ident();
            quote! { #name = self.#name }
        });
        let path = BUILD_URL_RX
            .replace_all(&operation.path, |cap: &Captures| {
                let param = cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_case(Case::Snake)
                    .to_rust_ident();
                format!("{{{param}}}")
            })
            .to_string();
        quote! {
            &format!(#path, #(#inputs),*)
        }
    }
}

pub fn authorize_request(spec: &hir::MirSpec) -> TokenStream {
    if spec_defines_auth(spec) {
        quote! {
           r = self.http_client.authenticate(r);
        }
    } else {
        quote! {}
    }
}

pub fn build_send_function(
    operation: &hir::Operation,
    spec: &hir::MirSpec,
) -> Function<TokenStream> {
    let assign_inputs = assign_inputs_to_request(&operation.parameters);
    let auth = authorize_request(spec);
    let response = operation.ret.to_rust_type();
    let method = syn::Ident::new(&operation.method, proc_macro2::Span::call_site());
    let url = build_url(operation);

    let ret = if matches!(operation.ret, Ty::Unit) {
        quote!(Ok(()))
    } else {
        quote!(res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)))
    };
    Function {
        name: "send".into(),
        ret: quote! {
            ::httpclient::InMemoryResult<#response>
        },
        body: quote! {
            let mut r = self.http_client.client.#method(#url);
            #assign_inputs
            #auth
            let res = r
                .send_awaiting_body()
                .await?;
            res.json()
        },
        async_: true,
        public: true,
        ..Function::default()
    }
}

pub fn build_struct_fields(inputs: &[Parameter], use_references: bool) -> Vec<Field<TokenStream>> {
    inputs
        .iter()
        .map(|input| {
            let mut tok = if use_references {
                input.ty.to_reference_type(quote!( 'a ))
            } else {
                input.ty.to_rust_type()
            };
            if input.optional {
                tok = quote! { Option<#tok> }
            }
            Field {
                name: input.name.clone(),
                ty: tok,
                visibility: Visibility::Public,
                ..Field::default()
            }
        })
        .collect()
}

/// Build the various "builder" methods for optional parameters for a request struct
pub fn build_request_struct_builder_methods(
    operation: &hir::Operation,
) -> Vec<Function<TokenStream>> {
    operation
        .parameters
        .iter()
        .filter(|a| a.optional)
        .map(|a| {
            let name = a.name.to_rust_ident();
            let mut arg_type = a.ty.to_reference_type(TokenStream::new());

            let mut body = if a.ty.is_reference_type() {
                quote! {
                    self.#name = Some(#name.to_owned());
                    self
                }
            } else {
                quote! {
                    self.#name = Some(#name);
                    self
                }
            };
            if let Some(Ty::String) = a.ty.inner_iterable() {
                arg_type = quote!(impl IntoIterator<Item = impl AsRef<str>>);
                body = quote! {
                    self.#name = Some(#name.into_iter().map(|s| s.as_ref().to_owned()).collect());
                    self
                };
            }
            let name: Ident = a.name.to_rust_ident();
            Function {
                doc: doc(format!("Set the value of the {} field.", name.0)),
                name,
                args: vec![FnArg {
                    name: a.name.to_rust_ident().into(),
                    ty: arg_type,
                    default: None,
                    treatment: None,
                }],
                ret: quote! {Self},
                body,
                public: true,
                ..Function::default()
            }
        })
        .collect()
}

pub fn build_request_struct(
    operation: &hir::Operation,
    spec: &hir::MirSpec,
    opt: &LibraryOptions,
) -> Vec<Class<TokenStream>> {
    let mut instance_fields = build_struct_fields(&operation.parameters, false);
    instance_fields.insert(
        0,
        Field {
            name: Name::new("http_client"),
            ty: {
                let c = opt.client_name().to_rust_struct();
                quote! { &'a #c }
            },
            visibility: Visibility::Crate,
            ..Field::default()
        },
    );

    let mut instance_methods = vec![build_send_function(operation, spec)];
    let mut_self_instance_methods = build_request_struct_builder_methods(operation);

    let doc = doc("Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.");
    let mut result = vec![Class {
        name: operation.request_struct_name().to_rust_struct(),
        doc,
        instance_fields,
        instance_methods,
        mut_self_instance_methods,
        // We need this lifetime because we hold a ref to the client.
        lifetimes: vec!["'a".to_string()],
        public: true,
        decorators: vec![quote! {#[derive(Clone)]}],
        ..Class::default()
    }];

    if operation.use_required_struct(Language::Rust) {
        let lifetimes = if operation
            .parameters
            .iter()
            .any(|param| param.ty.is_reference_type())
        {
            vec!["'a".to_string()]
        } else {
            vec![]
        };
        result.push(Class {
            name: operation.required_struct_name().to_rust_struct(),
            instance_fields: {
                let required = operation
                    .parameters
                    .iter()
                    .filter(|i| !i.optional)
                    .cloned()
                    .collect::<Vec<_>>();
                build_struct_fields(&required, true)
            },
            public: true,
            lifetimes,
            ..Class::default()
        });
    }

    result
}

pub fn build_request_structs(spec: &hir::MirSpec, opt: &LibraryOptions) -> Vec<Class<TokenStream>> {
    let mut result = vec![];
    for operation in &spec.operations {
        result.extend(build_request_struct(operation, spec, opt));
    }
    result
}

pub fn generate_request_model_rs(spec: &hir::MirSpec, opt: &LibraryOptions) -> TokenStream {
    let classes = build_request_structs(spec, opt);
    let mut request_structs = classes
        .into_iter()
        .map(|c| c.to_rust_code())
        .collect::<Vec<_>>();
    let client_name = opt.client_name().to_rust_struct();
    quote! {
        use crate::#client_name;

        #(#request_structs)*
    }
}
