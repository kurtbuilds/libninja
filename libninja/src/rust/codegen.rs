use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

pub use example::*;
pub use ident::*;
use mir::Ident;
use mir_rust::{ToRustCode, ToRustIdent};
pub use ty::*;

mod example;
mod ident;
mod ty;

#[cfg(test)]
mod tests {
    use mir::{import, Import};

    use crate::rust::codegen::{ToRustCode, ToRustIdent};

    #[test]
    fn test_to_ident() {
        assert_eq!("meta/root".to_rust_ident().0, "meta_root");
    }

    #[test]
    fn test_to_ident1() {
        assert_eq!(
            "get-phone-checks-v0.1".to_rust_ident().0,
            "get_phone_checks_v0_1"
        );
    }

    #[test]
    fn test_star() {
        let i = import!("super::*");
        assert_eq!(i.to_rust_code().to_string(), "use super :: * ;");
        let i = Import::new("super", vec!["*"]);
        assert_eq!(i.to_rust_code().to_string(), "use super :: { * } ;");
    }

    #[test]
    fn test_import() {
        let import = import!("plaid::model::LinkTokenCreateRequestUser");
        assert_eq!(
            import.to_rust_code().to_string(),
            "use plaid :: model :: LinkTokenCreateRequestUser ;"
        );
        let import = import!("plaid::model", LinkTokenCreateRequestUser, Foobar);
        assert_eq!(
            import.to_rust_code().to_string(),
            "use plaid :: model :: { LinkTokenCreateRequestUser , Foobar } ;"
        );

        let import = Import::alias("plaid::model", "foobar");
        assert_eq!(
            import.to_rust_code().to_string(),
            "use plaid :: model as foobar ;"
        );

        let import = Import::package("foo_bar");
        assert_eq!(import.to_rust_code().to_string(), "use foo_bar ;");
    }
}

pub fn serde_rename(one: &str, two: &Ident) -> TokenStream {
    if one != &two.0 {
        quote!(#[serde(rename = #one)])
    } else {
        TokenStream::new()
    }
}
