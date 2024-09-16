use crate::ident::ToRustIdent;
use crate::ty::ToRustType;
use convert_case::{Case, Casing};
use hir::{Enum, HirField, HirSpec, NewType, Record, Struct};
use mir::Ty;
use proc_macro2::TokenStream;
use quote::quote;

pub fn to_rust_example_value(ty: &Ty, name: &str, spec: &HirSpec, use_ref_value: bool) -> TokenStream {
    match ty {
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
            let inner = to_rust_example_value(inner, name, spec, use_ref_value);
            if use_ref_value {
                quote!(&[#inner])
            } else {
                quote!(vec![#inner])
            }
        }
        Ty::Model(model) => {
            let record = spec.get_record(model).expect("record not found");
            let force_ref = model.ends_with("Required");
            match record {
                Record::Struct(Struct {
                    name: _name,
                    fields,
                    nullable: _,
                    docs: _docs,
                }) => {
                    let fields = fields.iter().map(|(name, field)| {
                        let not_ref = !force_ref || field.optional;
                        let mut value = to_rust_example_value(&field.ty, name, spec, !not_ref);
                        let name = name.to_rust_ident();
                        if field.optional {
                            value = quote!(Some(#value));
                        }
                        quote!(#name: #value)
                    });
                    let model = model.to_rust_struct();
                    quote!(#model{#(#fields),*}).into()
                }
                Record::NewType(NewType {
                    name,
                    fields,
                    doc: _docs,
                }) => {
                    let fields = fields.iter().map(|f| to_rust_example_value(&f.ty, name, spec, false));
                    let name = name.to_rust_struct();
                    quote!(#name(#(#fields),*))
                }
                Record::Enum(Enum {
                    name: _,
                    variants,
                    doc: _docs,
                }) => {
                    let variant = variants.first().unwrap();
                    let variant = if let Some(a) = &variant.alias {
                        a.to_rust_struct()
                    } else {
                        variant.value.to_rust_struct()
                    };
                    let model = model.to_rust_struct();
                    quote!(#model::#variant)
                }
                Record::TypeAlias(name, HirField { ty, optional, .. }) => {
                    let not_ref = !force_ref || !optional;
                    let ty = to_rust_example_value(ty, name, spec, not_ref);
                    if *optional {
                        quote!(Some(#ty))
                    } else {
                        quote!(#ty)
                    }
                }
            }
        }
        Ty::Unit => quote!(()),
        Ty::Any(_) => quote!(serde_json::json!({})),
        Ty::Date { .. } => quote!(chrono::Utc::now().date_naive()),
        Ty::DateTime { .. } => quote!(chrono::Utc::now()),
        Ty::Currency { .. } => quote!(rust_decimal_macros::dec!(100.01)),
        Ty::HashMap(_) => quote!(std::collections::HashMap::new()),
    }
}
