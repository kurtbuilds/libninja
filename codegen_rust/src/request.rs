use std::default::Default;
use std::fs;
use std::sync::OnceLock;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use regex::Captures;

use hir::{Config, HirSpec, Language, Location, Operation, Parameter};
use mir::{import, Arg, Class, Doc, Field, File, Function, Ident, Import, Item, Ty, Visibility};

use mir_rust::{derives_to_tokens, ToRustCode, ToRustIdent, ToRustType};

use crate::{client::build_api_client_method, write_rust, Modified};
use std::io::Result;

pub fn write_request_module(spec: &HirSpec, cfg: &Config, m: &mut Modified) -> Result<()> {
    let src = cfg.src();
    let imports = vec![];
    fs::create_dir_all(src.join("request"))?;
    let mut modules: Vec<(Ident, Ident)> = vec![];

    for operation in &spec.operations {
        modules.push((
            Ident(operation.file_name()),
            operation.request_struct_name().to_rust_struct(),
        ));
        let file = make_single_module(operation, &spec, cfg);
        let fname = operation.file_name();
        let path = src.join("request").join(&fname).with_extension("rs");
        write_rust(&path, file, m)?;
    }
    let items = modules
        .into_iter()
        .map(|(m, s)| {
            Item::Block(quote! {
                pub mod #m;
                pub use #m::#s;
            })
        })
        .collect();
    let file = File {
        imports,
        items,
        ..File::default()
    };
    write_rust(&src.join("request").join("mod.rs"), file, m)
}

pub fn make_single_module(operation: &Operation, spec: &HirSpec, cfg: &Config) -> File<TokenStream> {
    let client_name = cfg.client_name();
    let authenticate = spec
        .has_security()
        .then(|| {
            quote! {
                r = self.client.authenticate(r);
            }
        })
        .unwrap_or_default();

    let mut imports: Vec<Import> = vec![
        import!(crate, FluentRequest),
        import!(serde, Serialize, Deserialize),
        import!(httpclient, InMemoryResponseExt),
    ];
    let request_structs = build_request_struct(operation, spec, &cfg, &mut imports);
    let struct_name = request_structs[0].name.clone();
    let response = operation.ret.to_rust_type();
    let method = Ident(operation.method.clone());
    let url = make_url(&operation);
    let builder_methods = build_request_struct_builder_methods(&operation)
        .into_iter()
        .map(|s| s.to_rust_code());

    let assign_inputs = assign_inputs_to_request(&operation.parameters);
    let output = if operation.ret.is_primitive() {
        quote! { #response }
    } else {
        quote! { crate::model::#response }
    };

    let impl_block = quote! {
        impl FluentRequest<'_, #struct_name> {
            #(#builder_methods)*
        }
        impl<'a> ::std::future::IntoFuture for FluentRequest<'a, #struct_name> {
            type Output = httpclient::InMemoryResult<#output>;
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
    let mut items: Vec<Item<TokenStream>> = request_structs.into_iter().map(|s| Item::Class(s)).collect();
    items.push(Item::Block(impl_block));
    let client_method = build_api_client_method(operation);
    items.push(Item::Block(quote! {
        impl crate::#client_name {
            #client_method
        }
    }));
    File {
        attributes: vec![],
        doc: None,
        imports,
        items,
        modules: Vec::new(),
    }
}

pub fn assign_inputs_to_request(inputs: &[Parameter]) -> TokenStream {
    let params_except_path: Vec<&Parameter> = inputs
        .iter()
        .filter(|&input| input.location != Location::Path)
        .collect();
    if params_except_path
        .iter()
        .all(|&input| input.location == Location::Query)
    {
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
                let value_identifier = if input.ty.is_iterable() && input.location != Location::Body {
                    quote! { item }
                } else if input.optional {
                    quote! { unwrapped }
                } else {
                    quote! { self.params.#field }
                };
                match input.location {
                    Location::Path => panic!("Should be filtered."),
                    Location::Body => quote! {
                        r = r.json(serde_json::json!({#param_key: #value_identifier}));
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
pub fn make_url(operation: &Operation) -> TokenStream {
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
        let fix = FIX_PLACEHOLDERS.get_or_init(|| regex::Regex::new("\\{([_\\w]+)\\}").unwrap());
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

fn add_model_import(imports: &mut Vec<Import>, model: &str) {
    for import in imports.iter_mut() {
        if import.path == "crate::model" {
            for item in &mut import.imports {
                if item.name == model {
                    return;
                }
            }
            import.imports.push(model.into());
            return;
        }
    }
    imports.push(Import::new("crate::model", vec![model.to_string()]));
}

pub fn make_struct_fields(
    inputs: &[Parameter],
    use_references: bool,
    imports: &mut Vec<Import>,
) -> Vec<Field<TokenStream>> {
    inputs
        .iter()
        .map(|input| {
            if let Some(m) = input.ty.inner_model() {
                add_model_import(imports, m);
            }
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
pub fn build_request_struct_builder_methods(operation: &Operation) -> Vec<Function<TokenStream>> {
    operation
        .parameters
        .iter()
        .filter(|a| a.optional)
        .map(|a| {
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
                arg_type = quote!(impl IntoIterator<Item = impl AsRef<str>>);
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
                    Arg::SelfArg {
                        mutable: true,
                        reference: false,
                    },
                    Arg::Basic {
                        name: a.name.to_rust_ident(),
                        ty: arg_type,
                        default: None,
                    },
                ],
                ret: quote! {Self},
                body,
                vis: Visibility::Public,
                ..Function::default()
            }
        })
        .collect()
}

pub fn build_request_struct(
    operation: &Operation,
    _spec: &HirSpec,
    opt: &Config,
    imports: &mut Vec<Import>,
) -> Vec<Class<TokenStream>> {
    let instance_fields = make_struct_fields(&operation.parameters, false, imports);

    let fn_name = operation.name.to_rust_ident().0;
    let response = operation.ret.to_rust_type().to_string().replace(" ", "");
    let client = opt.client_name().to_string().replace(" ", "");
    let derives = derives_to_tokens(&opt.derives);
    let doc = Some(Doc(format!(
        r#"You should use this struct via [`{client}::{fn_name}`].

On request success, this will return a [`{response}`]."#,
    )));

    let mut result = vec![Class {
        name: operation.request_struct_name().to_rust_struct(),
        doc,
        fields: instance_fields,
        lifetimes: vec![],
        vis: Visibility::Public,
        attributes: vec![quote! {#[derive(Debug, Clone, Serialize, Deserialize #derives)]}],
        ..Class::default()
    }];

    if operation.use_required_struct(Language::Rust) {
        let lifetimes = if operation.parameters.iter().any(|param| param.ty.is_reference_type()) {
            vec!["'a".to_string()]
        } else {
            vec![]
        };
        result.push(Class {
            name: operation.required_struct_name().to_rust_struct(),
            fields: {
                let required = operation
                    .parameters
                    .iter()
                    .filter(|i| !i.optional)
                    .cloned()
                    .collect::<Vec<_>>();
                make_struct_fields(&required, true, imports)
            },
            vis: Visibility::Public,
            lifetimes,
            ..Class::default()
        });
    }
    result
}

// pub fn build_request_structs(spec: &HirSpec, opt: &Config) -> Vec<Class<TokenStream>> {
//     let mut result = vec![];
//     for operation in &spec.operations {
//         result.extend(build_request_struct(operation, spec, opt));
//     }
//     result
// }

// pub fn generate_request_model_rs(spec: &HirSpec, opt: &Config) -> TokenStream {
//     let classes = build_request_structs(spec, opt);
//     let request_structs = classes
//         .into_iter()
//         .map(|c| c.to_rust_code())
//         .collect::<Vec<_>>();
//     let client_name = opt.client_name().to_rust_struct();
//     quote! {
//         use crate::#client_name;
//
//         #(#request_structs)*
//     }
// }
