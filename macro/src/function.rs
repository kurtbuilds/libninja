use syn::parse::Parse;

use syn::{Signature, Visibility};

pub struct FnHeader {
    pub vis: Visibility,
    pub sig: Signature,
}

impl Parse for FnHeader {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(FnHeader {
            vis: input.parse()?,
            sig: input.parse()?,
        })
    }
}
