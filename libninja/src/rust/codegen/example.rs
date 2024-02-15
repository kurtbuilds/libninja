use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use hir::{HirField, HirSpec, Language, NewType, Operation, Parameter, Record, StrEnum, Struct};
use ln_macro::rfunction;
use mir::{File, Import, Ty};

use crate::PackageConfig;
use crate::rust::codegen::{ToRustCode, ToRustType};
use crate::rust::codegen::ToRustIdent;
use mir_rust::format_code;

pub trait ToRustExample {
    fn to_rust_example(&self, spec: &HirSpec) -> anyhow::Result<TokenStream>;
}

impl ToRustExample for Parameter {
    fn to_rust_example(&self, spec: &HirSpec) -> anyhow::Result<TokenStream> {
        to_rust_example_value(&self.ty, &self.name, spec, false)
    }
}


pub fn generate_example(operation: &Operation, opt: &PackageConfig, spec: &HirSpec) -> anyhow::Result<String> {
    let args = operation.function_args(Language::Rust);
    let declarations = args.iter().map(|p| {
        let ident = p.name.to_rust_ident();
        let value = to_rust_example_value(&p.ty, &p.name, spec, true)?;
        Ok(quote! {
            let #ident = #value;
        })
    }).collect::<anyhow::Result<Vec<_>, anyhow::Error>>()?;
    let fn_args = args.iter().map(|p| p.name.to_rust_ident());
    let optionals = operation.optional_args().into_iter().map(|p| {
        let ident = p.name.to_rust_ident();
        let value = to_rust_example_value(&p.ty, &p.name, spec, true)?;
        Ok(quote! {
            .#ident(#value)
        })
    }).collect::<anyhow::Result<Vec<_>, anyhow::Error>>()?;
    let qualified_client = format!("{}::{}", opt.package_name, opt.client_name().to_rust_struct());
    let mut imports = vec![
        Import::package(&qualified_client),
        Import::package(&format!("{}::model::*", opt.package_name)),
    ];
    if operation.use_required_struct(Language::Rust) {
        let struct_name = operation.required_struct_name().to_rust_struct().to_string();
        imports.push(Import::package(&format!("{}::request::{}", opt.package_name, struct_name)));
    }
    let operation = operation.name.to_rust_ident();
    let client = opt.client_name().to_rust_struct();
    let mut main = rfunction!(async main() {
        let client = #client::from_env();
        #(#declarations)*
        let response = client.#operation(#(#fn_args),*)
            #(#optionals)*
            .await
            .unwrap();
        println!("{:#?}", response);
    });
    main.annotations.push("tokio::main".to_string());

    let example = File {
        imports,
        functions: vec![main],
        ..File::default()
    };
    let code = example.to_rust_code();
    Ok(format_code(code))
}

pub fn to_rust_example_value(ty: &Ty, name: &str, spec: &HirSpec, use_ref_value: bool) -> anyhow::Result<TokenStream> {
    let s = match ty {
        Ty::String => {
            let s = format!("your {}", name.to_case(Case::Lower));
            if use_ref_value {
                quote!(#s)
            } else {
                quote!(#s.to_owned())
            }
        }
        Ty::Integer { .. } => quote!(1),
        Ty::Float => quote!(1.0),
        Ty::Boolean => quote!(true),
        Ty::Array(inner) => {
            let use_ref_value = if !inner.is_reference_type() {
                false
            } else {
                use_ref_value
            };
            let inner = to_rust_example_value(inner, name, spec, use_ref_value)?;
            if use_ref_value {
                quote!(&[#inner])
            } else {
                quote!(vec![#inner])
            }
        }
        Ty::Model(model) => {
            let record = spec.get_record(model)?;
            let force_ref = model.ends_with("Required");
            match record {
                Record::Struct(Struct { name: _name, fields, nullable, docs: _docs }) => {
                    let fields = fields.iter().map(|(name, field)| {
                        let not_ref = !force_ref || field.optional;
                        let mut value = to_rust_example_value(&field.ty, name, spec, !not_ref)?;
                        let name = name.to_rust_ident();
                        if field.optional {
                            value = quote!(Some(#value));
                        }
                        Ok(quote!(#name: #value))
                    }).collect::<Result<Vec<_>, anyhow::Error>>()?;
                    let model = model.to_rust_struct();
                    quote!(#model{#(#fields),*})
                }
                Record::NewType(NewType { name, fields, docs: _docs }) => {
                    let fields = fields.iter().map(|f| {
                        to_rust_example_value(&f.ty, name, spec, false)
                    }).collect::<Result<Vec<_>, _>>()?;
                    let name = name.to_rust_struct();
                    quote!(#name(#(#fields),*))
                }
                Record::Enum(StrEnum { name, variants, docs: _docs }) => {
                    let variant = variants.first().unwrap();
                    let variant = variant.to_rust_struct();
                    let model = model.to_rust_struct();
                    quote!(#model::#variant)
                }
                Record::TypeAlias(name, HirField { ty, optional, .. }) => {
                    let not_ref = !force_ref || !optional;
                    let ty = to_rust_example_value(ty, name, spec, not_ref)?;
                    if *optional {
                        quote!(Some(#ty))
                    } else {
                        quote!(#ty)
                    }
                }
            }
        }
        Ty::Unit => quote!(()),
        Ty::Any => quote!(serde_json::json!({})),
        Ty::Date { .. } => quote!(chrono::Utc::now().date_naive()),
        Ty::DateTime { .. } => quote!(chrono::Utc::now()),
        Ty::Currency { .. } => quote!(rust_decimal_macros::dec!(100.01))
    };
    Ok(s)
}
