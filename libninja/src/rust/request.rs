use std::default::Default;
use std::sync::OnceLock;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex::Captures;

use hir::{HirSpec, Operation};
use hir::{Language, Location, Parameter};
use ln_core::PackageConfig;
use mir::{Class, Field, FnArg2, Function, Ident, Visibility};
use mir::Doc;
use mir::Ty;
use mir_rust::{ToRustCode, ToRustIdent};

use crate::rust::codegen::ToRustType;

use super::lower_hir::derives_to_tokens;

pub fn assign_inputs_to_request(inputs: &[Parameter]) -> TokenStream {
    let params_except_path: Vec<&Parameter> = inputs.iter().filter(|&input| input.location != Location::Path).collect();
    if params_except_path.iter().all(|&input| input.location == Location::Query) {
        return quote! {
            r = r.set_query(self.params);
        };
    }
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
                    quote! { self.params.#field }
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
                    quote! { self.params.#field }
                };
                assign = quote! {
                    for item in #container {
                        #assign
                    }
                };
            }

            if input.optional {
                assign = quote! {
                    if let Some(ref unwrapped) = self.params.#field {
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

/// This is complicated because we need to interpolate any param values.
pub fn build_url(operation: &Operation) -> TokenStream {
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
        static FIX_PLACEHOLDERS: OnceLock<regex::Regex> = OnceLock::new();
        let fix = FIX_PLACEHOLDERS.get_or_init(||
            regex::Regex::new("\\{([_\\w]+)\\}").unwrap()
        );
        let inputs = inputs.into_iter().map(|input| {
            let name = input.name.to_rust_ident();
            quote! { #name = self.params.#name }
        });
        let path = fix
            .replace_all(&operation.path, |cap: &Captures| {
                format!("{{{}}}", cap.get(1).unwrap().as_str().to_case(Case::Snake))
            })
            .to_string();
        quote! {
            &format!(#path, #(#inputs),*)
        }
    }
}

// pub fn authorize_request(spec: &HirSpec) -> TokenStream {
//     if spec_defines_auth(spec) {
//         quote! {
//            r = self.http_client.authenticate(r);
//         }
//     } else {
//         quote! {}
//     }
// }

// pub fn build_send_function(operation: &Operation, spec: &HirSpec) -> Function<TokenStream> {
//     let assign_inputs = assign_inputs_to_request(&operation.parameters);
//     let auth = authorize_request(spec);
//     let response = operation.ret.to_rust_type();
//     let method = syn::Ident::new(&operation.method, proc_macro2::Span::call_site());
//     let url = build_url(operation);
//
//     let ret = if matches!(operation.ret , Ty::Unit) {
//         quote!(Ok(()))
//     } else {
//         quote!(res
//             .json()
//             .await
//             .map_err(|e| anyhow::anyhow!("{:?}", e))
//         )
//     };
//     Function {
//         name: "send".into(),
//         ret: quote! {
//             ::httpclient::InMemoryResult<#response>
//         },
//         body: quote! {
//             let mut r = self.http_client.client.#method(#url);
//             #assign_inputs
//             #auth
//             let res = r
//                 .await?;
//             res.json().map_err(Into::into)
//         },
//         async_: true,
//         public: true,
//         ..Function::default()
//     }
// }

pub fn build_struct_fields(
    inputs: &[Parameter],
    use_references: bool,
) -> Vec<Field<TokenStream>> {
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
                name: input.name.to_rust_ident(),
                ty: tok,
                vis: Visibility::Public,
                ..Field::default()
            }
        })
        .collect()
}

/// Build the various "builder" methods for optional parameters for a request struct
pub fn build_request_struct_builder_methods(
    operation: &Operation,
) -> Vec<Function<TokenStream>> {
    operation.parameters.iter().filter(|a| a.optional).map(|a| {
        let name = a.name.to_rust_ident();
        let mut arg_type = a.ty.to_reference_type(TokenStream::new());

        let mut body = if a.ty.is_reference_type() {
            quote! {
                self.params.#name = Some(#name.to_owned());
                self
            }
        } else {
            quote! {
                self.params.#name = Some(#name);
                self
            }
        };
        if let Some(Ty::String) = a.ty.inner_iterable() {
            arg_type = quote!( impl IntoIterator<Item = impl AsRef<str>> );
            body = quote! {
                self.params.#name = Some(#name.into_iter().map(|s| s.as_ref().to_owned()).collect());
                self
            };
        }
        let name: Ident = a.name.to_rust_ident();
        Function {
            doc: Some(Doc(format!("Set the value of the {} field.", name.0))),
            name,
            args: vec![
                FnArg2::SelfArg { mutable: true, reference: false },
                FnArg2::Basic {
                    name: a.name.to_rust_ident(),
                    ty: arg_type,
                    default: None,
                },
            ],
            ret: quote! {Self},
            body,
            public: true,
            ..Function::default()
        }
    }).collect()
}

pub fn build_request_struct(
    operation: &Operation,
    spec: &HirSpec,
    opt: &PackageConfig,
) -> Vec<Class<TokenStream>> {
    let mut instance_fields = build_struct_fields(&operation.parameters, false);
    // instance_fields.insert(
    //     0,
    //     Field {
    //         name: "http_client".to_string(),
    //         ty: {
    //             let c = opt.client_name().to_rust_struct();
    //             quote! { &'a #c }
    //         },
    //         visibility: Visibility::Crate,
    //         ..Field::default()
    //     },
    // );

    let fn_name = operation.name.to_rust_ident().0;
    let response = operation.ret.to_rust_type().to_string().replace(" ", "");
    let client = opt.client_name().to_rust_struct().to_string().replace(" ", "");
    let derives = derives_to_tokens(&opt.derives);
    let doc = Some(Doc(format!(r#"You should use this struct via [`{client}::{fn_name}`].

On request success, this will return a [`{response}`]."#, )));
        
    let mut result = vec![Class {
        name: operation.request_struct_name().to_rust_struct(),
        doc,
        instance_fields,
        lifetimes: vec![],
        public: true,
        decorators: vec![quote! {#[derive(Debug, Clone, Serialize, Deserialize #derives)]}],
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

pub fn build_request_structs(spec: &HirSpec, opt: &PackageConfig) -> Vec<Class<TokenStream>> {
    let mut result = vec![];
    for operation in &spec.operations {
        result.extend(build_request_struct(operation, spec, opt));
    }
    result
}

pub fn generate_request_model_rs(spec: &HirSpec, opt: &PackageConfig) -> TokenStream {
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
