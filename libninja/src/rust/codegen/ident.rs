use mir::Ident;

use crate::rust::codegen;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::codegen::{ToRustIdent, sanitize_filename};

    #[test]
    fn test_filename() {
        let s = "SdAddress.contractor1099";
        assert_eq!(String::from(s).to_rust_ident().0, "sd_address_contractor1099");
        assert_eq!(sanitize_filename(s), "sd_address_contractor1099");
    }
}