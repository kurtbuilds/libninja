use quote::ToTokens;
use syn::TypePath;

#[test]
fn test_type_path() {
    let t = ::syn::parse_str::<TypePath>("std::collections::HashMap").unwrap();
    let code = t.to_token_stream().to_string();
    assert_eq!(code, "std :: collections :: HashMap");
}