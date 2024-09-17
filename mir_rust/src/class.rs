use crate::{derives_to_tokens, CanDerive, ToRustCode, ToRustIdent, ToRustType};
use hir::{Config, HirField, HirSpec, Struct};
use mir::{
    Class, DateSerialization, DecimalSerialization, Field, Function, Ident, IntegerSerialization, Item, Ty, Visibility,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;

fn lifetimes_to_tokens(lifetimes: &[String]) -> TokenStream {
    if lifetimes.is_empty() {
        return TokenStream::new();
    }
    let lifetimes = lifetimes.iter().map(|l| {
        let name = syn::Lifetime::new(l, Span::call_site());
        quote! { # name }
    });
    quote! { < # ( # lifetimes), * > }
}

fn methods_to_tokens(methods: Vec<Function<TokenStream>>, name: &Ident, lifetimes: &TokenStream) -> TokenStream {
    if methods.is_empty() {
        return TokenStream::new();
    }
    let methods = methods.into_iter().map(|m| m.to_rust_code());
    quote! {
        impl #lifetimes #name #lifetimes {
            #(#methods)*
        }
    }
}

impl ToRustCode for Class<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let Class {
            vis,
            name,
            doc,
            fields,
            methods,
            attributes,
            lifetimes,
            items,
            imports,
        } = self;
        let vis = vis.to_rust_code();
        let fields = fields.into_iter().map(|f| f.to_rust_code());
        let lifetimes = lifetimes_to_tokens(&lifetimes);

        let methods = methods_to_tokens(methods, &name, &lifetimes);
        let items = items.into_iter().map(|f| f.to_rust_code());
        let imports = imports.into_iter().map(|i| i.to_rust_code());
        quote! {
            #(#imports)*
            #doc
            #(#attributes)*
            #vis struct #name #lifetimes {
                #(#fields,)*
            }
            #methods
            #(#items)*
        }
    }
}

impl ToRustCode for Field<TokenStream> {
    fn to_rust_code(self) -> TokenStream {
        let name = self.name;
        let ty = if self.optional {
            let ty = self.ty;
            quote! { Option<#ty> }
        } else {
            self.ty
        };
        let vis = self.vis.to_rust_code();
        let doc = self.doc.to_rust_code();
        let attributes = self.attributes;
        quote! {
            #doc
            #(#attributes)*
            #vis #name: #ty
        }
    }
}

fn field_attributes(f: &HirField, name: &str, config: &Config) -> Vec<TokenStream> {
    let mut attributes = Vec::new();
    let rust_ident = name.to_rust_ident();
    if rust_ident != name {
        if f.flatten {
            attributes.push(quote! {
                #[serde(flatten)]
            });
        } else {
            attributes.push(quote! {
                #[serde(rename = #name)]
            });
        }
        if config.ormlite {
            attributes.push(quote! {
                #[cfg_attr(feature = "ormlite", ormlite(column = #name))]
            });
        }
    }
    if f.optional {
        attributes.push(quote! {
            #[serde(default, skip_serializing_if = "Option::is_none")]
        });
    } else if f.ty.is_iterable() {
        attributes.push(quote! {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
        });
    } else if matches!(f.ty, Ty::Any(_)) {
        attributes.push(quote! {
            #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
        });
    }
    if f.ty.inner_model().is_some() && config.ormlite {
        attributes.push(quote! {
            #[ormlite(json)]
        });
    }
    match f.ty {
        Ty::Integer { ser: serialization } => match serialization {
            IntegerSerialization::Simple => {}
            IntegerSerialization::String => {
                attributes.push(quote! {
                    #[serde(with = "crate::serde::option_i64_str")]
                });
            }
            IntegerSerialization::NullAsZero => {
                attributes.push(quote! {
                    #[serde(with = "crate::serde::option_i64_null_as_zero")]
                });
            }
        },
        Ty::Date { ser: serialization } => match serialization {
            DateSerialization::Iso8601 => {}
            DateSerialization::Integer => {
                attributes.push(quote! {
                    #[serde(with = "crate::serde::option_chrono_naive_date_as_int")]
                });
            }
        },
        Ty::Currency {
            ser: DecimalSerialization::String,
        } => {
            if f.optional {
                attributes.push(quote! {
                    #[serde(with = "rust_decimal::serde::str_option")]
                });
            } else {
                attributes.push(quote! {
                    #[serde(with = "rust_decimal::serde::str")]
                });
            }
        }
        _ => {}
    }
    attributes
}

fn class_fields(s: &Struct, config: &Config) -> Vec<Field<TokenStream>> {
    s.fields
        .iter()
        .map(|(name, field)| {
            let attributes = field_attributes(field, name, config);
            let ty = field.ty.to_rust_type();
            let mut optional = field.optional;
            match field.ty {
                Ty::Integer {
                    ser: IntegerSerialization::NullAsZero | IntegerSerialization::String,
                } => {
                    optional = true;
                }
                Ty::Date {
                    ser: DateSerialization::Integer,
                } => {
                    optional = true;
                }
                _ => {}
            }
            Field {
                name: name.to_rust_ident(),
                ty,
                vis: Visibility::Public,
                attributes,
                optional,
                doc: field.doc.clone(),
                ..Field::default()
            }
        })
        .collect()
}

pub struct RefTarget {
    name: String,
    ty: Ty,
}

fn ref_target(s: &Struct) -> Option<RefTarget> {
    s.fields
        .iter()
        .find(|(_, f)| f.flatten && !f.optional)
        .map(|(name, f)| RefTarget {
            name: name.clone(),
            ty: f.ty.clone(),
        })
}

pub fn make_class(s: &Struct, config: &Config, spec: &HirSpec) -> Class<TokenStream> {
    let default = s.implements_default(spec).then(|| {
        quote! { , Default }
    });
    let derives = derives_to_tokens(&config.derives);

    let doc = s.docs.clone();

    let name = s.name.to_rust_struct();
    let fields = class_fields(s, config);
    let deref = ref_target(s)
        .map(|t| {
            let target = t.name.to_rust_ident();
            let ty = t.ty.to_rust_type();
            quote! {
                impl std::ops::Deref for #name {
                    type Target = #ty;
                    fn deref(&self) -> &Self::Target {
                        &self.#target
                    }
                }
                impl std::ops::DerefMut for #name {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.#target
                    }
                }
            }
        })
        .unwrap_or_default();

    let impl_blocks = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }
        #deref
    };
    let attributes = vec![quote! {
        #[derive(Debug, Clone, Serialize, Deserialize #default #derives)]
    }];
    Class {
        vis: Visibility::Public,
        name,
        doc,
        fields,
        methods: vec![],
        attributes,
        lifetimes: vec![],
        items: vec![Item::Block(impl_blocks)],
        imports: vec![],
    }
}

impl CanDerive for Struct {
    fn implements_default(&self, spec: &HirSpec) -> bool {
        self.fields.iter().all(|(_, f)| f.ty.implements_default(spec))
    }

    fn implements_dummy(&self, spec: &HirSpec) -> bool {
        self.fields.iter().all(|(_, f)| f.ty.implements_dummy(spec))
    }
}
