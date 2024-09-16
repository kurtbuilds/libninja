use clap::builder::Str;
use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use hir::{HirField, HirSpec, Record, Struct};
use mir::Ty;
use mir::Import;

fn implements_default(s: &Struct, spec: &HirSpec) -> bool {
    s.fields.values().all(|f| f.implements_default(spec))
}

fn derive_default(s: &Struct, spec: &HirSpec) -> TokenStream {
        if implements_default(s, spec) {
            quote! { , Default }
        } else {
            TokenStream::new()
        }
    }
}

pub trait RecordExt {
    fn imports(&self, path: &str) -> Option<Import>;
}

impl RecordExt for Record {}

pub trait HirFieldExt {
    fn implements_default(&self, spec: &HirSpec) -> bool;
}

impl HirFieldExt for HirField {
    fn implements_default(&self, spec: &HirSpec) -> bool {
        self.optional || self.ty.implements_default(spec)
    }
}

#[cfg(test)]
mod tests {
    use hir::HirField;
    use mir::Ty;
    use mir_rust::format_code;

    use super::*;

    