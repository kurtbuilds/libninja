use mir::Ident;

pub trait ToRustIdent {
    fn to_rust_struct(&self) -> Ident;
    fn to_rust_ident(&self) -> Ident;
}

impl ToRustIdent for String {
    fn to_rust_struct(&self) -> Ident {
        crate::sanitize_struct(self)
    }

    fn to_rust_ident(&self) -> Ident {
        crate::sanitize_ident(self)
    }
}

impl ToRustIdent for &str {
    fn to_rust_struct(&self) -> Ident {
        crate::sanitize_struct(self)
    }

    fn to_rust_ident(&self) -> Ident {
        crate::sanitize_ident(self)
    }
}
