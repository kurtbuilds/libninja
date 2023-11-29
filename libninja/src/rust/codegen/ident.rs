use convert_case::{Case, Casing};
use hir::{Ident, Name};
use crate::rust::codegen;

pub trait ToRustIdent {
    fn to_rust_struct(&self) -> Ident;
    fn to_filename(&self) -> String;
    fn to_rust_ident(&self) -> Ident;
}

impl ToRustIdent for Name {
    fn to_rust_struct(&self) -> Ident {
        Ident(codegen::sanitize_struct(&self.0))
    }

    fn to_filename(&self) -> String {
        sanitize_filename(&self.0)
    }

    fn to_rust_ident(&self) -> Ident {
        Ident(codegen::sanitize(&self.0))
    }
}


impl ToRustIdent for String {
    fn to_rust_struct(&self) -> Ident {
        Ident(codegen::sanitize_struct(self.as_str()))
    }

    fn to_filename(&self) -> String {
        sanitize_filename(self.as_str())
    }

    fn to_rust_ident(&self) -> Ident {
        Ident(codegen::sanitize(self.as_str()))
    }
}

fn sanitize_filename(s: &str) -> String {
    codegen::sanitize(s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename() {
        let s = "SdAddress.contractor1099";
        assert_eq!(String::from(s).to_rust_ident().0, "sd_address_contractor1099");
        assert_eq!(sanitize_filename(s), "sd_address_contractor1099");
    }
}