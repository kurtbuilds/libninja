use proc_macro2::TokenStream;
use ln_macro::rfunction;

use hir::Function;
use quote::quote;

#[test]
fn test_quote_body() {
    let s: Function<TokenStream> = rfunction!(add(a: i32, b: i32) -> i32 {
        println!("Hello, World!")
    });
    assert_eq!(s.name.0, "add");
    assert_eq!(s.body.to_string(), "println ! (\"Hello, World!\")");
    assert_eq!(s.ret.to_string(), "i32");
    assert_eq!(s.args.len(), 2);
    assert_eq!(s.args[0].ty.to_string(), "i32");
    assert_eq!(s.args[1].ty.to_string(), "i32");
}

#[test]
fn test_regression1() {
    let client = quote!(Client);
    let declarations = vec![
        quote!(let a = 1),
        quote!(let b = 2),
        quote!(let c = 3),
    ];
    let operation = quote!(link_token_create);
    let fn_args = vec![
        quote!(a),
        quote!(b),
        quote!(c),
    ];
    let main = rfunction!(main() {
        let client = #client::from_env();
        #(#declarations)*
        let response = client.#operation(#(#fn_args),*)
            .send()
            .await
            .unwrap();
        println!("{:#?}", response);
    });
    assert_eq!(main.body.to_string(), "let client = Client :: from_env () ; let a = 1 let b = 2 let c = 3 let response = client . link_token_create (a , b , c) . send () . await . unwrap () ; println ! (\"{:#?}\" , response) ;");
}